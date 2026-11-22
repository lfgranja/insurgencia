#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Exportação soberana de conta e personagem do Núcleo Insurgência (GDD §15.9).
//!
//! O formato é um objeto JSON com chaves ordenadas: [`exportar`] serializa por
//! `BTreeMap`, e [`importar`] valida o tipo de cada campo, recusa `versao`
//! divergente e recusa campos ausentes. Nenhuma instância guarda chave de
//! exportação: o arquivo produzido é aberto e transportável.

use std::collections::BTreeMap;

use serde_json::{Map, Value};
use thiserror::Error;

/// Versão do formato de exportação de conta e personagem.
pub const VERSAO_EXPORTACAO: u8 = 1;

/// SHA-256 da cadeia canônica `insurgencia-regras-canonicas-v1`, que identifica
/// o conjunto de regras canônico desta versão do formato.
pub const HASH_REGRAS_CANONICO: &str =
    "b333a8abb902ddadf20b44cd3d3f95e420f4d6b808d6405de61e84f263760673";

/// Falhas ao exportar ou importar uma conta.
// O nome público é fixado pela feature 025; `exportacao` é o módulo raiz.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ErroExportacao {
    /// JSON inválido ou raiz diferente de objeto.
    #[error("JSON invalido: {0}")]
    JsonInvalido(String),
    /// Campo obrigatório ausente ou com tipo inesperado.
    #[error("campo obrigatorio ausente ou invalido: {0}")]
    CampoAusente(&'static str),
    /// Versão do formato diferente da versão aceita por esta crate.
    #[error("versao de exportacao divergente: esperada {esperada}, encontrada {encontrada}")]
    VersaoDivergente {
        /// Versão aceita por esta crate.
        esperada: u8,
        /// Versão encontrada no JSON importado.
        encontrada: u8,
    },
}

/// Conta e personagem exportados em formato aberto.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContaExportada {
    /// Identificador da conta na instância de origem.
    pub conta_id: String,
    /// Identificador do personagem na instância de origem.
    pub personagem_id: String,
    /// Data de criação, preservada como texto (sem relógio nem fuso).
    pub criado_em: String,
    /// Versão do formato de exportação.
    pub versao: u8,
    /// Identificador do conjunto de regras canônico no momento da exportação.
    pub hash_regras: String,
}

/// Exporta conta e personagem como JSON determinístico.
///
/// As chaves são ordenadas por `BTreeMap`, sem espaço em branco, e o campo
/// `hash_regras` registra o conjunto de regras canônico desta versão.
///
/// # Errors
///
/// Retorna [`ErroExportacao::JsonInvalido`] se a serialização falhar.
pub fn exportar(
    conta_id: &str,
    personagem_id: &str,
    criado_em: &str,
) -> Result<String, ErroExportacao> {
    let mut dados = BTreeMap::new();
    dados.insert("conta_id", Value::String(conta_id.to_owned()));
    dados.insert("criado_em", Value::String(criado_em.to_owned()));
    dados.insert(
        "hash_regras",
        Value::String(HASH_REGRAS_CANONICO.to_owned()),
    );
    dados.insert("personagem_id", Value::String(personagem_id.to_owned()));
    dados.insert("versao", Value::from(VERSAO_EXPORTACAO));

    serde_json::to_string(&dados).map_err(|erro| ErroExportacao::JsonInvalido(erro.to_string()))
}

/// Importa um JSON de conta e personagem, validando campos e versão.
///
/// # Errors
///
/// Retorna [`ErroExportacao::JsonInvalido`] para JSON inválido ou raiz que não
/// seja objeto, [`ErroExportacao::CampoAusente`] para campo obrigatório ausente
/// ou inválido e [`ErroExportacao::VersaoDivergente`] para versão diferente de
/// [`VERSAO_EXPORTACAO`].
pub fn importar(json: &str) -> Result<ContaExportada, ErroExportacao> {
    let valor: Value = serde_json::from_str(json)
        .map_err(|erro| ErroExportacao::JsonInvalido(erro.to_string()))?;
    let objeto = valor
        .as_object()
        .ok_or_else(|| ErroExportacao::JsonInvalido("raiz nao e objeto".to_owned()))?;

    let versao = ler_versao(objeto)?;

    Ok(ContaExportada {
        conta_id: ler_texto(objeto, "conta_id")?,
        personagem_id: ler_texto(objeto, "personagem_id")?,
        criado_em: ler_texto(objeto, "criado_em")?,
        versao,
        hash_regras: ler_texto(objeto, "hash_regras")?,
    })
}

/// Lê um campo de texto obrigatório.
///
/// # Errors
///
/// Retorna [`ErroExportacao::CampoAusente`] se o campo não existir ou não for texto.
fn ler_texto(objeto: &Map<String, Value>, nome: &'static str) -> Result<String, ErroExportacao> {
    objeto
        .get(nome)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or(ErroExportacao::CampoAusente(nome))
}

