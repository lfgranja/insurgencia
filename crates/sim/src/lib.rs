#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Simulação autoritativa e determinística do Núcleo Insurgência.
//!
//! Espelha a máquina de estados de `nucleo/mundo.gd` e a validação de
//! `nucleo/sim/validacao.gd`, sem depender da engine.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use sha2::{Digest, Sha256};
use thiserror::Error;

/// Passo fixo da simulação, em segundos (`1 / 30`).
pub const PASSO_SEGUNDOS: f64 = 1.0 / 30.0;

/// Ticks necessários para concluir uma coleta de materiais (6 s a 30 Hz).
pub const TICKS_COLETA: i64 = 180;

/// Custo em materiais para construir uma barricada.
pub const CUSTO_BARREIRA_MATERIAIS: i64 = 20;

/// Ticks de canalização necessários para concluir uma barricada (1,5 s a 30 Hz).
pub const TICKS_CONSTRUCAO_BARREIRA: i64 = 45;

/// Pontos de vida de uma barricada recém-construída.
pub const PV_BARREIRA: i64 = 200;

/// Duração de uma barricada em ticks (12 s a 30 Hz).
pub const TICKS_DURACAO_BARREIRA: i64 = 360;

/// Custo em materiais para construir uma ponte provisória.
pub const CUSTO_PONTE_MATERIAIS: i64 = 15;

/// Ticks de canalização necessários para concluir uma ponte provisória (1,5 s a 30 Hz).
pub const TICKS_CONSTRUCAO_PONTE: i64 = 45;

/// Pontos de vida de uma ponte recém-construída.
pub const PV_PONTE: i64 = 60;

/// Duração de uma ponte em ticks (45 s a 30 Hz).
pub const TICKS_DURACAO_PONTE: i64 = 1350;

/// Ticks de defesa ininterrupta necessários para um ponto de autogestão (10 s a 30 Hz).
pub const TICKS_AUTOGESTAO_POR_PONTO: i64 = 300;

/// Pontos de autogestão creditados por uma defesa completa de barricada.
pub const PONTOS_AUTOGESTAO_POR_DEFESA: i64 = 1;

/// Número máximo de barricadas ativas por jogador.
pub const MAX_BARREIRAS_POR_JOGADOR: usize = 2;

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
    "construir_ponte",
];

/// Compara dois pontos sem depender de tolerância de arredondamento.
fn ponto_igual(a: (f64, f64), b: (f64, f64)) -> bool {
    a.0.total_cmp(&b.0).is_eq() && a.1.total_cmp(&b.1).is_eq()
}

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
    /// Materiais acumulados.
    pub materiais: i64,
    /// Suprimentos acumulados.
    pub suprimentos: i64,
    /// Ticks de coleta acumulados no ponto corrente.
    pub coleta_ticks: i64,
    /// Ponto fixado pela coleta corrente.
    pub coleta_ponto: Option<(f64, f64)>,
    /// Ticks de construção acumulados no ponto corrente.
    pub construcao_ticks: i64,
    /// Ponto fixado pela construção corrente.
    pub construcao_ponto: Option<(f64, f64)>,
    /// Pontos de autogestão acumulados por defesas de barricada.
    pub pontos_autogestao: i64,
    /// Ticks de construção de ponte acumulados no ponto corrente.
    pub ponte_ticks: i64,
    /// Ponto fixado pela construção de ponte corrente.
    pub ponte_ponto: Option<(f64, f64)>,
}

/// Barricada ativa no mundo, identificada por `id`.
#[derive(Clone, Debug)]
pub struct Barreira {
    /// Identificador monotônico da barricada.
    pub id: u64,
    /// `peer_id` do jogador que construiu a barricada.
    pub dono: u64,
    /// Posição no plano `(x, y)`.
    pub posicao: (f64, f64),
    /// Pontos de vida restantes; zero ou menos destrói a barricada.
    pub pv: i64,
    /// Ticks restantes de duração; zero expira a barricada.
    pub ticks_restantes: i64,
    /// Ticks de defesa contínua acumulados no ponto da barricada.
    pub defesa_ticks: i64,
}

