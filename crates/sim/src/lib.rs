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

/// Ticks de exposição à Propaganda necessários para converter um Súdito (4 s a 30 Hz).
pub const TICKS_PROPAGANDA: i64 = 120;

/// Ticks de reversão da conversão de um Súdito após dano de jogador (3 s a 30 Hz).
pub const TICKS_REVERSAO_SUDITO: i64 = 90;

/// Ticks contínuos de presença de Súdito convertido para controlar uma ilha
/// (10 s a 30 Hz; derivação de protótipo, não número canônico).
pub const TICKS_CONTROLE_ILHA: i64 = 300;

/// Limite de ticks da partida: 20 min a 30 Hz (`[HIPÓTESE]`/protótipo, não cânone).
pub const TICKS_LIMITE_PARTIDA: i64 = 36_000;

/// Ticks por nível de papel material: 30 s a 30 Hz (`[HIPÓTESE]`/protótipo, não cânone).
pub const TICKS_POR_NIVEL: i64 = 900;

/// Nível máximo do protótipo (`[HIPÓTESE]`/GDD §10.1, não cânone).
pub const NIVEL_MAXIMO: u8 = 12;

/// Número máximo de ilhas distintas perdidas antes da dissolução da Comuna
/// (`[HIPÓTESE]`/protótipo, não cânone).
pub const MAX_RECONQUISTAS: usize = 3;

/// Número de Nós de Controle do protótipo (`[HIPÓTESE]` da GDD §6.2, nunca cânone).
pub const TOTAL_NOS: usize = 3;

/// Resistência Ideológica inicial (`[HIPÓTESE]`/derivação de protótipo, não cânone).
pub const RESISTENCIA_INICIAL: i64 = 100;

/// Resistência removida por cada Nó convertido (`[HIPÓTESE]`/protótipo, não cânone).
pub const CONTRIBUICAO_NO: i64 = 30;

/// Teto de dano por Nó (`[HIPÓTESE]`/protótipo, não cânone).
pub const DANO_MAXIMO_NOS: i64 = 20;

/// Piso da Resistência enquanto restar Nó não convertido (`[HIPÓTESE]`, não cânone).
pub const PISO_DANO_COM_NOS: i64 = 10;

/// Ticks de Propaganda para converter um Nó (20 s × 30 Hz; protótipo, não cânone).
pub const TICKS_PROPAGANDA_NO: i64 = 600;

/// Ticks de inatividade de um Nó após dano (20 s × 30 Hz; protótipo, não cânone).
pub const TICKS_INATIVIDADE_NO: i64 = 600;

/// Pulso final que encerra a Resistência (`[HIPÓTESE]`/protótipo, não cânone).
pub const PULSO_FINAL: i64 = 10;

/// Fôlego máximo por jogador: `[HIPÓTESE]` da GDD §10.1, escolha de protótipo, não cânone.
pub const FOLEGO_MAXIMO: i64 = 100;

/// Ticks por +1 Fôlego fora de combate (10/s a 30 Hz): `[HIPÓTESE]`/protótipo, não cânone.
pub const TICKS_REGENERACAO_FORA_COMBATE: i64 = 3;

/// Ticks por +1 Fôlego em combate (5/s a 30 Hz): `[HIPÓTESE]`/protótipo, não cânone.
pub const TICKS_REGENERACAO_COMBATE: i64 = 6;

/// Custo de Fôlego de `atacar_forte`: `[HIPÓTESE]` de protótipo, não cânone.
pub const CUSTO_ATAQUE_FOLEGO: i64 = 30;

/// Dano de `atacar_forte`: `[HIPÓTESE]` de protótipo, não cânone.
pub const DANO_ATAQUE: i64 = 10;

/// Alcance de contato do ataque, em metros: `[HIPÓTESE]`/protótipo, não cânone.
pub const ALCANCE_CONTATO_M: f64 = 2.0;

/// Duração do estado de combate em ticks (3 s a 30 Hz): `[HIPÓTESE]`/protótipo, não cânone.
pub const TICKS_COMBATE: i64 = 90;

/// Pontos de vida de um PNJ defensor: `[HIPÓTESE]`/protótipo (8 dmg/s por 3 s).
pub const PV_PNJ: i64 = 24;

/// Dano do PNJ ao jogador: `[HIPÓTESE]`/protótipo, nunca cânone.
pub const DANO_ATAQUE_PNJ: i64 = 8;

/// Ticks entre ataques do PNJ (3 s a 30 Hz): `[HIPÓTESE]`/derivação de protótipo.
pub const TICKS_ATAQUE_PNJ: i64 = 90;

/// Alcance do ataque do PNJ, em metros: derivação de `ALCANCE_CONTATO_M`.
pub const ALCANCE_ATAQUE_PNJ_M: f64 = ALCANCE_CONTATO_M;

/// Efetivo padrão de PNJs do protótipo (`[HIPÓTESE]`/GDD §8.1, não cânone).
pub const TOTAL_PNJS_PADRAO: usize = 4;

/// Nome alternativo do custo na spec 047 (`[HIPÓTESE]`/protótipo, não cânone).
pub const CUSTO_ATAQUE_FORTE: i64 = CUSTO_ATAQUE_FOLEGO;

/// Nome alternativo do dano na spec 047 (`[HIPÓTESE]`/protótipo, não cânone).
pub const DANO_ATAQUE_FORTE: i64 = DANO_ATAQUE;

/// Nome alternativo da regeneração fora de combate na spec 047 (`[HIPÓTESE]`, não cânone).
pub const FOLEGO_REGENERACAO_FORA: i64 = 10;

/// Nome alternativo da regeneração em combate na spec 047 (`[HIPÓTESE]`, não cânone).
pub const FOLEGO_REGENERACAO_COMBATE: i64 = 5;

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
    "propaganda",
    "propaganda_no",
    "atacar_no",
    "atacar_pnj",
];

/// Compara dois pontos sem depender de tolerância de arredondamento.
fn ponto_igual(a: (f64, f64), b: (f64, f64)) -> bool {
    a.0.total_cmp(&b.0).is_eq() && a.1.total_cmp(&b.1).is_eq()
}

