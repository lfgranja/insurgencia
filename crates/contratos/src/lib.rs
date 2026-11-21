#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Contratos de rede do Núcleo Insurgência, independentes de engine.
//!
//! Espelham `nucleo/contratos/protocolo.gd`: versão do protocolo, identificador
//! do módulo, tipos de pacote, códigos de recusa, metadados de snapshot e a
//! normalização que descarta chaves fora do contrato.

use std::collections::BTreeMap;

use serde_json::{json, Value};
use thiserror::Error;

/// Versão do protocolo de rede (`PROTOCOL_VERSION = 1`).
pub const PROTOCOL_VERSION: u8 = 1;

/// Identificador do módulo de exemplo (`MODULO_ID`).
pub const MODULO_ID: &str = "insurgencia.exemplo";

/// Recusa por versão de protocolo divergente (`REJEICAO_VERSAO`).
pub const REJEICAO_VERSAO: &str = "VERSAO_PROTOCOLO_DIVERGENTE";

/// Recusa por hash de módulo divergente (`REJEICAO_HASH`).
pub const REJEICAO_HASH: &str = "HASH_MODULO_DIVERGENTE";

/// Recusa por módulo ausente (`REJEICAO_MODULO`).
pub const REJEICAO_MODULO: &str = "MODULO_AUSENTE";

/// Recusa por manifesto inválido (`REJEICAO_MANIFESTO`).
pub const REJEICAO_MANIFESTO: &str = "MANIFEST_INVALID";

/// Falhas de conversão do contrato de rede.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum ErroProtocolo {
    /// Valor de tipo de pacote fora de `1..=6`.
    #[error("tipo de pacote desconhecido: {0}")]
    TipoPacoteDesconhecido(u8),
}

/// Tipos de pacote do contrato, com os discriminantes da wire (`#[repr(u8)]`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum TipoPacote {
    /// `JOIN = 1`.
    Join = 1,
    /// `JOIN_ACEITO = 2`.
    JoinAceito = 2,
    /// `JOIN_RECUSADO = 3`.
    JoinRecusado = 3,
    /// `COMANDO = 4`.
    Comando = 4,
    /// `SNAPSHOT = 5`.
    Snapshot = 5,
    /// `EVENTO_DESAFIO = 6`.
    EventoDesafio = 6,
}

impl TipoPacote {
    /// Alias do contrato para `JOIN = 1`.
    pub const JOIN: Self = Self::Join;
    /// Alias do contrato para `JOIN_ACEITO = 2`.
    pub const JOIN_ACEITO: Self = Self::JoinAceito;
    /// Alias do contrato para `JOIN_RECUSADO = 3`.
    pub const JOIN_RECUSADO: Self = Self::JoinRecusado;
    /// Alias do contrato para `COMANDO = 4`.
    pub const COMANDO: Self = Self::Comando;
    /// Alias do contrato para `SNAPSHOT = 5`.
    pub const SNAPSHOT: Self = Self::Snapshot;
    /// Alias do contrato para `EVENTO_DESAFIO = 6`.
    pub const EVENTO_DESAFIO: Self = Self::EventoDesafio;
}

impl TryFrom<u8> for TipoPacote {
    type Error = ErroProtocolo;

    /// Converte o inteiro da wire em tipo de pacote.
    ///
    /// # Errors
    ///
    /// Retorna [`ErroProtocolo::TipoPacoteDesconhecido`] quando o valor está
    /// fora de `1..=6`.
    fn try_from(valor: u8) -> Result<Self, Self::Error> {
        match valor {
            1 => Ok(Self::Join),
            2 => Ok(Self::JoinAceito),
            3 => Ok(Self::JoinRecusado),
            4 => Ok(Self::Comando),
            5 => Ok(Self::Snapshot),
            6 => Ok(Self::EventoDesafio),
            outro => Err(ErroProtocolo::TipoPacoteDesconhecido(outro)),
        }
    }
}

