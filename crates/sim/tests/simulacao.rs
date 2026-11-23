#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use insurgencia_sim::{CodigoRecusa, Comando, Mundo, ValidacaoComandos, TICKS_COLETA};

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

/// Monta um comando `coletar_materiais` com os monotônicos já preenchidos.
#[must_use]
fn comando_coleta(peer_id: u64, seq: i64, request_id: i64, ponto: (f64, f64)) -> Comando {
    Comando {
        peer_id,
        acao: "coletar_materiais".to_string(),
        seq,
        request_id,
        destino: Some(ponto),
    }
}

/// Agenda um `mover` somente após validá-lo contra o estado corrente.
fn agendar_mover(mundo: &mut Mundo, peer_id: u64, seq: i64, request_id: i64, destino: (f64, f64)) {
    assert!(
        ValidacaoComandos::validar(mundo, peer_id, "mover", seq, request_id, Some(destino)).is_ok()
    );
    mundo.agendar_comando(comando_mover(peer_id, seq, request_id, destino));
}

/// Agenda uma coleta somente após validá-la contra o estado corrente.
fn agendar_coleta(mundo: &mut Mundo, peer_id: u64, seq: i64, request_id: i64, ponto: (f64, f64)) {
    assert!(ValidacaoComandos::validar(
        mundo,
        peer_id,
        "coletar_materiais",
        seq,
        request_id,
        Some(ponto),
    )
    .is_ok());
    mundo.agendar_comando(comando_coleta(peer_id, seq, request_id, ponto));
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

#[test]
fn coleta_apos_180_ticks() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.5, 0.25);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].coleta_ponto, None);

    for seq in 2..=TICKS_COLETA {
        agendar_coleta(&mut mundo, 1, seq, seq, ponto);
        mundo.avancar_tick();
    }
    let seq = 1 + TICKS_COLETA;

    {
        let jogador = &mundo.jogadores[&1];
        assert_eq!(jogador.coleta_ticks, TICKS_COLETA - 1);
        assert_eq!(jogador.coleta_ponto, Some(ponto));
        assert_eq!(jogador.materiais, 0);
        assert_eq!(jogador.suprimentos, 0);
    }

    agendar_coleta(&mut mundo, 1, seq, seq, ponto);
    mundo.avancar_tick();
    {
        let jogador = &mundo.jogadores[&1];
        assert_eq!(jogador.coleta_ticks, 0);
        assert_eq!(jogador.coleta_ponto, Some(ponto));
        assert_eq!(jogador.materiais, 2);
        assert_eq!(jogador.suprimentos, 3);
    }

    // O ponto permanece fixado, permitindo renda contínua a cada 180 ticks.
    let proximo_seq = 2 + TICKS_COLETA;
    for seq in proximo_seq..(proximo_seq + TICKS_COLETA) {
        agendar_coleta(&mut mundo, 1, seq, seq, ponto);
        mundo.avancar_tick();
    }
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.coleta_ticks, 0);
    assert_eq!(jogador.coleta_ponto, Some(ponto));
    assert_eq!(jogador.materiais, 4);
    assert_eq!(jogador.suprimentos, 6);
}

#[test]
fn dupla_coleta_no_mesmo_tick_nao_acelera_credito() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.25, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    let mut seq = 2_i64;
    // Dois comandos válidos por tick não podem valer dois ticks de permanência.
    for tick in 0..(TICKS_COLETA - 1) {
        agendar_coleta(&mut mundo, 1, seq, seq, ponto);
        seq += 1;
        agendar_coleta(&mut mundo, 1, seq, seq, ponto);
        seq += 1;
        mundo.avancar_tick();
        let jogador = &mundo.jogadores[&1];
        assert_eq!(jogador.coleta_ticks, tick + 1, "progresso no tick {tick}");
        assert_eq!(jogador.materiais, 0, "credito antecipado no tick {tick}");
        assert_eq!(jogador.suprimentos, 0, "credito antecipado no tick {tick}");
    }

    agendar_coleta(&mut mundo, 1, seq, seq, ponto);
    seq += 1;
    agendar_coleta(&mut mundo, 1, seq, seq, ponto);
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.coleta_ticks, 0);
    assert_eq!(jogador.coleta_ponto, Some(ponto));
    assert_eq!(jogador.materiais, 2);
    assert_eq!(jogador.suprimentos, 3);
    assert_eq!(jogador.seq, seq);
    assert_eq!(jogador.request_id, seq);
}

#[test]
fn saida_antes_zera() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let origem = (0.25, 0.0);
    let fora = (0.25, 0.5);
    agendar_mover(&mut mundo, 1, 1, 1, origem);
    mundo.avancar_tick();

    for seq in 2..(2 + 100) {
        agendar_coleta(&mut mundo, 1, seq, seq, origem);
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&1].coleta_ticks, 100);

    let mut seq = 2 + 100;

    agendar_mover(&mut mundo, 1, seq, seq, fora);
    mundo.avancar_tick();
    seq += 1;
    {
        let jogador = &mundo.jogadores[&1];
        assert_eq!(jogador.coleta_ticks, 0);
        assert_eq!(jogador.coleta_ponto, None);
        assert_eq!(jogador.materiais, 0);
        assert_eq!(jogador.suprimentos, 0);
    }

    agendar_mover(&mut mundo, 1, seq, seq, origem);
    mundo.avancar_tick();
    seq += 1;
    assert_eq!(mundo.jogadores[&1].coleta_ponto, None);

    for seq in seq..(seq + TICKS_COLETA) {
        agendar_coleta(&mut mundo, 1, seq, seq, origem);
        mundo.avancar_tick();
    }
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.coleta_ticks, 0);
    assert_eq!(jogador.coleta_ponto, Some(origem));
    assert_eq!(jogador.materiais, 2);
    assert_eq!(jogador.suprimentos, 3);
}

