#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Instrumentação do Núcleo Insurgência para as métricas da seção 17 do GDD.
//!
//! Cada amostra é sintética e determinística: [`correr_partidas`] deriva as
//! partidas de uma mistura aritmética fixa, sem relógio, rede ou arquivos. Os
//! limiares declarados são `[HIPOTESE]` e devem ser recalibrados uma única vez
//! após 20 partidas instrumentadas, conforme a seção 17 do GDD.

use std::collections::BTreeSet;

use insurgencia_contratos::PROTOCOL_VERSION;
use thiserror::Error;

/// Mediana máxima, em segundos, até a primeira captura `[HIPOTESE]`.
pub const LIMITE_MEDIANA_CAPTURA_S: usize = 90;
/// p95 máximo, em segundos, até a primeira captura `[HIPOTESE]`.
pub const LIMITE_P95_CAPTURA_S: usize = 150;
/// Taxa mínima de conclusão do desafio sincronizado, em porcentagem `[HIPOTESE]`.
pub const LIMITE_SINCRONIA_MIN_PCT: usize = 60;
/// Taxa máxima de conclusão do desafio sincronizado, em porcentagem `[HIPOTESE]`.
pub const LIMITE_SINCRONIA_MAX_PCT: usize = 90;
/// RTT mediano máximo em servidor local, em milissegundos `[HIPOTESE]`.
pub const LIMITE_RTT_MEDIANO_MS: usize = 80;
/// RTT mediano máximo em servidor comunitário, em milissegundos `[HIPOTESE]`.
pub const LIMITE_RTT_COMUNITARIO_MS: usize = 150;
/// p95 máximo de RTT, em milissegundos `[HIPOTESE]`.
pub const LIMITE_RTT_P95_MS: usize = 250;
/// Percentual mínimo de respostas "sem atraso perceptível" `[HIPOTESE]`.
pub const LIMITE_RESPOSTA_POSITIVA_PCT: usize = 90;
/// Servidores comunitários ativos em 3 meses `[HIPOTESE]`.
pub const LIMITE_SERVIDORES_3M: usize = 5;
/// Servidores comunitários ativos em 12 meses `[HIPOTESE]`.
pub const LIMITE_SERVIDORES_12M: usize = 20;
/// Módulos públicos distintos em 3 meses `[HIPOTESE]`.
pub const LIMITE_MODULOS_3M: usize = 3;
/// Módulos públicos distintos em 12 meses `[HIPOTESE]`.
pub const LIMITE_MODULOS_12M: usize = 10;
/// Forks publicados com pacote de personagens diferente `[HIPOTESE]`.
pub const LIMITE_FORKS: usize = 2;
/// Percentual mínimo de partidas concluídas por vitória `[HIPOTESE]`.
pub const LIMITE_CONCLUSAO_PCT: usize = 50;

/// Módulos públicos distintos que podem registrar partida na amostra.
const MODULOS_PUBLICOS: [&str; 8] = [
    "exemplo",
    "artesao",
    "guerreiro",
    "ilha-topo",
    "ilha-meio",
    "ilha-inferior",
    "central",
    "variante",
];

/// Eventos gravados no Núcleo antes da Fase 4 (seção 17 do GDD).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EventoNucleo {
    /// Primeira ilha de rota capturada.
    CapturaIlha,
    /// Tentativa de ativação na janela de 5 s.
    TentativaSincronia,
    /// Divergência detectada entre cliente e servidor.
    DivergenciaClienteServidor,
    /// Início de partida instrumentada.
    PartidaIniciada,
    /// Vitória da Ilha Central.
    Vitoria,
}

/// Falhas da instrumentação de métricas.
// O nome público é fixado pela feature 024; `metricas` é o módulo raiz.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ErroMetricas {
    /// A amostra não contém partidas, portanto não há mediana nem p95.
    #[error("amostra vazia: correr_partidas exige n >= 1")]
    SemPartidas,
}