/// Nível mínimo exigido por cada ação (`[HIPÓTESE]`/protótipo, não cânone).
///
/// Mapa Q1/W2/E3/R6: ações sem slot permanecem livres (nível 1).
fn nivel_minimo_para_acao(acao: &str) -> u8 {
    match acao {
        "construir_barreira" | "construir_ponte" => 2,
        "propaganda" | "propaganda_no" => 3,
        "atacar_no" => 6,
        _ => 1,
    }
}

/// Origem do alvo de Nó fixado no jogador.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AcaoNo {
    /// Propaganda em curso; também o padrão quando o alvo é injetado diretamente.
    Propaganda,
    /// Dano aplicado; sozinho, não inicia Propaganda em um Nó intacto.
    Ataque,
}

/// Papel material do jogador (`[HIPÓTESE]`/GDD §10.1 e §10.8:744-759, protótipo).
///
/// São rótulos de papéis materiais, não classes mágicas; nenhuma variante altera
/// dano, custo, alcance, recarga, Fôlego ou desbloqueio neste ciclo.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Funcao {
    /// Artesão: papel fixo atribuído na criação do jogador.
    Artesao,
    /// Guerreiro: rótulo material de combate.
    Guerreiro,
    /// Intelectual: rótulo material de Propaganda.
    Intelectual,
    /// Camponês: rótulo material de coleta e construção.
    Campones,
    /// Agitador: rótulo material de mobilização.
    Agitador,
}

impl Funcao {
    /// Nome acentuado do papel para exibição (`[HIPÓTESE]`/protótipo, não cânone).
    #[must_use]
    pub fn nome(self) -> &'static str {
        match self {
            Self::Artesao => "Artesão",
            Self::Guerreiro => "Guerreiro",
            Self::Intelectual => "Intelectual",
            Self::Campones => "Camponês",
            Self::Agitador => "Agitador",
        }
    }
}

impl std::fmt::Display for Funcao {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.nome())
    }
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
    /// Papel material fixo na criação (`[HIPÓTESE]`/protótipo, não cânone).
    pub funcao: Funcao,
    /// Nível corrente, de 1 a `NIVEL_MAXIMO` (`[HIPÓTESE]`/protótipo, não cânone).
    pub nivel: u8,
    /// Ticks acumulados para o próximo nível (`[HIPÓTESE]`/protótipo, não cânone).
    pub nivel_ticks: i64,
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
    /// Súdito alvejado pela Propaganda em curso.
    pub propaganda_alvo: Option<u64>,
    /// Nó de Controle alvejado por `propaganda_no` ou `atacar_no`.
    pub no_alvo: Option<u64>,
    no_acao_no: AcaoNo,
    /// Fôlego corrente, limitado a `FOLEGO_MAXIMO` (`[HIPÓTESE]`/protótipo).
    pub folego: i64,
    /// Ticks acumulados para a próxima regeneração de Fôlego (`[HIPÓTESE]`/protótipo).
    pub folego_ticks: i64,
    /// Ticks restantes de combate; a taxa de regeneração muda enquanto for positivo.
    pub combate_ticks: i64,
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

/// Súdito inimigo cuja conversão é exercitada pela Propaganda.
#[derive(Clone, Copy, Debug)]
pub struct Sudito {
    /// Identificador determinístico fornecido na injeção.
    pub id: u64,
    /// Posição no plano `(x, y)`.
    pub posicao: (f64, f64),
    /// `true` ao completar a exposição canônica: deixa de atacar e panfleteia.
    pub convertido: bool,
    /// Ticks de exposição acumulados no Súdito.
    pub propaganda_ticks: i64,
    /// Ticks restantes de reversão após dano de jogador.
    pub reversao_ticks: i64,
}

/// Ilha cujo controle depende da presença contínua de Súdito da Comuna.
///
/// Sem mapa no protótipo, a presença de qualquer Súdito convertido é global.
#[derive(Clone, Copy, Debug)]
pub struct Ilha {
    /// Identificador determinístico da ilha.
    pub id: u64,
    /// `true` após `TICKS_CONTROLE_ILHA` ticks contínuos de presença.
    pub controlada: bool,
    /// Ticks contínuos de presença acumulados, saturando em `TICKS_CONTROLE_ILHA`.
    pub controle_ticks: i64,
}

/// Nó de Controle cuja conversão depende de Propaganda canalizada.
///
/// Todos os números e a posição fixa são derivações de protótipo (`[HIPÓTESE]`), não cânone.
#[derive(Clone, Copy, Debug)]
pub struct NoControle {
    /// Identificador determinístico do Nó.
    pub id: u64,
    /// Posição fixa no plano `(x, y)`; simplificação de mapa do protótipo.
    pub posicao: (f64, f64),
    /// `true` ao completar `TICKS_PROPAGANDA_NO` ticks de Propaganda.
    pub convertido: bool,
    /// Ticks de Propaganda acumulados, saturando em `TICKS_PROPAGANDA_NO`.
    pub propaganda_ticks: i64,
    /// Ticks restantes de inatividade após dano (`[HIPÓTESE]`/protótipo).
    pub inativo_ticks: i64,
}

/// Defensor PNJ na simulação, com cadência de ataque própria.
///
/// Todos os números associados são `[HIPÓTESE]`/derivação de protótipo, não cânone.
#[derive(Clone, Copy, Debug)]
pub struct Pnj {
    /// Identificador determinístico fornecido na injeção.
    pub id: u64,
    /// Posição no plano `(x, y)`.
    pub posicao: (f64, f64),
    /// Pontos de vida restantes; zero inativa o PNJ sem remover a entrada.
    pub pv: i64,
    /// `false` quando os PV chegam a zero; PNJ inativo não ataca nem é alvo.
    pub ativo: bool,
    /// Ticks acumulados até o próximo ataque, em `TICKS_ATAQUE_PNJ`.
    pub ataque_ticks: i64,
}

