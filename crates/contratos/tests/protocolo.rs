#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use insurgencia_contratos::{
    normalizar_pacote, ErroProtocolo, SnapshotMeta, TipoPacote, MODULO_ID, PROTOCOL_VERSION,
    REJEICAO_HASH, REJEICAO_MANIFESTO, REJEICAO_MODULO, REJEICAO_VERSAO,
};
use serde_json::Value;
use std::collections::BTreeMap;

#[test]
fn constantes_do_contrato() {
    assert_eq!(PROTOCOL_VERSION, 1);
    assert_eq!(MODULO_ID, "insurgencia.exemplo");
}

#[test]
fn try_from_aceita_os_seis_discriminantes() {
    assert_eq!(TipoPacote::try_from(1), Ok(TipoPacote::Join));
    assert_eq!(TipoPacote::try_from(2), Ok(TipoPacote::JoinAceito));
    assert_eq!(TipoPacote::try_from(3), Ok(TipoPacote::JoinRecusado));
    assert_eq!(TipoPacote::try_from(4), Ok(TipoPacote::Comando));
    assert_eq!(TipoPacote::try_from(5), Ok(TipoPacote::Snapshot));
    assert_eq!(TipoPacote::try_from(6), Ok(TipoPacote::EventoDesafio));
}

#[test]
fn try_from_recusa_valores_fora_de_um_a_seis() {
    let zero = TipoPacote::try_from(0);
    let sete = TipoPacote::try_from(7);

    assert!(zero.is_err());
    assert!(sete.is_err());
    assert!(matches!(
        zero,
        Err(ErroProtocolo::TipoPacoteDesconhecido(0))
    ));
    assert!(matches!(
        sete,
        Err(ErroProtocolo::TipoPacoteDesconhecido(7))
    ));
}

#[test]
fn codigos_de_recusa_sao_os_literais_do_contrato() {
    assert_eq!(REJEICAO_VERSAO, "VERSAO_PROTOCOLO_DIVERGENTE");
    assert_eq!(REJEICAO_HASH, "HASH_MODULO_DIVERGENTE");
    assert_eq!(REJEICAO_MODULO, "MODULO_AUSENTE");
    assert_eq!(REJEICAO_MANIFESTO, "MANIFEST_INVALID");
}

#[test]
fn snapshot_meta_to_dict_tem_cinco_campos_corretos() {
    let meta = SnapshotMeta::new(1, 42, 7, 99, String::from("abc123"));
    let dicionario = meta.to_dict();
    let esperado = BTreeMap::from([
        (String::from("protocol_version"), String::from("1")),
        (String::from("epoch"), String::from("42")),
        (String::from("match_id"), String::from("7")),
        (String::from("tick"), String::from("99")),
        (String::from("content_hash"), String::from("abc123")),
    ]);

    assert_eq!(dicionario.len(), 5);
    assert_eq!(dicionario, esperado);
}

#[test]
fn normalizar_pacote_preserva_conhecido_descarta_forjado_e_insere_tipo() {
    let dados = BTreeMap::from([
        (
            String::from("protocol_version"),
            Value::from(PROTOCOL_VERSION),
        ),
        (String::from("module_id"), Value::from(MODULO_ID)),
        (String::from("content_hash"), Value::from("abc123")),
        (String::from("tipo"), Value::from(99)),
        (String::from("campo_forjado"), Value::from("injetado")),
    ]);
    let resultado = normalizar_pacote(1, dados);

    assert_eq!(
        resultado.get("protocol_version"),
        Some(&Value::from(PROTOCOL_VERSION))
    );
    assert_eq!(resultado.get("module_id"), Some(&Value::from(MODULO_ID)));
    assert_eq!(resultado.get("content_hash"), Some(&Value::from("abc123")));
    assert_eq!(resultado.get("tipo"), Some(&Value::from(1)));
    assert!(!resultado.contains_key("campo_forjado"));
    assert_eq!(resultado.len(), 4);
}

#[test]
fn mesma_entrada_produz_mesmo_mapa_em_duas_chamadas() {
    let dados = BTreeMap::from([
        (String::from("tick"), Value::from(12)),
        (String::from("epoch"), Value::from(3)),
        (String::from("match_id"), Value::from(8)),
        (String::from("campo_forjado"), Value::from(true)),
    ]);
    let primeiro = normalizar_pacote(5, dados.clone());
    let segundo = normalizar_pacote(5, dados);

    assert_eq!(primeiro, segundo);

    let meta = SnapshotMeta::new(PROTOCOL_VERSION, 3, 8, 12, String::from("abc123"));
    let primeiro_dict = meta.to_dict();
    let segundo_dict = meta.to_dict();

    assert_eq!(primeiro_dict, segundo_dict);
}