/// Tipo de cliente que jogou a partida instrumentada.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TipoCliente {
    /// Cliente canônico, sem modificações.
    Padrao,
    /// Última partida da sessão jogada por cliente modificado.
    Modificado,
}

/// Percepção da latência da resposta reportada pelo cliente.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PercepcaoLatencia {
    /// Resposta sem atraso perceptível.
    SemAtraso,
    /// Resposta com atraso perceptível.
    ComAtraso,
}

/// Origem do conteúdo publicado associado à partida.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProvenienciaConteudo {
    /// Módulo público canônico.
    Modulo,
    /// Fork publicado, contado à parte dos módulos.
    Fork,
}

/// Desfecho da partida instrumentada.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DesfechoPartida {
    /// Partida concluída por vitória da Ilha Central.
    Vitoria,
    /// Partida sem vitória da Ilha Central.
    NaoVitoria,
}

/// Registro de uma partida instrumentada no Núcleo.
///
/// Os campos correspondem às seis linhas da tabela da seção 17 do GDD; as
/// divergências cliente-servidor são registradas como evento, não como meta.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistroPartida {
    /// Identificador monotônico da partida na amostra.
    pub id_partida: usize,
    /// Versão do protocolo de rede usada na instrumentação.
    pub versao_protocolo: u8,
    /// Última partida da sessão jogada por cliente modificado.
    pub cliente_modificado: TipoCliente,
    /// Eventos do Núcleo, em ordem de ocorrência.
    pub eventos: Vec<EventoNucleo>,
    /// Métrica 1: segundos da partida até a primeira captura de ilha de rota.
    pub tempo_ate_primeira_captura_s: usize,
    /// Métrica 2: ativações bem-sucedidas na janela de 5 s.
    pub sincronias_concluidas: usize,
    /// Métrica 2: tentativas na janela de 5 s.
    pub tentativas_sincronia: usize,
    /// Métrica 3: RTT reportado pelo cliente, em milissegundos.
    pub rtt_cliente_ms: usize,
    /// Métrica 3: resposta "sem atraso perceptível".
    pub sem_atraso_perceptivel: PercepcaoLatencia,
    /// Métrica 4: identificador do servidor; zero é o servidor oficial.
    pub servidor_id: usize,
    /// Métrica 5: módulo público distintamente identificado com partida registrada.
    pub modulo_publicado: &'static str,
    /// Métrica 5: fork publicado, contado à parte dos módulos.
    pub fork_publicado: ProvenienciaConteudo,
    /// Partida concluída por vitória da Ilha Central.
    pub vitoria: DesfechoPartida,
    /// Divergências cliente-servidor detectadas pelo Núcleo nesta partida.
    pub divergencias_cliente_servidor: usize,
}

/// Conjunto de registros e agregações das métricas da seção 17.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Relatorio {
    /// Registros das partidas instrumentadas, na ordem em que ocorreram.
    pub registros: Vec<RegistroPartida>,
}

impl Relatorio {
    /// Mediana do tempo até a primeira captura, em segundos.
    #[must_use]
    pub fn mediana_tempo_captura(&self) -> usize {
        mediana(&self.tempos_captura())
    }

    /// p95 do tempo até a primeira captura, em segundos.
    #[must_use]
    pub fn p95_tempo_captura(&self) -> usize {
        percentil95(&self.tempos_captura())
    }

    /// Taxa agregada de conclusão do desafio sincronizado, em porcentagem.
    #[must_use]
    pub fn taxa_conclusao_sincronia(&self) -> usize {
        let tentativas: usize = self.registros.iter().map(|r| r.tentativas_sincronia).sum();
        let concluidas: usize = self.registros.iter().map(|r| r.sincronias_concluidas).sum();
        porcentagem(concluidas, tentativas)
    }

    /// RTT mediano reportado pelo cliente, em milissegundos.
    #[must_use]
    pub fn rtt_mediano_cliente(&self) -> usize {
        mediana(&self.rtts_cliente())
    }