/// Ponte provisória ativa no mundo, identificada por `id`.
#[derive(Clone, Debug)]
pub struct Ponte {
    /// Identificador monotônico da ponte.
    pub id: u64,
    /// `peer_id` do jogador que construiu a ponte.
    pub dono: u64,
    /// Posição no plano `(x, y)`.
    pub posicao: (f64, f64),
    /// Pontos de vida restantes; zero ou menos destrói a ponte.
    pub pv: i64,
    /// Ticks restantes de duração; zero expira a ponte.
    pub ticks_restantes: i64,
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
    /// Destino opcional, usado por `mover`, `coletar_materiais`,
    /// `construir_barreira` e `construir_ponte`.
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
    /// Barricadas ativas indexadas por `id`.
    pub barreiras: BTreeMap<u64, Barreira>,
    /// Próximo identificador de barricada, começando em um.
    pub proxima_barreira_id: u64,
    /// Pontes provisórias ativas indexadas por `id`.
    pub pontes: BTreeMap<u64, Ponte>,
    /// Próximo identificador de ponte, começando em um.
    pub proxima_ponte_id: u64,
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
            barreiras: BTreeMap::new(),
            proxima_barreira_id: 1,
            pontes: BTreeMap::new(),
            proxima_ponte_id: 1,
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
                materiais: 0,
                suprimentos: 0,
                coleta_ticks: 0,
                coleta_ponto: None,
                construcao_ticks: 0,
                construcao_ponto: None,
                pontos_autogestao: 0,
                ponte_ticks: 0,
                ponte_ponto: None,
            },
        );
    }

    /// Remove um jogador pelo `peer_id`.
    pub fn remover_jogador(&mut self, peer_id: u64) {
        self.jogadores.remove(&peer_id);
    }

    /// Aplica dano a uma barricada, removendo-a se os PV chegam a zero ou menos.
    ///
    /// Retorna `true` quando a barricada foi removida e `false` quando o `id`
    /// não existe ou a barricada permanece viva.
    pub fn aplicar_dano_a_barreira(&mut self, id: u64, dano: i64) -> bool {
        if dano <= 0 {
            return false;
        }
        let mut destruida = false;
        if let Some(barreira) = self.barreiras.get_mut(&id) {
            barreira.pv = barreira.pv.saturating_sub(dano);
            destruida = barreira.pv <= 0;
        }
        if destruida {
            self.barreiras.remove(&id);
        }
        destruida
    }

    /// Aplica dano a uma ponte, removendo-a se os PV chegam a zero ou menos.
    ///
    /// Retorna `true` quando a ponte foi removida e `false` quando o `id`
    /// não existe ou a ponte permanece viva.
    pub fn aplicar_dano_a_ponte(&mut self, id: u64, dano: i64) -> bool {
        if dano <= 0 {
            return false;
        }
        let mut destruida = false;
        if let Some(ponte) = self.pontes.get_mut(&id) {
            ponte.pv = ponte.pv.saturating_sub(dano);
            destruida = ponte.pv <= 0;
        }
        if destruida {
            self.pontes.remove(&id);
        }
        destruida
    }

    /// Enfileira um comando para o próximo tick.
    pub fn agendar_comando(&mut self, comando: Comando) {
        self.comandos_pendentes.push_back(comando);
    }

    /// Avança um tick nas fases `prepare`, `skills`, `defesa` e `finish`.
    pub fn avancar_tick(&mut self) {
        self.fase_prepare();
        self.fase_skills();
        self.fase_defesa();
        self.fase_finish();
        self.tick += 1;
    }

    /// Seleciona os comandos pendentes e esvazia a fila de entrada.
    fn fase_prepare(&mut self) {
        self.comandos_do_tick = self.comandos_pendentes.drain(..).collect();
    }

    /// Aplica os comandos selecionados ao estado dos jogadores.
    fn fase_skills(&mut self) {
        let mut coleta_avancada: BTreeSet<u64> = BTreeSet::new();
        let mut construcao_avancada: BTreeSet<u64> = BTreeSet::new();
        self.avancar_construcoes_automaticas(&mut construcao_avancada);
        let mut construcao_ponte_avancada: BTreeSet<u64> = BTreeSet::new();
        self.avancar_construcoes_ponte_automaticas(&mut construcao_ponte_avancada);
        let comandos = std::mem::take(&mut self.comandos_do_tick);

        for comando in &comandos {
            if comando.acao == "mover" {
                if let Some(jogador) = self.jogadores.get_mut(&comando.peer_id) {
                    if let Some(destino) = comando.destino {
                        if !ponto_igual(destino, jogador.posicao) {
                            jogador.coleta_ticks = 0;
                            jogador.coleta_ponto = None;
                            jogador.construcao_ticks = 0;
                            jogador.construcao_ponto = None;
                            jogador.ponte_ticks = 0;
                            jogador.ponte_ponto = None;
                            jogador.posicao = destino;
                        }
                    }
                }
            } else if comando.acao == "coletar_materiais" {
                self.aplicar_coleta(comando, &mut coleta_avancada);
            } else if comando.acao == "construir_barreira" {
                self.aplicar_construcao(comando, &mut construcao_avancada);
            } else if comando.acao == "construir_ponte" {
                self.aplicar_construcao_ponte(comando, &mut construcao_ponte_avancada);
            }
            if let Some(jogador) = self.jogadores.get_mut(&comando.peer_id) {
                jogador.seq = comando.seq;
                jogador.request_id = comando.request_id;
            }
        }
        self.avancar_coletas_automaticas(&mut coleta_avancada);
        self.comandos_do_tick = comandos;
    }

    /// Avança a defesa das barricadas intactas, creditando autogestão a cada 300 ticks.
    fn fase_defesa(&mut self) {
        for barreira in self.barreiras.values_mut() {
            if barreira.pv <= 0 || barreira.ticks_restantes <= 0 {
                barreira.defesa_ticks = 0;
                continue;
            }
            // `fase_finish` remove a barricada quando `ticks_restantes` chega a
            // 1 neste tick: não acumular nem creditar defesa que não sobrevive.
            if barreira.ticks_restantes <= 1 {
                barreira.defesa_ticks = 0;
                continue;
            }
            let Some(jogador) = self.jogadores.get_mut(&barreira.dono) else {
                barreira.defesa_ticks = 0;
                continue;
            };
            if !ponto_igual(jogador.posicao, barreira.posicao) {
                barreira.defesa_ticks = 0;
                continue;
            }
            barreira.defesa_ticks += 1;
            if barreira.defesa_ticks >= TICKS_AUTOGESTAO_POR_PONTO {
                jogador.pontos_autogestao = jogador
                    .pontos_autogestao
                    .saturating_add(PONTOS_AUTOGESTAO_POR_DEFESA);
                barreira.defesa_ticks = 0;
            }
        }
    }

    /// Aplica um comando de coleta, no máximo uma vez por jogador a cada tick.
    fn aplicar_coleta(&mut self, comando: &Comando, avancada: &mut BTreeSet<u64>) {
        if avancada.contains(&comando.peer_id) {
            return;
        }
        let Some(ponto) = comando.destino else {
            return;
        };
        let Some(jogador) = self.jogadores.get_mut(&comando.peer_id) else {
            return;
        };
        if !ponto_igual(ponto, jogador.posicao) {
            return;
        }
        jogador.coleta_ticks = match jogador.coleta_ponto {
            Some(anterior) if ponto_igual(anterior, ponto) => jogador.coleta_ticks,
            _ => 0,
        };
        jogador.coleta_ponto = Some(ponto);
        jogador.coleta_ticks += 1;
        avancada.insert(comando.peer_id);
        if jogador.coleta_ticks == TICKS_COLETA {
            self.concluir_coleta(comando.peer_id);
        }
    }

    /// Avança um tick as coletas em curso, sem depender de comandos.
    fn avancar_coletas_automaticas(&mut self, avancada: &mut BTreeSet<u64>) {
        let mut concluidas: Vec<u64> = Vec::new();
        for (&peer_id, jogador) in &mut self.jogadores {
            if avancada.contains(&peer_id) {
                continue;
            }
            let Some(ponto) = jogador.coleta_ponto else {
                continue;
            };
            if !ponto_igual(ponto, jogador.posicao) {
                jogador.coleta_ticks = 0;
                jogador.coleta_ponto = None;
                continue;
            }
            jogador.coleta_ticks += 1;
            avancada.insert(peer_id);
            if jogador.coleta_ticks == TICKS_COLETA {
                concluidas.push(peer_id);
            }
        }
        for peer_id in concluidas {
            self.concluir_coleta(peer_id);
        }
    }

    /// Crédito atômico e limpeza da canalização de coleta concluída.
    fn concluir_coleta(&mut self, peer_id: u64) {
        let Some(jogador) = self.jogadores.get_mut(&peer_id) else {
            return;
        };
        jogador.suprimentos += 3;
        jogador.materiais += 2;
        jogador.coleta_ticks = 0;
        jogador.coleta_ponto = None;
    }

    /// Aplica um comando de construção com guarda de saldo e débito atômico.
    fn aplicar_construcao(&mut self, comando: &Comando, avancada: &mut BTreeSet<u64>) {
        if avancada.contains(&comando.peer_id) {
            return;
        }
        let Some(ponto) = comando.destino else {
            return;
        };
        let Some(jogador) = self.jogadores.get_mut(&comando.peer_id) else {
            return;
        };
        // Comando forjado com saldo insuficiente: não inicia a canalização.
        if jogador.materiais < CUSTO_BARREIRA_MATERIAIS {
            avancada.insert(comando.peer_id);
            return;
        }
        if !ponto_igual(ponto, jogador.posicao) {
            return;
        }
        jogador.construcao_ticks = match jogador.construcao_ponto {
            Some(anterior) if ponto_igual(anterior, ponto) => jogador.construcao_ticks,
            _ => 0,
        };
        jogador.construcao_ponto = Some(ponto);
        jogador.construcao_ticks += 1;
        avancada.insert(comando.peer_id);
        if jogador.construcao_ticks == TICKS_CONSTRUCAO_BARREIRA {
            self.concluir_construcao(comando.peer_id, ponto);
        }
    }

    /// Avança um tick as canalizações em curso, sem depender de comandos.
    fn avancar_construcoes_automaticas(&mut self, avancada: &mut BTreeSet<u64>) {
        let mut concluidas: Vec<(u64, (f64, f64))> = Vec::new();
        for (&peer_id, jogador) in &mut self.jogadores {
            let Some(ponto) = jogador.construcao_ponto else {
                continue;
            };
            if jogador.materiais < CUSTO_BARREIRA_MATERIAIS || !ponto_igual(ponto, jogador.posicao)
            {
                jogador.construcao_ticks = 0;
                jogador.construcao_ponto = None;
                continue;
            }
            jogador.construcao_ticks += 1;
            avancada.insert(peer_id);
            if jogador.construcao_ticks == TICKS_CONSTRUCAO_BARREIRA {
                concluidas.push((peer_id, ponto));
            }
        }
        for (peer_id, ponto) in concluidas {
            self.concluir_construcao(peer_id, ponto);
        }
    }

    /// Débito atômico, criação da barreira e limpeza da canalização.
    fn concluir_construcao(&mut self, peer_id: u64, ponto: (f64, f64)) {
        let Some(jogador) = self.jogadores.get_mut(&peer_id) else {
            return;
        };
        let com_saldo = jogador.materiais >= CUSTO_BARREIRA_MATERIAIS;
        if com_saldo {
            jogador.materiais -= CUSTO_BARREIRA_MATERIAIS;
        }
        jogador.construcao_ticks = 0;
        jogador.construcao_ponto = None;
        if com_saldo {
            self.criar_barreira(peer_id, ponto);
        }
    }

    /// Cria a barricada e remove a mais antiga quando o limite é excedido.
    fn criar_barreira(&mut self, dono: u64, posicao: (f64, f64)) {
        let id = self.proxima_barreira_id;
        self.proxima_barreira_id += 1;
        self.barreiras.insert(
            id,
            Barreira {
                id,
                dono,
                posicao,
                pv: PV_BARREIRA,
                // O tick de criação consome a unidade extra: o valor
                // serializado ao fim dele é `TICKS_DURACAO_BARREIRA`.
                ticks_restantes: TICKS_DURACAO_BARREIRA + 1,
                defesa_ticks: 0,
            },
        );
        if self
            .barreiras
            .values()
            .filter(|barreira| barreira.dono == dono)
            .count()
            > MAX_BARREIRAS_POR_JOGADOR
        {
            let antiga = self
                .barreiras
                .values()
                .filter(|barreira| barreira.dono == dono)
                .map(|barreira| barreira.id)
                .min();
            if let Some(id_antigo) = antiga {
                self.barreiras.remove(&id_antigo);
            }
        }
    }

    /// Aplica um comando de construção de ponte com guarda de saldo e débito atômico.
    fn aplicar_construcao_ponte(&mut self, comando: &Comando, avancada: &mut BTreeSet<u64>) {
        if avancada.contains(&comando.peer_id) {
            return;
        }
        let Some(ponto) = comando.destino else {
            return;
        };
        let Some(jogador) = self.jogadores.get_mut(&comando.peer_id) else {
            return;
        };
        // Comando forjado com saldo insuficiente: não inicia a canalização.
        if jogador.materiais < CUSTO_PONTE_MATERIAIS {
            avancada.insert(comando.peer_id);
            return;
        }
        if !ponto_igual(ponto, jogador.posicao) {
            return;
        }
        jogador.ponte_ticks = match jogador.ponte_ponto {
            Some(anterior) if ponto_igual(anterior, ponto) => jogador.ponte_ticks,
            _ => 0,
        };
        jogador.ponte_ponto = Some(ponto);
        jogador.ponte_ticks += 1;
        avancada.insert(comando.peer_id);
        if jogador.ponte_ticks == TICKS_CONSTRUCAO_PONTE {
            self.concluir_construcao_ponte(comando.peer_id, ponto);
        }
    }

    /// Avança um tick as canalizações de ponte em curso, sem depender de comandos.
    fn avancar_construcoes_ponte_automaticas(&mut self, avancada: &mut BTreeSet<u64>) {
        let mut concluidas: Vec<(u64, (f64, f64))> = Vec::new();
        for (&peer_id, jogador) in &mut self.jogadores {
            let Some(ponto) = jogador.ponte_ponto else {
                continue;
            };
            if jogador.materiais < CUSTO_PONTE_MATERIAIS || !ponto_igual(ponto, jogador.posicao) {
                jogador.ponte_ticks = 0;
                jogador.ponte_ponto = None;
                continue;
            }
            jogador.ponte_ticks += 1;
            avancada.insert(peer_id);
            if jogador.ponte_ticks == TICKS_CONSTRUCAO_PONTE {
                concluidas.push((peer_id, ponto));
            }
        }
        for (peer_id, ponto) in concluidas {
            self.concluir_construcao_ponte(peer_id, ponto);
        }
    }

    /// Débito atômico, criação da ponte e limpeza da canalização.
    fn concluir_construcao_ponte(&mut self, peer_id: u64, ponto: (f64, f64)) {
        let Some(jogador) = self.jogadores.get_mut(&peer_id) else {
            return;
        };
        let com_saldo = jogador.materiais >= CUSTO_PONTE_MATERIAIS;
        if com_saldo {
            jogador.materiais -= CUSTO_PONTE_MATERIAIS;
        }
        jogador.ponte_ticks = 0;
        jogador.ponte_ponto = None;
        if com_saldo {
            self.criar_ponte(peer_id, ponto);
        }
    }

    /// Cria a ponte com PV e duração cheios no tick da conclusão.
    fn criar_ponte(&mut self, dono: u64, posicao: (f64, f64)) {
        let id = self.proxima_ponte_id;
        self.proxima_ponte_id += 1;
        self.pontes.insert(
            id,
            Ponte {
                id,
                dono,
                posicao,
                pv: PV_PONTE,
                // O tick de criação consome a unidade extra: o valor
                // serializado ao fim dele é `TICKS_DURACAO_PONTE`.
                ticks_restantes: TICKS_DURACAO_PONTE + 1,
            },
        );
    }

    /// Descarta os comandos já aplicados e expira barricadas e pontes vencidas.
    fn fase_finish(&mut self) {
        self.comandos_do_tick.clear();
        self.barreiras.retain(|_, barreira| {
            // A unidade extra da criação é consumida no próprio tick de criação;
            // o último tick decrementa de 1 para 0 e remove a barricada.
            if barreira.ticks_restantes > 0 {
                barreira.ticks_restantes -= 1;
            }
            barreira.ticks_restantes > 0
        });
        self.pontes.retain(|_, ponte| {
            if ponte.ticks_restantes > 0 {
                ponte.ticks_restantes -= 1;
            }
            ponte.ticks_restantes > 0
        });
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
                    "{0}:{1}:{2}:{3}:{4}:{5}:{6}:{7}:{8}:{9:?}:{10}:{11:?}:{12}:{13}:{14:?}",
                    peer_id,
                    x,
                    y,
                    jogador.vida,
                    jogador.seq,
                    jogador.request_id,
                    jogador.materiais,
                    jogador.suprimentos,
                    jogador.coleta_ticks,
                    jogador.coleta_ponto,
                    jogador.construcao_ticks,
                    jogador.construcao_ponto,
                    jogador.pontos_autogestao,
                    jogador.ponte_ticks,
                    jogador.ponte_ponto,
                )
            })
            .collect::<Vec<_>>()
            .join(";");
        let barreiras: String = self
            .barreiras
            .values()
            .map(|barreira| {
                let (x, y) = barreira.posicao;
                let id = barreira.id;
                let dono = barreira.dono;
                let pv = barreira.pv;
                let ticks = barreira.ticks_restantes;
                let defesa = barreira.defesa_ticks;
                format!("{id}:{dono}:{x}:{y}:{pv}:{ticks}:{defesa}")
            })
            .collect::<Vec<_>>()
            .join(";");
        let pontes: String = self
            .pontes
            .values()
            .map(|ponte| {
                let (x, y) = ponte.posicao;
                let id = ponte.id;
                let dono = ponte.dono;
                let pv = ponte.pv;
                let ticks = ponte.ticks_restantes;
                format!("{id}:{dono}:{x}:{y}:{pv}:{ticks}")
            })
            .collect::<Vec<_>>()
            .join(";");
        let entrada = format!(
            "{tick}|{epoch}|{match_id}|{jogadores}|{barreiras}|{pontes}",
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
    /// Destino ausente ou fora do ponto para `mover`, `coletar_materiais`,
    /// `construir_barreira` ou `construir_ponte`.
    #[error("destino ausente ou fora do ponto")]
    DestinoInvalido,
    /// Deslocamento acima de `LIMITE_MOVIMENTO_M`.
    #[error("movimento fora do limite")]
    MovimentoForaDoLimite,
    /// Materiais abaixo do custo de uma barricada ou ponte.
    #[error("materiais insuficientes")]
    SaldoInsuficiente,
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
    /// par, replay, ação, destino (incluindo o limite de distância de `mover`)
    /// e saldo para `construir_barreira` e `construir_ponte`.
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

        if acao == "coletar_materiais" {
            let ponto = destino.ok_or(CodigoRecusa::DestinoInvalido)?;
            if !ponto_igual(ponto, jogador.posicao) {
                return Err(CodigoRecusa::DestinoInvalido);
            }
        }

        if acao == "construir_barreira" {
            let ponto = destino.ok_or(CodigoRecusa::DestinoInvalido)?;
            if !ponto_igual(ponto, jogador.posicao) {
                return Err(CodigoRecusa::DestinoInvalido);
            }
            if jogador.materiais < CUSTO_BARREIRA_MATERIAIS {
                return Err(CodigoRecusa::SaldoInsuficiente);
            }
        }

        if acao == "construir_ponte" {
            let ponto = destino.ok_or(CodigoRecusa::DestinoInvalido)?;
            if !ponto_igual(ponto, jogador.posicao) {
                return Err(CodigoRecusa::DestinoInvalido);
            }
            if jogador.materiais < CUSTO_PONTE_MATERIAIS {
                return Err(CodigoRecusa::SaldoInsuficiente);
            }
        }

        Ok(())
    }
}
