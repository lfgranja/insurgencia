#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Simulação autoritativa e determinística do Núcleo Insurgência.
//!
//! Espelha a máquina de estados de `nucleo/mundo.gd` e a validação de
//! `nucleo/sim/validacao.gd`, sem depender da engine.

use std::collections::{BTreeMap, VecDeque};

use sha2::{Digest, Sha256};
use thiserror::Error;

/// Passo fixo da simulação, em segundos (`1 / 30`).
pub const PASSO_SEGUNDOS: f64 = 1.0 / 30.0;

/// Distância máxima de um comando `mover`, em metros.
const LIMITE_MOVIMENTO_M: f64 = 1.0;

/// Ações previstas no contrato (`ACOES_PERMITIDAS`).
const ACOES_PERMITIDAS: &[&str] = &[
    "mover",
    "sinalizar_sincronia",
    "sinalizar_alavanca",
    "coletar_materiais",
    "construir_barreira",
    "atacar_forte",
];

/// Estado de um jogador na simulação.
#[derive(Clone, Copy, Debug)]
pub struct Jogador {
    /// Posição no plano `(x, y)`.
    pub posicao: (f64, f64),
    /// Pontos de vida.
    pub vida: i64,
    /// Último sequencial aceito.
    pub seq: i64,
    /// Último identificador de requisição aceito.
    pub request_id: i64,
}

/// Comando enfileirado por um par para o próximo tick.
#[derive(Clone, Debug)]
pub struct Comando {
    /// Identificador do par que enviou o comando.
    pub peer_id: u64,
    /// Ação solicitada.
    pub acao: String,
    /// Sequencial monotônico do comando.
    pub seq: i64,
    /// Identificador de requisição monotônico do comando.
    pub request_id: i64,
    /// Destino opcional, usado por `mover`.
    pub destino: Option<(f64, f64)>,
}

/// Mundo autoritativo com passo fixo de 30 Hz.
#[derive(Debug)]
pub struct Mundo {
    /// Tick corrente.
    pub tick: i64,
    /// Epoch monotônico gerado pelo servidor.
    pub epoch: i64,
    /// Identificador da partida.
    pub match_id: i64,
    /// Jogadores indexados por `peer_id`.
    pub jogadores: BTreeMap<u64, Jogador>,
    /// Comandos aguardando o próximo tick.
    pub comandos_pendentes: VecDeque<Comando>,
    /// Comandos selecionados para o tick corrente.
    pub comandos_do_tick: Vec<Comando>,
}

impl Default for Mundo {
    fn default() -> Self {
        Self::new()
    }
}

impl Mundo {
    /// Cria um mundo zerado, com `epoch` e `match_id` iguais a um.
    #[must_use]
    pub fn new() -> Self {
        Self {
            tick: 0,
            epoch: 1,
            match_id: 1,
            jogadores: BTreeMap::new(),
            comandos_pendentes: VecDeque::new(),
            comandos_do_tick: Vec::new(),
        }
    }

    /// Adiciona (ou reinicia) um jogador na origem.
    pub fn adicionar_jogador(&mut self, peer_id: u64) {
        self.jogadores.insert(
            peer_id,
            Jogador {
                posicao: (0.0, 0.0),
                vida: 100,
                seq: 0,
                request_id: 0,
            },
        );
    }

    /// Remove um jogador pelo `peer_id`.
    pub fn remover_jogador(&mut self, peer_id: u64) {
        self.jogadores.remove(&peer_id);
    }

    /// Enfileira um comando para o próximo tick.
    pub fn agendar_comando(&mut self, comando: Comando) {
        self.comandos_pendentes.push_back(comando);
    }

    /// Avança um tick nas fases `prepare`, `skills` e `finish`.
    pub fn avancar_tick(&mut self) {
        self.fase_prepare();
        self.fase_skills();
        self.fase_finish();
        self.tick += 1;
    }