    /// p95 do RTT reportado pelo cliente, em milissegundos.
    #[must_use]
    pub fn rtt_p95_cliente(&self) -> usize {
        percentil95(&self.rtts_cliente())
    }

    /// Percentual de respostas "sem atraso perceptível".
    #[must_use]
    pub fn taxa_resposta_sem_atraso(&self) -> usize {
        let positivas = self
            .registros
            .iter()
            .filter(|r| r.sem_atraso_perceptivel == PercepcaoLatencia::SemAtraso)
            .count();
        porcentagem(positivas, self.registros.len())
    }

    /// Servidores comunitários com ao menos uma partida completa na amostra.
    #[must_use]
    pub fn servidores_comunitarios_ativos(&self) -> usize {
        let ativos: BTreeSet<usize> = self
            .registros
            .iter()
            .filter(|r| r.servidor_id > 0 && r.vitoria == DesfechoPartida::Vitoria)
            .map(|r| r.servidor_id)
            .collect();
        ativos.len()
    }

    /// Quantidade de módulos públicos distintos com partida registrada.
    #[must_use]
    pub fn modulos_publicados_distintos(&self) -> usize {
        self.registros
            .iter()
            .map(|r| r.modulo_publicado)
            .collect::<BTreeSet<_>>()
            .len()
    }

    /// Quantidade de forks publicados, contados à parte dos módulos.
    #[must_use]
    pub fn forks_publicados(&self) -> usize {
        self.registros
            .iter()
            .filter(|r| r.fork_publicado == ProvenienciaConteudo::Fork)
            .count()
    }

    /// Percentual de partidas concluídas por vitória da Ilha Central.
    #[must_use]
    pub fn taxa_partidas_concluidas(&self) -> usize {
        let vitorias = self
            .registros
            .iter()
            .filter(|r| r.vitoria == DesfechoPartida::Vitoria)
            .count();
        porcentagem(vitorias, self.registros.len())
    }

    /// Total de divergências cliente-servidor gravadas no Núcleo.
    #[must_use]
    pub fn total_divergencias(&self) -> usize {
        self.registros
            .iter()
            .map(|r| r.divergencias_cliente_servidor)
            .sum()
    }

    /// Quantidade de eventos de um dado tipo em toda a amostra.
    #[must_use]
    pub fn contagem_eventos(&self, evento: EventoNucleo) -> usize {
        self.registros
            .iter()
            .map(|r| r.eventos.iter().filter(|e| **e == evento).count())
            .sum()
    }

    /// Série de tempos até a primeira captura.
    fn tempos_captura(&self) -> Vec<usize> {
        self.registros
            .iter()
            .map(|r| r.tempo_ate_primeira_captura_s)
            .collect()
    }

    /// Série de RTTs reportados pelo cliente.
    fn rtts_cliente(&self) -> Vec<usize> {
        self.registros.iter().map(|r| r.rtt_cliente_ms).collect()
    }
}

/// Simula `n` partidas consecutivas de forma determinística.
///
/// Quando `cliente_modificado` é verdadeiro, a última partida da sessão é
/// jogada por um cliente modificado; as demais usam o cliente canônico.
///
/// # Errors
///
/// Retorna [`ErroMetricas::SemPartidas`] quando `n` é zero, pois mediana e p95
/// não estão definidos para uma amostra vazia.
pub fn correr_partidas(n: usize, cliente_modificado: bool) -> Result<Relatorio, ErroMetricas> {
    if n == 0 {
        return Err(ErroMetricas::SemPartidas);
    }

    let mut registros = Vec::with_capacity(n);
    for id_partida in 1..=n {
        registros.push(simular_partida(id_partida, n, cliente_modificado));
    }
    Ok(Relatorio { registros })
}

