#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Porta Rust do contrato de manifesto de compatibilidade da spec 011.
//!
//! Espelha `nucleo/compatibilidade/manifesto.gd`: os seis campos de
//! `manifesto.json`, os códigos literais de recusa e a ordem de validação
//! formato → versão → módulo → hash → regras → divergência de hash.
//! Nenhuma função deste módulo faz I/O: quem chama informa o hash das regras.

use thiserror::Error;

/// Código literal para arquivo de regras ausente (`REGRAS_AUSENTES`).
pub const REGRAS_AUSENTES: &str = "REGRAS_AUSENTES";

/// Manifesto de compatibilidade com os seis campos obrigatórios de `manifesto.json`.
#[allow(clippy::module_name_repetitions)] // nome fixado pelo contrato (`GDScript`)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManifestoCompatibilidade {
    /// Versão do formato do manifesto (`manifest_version`).
    pub manifest_version: String,
    /// Versão do protocolo de rede (`protocol_version`).
    pub protocol_version: u8,
    /// Identificador do módulo (`module_id`).
    pub module_id: String,
    /// SHA-256 hexadecimal do arquivo de regras (`content_hash`).
    pub content_hash: String,
    /// Data/hora da última verificação (`checked_at`).
    pub checked_at: String,
    /// Versão da ferramenta que gerou o manifesto (`tool_version`).
    pub tool_version: String,
}

/// Recusas do gate de compatibilidade, com os códigos literais da spec 011.
#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum ErroCompatibilidade {
    /// Campo ausente, tipo inválido ou hash malformado (`MANIFEST_INVALID`).
    #[error("manifesto inválido")]
    ManifestoInvalido,
    /// Versão de protocolo divergente (`VERSAO_PROTOCOLO_DIVERGENTE`).
    #[error("versão de protocolo divergente")]
    VersaoProtocoloDivergente,
    /// Módulo ausente ou divergente (`MODULO_AUSENTE`).
    #[error("módulo ausente")]
    ModuloAusente,
    /// Arquivo de regras ausente (`REGRAS_AUSENTES`).
    #[error("arquivo de regras ausente")]
    RegrasAusentes,
    /// Hash das regras divergente (`HASH_MODULO_DIVERGENTE`).
    #[error("hash de módulo divergente")]
    HashModuloDivergente,
}

impl ErroCompatibilidade {
    /// Devolve o código literal oficial da recusa.
    #[must_use]
    pub const fn codigo(&self) -> &'static str {
        match self {
            Self::ManifestoInvalido => crate::REJEICAO_MANIFESTO,
            Self::VersaoProtocoloDivergente => crate::REJEICAO_VERSAO,
            Self::ModuloAusente => crate::REJEICAO_MODULO,
            Self::RegrasAusentes => REGRAS_AUSENTES,
            Self::HashModuloDivergente => crate::REJEICAO_HASH,
        }
    }
}

/// Valida o formato do manifesto: exige os seis campos com seus tipos.
///
/// Não verifica o comprimento do hash — essa checagem pertence a [`conferir`].
///
/// # Errors
///
/// Devolve [`ErroCompatibilidade::ManifestoInvalido`] quando o texto não é um
/// objeto JSON, quando algum dos seis campos falta ou quando um campo tem tipo
/// incompatível com o contrato.
pub fn validar_formato(json: &str) -> Result<ManifestoCompatibilidade, ErroCompatibilidade> {
    let valor: serde_json::Value =
        serde_json::from_str(json).map_err(|_| ErroCompatibilidade::ManifestoInvalido)?;
    let objeto = valor
        .as_object()
        .ok_or(ErroCompatibilidade::ManifestoInvalido)?;

    let texto = |chave: &str| -> Result<String, ErroCompatibilidade> {
        objeto
            .get(chave)
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .ok_or(ErroCompatibilidade::ManifestoInvalido)
    };

    let protocol_version = objeto
        .get("protocol_version")
        .and_then(serde_json::Value::as_u64)
        .and_then(|numero| u8::try_from(numero).ok())
        .ok_or(ErroCompatibilidade::ManifestoInvalido)?;

    Ok(ManifestoCompatibilidade {
        manifest_version: texto("manifest_version")?,
        protocol_version,
        module_id: texto("module_id")?,
        content_hash: texto("content_hash")?,
        checked_at: texto("checked_at")?,
        tool_version: texto("tool_version")?,
    })
}

/// Confere o manifesto contra a versão local, o módulo e o hash das regras.
///
/// A ordem das verificações é a da spec 011: formato, versão, módulo, formato
/// do hash, presença das regras e, por fim, igualdade do hash. Esta função não
/// faz I/O — o chamador informa `hash_regras` (o `SHA-256` do arquivo de regras
/// em hexadecimal).
///
/// # Errors
///
/// Devolve, nesta ordem: [`ErroCompatibilidade::ManifestoInvalido`] se o
/// formato for recusado ou o hash não tiver 64 caracteres hexadecimais;
/// [`ErroCompatibilidade::VersaoProtocoloDivergente`] se a versão divergir;
/// [`ErroCompatibilidade::ModuloAusente`] se o módulo divergir;
/// [`ErroCompatibilidade::RegrasAusentes`] se `hash_regras` for `None`; e
/// [`ErroCompatibilidade::HashModuloDivergente`] se os hashes divergirem.
pub fn conferir(
    json: &str,
    modulo_id: &str,
    versao_local: u8,
    hash_regras: Option<&str>,
) -> Result<ManifestoCompatibilidade, ErroCompatibilidade> {
    let manifesto = validar_formato(json)?;

    if manifesto.protocol_version != versao_local {
        return Err(ErroCompatibilidade::VersaoProtocoloDivergente);
    }
    if manifesto.module_id != modulo_id {
        return Err(ErroCompatibilidade::ModuloAusente);
    }

    let hash_bem_formado = manifesto.content_hash.len() == 64
        && manifesto
            .content_hash
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit());
    if !hash_bem_formado {
        return Err(ErroCompatibilidade::ManifestoInvalido);
    }

    let hash_regras = hash_regras.ok_or(ErroCompatibilidade::RegrasAusentes)?;
    if !hash_regras.eq_ignore_ascii_case(&manifesto.content_hash) {
        return Err(ErroCompatibilidade::HashModuloDivergente);
    }

    Ok(manifesto)
}