/// Desfecho observável da partida (`[HIPÓTESE]`/protótipo, não cânone).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResultadoPartida {
    /// Autogestão estabelecida pela conversão dos Nós e pelo pulso final.
    Vitoria,
    /// Comuna dissolvida ao atingir o limite de ticks da partida.
    DerrotaPorTempo,
    /// Comuna dissolvida após o limite de ilhas distintas perdidas.
    DerrotaPorReconquista,
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
    /// `construir_barreira`, `construir_ponte` e `propaganda`.
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
    /// Súditos indexados por `id`.
    pub suditos: BTreeMap<u64, Sudito>,
    /// PNJs defensores indexados por `id`, em ordem determinística.
    pub pnjs: BTreeMap<u64, Pnj>,
    /// Ilhas indexadas por `id`, em ordem determinística.
    pub ilhas: BTreeMap<u64, Ilha>,
    /// Ids das ilhas distintas que emitiram `ilha_perdida`, em ordem crescente.
    pub ilhas_reconquistadas: BTreeSet<u64>,
    /// Nós de Controle indexados por `id`, em ordem determinística.
    pub nos: BTreeMap<u64, NoControle>,
    /// Resistência Ideológica corrente (`[HIPÓTESE]`/protótipo, não cânone).
    pub resistencia_ideologica: i64,
    /// Desfecho da partida; `None` até o pulso final de vitória.
    pub resultado: Option<ResultadoPartida>,
    /// Fila de sinais pendentes em ordem FIFO; não é um histórico completo
    /// dos sinais emitidos.
    pub sinais: VecDeque<String>,
    /// Última recusa aplicada pelo gate de nível no núcleo; não é limpa por
    /// comandos aceitos (`[HIPÓTESE]`/protótipo, não cânone).
    pub ultima_recusa: Option<CodigoRecusa>,
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
            suditos: BTreeMap::new(),
            pnjs: BTreeMap::new(),
            ilhas: BTreeMap::new(),
            ilhas_reconquistadas: BTreeSet::new(),
            sinais: VecDeque::new(),
            ultima_recusa: None,
            nos: BTreeMap::new(),
            resistencia_ideologica: RESISTENCIA_INICIAL,
            resultado: None,
        }
    }

    /// Adiciona (ou reinicia) um jogador na origem.
    pub fn adicionar_jogador(&mut self, peer_id: u64) {
        self.jogadores.insert(
            peer_id,
            Jogador {
                posicao: (0.0, 0.0),
                vida: 100,
                folego: FOLEGO_MAXIMO,
                folego_ticks: 0,
                combate_ticks: 0,
                seq: 0,
                request_id: 0,
                funcao: Funcao::Artesao,
                nivel: 1,
                nivel_ticks: 0,
                materiais: 0,
                suprimentos: 0,
                coleta_ticks: 0,
                coleta_ponto: None,
                construcao_ticks: 0,
                construcao_ponto: None,
                pontos_autogestao: 0,
                ponte_ticks: 0,
                ponte_ponto: None,
                propaganda_alvo: None,
                no_alvo: None,
                no_acao_no: AcaoNo::Propaganda,
            },
        );
    }

    /// Remove um jogador pelo `peer_id`.
    pub fn remover_jogador(&mut self, peer_id: u64) {
        self.jogadores.remove(&peer_id);
    }

    /// Injeta (ou reinicia) um Súdito com `id` e `posicao` determinísticos.
    pub fn adicionar_sudito(&mut self, id: u64, posicao: (f64, f64)) {
        self.suditos.insert(
            id,
            Sudito {
                id,
                posicao,
                convertido: false,
                propaganda_ticks: 0,
                reversao_ticks: 0,
            },
        );
    }

    /// Injeta (ou reinicia) um PNJ com `id` e `posicao` determinísticos.
    pub fn adicionar_pnj(&mut self, id: u64, posicao: (f64, f64)) {
        self.pnjs.insert(
            id,
            Pnj {
                id,
                posicao,
                pv: PV_PNJ,
                ativo: true,
                ataque_ticks: 0,
            },
        );
    }

    /// Aplica dano de jogador a um PNJ, inativando-o ao zerar os PV.
    ///
    /// Dano não positivo ou `id` inexistente não altera o estado. A entrada
    /// permanece no mapa após a morte; o retorno é `true` somente na transição
    /// para inativo nesta chamada.
    pub fn aplicar_dano_a_pnj(&mut self, id: u64, dano: i64) -> bool {
        if dano <= 0 {
            return false;
        }
        let Some(pnj) = self.pnjs.get_mut(&id) else {
            return false;
        };
        pnj.pv = pnj.pv.saturating_sub(dano).max(0);
        if pnj.pv == 0 && pnj.ativo {
            pnj.ativo = false;
            return true;
        }
        false
    }

    /// Insere (ou reinicia) uma ilha com progresso de controle zerado.
    pub fn adicionar_ilha(&mut self, id: u64) {
        self.ilhas.insert(
            id,
            Ilha {
                id,
                controlada: false,
                controle_ticks: 0,
            },
        );
    }

    /// Insere (ou reinicia) um Nó com progresso zerado na posição fixa informada.
    pub fn adicionar_no(&mut self, id: u64, posicao: (f64, f64)) {
        self.nos.insert(
            id,
            NoControle {
                id,
                posicao,
                convertido: false,
                propaganda_ticks: 0,
                inativo_ticks: 0,
            },
        );
    }

    /// Aplica dano de jogador a um Súdito, iniciando ou reiniciando a reversão.
    ///
    /// Dano não positivo ou `id` inexistente não altera o estado. Cada dano
    /// reinicia a contagem de `TICKS_REVERSAO_SUDITO`.
    pub fn aplicar_dano_a_sudito(&mut self, id: u64, dano: i64) {
        if dano <= 0 {
            return;
        }
        let Some(sudito) = self.suditos.get_mut(&id) else {
            return;
        };
        sudito.reversao_ticks = TICKS_REVERSAO_SUDITO;
    }

    /// Decrementa em um tick as reversões em curso, em ordem estável de `id`.
    fn avancar_reversoes(&mut self) {
        let em_reversao: Vec<u64> = self
            .suditos
            .iter()
            .filter(|(_, sudito)| sudito.reversao_ticks > 0)
            .map(|(&id, _)| id)
            .collect();
        for id in em_reversao {
            self.avancar_reversao(id);
        }
    }

    /// Avança a reversão de um Súdito, concluindo-a quando a contagem zera.
    fn avancar_reversao(&mut self, id: u64) {
        let Some(sudito) = self.suditos.get_mut(&id) else {
            return;
        };
        sudito.reversao_ticks -= 1;
        if sudito.reversao_ticks == 0 && sudito.convertido {
            sudito.convertido = false;
            sudito.propaganda_ticks = 0;
        }
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

    /// Avança um tick nas fases `prepare`, `skills`, `defesa`, `ataque_pnjs`,
    /// `controle_ilhas`, `vitoria` e `finish`; a derrota é avaliada após o incremento.
    pub fn avancar_tick(&mut self) {
        let convertidos_no_inicio = self.nos.values().filter(|no| no.convertido).count();
        self.fase_prepare();
        self.fase_skills();
        self.fase_defesa();
        self.fase_ataque_pnjs();
        self.fase_controle_ilhas();
        self.fase_vitoria(convertidos_no_inicio);
        self.fase_finish();
        self.avancar_reversoes();
        self.tick += 1;
        self.fase_derrota();
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
        let mut propaganda_avancada: BTreeSet<u64> = BTreeSet::new();
        self.avancar_propagandas_automaticas(&mut propaganda_avancada);
        let mut propaganda_no_avancada: BTreeSet<u64> = BTreeSet::new();
        self.avancar_propagandas_no_automaticas(&mut propaganda_no_avancada);
        let mut ataque_no_avancado: BTreeSet<u64> = BTreeSet::new();
        let mut atacantes_pnj_avancado: BTreeSet<u64> = BTreeSet::new();
        let comandos = std::mem::take(&mut self.comandos_do_tick);
        let mut ataques_realizados: BTreeSet<u64> = BTreeSet::new();

        for comando in &comandos {
            let mut atualizar_monotonicos = true;
            // O gate de nível também vale no núcleo: a validação isolada não
            // impede um chamador de enfileirar diretamente um comando acima do
            // nível do jogador. O comando recusado não executa e não atualiza
            // `seq`/`request_id`, portanto não é considerado aceito.
            let Some(jogador) = self.jogadores.get(&comando.peer_id) else {
                continue;
            };
            if jogador.nivel < nivel_minimo_para_acao(&comando.acao) {
                self.ultima_recusa = Some(CodigoRecusa::AcaoBloqueada);
                continue;
            }
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
                            if let Some(id) = jogador.propaganda_alvo.take() {
                                if let Some(sudito) = self.suditos.get_mut(&id) {
                                    sudito.propaganda_ticks = 0;
                                }
                            }
                            jogador.no_alvo = None;
                            jogador.posicao = destino;
                        }
                    }
                }
            } else if comando.acao == "atacar_forte" {
                self.aplicar_ataque(comando, &mut ataques_realizados);
            } else if comando.acao == "coletar_materiais" {
                self.aplicar_coleta(comando, &mut coleta_avancada);
            } else if comando.acao == "construir_barreira" {
                self.aplicar_construcao(comando, &mut construcao_avancada);
            } else if comando.acao == "construir_ponte" {
                self.aplicar_construcao_ponte(comando, &mut construcao_ponte_avancada);
            } else if comando.acao == "propaganda" {
                self.aplicar_propaganda(comando, &mut propaganda_avancada);
            } else if comando.acao == "propaganda_no" {
                self.aplicar_propaganda_no(comando, &mut propaganda_no_avancada);
            } else if comando.acao == "atacar_no" {
                self.aplicar_atacar_no(comando, &mut ataque_no_avancado);
            } else if comando.acao == "atacar_pnj" {
                atualizar_monotonicos =
                    self.aplicar_ataque_pnj(comando, &mut atacantes_pnj_avancado);
            }
            if atualizar_monotonicos {
                if let Some(jogador) = self.jogadores.get_mut(&comando.peer_id) {
                    jogador.seq = comando.seq;
                    jogador.request_id = comando.request_id;
                }
            }
        }
        self.avancar_coletas_automaticas(&mut coleta_avancada);
        self.comandos_do_tick = comandos;
    }

    /// Aplica no máximo um `atacar_forte` por atacante a cada tick.
    fn aplicar_ataque(&mut self, comando: &Comando, ataques: &mut BTreeSet<u64>) {
        let Some(destino) = comando.destino else {
            return;
        };
        let Some((origem, folego)) = self
            .jogadores
            .get(&comando.peer_id)
            .map(|jogador| (jogador.posicao, jogador.folego))
        else {
            return;
        };
        let alvo = self
            .jogadores
            .iter()
            .find(|(&peer_id, jogador)| {
                peer_id != comando.peer_id && ponto_igual(jogador.posicao, destino)
            })
            .map(|(&peer_id, _)| peer_id);
        let Some(alvo) = alvo else {
            return;
        };
        if (destino.0 - origem.0).hypot(destino.1 - origem.1) > ALCANCE_CONTATO_M {
            return;
        }
        if folego < CUSTO_ATAQUE_FOLEGO {
            return;
        }
        // A cota só é consumida por um ataque válido: um comando recusado não pode
        // suprimir um ataque válido do mesmo atacante neste tick.
        if !ataques.insert(comando.peer_id) {
            return;
        }
        if let Some(atacante) = self.jogadores.get_mut(&comando.peer_id) {
            atacante.folego = atacante.folego.saturating_sub(CUSTO_ATAQUE_FOLEGO);
            // A unidade extra é consumida por `fase_finish` no próprio tick do ataque.
            atacante.combate_ticks = TICKS_COMBATE + 1;
        }
        if let Some(vitima) = self.jogadores.get_mut(&alvo) {
            // FR-001/SC-001 da spec 047: vida nunca negativa, mesmo partindo de valor < 0.
            vitima.vida = vitima.vida.saturating_sub(DANO_ATAQUE).max(0);
            vitima.combate_ticks = TICKS_COMBATE + 1;
        }
    }

    /// Aplica no máximo um `atacar_pnj` por atacante a cada tick.
    ///
    /// O `destino` identifica o PNJ ativo; o atacante precisa estar a até
    /// `ALCANCE_CONTATO_M` (2 m) desse ponto. Retorna `true` somente quando o
    /// dano foi aplicado, para que os monotônicos avancem só com comandos aceitos.
    fn aplicar_ataque_pnj(&mut self, comando: &Comando, atacantes: &mut BTreeSet<u64>) -> bool {
        let Some(destino) = comando.destino else {
            return false;
        };
        let Some(jogador) = self.jogadores.get(&comando.peer_id) else {
            return false;
        };
        if (destino.0 - jogador.posicao.0).hypot(destino.1 - jogador.posicao.1) > ALCANCE_CONTATO_M
        {
            return false;
        }
        let Some(id) = self.pnj_id_ativo_em_ponto(destino) else {
            return false;
        };
        if !atacantes.insert(comando.peer_id) {
            return false;
        }
        self.aplicar_dano_a_pnj(id, DANO_ATAQUE);
        true
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

    /// Avança a cadência dos PNJs ativos e resolve seus ataques em ordem de `id`.
    ///
    /// Ao atingir `TICKS_ATAQUE_PNJ`, o PNJ atinge o primeiro jogador em ordem de
    /// `peer_id` dentro de `ALCANCE_ATAQUE_PNJ_M` (2 m) e zera o contador. Sem
    /// alvo em alcance, o contador também zera, sem dano.
    fn fase_ataque_pnjs(&mut self) {
        let ids: Vec<u64> = self
            .pnjs
            .values()
            .filter(|pnj| pnj.ativo)
            .map(|pnj| pnj.id)
            .collect();
        for id in ids {
            let Some(pnj) = self.pnjs.get_mut(&id) else {
                continue;
            };
            if !pnj.ativo {
                continue;
            }
            pnj.ataque_ticks += 1;
            if pnj.ataque_ticks < TICKS_ATAQUE_PNJ {
                continue;
            }
            pnj.ataque_ticks = 0;
            let origem = pnj.posicao;
            let alvo = self
                .jogadores
                .iter()
                .filter(|(_, jogador)| {
                    (jogador.posicao.0 - origem.0).hypot(jogador.posicao.1 - origem.1)
                        <= ALCANCE_ATAQUE_PNJ_M
                })
                .map(|(&peer_id, _)| peer_id)
                .min();
            if let Some(peer_id) = alvo {
                if let Some(jogador) = self.jogadores.get_mut(&peer_id) {
                    jogador.vida = jogador.vida.saturating_sub(DANO_ATAQUE_PNJ).max(0);
                }
            }
        }
    }

    /// Avança o controle das ilhas pela presença de Súdito convertido.
    ///
    /// Protótipo sem mapa: qualquer Súdito com `convertido == true` conta como
    /// presença global em todas as ilhas. A contagem é contínua: um tick sem
    /// presença zera o progresso e sinaliza a perda das ilhas afetadas.
    fn fase_controle_ilhas(&mut self) {
        let presenca = self.suditos.values().any(|sudito| sudito.convertido);
        let Self {
            ilhas,
            ilhas_reconquistadas,
            sinais,
            ..
        } = &mut *self;
        for (id, ilha) in ilhas.iter_mut() {
            if presenca {
                if ilha.controle_ticks < TICKS_CONTROLE_ILHA {
                    ilha.controle_ticks += 1;
                }
                if ilha.controle_ticks == TICKS_CONTROLE_ILHA && !ilha.controlada {
                    ilha.controlada = true;
                    sinais.push_back(format!("ilha_convertida:{id}"));
                }
            } else {
                if ilha.controlada || ilha.controle_ticks > 0 {
                    sinais.push_back(format!("ilha_perdida:{id}"));
                    ilhas_reconquistadas.insert(*id);
                }
                ilha.controlada = false;
                ilha.controle_ticks = 0;
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
        if !ponto_igual(ponto, jogador.posicao) {
            return;
        }
        // Comando forjado com saldo insuficiente: não inicia a canalização.
        if jogador.materiais < CUSTO_PONTE_MATERIAIS {
            avancada.insert(comando.peer_id);
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

    /// Encontra o Súdito em `ponto`, preferindo o menor `id` em caso de empate.
    fn sudito_id_em_ponto(&self, ponto: (f64, f64)) -> Option<u64> {
        self.suditos
            .iter()
            .find(|(_, sudito)| ponto_igual(sudito.posicao, ponto))
            .map(|(&id, _)| id)
    }

    /// Encontra o PNJ ativo em `ponto`, preferindo o menor `id` em caso de empate.
    fn pnj_id_ativo_em_ponto(&self, ponto: (f64, f64)) -> Option<u64> {
        self.pnjs
            .iter()
            .find(|(_, pnj)| pnj.ativo && ponto_igual(pnj.posicao, ponto))
            .map(|(&id, _)| id)
    }

    /// Fixa o alvo e avança a exposição em um tick, no máximo uma vez por Súdito.
    fn aplicar_propaganda(&mut self, comando: &Comando, avancada: &mut BTreeSet<u64>) {
        let Some(ponto) = comando.destino else {
            return;
        };
        let Some(id) = self.sudito_id_em_ponto(ponto) else {
            return;
        };
        let Some(jogador) = self.jogadores.get_mut(&comando.peer_id) else {
            return;
        };
        if !ponto_igual(ponto, jogador.posicao) {
            return;
        }
        jogador.propaganda_alvo = Some(id);
        if avancada.contains(&id) {
            return;
        }
        avancada.insert(id);
        self.avancar_propaganda_sudito(id);
    }

    /// Avança a exposição automática enquanto o jogador permanece no ponto do alvo.
    fn avancar_propagandas_automaticas(&mut self, avancada: &mut BTreeSet<u64>) {
        let alvos: Vec<(u64, u64, (f64, f64))> = self
            .jogadores
            .iter()
            .filter_map(|(&peer_id, jogador)| {
                jogador
                    .propaganda_alvo
                    .map(|id| (peer_id, id, jogador.posicao))
            })
            .collect();
        for (peer_id, id, posicao) in alvos {
            let Some(sudito) = self.suditos.get(&id) else {
                continue;
            };
            if !ponto_igual(sudito.posicao, posicao) {
                if let Some(jogador) = self.jogadores.get_mut(&peer_id) {
                    jogador.propaganda_alvo = None;
                }
                continue;
            }
            if avancada.contains(&id) {
                continue;
            }
            avancada.insert(id);
            self.avancar_propaganda_sudito(id);
        }
    }

    /// Avança a exposição e converte o Súdito ao atingir o limiar canônico.
    fn avancar_propaganda_sudito(&mut self, id: u64) {
        let Some(sudito) = self.suditos.get_mut(&id) else {
            return;
        };
        if sudito.convertido {
            return;
        }
        sudito.propaganda_ticks += 1;
        if sudito.propaganda_ticks == TICKS_PROPAGANDA {
            sudito.convertido = true;
        }
    }

    /// Encontra o Nó em `ponto`, preferindo o menor `id` em caso de empate.
    fn no_id_em_ponto(&self, ponto: (f64, f64)) -> Option<u64> {
        self.nos
            .iter()
            .find(|(_, no)| ponto_igual(no.posicao, ponto))
            .map(|(&id, _)| id)
    }

    /// Fixa o alvo e avança a Propaganda em um tick, no máximo uma vez por Nó.
    fn aplicar_propaganda_no(&mut self, comando: &Comando, avancada: &mut BTreeSet<u64>) {
        let Some(ponto) = comando.destino else {
            return;
        };
        let Some(id) = self.no_id_em_ponto(ponto) else {
            return;
        };
        let Some(jogador) = self.jogadores.get_mut(&comando.peer_id) else {
            return;
        };
        if !ponto_igual(ponto, jogador.posicao) {
            return;
        }
        jogador.no_alvo = Some(id);
        jogador.no_acao_no = AcaoNo::Propaganda;
        if avancada.contains(&id) {
            return;
        }
        avancada.insert(id);
        self.avancar_propaganda_no(id);
    }

    /// Avança a Propaganda automática enquanto o jogador permanece no ponto do Nó.
    fn avancar_propagandas_no_automaticas(&mut self, avancada: &mut BTreeSet<u64>) {
        let alvos: Vec<(u64, u64, (f64, f64), AcaoNo)> = self
            .jogadores
            .iter()
            .filter_map(|(&peer_id, jogador)| {
                jogador
                    .no_alvo
                    .map(|id| (peer_id, id, jogador.posicao, jogador.no_acao_no))
            })
            .collect();
        for (peer_id, id, posicao, acao) in alvos {
            let Some(no) = self.nos.get(&id) else {
                continue;
            };
            if !ponto_igual(no.posicao, posicao) {
                if let Some(jogador) = self.jogadores.get_mut(&peer_id) {
                    jogador.no_alvo = None;
                }
                continue;
            }
            let sem_propaganda_previa = acao == AcaoNo::Ataque && no.propaganda_ticks == 0;
            if no.convertido
                || no.inativo_ticks > 0
                || avancada.contains(&id)
                || sem_propaganda_previa
            {
                continue;
            }
            avancada.insert(id);
            self.avancar_propaganda_no(id);
        }
    }

    /// Avança a exposição e converte o Nó ao atingir o limiar de protótipo.
    fn avancar_propaganda_no(&mut self, id: u64) {
        let convertido_agora = {
            let Some(no) = self.nos.get_mut(&id) else {
                return;
            };
            if no.convertido || no.inativo_ticks > 0 {
                return;
            }
            no.propaganda_ticks = (no.propaganda_ticks + 1).min(TICKS_PROPAGANDA_NO);
            no.convertido = no.propaganda_ticks == TICKS_PROPAGANDA_NO;
            no.convertido
        };
        if convertido_agora {
            if self.nos.len() == TOTAL_NOS && self.nos.values().all(|no| no.convertido) {
                // O último Nó deixa a barra em 10 para o pulso final do tick seguinte.
                self.resistencia_ideologica = Self::resistencia_pre_pulso();
            } else {
                self.resistencia_ideologica = self
                    .resistencia_ideologica
                    .saturating_sub(CONTRIBUICAO_NO)
                    .max(PISO_DANO_COM_NOS);
            }
        }
    }

    /// Fixa o alvo e aplica dano limitado, sem converter o Nó nem apagar Propaganda.
    /// Dano nulo não inicia inatividade.
    fn aplicar_atacar_no(&mut self, comando: &Comando, avancada: &mut BTreeSet<u64>) {
        let Some(ponto) = comando.destino else {
            return;
        };
        let Some(id) = self.no_id_em_ponto(ponto) else {
            return;
        };
        let Some(jogador) = self.jogadores.get_mut(&comando.peer_id) else {
            return;
        };
        if !ponto_igual(ponto, jogador.posicao) {
            return;
        }
        jogador.no_alvo = Some(id);
        jogador.no_acao_no = AcaoNo::Ataque;
        if avancada.contains(&id) || self.resultado.is_some() {
            return;
        }
        avancada.insert(id);
        if self.nos.values().filter(|no| no.convertido).count() >= TOTAL_NOS {
            return;
        }
        let Some(no) = self.nos.get(&id) else {
            return;
        };
        if no.convertido || no.inativo_ticks > 0 {
            return;
        }
        let mut piso = RESISTENCIA_INICIAL - DANO_MAXIMO_NOS;
        for _ in self.nos.values().filter(|no| no.convertido) {
            piso -= CONTRIBUICAO_NO;
        }
        let piso = piso.max(PISO_DANO_COM_NOS);
        let dano = self
            .resistencia_ideologica
            .saturating_sub(piso)
            .min(DANO_MAXIMO_NOS);
        if dano == 0 {
            return;
        }
        self.resistencia_ideologica = self.resistencia_ideologica.saturating_sub(dano);
        if let Some(no) = self.nos.get_mut(&id) {
            // A unidade extra é consumida por `fase_vitoria` no próprio tick do dano.
            no.inativo_ticks = TICKS_INATIVIDADE_NO + 1;
        }
    }

    /// Resistência deixada pelas três conversões antes do pulso final (`[HIPÓTESE]`).
    fn resistencia_pre_pulso() -> i64 {
        let mut resistencia = RESISTENCIA_INICIAL;
        for _ in 0..TOTAL_NOS {
            resistencia -= CONTRIBUICAO_NO;
        }
        resistencia
    }

    /// Decrementa a inatividade dos Nós e aplica o pulso final de vitória.
    ///
    /// A contagem de convertidos no início do tick garante que a terceira conversão
    /// gere o pulso apenas no tick seguinte (`[HIPÓTESE]`/protótipo, não cânone).
    fn fase_vitoria(&mut self, convertidos_no_inicio: usize) {
        for no in self.nos.values_mut() {
            no.inativo_ticks = no.inativo_ticks.saturating_sub(1).max(0);
        }
        if self.resultado.is_some()
            || self.nos.len() != TOTAL_NOS
            || convertidos_no_inicio != TOTAL_NOS
        {
            return;
        }
        if self.resistencia_ideologica != Self::resistencia_pre_pulso() {
            return;
        }
        self.resistencia_ideologica = self.resistencia_ideologica.saturating_sub(PULSO_FINAL);
        self.resultado = Some(ResultadoPartida::Vitoria);
    }

    /// Registra a derrota por tempo ou por reconquistas, sem sobrescrever a vitória.
    ///
    /// A guarda inicial congela qualquer desfecho já definido por `fase_vitoria`,
    /// que roda antes desta fase no mesmo tick (`[HIPÓTESE]`/protótipo, não cânone).
    fn fase_derrota(&mut self) {
        if self.resultado.is_some() {
            return;
        }
        if self.tick >= TICKS_LIMITE_PARTIDA {
            self.resultado = Some(ResultadoPartida::DerrotaPorTempo);
        } else if self.ilhas_reconquistadas.len() >= MAX_RECONQUISTAS {
            self.resultado = Some(ResultadoPartida::DerrotaPorReconquista);
        }
    }

    /// Descarta comandos, regenera Fôlego e expira barricadas e pontes vencidas.
    fn fase_finish(&mut self) {
        self.comandos_do_tick.clear();
        self.avancar_niveis();
        self.regenerar_folego();
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

    /// Incrementa o nível a cada `TICKS_POR_NIVEL` ticks, saturando em `NIVEL_MAXIMO`.
    ///
    /// Usa apenas o acumulador do jogador e a ordem estável de `peer_id`
    /// (`[HIPÓTESE]`/protótipo, não cânone).
    fn avancar_niveis(&mut self) {
        for jogador in self.jogadores.values_mut() {
            if jogador.nivel >= NIVEL_MAXIMO {
                continue;
            }
            jogador.nivel_ticks += 1;
            if jogador.nivel_ticks >= TICKS_POR_NIVEL {
                jogador.nivel = (jogador.nivel + 1).min(NIVEL_MAXIMO);
                jogador.nivel_ticks = 0;
            }
        }
    }

    /// Regenera o Fôlego com acumulador próprio, em ordem estável de `peer_id`.
    ///
    /// Em combate a taxa é +1 a cada 6 ticks; fora de combate, +1 a cada 3 ticks
    /// (`[HIPÓTESE]`/protótipo, não cânone). `combate_ticks` decresce um por tick.
    fn regenerar_folego(&mut self) {
        for jogador in self.jogadores.values_mut() {
            jogador.folego_ticks += 1;
            let periodo = if jogador.combate_ticks > 0 {
                jogador.combate_ticks -= 1;
                TICKS_REGENERACAO_COMBATE
            } else {
                TICKS_REGENERACAO_FORA_COMBATE
            };
            if jogador.folego_ticks >= periodo {
                jogador.folego_ticks = 0;
                jogador.folego = (jogador.folego + 1).min(FOLEGO_MAXIMO);
            }
        }
    }

    /// Serializa os jogadores em ordem de `peer_id` para o hash canônico.
    fn serializar_jogadores(&self) -> String {
        self.jogadores
            .iter()
            .map(|(&peer_id, jogador)| {
                let (x, y) = jogador.posicao;
                format!(
                    concat!(
                        "{peer_id}:{x}:{y}:{vida}:{seq}:{request_id}:{materiais}:",
                        "{suprimentos}:{coleta_ticks}:{coleta_ponto:?}:{construcao_ticks}:",
                        "{construcao_ponto:?}:{pontos_autogestao}:{ponte_ticks}:",
                        "{ponte_ponto:?}:{propaganda_alvo:?}:{folego}:{folego_ticks}:",
                        "{combate_ticks}:{no_alvo:?}:{no_acao_no:?}:{funcao:?}:",
                        "{nivel}:{nivel_ticks}"
                    ),
                    peer_id = peer_id,
                    x = x,
                    y = y,
                    vida = jogador.vida,
                    seq = jogador.seq,
                    request_id = jogador.request_id,
                    materiais = jogador.materiais,
                    suprimentos = jogador.suprimentos,
                    coleta_ticks = jogador.coleta_ticks,
                    coleta_ponto = jogador.coleta_ponto,
                    construcao_ticks = jogador.construcao_ticks,
                    construcao_ponto = jogador.construcao_ponto,
                    pontos_autogestao = jogador.pontos_autogestao,
                    ponte_ticks = jogador.ponte_ticks,
                    ponte_ponto = jogador.ponte_ponto,
                    propaganda_alvo = jogador.propaganda_alvo,
                    folego = jogador.folego,
                    folego_ticks = jogador.folego_ticks,
                    combate_ticks = jogador.combate_ticks,
                    no_alvo = jogador.no_alvo,
                    no_acao_no = jogador.no_acao_no,
                    funcao = jogador.funcao,
                    nivel = jogador.nivel,
                    nivel_ticks = jogador.nivel_ticks,
                )
            })
            .collect::<Vec<_>>()
            .join(";")
    }

    /// Serializa barricadas, pontes, súditos, ilhas, reconquistas e sinais.
    fn serializar_estruturas(&self) -> String {
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
        let suditos: String = self
            .suditos
            .values()
            .map(|sudito| {
                let (x, y) = sudito.posicao;
                format!(
                    "{}:{}:{}:{}:{}:{}",
                    sudito.id,
                    x,
                    y,
                    sudito.convertido,
                    sudito.propaganda_ticks,
                    sudito.reversao_ticks
                )
            })
            .collect::<Vec<_>>()
            .join(";");
        let pnjs: String = self
            .pnjs
            .values()
            .map(|pnj| {
                let (x, y) = pnj.posicao;
                let id = pnj.id;
                let pv = pnj.pv;
                let ativo = pnj.ativo;
                let ticks = pnj.ataque_ticks;
                format!("{id}:{x}:{y}:{pv}:{ativo}:{ticks}")
            })
            .collect::<Vec<_>>()
            .join(";");
        let ilhas: String = self
            .ilhas
            .values()
            .map(|ilha| format!("{}:{}:{}", ilha.id, ilha.controlada, ilha.controle_ticks))
            .collect::<Vec<_>>()
            .join(";");
        let ilhas_reconquistadas: String = self
            .ilhas_reconquistadas
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(";");
        let sinais: String = self
            .sinais
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(";");
        format!("{barreiras}|{pontes}|{suditos}|{pnjs}|{ilhas}|{ilhas_reconquistadas}|{sinais}")
    }

    /// Calcula o hash SHA-256 canônico do estado corrente.
    #[must_use]
    pub fn hash_estado(&self) -> String {
        let jogadores = self.serializar_jogadores();
        let estruturas = self.serializar_estruturas();

        let nos: String = self
            .nos
            .values()
            .map(|no| {
                let (x, y) = no.posicao;
                let id = no.id;
                let convertido = no.convertido;
                let propaganda = no.propaganda_ticks;
                let inatividade = no.inativo_ticks;
                format!("{id}:{x}:{y}:{convertido}:{propaganda}:{inatividade}")
            })
            .collect::<Vec<_>>()
            .join(";");
        let resultado = match self.resultado {
            Some(ResultadoPartida::Vitoria) => "vitoria",
            Some(ResultadoPartida::DerrotaPorTempo) => "derrota_por_tempo",
            Some(ResultadoPartida::DerrotaPorReconquista) => "derrota_por_reconquista",
            None => "nenhum",
        };

        let ultima_recusa = match self.ultima_recusa {
            Some(codigo) => codigo.to_string(),
            None => "nenhuma".to_owned(),
        };
        let entrada = format!(
            concat!(
                "{tick}|{epoch}|{match_id}|{jogadores}|{estruturas}|",
                "{nos}|{resistencia}|{resultado}|{ultima_recusa}",
            ),
            tick = self.tick,
            epoch = self.epoch,
            match_id = self.match_id,
            jogadores = jogadores,
            estruturas = estruturas,
            nos = nos,
            resistencia = self.resistencia_ideologica,
            resultado = resultado,
            ultima_recusa = ultima_recusa,
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
    /// Ação fora do nível do jogador (`[HIPÓTESE]`/protótipo, não cânone).
    #[error("acao bloqueada por nivel")]
    AcaoBloqueada,
    /// `atacar_forte` sem alvo no destino ou além do alcance de contato.
    #[error("alvo invalido")]
    AlvoInvalido,
    /// Fôlego abaixo do custo de `atacar_forte`.
    #[error("folego insuficiente")]
    FolegoInsuficiente,
    /// Destino ausente ou fora do ponto para `mover`, `coletar_materiais`,
    /// `construir_barreira`, `construir_ponte`, `propaganda`, `propaganda_no` ou
    /// `atacar_no`; `atacar_pnj` recusa destino sem PNJ ativo ou além de 2 m do
    /// jogador. Nó inativo também recusa `atacar_no`.
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
    /// par, replay, ação, nível (`AcaoBloqueada`), destino (incluindo o limite
    /// de distância de `mover`), alvo/alcance e Fôlego para `atacar_forte`,
    /// saldo para `construir_barreira` e `construir_ponte`; `propaganda` exige
    /// Súdito no ponto do jogador, e `atacar_pnj` exige PNJ ativo em `destino`
    /// com o jogador a até 2 m desse ponto.
    /// `propaganda_no`/`atacar_no` exigem Nó no ponto do jogador e não exigem
    /// saldo ou Fôlego.
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

        // Mesmo mapa Q1/W2/E3/R6 usado pelo gate de `fase_skills`.
        if jogador.nivel < nivel_minimo_para_acao(acao) {
            return Err(CodigoRecusa::AcaoBloqueada);
        }

        if acao == "mover" {
            let (x, y) = destino.ok_or(CodigoRecusa::DestinoInvalido)?;
            let (origem_x, origem_y) = jogador.posicao;
            if (x - origem_x).hypot(y - origem_y) > LIMITE_MOVIMENTO_M {
                return Err(CodigoRecusa::MovimentoForaDoLimite);
            }
        }

        if acao == "atacar_forte" {
            let ponto = destino.ok_or(CodigoRecusa::AlvoInvalido)?;
            let alvo_no_ponto = mundo
                .jogadores
                .iter()
                .any(|(&peer, alvo)| peer != peer_id && ponto_igual(alvo.posicao, ponto));
            if !alvo_no_ponto {
                return Err(CodigoRecusa::AlvoInvalido);
            }
            let (origem_x, origem_y) = jogador.posicao;
            if (ponto.0 - origem_x).hypot(ponto.1 - origem_y) > ALCANCE_CONTATO_M {
                return Err(CodigoRecusa::AlvoInvalido);
            }
            if jogador.folego < CUSTO_ATAQUE_FOLEGO {
                return Err(CodigoRecusa::FolegoInsuficiente);
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

        if acao == "propaganda" {
            let ponto = destino.ok_or(CodigoRecusa::DestinoInvalido)?;
            let no_ponto = mundo.sudito_id_em_ponto(ponto).is_some();
            if !ponto_igual(ponto, jogador.posicao) || !no_ponto {
                return Err(CodigoRecusa::DestinoInvalido);
            }
        }

        if acao == "propaganda_no" || acao == "atacar_no" {
            let ponto = destino.ok_or(CodigoRecusa::DestinoInvalido)?;
            if !ponto_igual(ponto, jogador.posicao) {
                return Err(CodigoRecusa::DestinoInvalido);
            }
            let Some(id) = mundo.no_id_em_ponto(ponto) else {
                return Err(CodigoRecusa::DestinoInvalido);
            };
            if acao == "atacar_no" && mundo.nos[&id].inativo_ticks > 0 {
                return Err(CodigoRecusa::DestinoInvalido);
            }
        }

        if acao == "atacar_pnj" {
            let ponto = destino.ok_or(CodigoRecusa::DestinoInvalido)?;
            let distancia = (ponto.0 - jogador.posicao.0).hypot(ponto.1 - jogador.posicao.1);
            if distancia > ALCANCE_CONTATO_M || mundo.pnj_id_ativo_em_ponto(ponto).is_none() {
                return Err(CodigoRecusa::DestinoInvalido);
            }
        }

        Ok(())
    }
}