/// Constrói o registro de uma partida sem relógio, rede, arquivos ou aleatoriedade.
fn simular_partida(id_partida: usize, total: usize, cliente_modificado: bool) -> RegistroPartida {
    let alterado = cliente_modificado && id_partida == total;
    let tipo_cliente = if alterado {
        TipoCliente::Modificado
    } else {
        TipoCliente::Padrao
    };
    let h = misturar(id_partida);

    let tempo_ate_primeira_captura_s = (30 + h % 101)
        .saturating_add(if alterado { 15 } else { 0 })
        .min(LIMITE_P95_CAPTURA_S);
    let tentativas_sincronia = 6 + h % 15;
    let taxa_sincronia = (62 + h % 27).saturating_sub(if alterado { 10 } else { 0 });
    let sincronias_concluidas = tentativas_sincronia
        .saturating_mul(taxa_sincronia)
        .saturating_div(100);
    let rtt_cliente_ms = (35 + h % 61).saturating_add(if alterado { 40 } else { 0 });
    let sem_atraso_perceptivel = if rtt_cliente_ms <= 100 {
        PercepcaoLatencia::SemAtraso
    } else {
        PercepcaoLatencia::ComAtraso
    };
    let servidor_id = if h.is_multiple_of(4) { 0 } else { 1 + h % 6 };
    let modulo_publicado = MODULOS_PUBLICOS[h % MODULOS_PUBLICOS.len()];
    let fork_publicado = if h.is_multiple_of(7) {
        ProvenienciaConteudo::Fork
    } else {
        ProvenienciaConteudo::Modulo
    };
    let vitoria = if !h.is_multiple_of(4) && !alterado {
        DesfechoPartida::Vitoria
    } else {
        DesfechoPartida::NaoVitoria
    };
    let divergencias_cliente_servidor = if alterado { 1 + h % 3 } else { 0 };

    let mut eventos = Vec::with_capacity(3 + tentativas_sincronia + divergencias_cliente_servidor);
    eventos.push(EventoNucleo::PartidaIniciada);
    eventos.push(EventoNucleo::CapturaIlha);
    for _ in 0..tentativas_sincronia {
        eventos.push(EventoNucleo::TentativaSincronia);
    }
    for _ in 0..divergencias_cliente_servidor {
        eventos.push(EventoNucleo::DivergenciaClienteServidor);
    }
    if vitoria == DesfechoPartida::Vitoria {
        eventos.push(EventoNucleo::Vitoria);
    }

    RegistroPartida {
        id_partida,
        versao_protocolo: PROTOCOL_VERSION,
        cliente_modificado: tipo_cliente,
        eventos,
        tempo_ate_primeira_captura_s,
        sincronias_concluidas,
        tentativas_sincronia,
        rtt_cliente_ms,
        sem_atraso_perceptivel,
        servidor_id,
        modulo_publicado,
        fork_publicado,
        vitoria,
        divergencias_cliente_servidor,
    }
}

/// Mistura determinística de um identificador, com aritmética modular.
fn misturar(semente: usize) -> usize {
    let mut h = semente
        .wrapping_mul(2_654_435_761)
        .wrapping_add(1_013_904_223);
    h ^= h >> 15;
    h = h.wrapping_mul(2_246_822_519);
    h ^= h >> 12;
    h
}

/// Mediana de uma amostra, com ordenação própria e sem mutar a entrada.
fn mediana(valores: &[usize]) -> usize {
    let mut ordenado = valores.to_vec();
    ordenado.sort_unstable();
    match ordenado.len() {
        0 => 0,
        n if n.is_multiple_of(2) => ordenado[n / 2 - 1].saturating_add(ordenado[n / 2]) / 2,
        n => ordenado[n / 2],
    }
}

/// p95 de uma amostra, com a mesma ordenação usada pela mediana.
fn percentil95(valores: &[usize]) -> usize {
    let mut ordenado = valores.to_vec();
    ordenado.sort_unstable();
    if ordenado.is_empty() {
        return 0;
    }
    let indice = ordenado.len().saturating_mul(95).div_ceil(100) - 1;
    ordenado[indice]
}

/// Razão em porcentagem, com denominador zero definido como zero.
fn porcentagem(numerador: usize, denominador: usize) -> usize {
    if denominador == 0 {
        return 0;
    }
    numerador.saturating_mul(100) / denominador
}
