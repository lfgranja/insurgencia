#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use insurgencia_contratos::manifesto::{
    conferir, validar_formato, ErroCompatibilidade, ManifestoCompatibilidade, REGRAS_AUSENTES,
};
use insurgencia_contratos::{
    MODULO_ID, PROTOCOL_VERSION, REJEICAO_HASH, REJEICAO_MANIFESTO, REJEICAO_MODULO,
    REJEICAO_VERSAO,
};
use serde_json::{json, Value};

const CONTEUDO_HASH: &str = "b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746";

fn manifesto_original() -> Value {
    json!({
        "manifest_version": "1.0.0",
        "protocol_version": PROTOCOL_VERSION,
        "module_id": MODULO_ID,
        "content_hash": CONTEUDO_HASH,
        "checked_at": "2026-11-19T00:00:00Z",
        "tool_version": "4.7.2.stable.official",
    })
}

fn codigo_falha(resultado: Result<ManifestoCompatibilidade, ErroCompatibilidade>) -> &'static str {
    match resultado {
        Err(erro) => erro.codigo(),
        Ok(_) => panic!("era esperada uma recusa de compatibilidade"),
    }
}

#[test]
fn manifesto_original_aceito() {
    let json = manifesto_original().to_string();
    let resultado = conferir(&json, MODULO_ID, PROTOCOL_VERSION, Some(CONTEUDO_HASH));
    let manifesto = match resultado {
        Ok(manifesto) => manifesto,
        Err(erro) => panic!("manifesto original deveria ser aceito: {erro}"),
    };

    assert_eq!(manifesto.manifest_version, "1.0.0");
    assert_eq!(manifesto.protocol_version, PROTOCOL_VERSION);
    assert_eq!(manifesto.module_id, MODULO_ID);
    assert_eq!(manifesto.content_hash, CONTEUDO_HASH);
    assert_eq!(manifesto.checked_at, "2026-11-19T00:00:00Z");
    assert_eq!(manifesto.tool_version, "4.7.2.stable.official");
}

#[test]
fn manifesto_ausente_recusado() {
    assert_eq!(
        codigo_falha(conferir(
            "",
            MODULO_ID,
            PROTOCOL_VERSION,
            Some(CONTEUDO_HASH)
        )),
        REJEICAO_MANIFESTO
    );
    assert_eq!(
        codigo_falha(conferir(
            "{}",
            MODULO_ID,
            PROTOCOL_VERSION,
            Some(CONTEUDO_HASH)
        )),
        "MANIFEST_INVALID"
    );
    assert!(matches!(
        validar_formato("null"),
        Err(ErroCompatibilidade::ManifestoInvalido)
    ));

    let mut hash_curto = manifesto_original();
    hash_curto["content_hash"] =
        json!("000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(
        codigo_falha(conferir(
            &hash_curto.to_string(),
            MODULO_ID,
            PROTOCOL_VERSION,
            Some(CONTEUDO_HASH)
        )),
        REJEICAO_MANIFESTO
    );
}

#[test]
fn campo_checked_at_removido_recusado() {
    let incompleto = json!({
        "manifest_version": "1.0.0",
        "protocol_version": PROTOCOL_VERSION,
        "module_id": MODULO_ID,
        "content_hash": CONTEUDO_HASH,
        "tool_version": "4.7.2.stable.official",
    });
    assert_eq!(
        codigo_falha(conferir(
            &incompleto.to_string(),
            MODULO_ID,
            PROTOCOL_VERSION,
            Some(CONTEUDO_HASH)
        )),
        REJEICAO_MANIFESTO
    );
}

#[test]
fn divergencia_de_versao_falha() {
    let mut divergente = manifesto_original();
    divergente["protocol_version"] = json!(u32::from(PROTOCOL_VERSION) + 1);
    assert_eq!(
        codigo_falha(conferir(
            &divergente.to_string(),
            MODULO_ID,
            PROTOCOL_VERSION,
            Some(CONTEUDO_HASH)
        )),
        REJEICAO_VERSAO
    );

    // Versão é verificada antes do módulo: prevalece mesmo com module_id divergente.
    divergente["module_id"] = json!("");
    assert_eq!(
        codigo_falha(conferir(
            &divergente.to_string(),
            MODULO_ID,
            PROTOCOL_VERSION,
            Some(CONTEUDO_HASH)
        )),
        REJEICAO_VERSAO
    );
}

#[test]
fn divergencia_de_hash_falha() {
    let hash_errado = "0000000000000000000000000000000000000000000000000000000000000000";
    let mut divergente = manifesto_original();
    divergente["content_hash"] = json!(hash_errado);
    assert_eq!(
        codigo_falha(conferir(
            &divergente.to_string(),
            MODULO_ID,
            PROTOCOL_VERSION,
            Some(CONTEUDO_HASH)
        )),
        REJEICAO_HASH
    );
}

#[test]
fn modulo_ausente_falha() {
    let mut sem_modulo = manifesto_original();
    sem_modulo["module_id"] = json!("");
    assert_eq!(
        codigo_falha(conferir(
            &sem_modulo.to_string(),
            MODULO_ID,
            PROTOCOL_VERSION,
            Some(CONTEUDO_HASH)
        )),
        REJEICAO_MODULO
    );
}

#[test]
fn arquivo_de_regras_ausente_falha() {
    assert_eq!(REGRAS_AUSENTES, "REGRAS_AUSENTES");
    let json = manifesto_original().to_string();
    assert_eq!(
        codigo_falha(conferir(&json, MODULO_ID, PROTOCOL_VERSION, None)),
        REGRAS_AUSENTES
    );
}