/// Lê a versão e recusa qualquer valor diferente da versão aceita.
///
/// # Errors
///
/// Retorna [`ErroExportacao::CampoAusente`] se a versão estiver ausente ou fora
/// de `u8`, e [`ErroExportacao::VersaoDivergente`] se for diferente da versão
/// aceita.
fn ler_versao(objeto: &Map<String, Value>) -> Result<u8, ErroExportacao> {
    let versao = objeto
        .get("versao")
        .and_then(Value::as_u64)
        .and_then(|valor| u8::try_from(valor).ok())
        .ok_or(ErroExportacao::CampoAusente("versao"))?;

    if versao == VERSAO_EXPORTACAO {
        Ok(versao)
    } else {
        Err(ErroExportacao::VersaoDivergente {
            esperada: VERSAO_EXPORTACAO,
            encontrada: versao,
        })
    }
}

#[cfg(test)]
mod testes {
    use super::{
        exportar, importar, ContaExportada, ErroExportacao, HASH_REGRAS_CANONICO, VERSAO_EXPORTACAO,
    };

    #[test]
    fn exportar_produz_json_deterministico_e_ordenado() {
        let primeiro = match exportar("conta-1", "pers-1", "2026-11-21") {
            Ok(json) => json,
            Err(erro) => panic!("exportar falhou: {erro}"),
        };
        let segundo = match exportar("conta-1", "pers-1", "2026-11-21") {
            Ok(json) => json,
            Err(erro) => panic!("exportar falhou: {erro}"),
        };
        assert_eq!(primeiro, segundo);

        let esperado = format!(
            "{{\"conta_id\":\"conta-1\",\"criado_em\":\"2026-11-21\",\
             \"hash_regras\":\"{HASH_REGRAS_CANONICO}\",\
             \"personagem_id\":\"pers-1\",\"versao\":{VERSAO_EXPORTACAO}}}"
        );
        assert_eq!(primeiro, esperado);
    }

    #[test]
    fn exportar_e_importar_preservam_os_campos() {
        let json = match exportar("conta-2", "pers-2", "2026-11-20T23:59:00Z") {
            Ok(json) => json,
            Err(erro) => panic!("exportar falhou: {erro}"),
        };
        let conta = match importar(&json) {
            Ok(conta) => conta,
            Err(erro) => panic!("importar falhou: {erro}"),
        };
        assert_eq!(
            conta,
            ContaExportada {
                conta_id: "conta-2".to_owned(),
                personagem_id: "pers-2".to_owned(),
                criado_em: "2026-11-20T23:59:00Z".to_owned(),
                versao: VERSAO_EXPORTACAO,
                hash_regras: HASH_REGRAS_CANONICO.to_owned(),
            }
        );
    }

    #[test]
    fn importar_recusa_versao_divergente() {
        let divergente = VERSAO_EXPORTACAO + 1;
        let json = format!(
            "{{\"conta_id\":\"c\",\"criado_em\":\"d\",\"hash_regras\":\"h\",\
             \"personagem_id\":\"p\",\"versao\":{divergente}}}"
        );
        match importar(&json) {
            Ok(_) => panic!("versao divergente deveria ser recusada"),
            Err(ErroExportacao::VersaoDivergente {
                esperada,
                encontrada,
            }) => {
                assert_eq!(esperada, VERSAO_EXPORTACAO);
                assert_eq!(encontrada, divergente);
            }
            Err(erro) => panic!("erro inesperado: {erro}"),
        }
    }

    #[test]
    fn importar_recusa_campo_ausente() {
        let json = format!(
            "{{\"conta_id\":\"c\",\"criado_em\":\"d\",\"hash_regras\":\"h\",\
             \"versao\":{VERSAO_EXPORTACAO}}}"
        );
        match importar(&json) {
            Ok(_) => panic!("campo ausente deveria ser recusado"),
            Err(ErroExportacao::CampoAusente(nome)) => assert_eq!(nome, "personagem_id"),
            Err(erro) => panic!("erro inesperado: {erro}"),
        }
    }

    #[test]
    fn importar_recusa_json_invalido_e_raiz_nao_objeto() {
        assert!(matches!(
            importar("{nao e json"),
            Err(ErroExportacao::JsonInvalido(_))
        ));
        assert!(matches!(
            importar("[]"),
            Err(ErroExportacao::JsonInvalido(_))
        ));
    }

    #[test]
    fn importar_recusa_campos_com_tipos_invalidos() {
        let json_texto = format!(
            "{{\"conta_id\":7,\"criado_em\":\"d\",\"hash_regras\":\"h\",\
             \"personagem_id\":\"p\",\"versao\":{VERSAO_EXPORTACAO}}}"
        );
        assert!(matches!(
            importar(&json_texto),
            Err(ErroExportacao::CampoAusente("conta_id"))
        ));

        let json_versao = "{\"conta_id\":\"c\",\"criado_em\":\"d\",\"hash_regras\":\"h\",\
             \"personagem_id\":\"p\",\"versao\":\"1\"}";
        assert!(matches!(
            importar(json_versao),
            Err(ErroExportacao::CampoAusente("versao"))
        ));
    }
}