#[test]
fn recusa_sem_destino_e_fora_do_ponto() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    agendar_mover(&mut mundo, 1, 1, 1, (0.25, 0.0));
    mundo.avancar_tick();

    let sem_destino = ValidacaoComandos::validar(&mundo, 1, "coletar_materiais", 2, 2, None);
    assert_eq!(sem_destino, Err(CodigoRecusa::DestinoInvalido));

    let fora_do_ponto =
        ValidacaoComandos::validar(&mundo, 1, "coletar_materiais", 2, 2, Some((0.75, 0.0)));
    assert_eq!(fora_do_ponto, Err(CodigoRecusa::DestinoInvalido));

    let aceito =
        ValidacaoComandos::validar(&mundo, 1, "coletar_materiais", 2, 2, Some((0.25, 0.0)));
    assert!(aceito.is_ok());

    // Comandos recusados não devem mutar saldos nem progresso.
    for (seq, destino) in [(2_i64, None), (3_i64, Some((0.75_f64, 0.0)))] {
        mundo.agendar_comando(Comando {
            peer_id: 1,
            acao: "coletar_materiais".to_string(),
            seq,
            request_id: seq,
            destino,
        });
        mundo.avancar_tick();
    }

    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.materiais, 0);
    assert_eq!(jogador.suprimentos, 0);
    assert_eq!(jogador.coleta_ticks, 0);
    assert_eq!(jogador.coleta_ponto, None);
}

#[test]
fn hash_saldos_divergentes() {
    let mut com_saldo = Mundo::new();
    com_saldo.adicionar_jogador(1);
    let mut sem_saldo = Mundo::new();
    sem_saldo.adicionar_jogador(1);
    assert_eq!(com_saldo.hash_estado(), sem_saldo.hash_estado());

    if let Some(jogador) = com_saldo.jogadores.get_mut(&1) {
        jogador.materiais = 2;
        jogador.suprimentos = 3;
    }
    let com_materiais = com_saldo.hash_estado();
    assert_ne!(com_materiais, sem_saldo.hash_estado());
    assert_eq!(com_materiais.len(), 64);

    if let Some(jogador) = com_saldo.jogadores.get_mut(&1) {
        jogador.coleta_ticks = 179;
        jogador.coleta_ponto = Some((0.5, 0.0));
    }
    assert_ne!(com_saldo.hash_estado(), com_materiais);
}

#[test]
fn ausencia_consumo_parcial() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.materiais = 10;
        jogador.suprimentos = 7;
    }
    let ponto = (0.5, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    for seq in 2..=TICKS_COLETA {
        agendar_coleta(&mut mundo, 1, seq, seq, ponto);
        mundo.avancar_tick();
        let jogador = &mundo.jogadores[&1];
        assert_eq!(jogador.materiais, 10);
        assert_eq!(jogador.suprimentos, 7);
    }
    let seq = 1 + TICKS_COLETA;

    agendar_coleta(&mut mundo, 1, seq, seq, ponto);
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.materiais, 12);
    assert_eq!(jogador.suprimentos, 10);
    assert!(jogador.materiais >= 0);
    assert!(jogador.suprimentos >= 0);
}

#[test]
fn determinismo_coleta() {
    let mut primeira = Mundo::new();
    let mut segunda = Mundo::new();
    for mundo in [&mut primeira, &mut segunda] {
        mundo.adicionar_jogador(2);
        mundo.adicionar_jogador(1);
    }
    agendar_mover(&mut primeira, 1, 1, 1, (0.25, 0.0));
    agendar_mover(&mut primeira, 2, 1, 1, (0.0, 0.25));
    agendar_mover(&mut segunda, 2, 1, 1, (0.0, 0.25));
    agendar_mover(&mut segunda, 1, 1, 1, (0.25, 0.0));
    primeira.avancar_tick();
    segunda.avancar_tick();
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    for seq in 2..(2 + TICKS_COLETA) {
        agendar_coleta(&mut primeira, 1, seq, seq, (0.25, 0.0));
        agendar_coleta(&mut primeira, 2, seq, seq, (0.0, 0.25));
        agendar_coleta(&mut segunda, 2, seq, seq, (0.0, 0.25));
        agendar_coleta(&mut segunda, 1, seq, seq, (0.25, 0.0));
        primeira.avancar_tick();
        segunda.avancar_tick();
        assert_eq!(primeira.hash_estado(), segunda.hash_estado());
    }

    for peer_id in [1, 2] {
        let jogador = &primeira.jogadores[&peer_id];
        assert_eq!(jogador.materiais, 2);
        assert_eq!(jogador.suprimentos, 3);
        assert_eq!(jogador.coleta_ticks, 0);
    }
}