    /// Seleciona os comandos pendentes e esvazia a fila de entrada.
    fn fase_prepare(&mut self) {
        self.comandos_do_tick = self.comandos_pendentes.drain(..).collect();
    }

    /// Aplica os comandos selecionados ao estado dos jogadores.
    fn fase_skills(&mut self) {
        for comando in &self.comandos_do_tick {
            if let Some(jogador) = self.jogadores.get_mut(&comando.peer_id) {
                if comando.acao == "mover" {
                    if let Some(destino) = comando.destino {
                        jogador.posicao = destino;
                    }
                }
                jogador.seq = comando.seq;
                jogador.request_id = comando.request_id;
            }
        }
    }

    /// Descarta os comandos já aplicados.
    fn fase_finish(&mut self) {
        self.comandos_do_tick.clear();
    }

    /// Calcula o hash SHA-256 canônico do estado corrente.
    #[must_use]
    pub fn hash_estado(&self) -> String {
        let jogadores: String = self
            .jogadores
            .iter()
            .map(|(&peer_id, jogador)| {
                let (x, y) = jogador.posicao;
                format!(
                    "{peer_id}:{x}:{y}:{vida}:{seq}:{request_id}",
                    vida = jogador.vida,
                    seq = jogador.seq,
                    request_id = jogador.request_id,
                )
            })
            .collect::<Vec<_>>()
            .join(";");
        let entrada = format!(
            "{tick}|{epoch}|{match_id}|{jogadores}",
            tick = self.tick,
            epoch = self.epoch,
            match_id = self.match_id,
        );
        let resumo = Sha256::digest(entrada.as_bytes());
        format!("{resumo:x}")
    }
}

/// Motivos pelos quais um comando é recusado.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum CodigoRecusa {
    /// O `peer_id` não está presente no mundo.
    #[error("peer nao autorizado")]
    PeerNaoAutorizado,
    /// `seq` ou `request_id` não são monotônicos.
    #[error("seq ou request_id em replay")]
    SeqOuRequestReplay,
    /// A ação não pertence ao contrato.
    #[error("acao desconhecida")]
    AcaoDesconhecida,
    /// `mover` sem destino informado.
    #[error("destino ausente para mover")]
    DestinoInvalido,
    /// Deslocamento acima de `LIMITE_MOVIMENTO_M`.
    #[error("movimento fora do limite")]
    MovimentoForaDoLimite,
}

/// Validação isolada de comandos, espelhando `ValidacaoComandos` do contrato.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValidacaoComandos;

impl ValidacaoComandos {
    /// Valida um comando contra o estado autoritativo.
    ///
    /// # Errors
    ///
    /// Retorna o primeiro [`CodigoRecusa`] aplicável, na ordem do contrato:
    /// par, replay, ação e, para `mover`, destino e limite de distância.
    pub fn validar(
        mundo: &Mundo,
        peer_id: u64,
        acao: &str,
        seq: i64,
        request_id: i64,
        destino: Option<(f64, f64)>,
    ) -> Result<(), CodigoRecusa> {
        let jogador = mundo
            .jogadores
            .get(&peer_id)
            .ok_or(CodigoRecusa::PeerNaoAutorizado)?;

        if seq <= jogador.seq || request_id <= jogador.request_id {
            return Err(CodigoRecusa::SeqOuRequestReplay);
        }

        if !ACOES_PERMITIDAS.contains(&acao) {
            return Err(CodigoRecusa::AcaoDesconhecida);
        }

        if acao == "mover" {
            let (x, y) = destino.ok_or(CodigoRecusa::DestinoInvalido)?;
            let (origem_x, origem_y) = jogador.posicao;
            if (x - origem_x).hypot(y - origem_y) > LIMITE_MOVIMENTO_M {
                return Err(CodigoRecusa::MovimentoForaDoLimite);
            }
        }

        Ok(())
    }
}
