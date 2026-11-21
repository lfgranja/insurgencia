#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use insurgencia_sim::{CodigoRecusa, Comando, Mundo, ValidacaoComandos};

/// Monta um comando `mover` com os monotônicos já preenchidos.
#[must_use]
fn comando_mover(peer_id: u64, seq: i64, request_id: i64, destino: (f64, f64)) -> Comando {
    Comando {
        peer_id,
        acao: "mover".to_string(),
        seq,
        request_id,
        destino: Some(destino),
    }
}

#[test]
fn fases_prepare_skills_finish() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);

    assert_eq!(mundo.tick, 0);
    assert_eq!(mundo.epoch, 1);
    assert_eq!(mundo.match_id, 1);
    assert!(mundo.comandos_pendentes.is_empty());
    assert!(mundo.comandos_do_tick.is_empty());

    mundo.agendar_comando(comando_mover(1, 1, 1, (0.25, 0.0)));
    assert_eq!(mundo.comandos_pendentes.len(), 1);
    assert!(mundo.comandos_do_tick.is_empty());

    mundo.avancar_tick();

    assert_eq!(mundo.tick, 1);
    assert!(mundo.comandos_pendentes.is_empty());
    assert!(mundo.comandos_do_tick.is_empty());

    let jogador = &mundo.jogadores[&1];
    assert!((jogador.posicao.0 - 0.25).abs() < 1e-12);
    assert_eq!(jogador.seq, 1);
    assert_eq!(jogador.request_id, 1);
}

#[test]
fn movimento_aplica_destino() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(10);
    mundo.agendar_comando(comando_mover(10, 1, 1, (0.5, -0.25)));
    mundo.avancar_tick();

    let jogador = &mundo.jogadores[&10];
    assert!((jogador.posicao.0 - 0.5).abs() < 1e-12);
    assert!((jogador.posicao.1 + 0.25).abs() < 1e-12);
}

#[test]
fn replay_de_seq_e_request_id() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(2);
    mundo.agendar_comando(comando_mover(2, 5, 9, (0.0, 0.0)));
    mundo.avancar_tick();

    let replay_seq = ValidacaoComandos::validar(&mundo, 2, "mover", 5, 10, Some((0.1, 0.0)));
    assert_eq!(replay_seq, Err(CodigoRecusa::SeqOuRequestReplay));

    let replay_request = ValidacaoComandos::validar(&mundo, 2, "mover", 6, 9, Some((0.1, 0.0)));
    assert_eq!(replay_request, Err(CodigoRecusa::SeqOuRequestReplay));

    let aceito = ValidacaoComandos::validar(&mundo, 2, "mover", 6, 10, Some((0.1, 0.0)));
    assert!(aceito.is_ok());
}

#[test]
fn acao_desconhecida() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(3);

    let forjado = ValidacaoComandos::validar(&mundo, 3, "creditar_materiais", 1, 1, None);
    assert_eq!(forjado, Err(CodigoRecusa::AcaoDesconhecida));

    let desconhecida = ValidacaoComandos::validar(&mundo, 3, "voar", 1, 1, None);
    assert_eq!(desconhecida, Err(CodigoRecusa::AcaoDesconhecida));
}

#[test]
fn limite_de_movimento() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(4);

    let no_limite = ValidacaoComandos::validar(&mundo, 4, "mover", 1, 1, Some((0.6, 0.8)));
    assert!(no_limite.is_ok());

    let fora_do_limite = ValidacaoComandos::validar(&mundo, 4, "mover", 2, 2, Some((1.5, 0.0)));
    assert_eq!(fora_do_limite, Err(CodigoRecusa::MovimentoForaDoLimite));

    let sem_destino = ValidacaoComandos::validar(&mundo, 4, "mover", 3, 3, None);
    assert_eq!(sem_destino, Err(CodigoRecusa::DestinoInvalido));
}

#[test]
fn peer_nao_autorizado() {
    let mundo = Mundo::new();
    let resultado = ValidacaoComandos::validar(&mundo, 99, "mover", 1, 1, Some((0.0, 0.0)));
    assert_eq!(resultado, Err(CodigoRecusa::PeerNaoAutorizado));

    let mut mundo_com_outro = Mundo::new();
    mundo_com_outro.adicionar_jogador(1);
    let outro = ValidacaoComandos::validar(&mundo_com_outro, 99, "mover", 1, 1, Some((0.0, 0.0)));
    assert_eq!(outro, Err(CodigoRecusa::PeerNaoAutorizado));
}

#[test]
fn hash_deterministico() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(7);
    mundo.agendar_comando(comando_mover(7, 1, 1, (0.5, 0.5)));
    mundo.avancar_tick();

    let primeiro = mundo.hash_estado();
    let segundo = mundo.hash_estado();
    assert_eq!(primeiro, segundo);
    assert_eq!(primeiro.len(), 64);

    let mut outro = Mundo::new();
    outro.adicionar_jogador(7);
    outro.agendar_comando(comando_mover(7, 1, 1, (0.5, 0.5)));
    outro.avancar_tick();
    outro.avancar_tick();
    let diferente = outro.hash_estado();
    assert_ne!(diferente, primeiro);
}

#[test]
fn determinismo_de_duas_simulacoes_identicas() {
    let mut primeira = Mundo::new();
    let mut segunda = Mundo::new();

    for mundo in [&mut primeira, &mut segunda] {
        mundo.adicionar_jogador(1);
        mundo.adicionar_jogador(2);
        mundo.agendar_comando(comando_mover(1, 1, 1, (0.5, 0.0)));
        mundo.agendar_comando(comando_mover(2, 1, 1, (-0.25, 0.75)));
        for _ in 0..10 {
            mundo.avancar_tick();
        }
    }

    assert_eq!(primeira.hash_estado(), segunda.hash_estado());
}