/// Metadados que acompanham cada snapshot (`SnapshotMeta`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotMeta {
    /// Versão do protocolo.
    pub protocol_version: u8,
    /// Epoch monotônico gerado pelo servidor.
    pub epoch: u64,
    /// Identificador da partida.
    pub match_id: u64,
    /// Tick da simulação.
    pub tick: u64,
    /// Hash do conteúdo do módulo.
    pub content_hash: String,
}

impl SnapshotMeta {
    /// Constrói os metadados a partir dos cinco campos do contrato.
    #[must_use]
    pub const fn new(
        protocol_version: u8,
        epoch: u64,
        match_id: u64,
        tick: u64,
        content_hash: String,
    ) -> Self {
        Self {
            protocol_version,
            epoch,
            match_id,
            tick,
            content_hash,
        }
    }

    /// Serializa os cinco campos para um mapa de chaves e valores textuais.
    #[must_use]
    pub fn to_dict(&self) -> BTreeMap<String, String> {
        BTreeMap::from([
            (
                "protocol_version".to_string(),
                self.protocol_version.to_string(),
            ),
            ("epoch".to_string(), self.epoch.to_string()),
            ("match_id".to_string(), self.match_id.to_string()),
            ("tick".to_string(), self.tick.to_string()),
            ("content_hash".to_string(), self.content_hash.clone()),
        ])
    }
}

/// Valores padrão por tipo de pacote (`PADROES` do contrato `GDScript`).
fn padroes(tipo: TipoPacote) -> BTreeMap<String, Value> {
    match tipo {
        TipoPacote::Join => BTreeMap::from([
            ("protocol_version".to_string(), Value::from(0)),
            ("module_id".to_string(), Value::from("")),
            ("content_hash".to_string(), Value::from("")),
        ]),
        TipoPacote::JoinAceito | TipoPacote::JoinRecusado => BTreeMap::new(),
        TipoPacote::Comando => BTreeMap::from([
            ("request_id".to_string(), Value::from(0)),
            ("seq".to_string(), Value::from(0)),
            ("acao".to_string(), Value::from("")),
            ("alvo".to_string(), Value::from(0)),
            ("destino".to_string(), json!([0.0, 0.0])),
        ]),
        TipoPacote::Snapshot => BTreeMap::from([
            ("tick".to_string(), Value::from(0)),
            ("epoch".to_string(), Value::from(0)),
            ("match_id".to_string(), Value::from(0)),
            ("jogadores".to_string(), json!([])),
        ]),
        TipoPacote::EventoDesafio => BTreeMap::from([
            ("ilha".to_string(), Value::from("")),
            ("evento".to_string(), Value::from("")),
            ("tick".to_string(), Value::from(0)),
        ]),
    }
}

/// Normalização interna: usa `?` para recusar tipos fora do contrato.
fn normalizar(
    tipo: u8,
    dados: BTreeMap<String, Value>,
) -> Result<BTreeMap<String, Value>, ErroProtocolo> {
    let conhecido = TipoPacote::try_from(tipo)?;
    let mut resultado = padroes(conhecido);

    for (chave, valor) in dados {
        if let Some(destino) = resultado.get_mut(&chave) {
            *destino = valor;
        }
    }

    resultado.insert("tipo".to_string(), Value::from(tipo));
    Ok(resultado)
}

/// Normaliza um pacote: preenche os padrões do tipo, sobrescreve apenas as
/// chaves previstas no contrato e registra `tipo`. Tipos desconhecidos geram
/// somente a chave `tipo`, sem confiar em chaves fornecidas pelo cliente.
#[must_use]
pub fn normalizar_pacote(
    tipo: u8,
    dados: BTreeMap<String, serde_json::Value>,
) -> BTreeMap<String, serde_json::Value> {
    if let Ok(resultado) = normalizar(tipo, dados) {
        resultado
    } else {
        BTreeMap::from([("tipo".to_string(), Value::from(tipo))])
    }
}
