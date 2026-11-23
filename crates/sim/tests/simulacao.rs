#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use insurgencia_sim::{
    Barreira, CodigoRecusa, Comando, Funcao, Mundo, Ponte, ResultadoPartida, ValidacaoComandos,
    ALCANCE_ANALISE_MATERIALISTA_M, ALCANCE_ATAQUE_PNJ_M, BONUS_DANO_ANALISE_PERCENT,
    CONTRIBUICAO_NO, CUSTO_ANALISE_MATERIALISTA, CUSTO_ATAQUE_FOLEGO, CUSTO_BARREIRA_MATERIAIS,
    CUSTO_PONTE_MATERIAIS, DANO_ATAQUE, DANO_ATAQUE_PNJ, DANO_MAXIMO_NOS, FOLEGO_MAXIMO,
    MAX_BARREIRAS_POR_JOGADOR, MAX_RECONQUISTAS, NIVEL_MAXIMO, PONTOS_AUTOGESTAO_POR_DEFESA,
    PV_BARREIRA, PV_PNJ, PV_PONTE, RECARGA_ANALISE_MATERIALISTA_TICKS, RESISTENCIA_INICIAL,
    TICKS_ANALISE_MATERIALISTA, TICKS_ATAQUE_PNJ, TICKS_AUTOGESTAO_POR_PONTO, TICKS_COLETA,
    TICKS_COMBATE, TICKS_CONSTRUCAO_BARREIRA, TICKS_CONSTRUCAO_PONTE, TICKS_CONTROLE_ILHA,
    TICKS_DURACAO_BARREIRA, TICKS_DURACAO_PONTE, TICKS_INATIVIDADE_NO, TICKS_LIMITE_PARTIDA,
    TICKS_POR_NIVEL, TICKS_PROPAGANDA, TICKS_PROPAGANDA_NO, TICKS_REGENERACAO_COMBATE,
    TICKS_REGENERACAO_FORA_COMBATE, TICKS_REVERSAO_SUDITO, TOTAL_PNJS_PADRAO,
};

/// Eleva o nível do jogador ao mínimo exigido por uma ação (protótipo).
fn garantir_nivel(mundo: &mut Mundo, peer_id: u64, minimo: u8) {
    let Some(jogador) = mundo.jogadores.get_mut(&peer_id) else {
        return;
    };
    if jogador.nivel < minimo {
        jogador.nivel = minimo;
        jogador.nivel_ticks = 0;
    }
}

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

/// Monta um comando `atacar_forte` com os monotônicos já preenchidos.
#[must_use]
fn comando_ataque(peer_id: u64, seq: i64, request_id: i64, destino: (f64, f64)) -> Comando {
    Comando {
        peer_id,
        acao: "atacar_forte".to_string(),
        seq,
        request_id,
        destino: Some(destino),
    }
}

/// Agenda um ataque somente após validá-lo contra o estado corrente.
fn agendar_ataque(mundo: &mut Mundo, peer_id: u64, seq: i64, request_id: i64, destino: (f64, f64)) {
    assert!(ValidacaoComandos::validar(
        mundo,
        peer_id,
        "atacar_forte",
        seq,
        request_id,
        Some(destino),
    )
    .is_ok());
    mundo.agendar_comando(comando_ataque(peer_id, seq, request_id, destino));
}

/// Monta um comando `atacar_pnj` com os monotônicos já preenchidos.
#[must_use]
fn comando_ataque_pnj(peer_id: u64, seq: i64, request_id: i64, ponto: (f64, f64)) -> Comando {
    Comando {
        peer_id,
        acao: "atacar_pnj".to_string(),
        seq,
        request_id,
        destino: Some(ponto),
    }
}

/// Monta um mundo com jogador de nível 3 e um PNJ ativo na origem.
#[must_use]
fn mundo_para_analise() -> Mundo {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    garantir_nivel(&mut mundo, 1, 3);
    mundo.adicionar_pnj(1, (0.0, 0.0));
    mundo
}

/// Monta um comando `analise_materialista` com os monotônicos preenchidos.
#[must_use]
fn comando_analise(peer_id: u64, seq: i64, request_id: i64, ponto: (f64, f64)) -> Comando {
    Comando {
        peer_id,
        acao: "analise_materialista".to_string(),
        seq,
        request_id,
        destino: Some(ponto),
    }
}

/// Agenda uma análise somente após validá-la contra o estado corrente.
fn agendar_analise(mundo: &mut Mundo, peer_id: u64, seq: i64, request_id: i64, ponto: (f64, f64)) {
    assert!(ValidacaoComandos::validar(
        mundo,
        peer_id,
        "analise_materialista",
        seq,
        request_id,
        Some(ponto),
    )
    .is_ok());
    mundo.agendar_comando(comando_analise(peer_id, seq, request_id, ponto));
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
        assert_eq!(jogador.coleta_ponto, None);
        assert_eq!(jogador.materiais, 2);
        assert_eq!(jogador.suprimentos, 3);
    }

    // A conclusão libera o ponto; um novo comando reinicia o ciclo de 180 ticks.
    let proximo_seq = 2 + TICKS_COLETA;
    for seq in proximo_seq..(proximo_seq + TICKS_COLETA) {
        agendar_coleta(&mut mundo, 1, seq, seq, ponto);
        mundo.avancar_tick();
    }
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.coleta_ticks, 0);
    assert_eq!(jogador.coleta_ponto, None);
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
    assert_eq!(jogador.coleta_ponto, None);
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
    assert_eq!(jogador.coleta_ponto, None);
    assert_eq!(jogador.materiais, 2);
    assert_eq!(jogador.suprimentos, 3);
}

#[test]
fn saida_no_tick_179_nao_credita() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let origem = (0.5, 0.25);
    let fora = (0.25, 0.5);
    agendar_mover(&mut mundo, 1, 1, 1, origem);
    mundo.avancar_tick();

    // Comandos sustentam a canalização até restar um tick para o crédito.
    for seq in 2..TICKS_COLETA {
        agendar_coleta(&mut mundo, 1, seq, seq, origem);
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&1].coleta_ticks, TICKS_COLETA - 2);

    // Sem comando, o auto-avanço sustenta o tick 179 (TICKS_COLETA - 1).
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].coleta_ticks, TICKS_COLETA - 1);
    assert_eq!(mundo.jogadores[&1].materiais, 0);
    assert_eq!(mundo.jogadores[&1].suprimentos, 0);

    // A saída no tick 179 é aplicada antes do crédito automático.
    agendar_mover(&mut mundo, 1, TICKS_COLETA, TICKS_COLETA, fora);
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.posicao, fora);
    assert_eq!(jogador.coleta_ticks, 0);
    assert_eq!(jogador.coleta_ponto, None);
    assert_eq!(jogador.materiais, 0);
    assert_eq!(jogador.suprimentos, 0);
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

#[test]
fn coleta_continua_apos_um_unico_comando() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.5, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    // Um único comando válido inicia a coleta; os demais ticks não enviam comandos.
    agendar_coleta(&mut mundo, 1, 2, 2, ponto);
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].coleta_ticks, 1);
    assert_eq!(mundo.jogadores[&1].coleta_ponto, Some(ponto));

    for ticks in 2..TICKS_COLETA {
        mundo.avancar_tick();
        let jogador = &mundo.jogadores[&1];
        assert_eq!(jogador.coleta_ticks, ticks, "progresso em {ticks} ticks");
        assert_eq!(jogador.coleta_ponto, Some(ponto));
        assert_eq!(jogador.materiais, 0);
        assert_eq!(jogador.suprimentos, 0);
    }

    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.coleta_ticks, 0);
    assert_eq!(jogador.coleta_ponto, None);
    assert_eq!(jogador.materiais, 2);
    assert_eq!(jogador.suprimentos, 3);
}

/// Monta um comando `construir_barreira` com os monotônicos já preenchidos.
#[must_use]
fn comando_barreira(peer_id: u64, seq: i64, request_id: i64, ponto: (f64, f64)) -> Comando {
    Comando {
        peer_id,
        acao: "construir_barreira".to_string(),
        seq,
        request_id,
        destino: Some(ponto),
    }
}

/// Agenda uma construção somente após validá-la contra o estado corrente.
fn agendar_barreira(mundo: &mut Mundo, peer_id: u64, seq: i64, request_id: i64, ponto: (f64, f64)) {
    garantir_nivel(mundo, peer_id, 2);
    assert!(ValidacaoComandos::validar(
        mundo,
        peer_id,
        "construir_barreira",
        seq,
        request_id,
        Some(ponto),
    )
    .is_ok());
    mundo.agendar_comando(comando_barreira(peer_id, seq, request_id, ponto));
}

/// Conclui uma barricada em `ponto`, começando em `seq_base`.
fn concluir_barreira(mundo: &mut Mundo, peer_id: u64, seq_base: i64, ponto: (f64, f64)) {
    for offset in 0..TICKS_CONSTRUCAO_BARREIRA {
        let seq = seq_base + offset;
        agendar_barreira(mundo, peer_id, seq, seq, ponto);
        mundo.avancar_tick();
    }
}

/// Define diretamente o saldo de materiais de um jogador.
fn definir_materiais(mundo: &mut Mundo, peer_id: u64, materiais: i64) {
    if let Some(jogador) = mundo.jogadores.get_mut(&peer_id) {
        jogador.materiais = materiais;
    }
}

/// Monta uma barreira direta para verificações de hash.
#[must_use]
fn barreira_teste(
    id: u64,
    dono: u64,
    posicao: (f64, f64),
    pv: i64,
    ticks_restantes: i64,
) -> Barreira {
    Barreira {
        id,
        dono,
        posicao,
        pv,
        ticks_restantes,
        defesa_ticks: 0,
    }
}

/// Monta um comando `construir_ponte` com os monotônicos já preenchidos.
#[must_use]
fn comando_ponte(peer_id: u64, seq: i64, request_id: i64, ponto: (f64, f64)) -> Comando {
    Comando {
        peer_id,
        acao: "construir_ponte".to_string(),
        seq,
        request_id,
        destino: Some(ponto),
    }
}

/// Agenda uma construção de ponte somente após validá-la contra o estado corrente.
fn agendar_ponte(mundo: &mut Mundo, peer_id: u64, seq: i64, request_id: i64, ponto: (f64, f64)) {
    garantir_nivel(mundo, peer_id, 2);
    assert!(ValidacaoComandos::validar(
        mundo,
        peer_id,
        "construir_ponte",
        seq,
        request_id,
        Some(ponto),
    )
    .is_ok());
    mundo.agendar_comando(comando_ponte(peer_id, seq, request_id, ponto));
}

/// Conclui uma ponte em `ponto`, começando em `seq_base`.
fn concluir_ponte(mundo: &mut Mundo, peer_id: u64, seq_base: i64, ponto: (f64, f64)) {
    for offset in 0..TICKS_CONSTRUCAO_PONTE {
        let seq = seq_base + offset;
        agendar_ponte(mundo, peer_id, seq, seq, ponto);
        mundo.avancar_tick();
    }
}

/// Monta uma ponte direta para verificações de hash.
#[must_use]
fn ponte_teste(id: u64, dono: u64, posicao: (f64, f64), pv: i64, ticks_restantes: i64) -> Ponte {
    Ponte {
        id,
        dono,
        posicao,
        pv,
        ticks_restantes,
    }
}

#[test]
fn barricada_apos_45_ticks_debita_20() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.25, 0.5);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    for offset in 0..(TICKS_CONSTRUCAO_BARREIRA - 1) {
        let seq = 2 + offset;
        agendar_barreira(&mut mundo, 1, seq, seq, ponto);
        mundo.avancar_tick();
        let jogador = &mundo.jogadores[&1];
        assert_eq!(jogador.construcao_ticks, 1 + offset, "tick {offset}");
        assert_eq!(jogador.construcao_ponto, Some(ponto));
        assert_eq!(jogador.materiais, 100);
        assert!(mundo.barreiras.is_empty());
    }

    let seq = 1 + TICKS_CONSTRUCAO_BARREIRA;
    agendar_barreira(&mut mundo, 1, seq, seq, ponto);
    mundo.avancar_tick();

    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.materiais, 100 - CUSTO_BARREIRA_MATERIAIS);
    assert_eq!(jogador.construcao_ticks, 0);
    assert_eq!(jogador.construcao_ponto, None);
    assert_eq!(mundo.barreiras.len(), 1);
    let barreira = &mundo.barreiras[&1];
    assert_eq!(barreira.id, 1);
    assert_eq!(barreira.dono, 1);
    assert_eq!(barreira.posicao, ponto);
    assert_eq!(barreira.pv, PV_BARREIRA);
    assert_eq!(barreira.ticks_restantes, TICKS_DURACAO_BARREIRA);
    assert_eq!(mundo.proxima_barreira_id, 2);
}

#[test]
fn construcao_continua_apos_um_unico_comando() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.25, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    // Uma única tecla B inicia a canalização; os 44 ticks seguintes são
    // automáticos, sem qualquer comando adicional na fila.
    agendar_barreira(&mut mundo, 1, 2, 2, ponto);
    for tick in 0..TICKS_CONSTRUCAO_BARREIRA {
        mundo.avancar_tick();
        assert!(mundo.comandos_pendentes.is_empty());
        assert!(mundo.comandos_do_tick.is_empty());
        assert_eq!(mundo.jogadores[&1].seq, 2, "tick {tick}");
        if tick < TICKS_CONSTRUCAO_BARREIRA - 1 {
            assert_eq!(
                mundo.jogadores[&1].construcao_ticks,
                tick + 1,
                "tick {tick}"
            );
            assert_eq!(mundo.jogadores[&1].construcao_ponto, Some(ponto));
            assert_eq!(mundo.jogadores[&1].materiais, 100);
            assert!(mundo.barreiras.is_empty(), "tick {tick}");
        }
    }

    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.materiais, 100 - CUSTO_BARREIRA_MATERIAIS);
    assert_eq!(jogador.construcao_ticks, 0);
    assert_eq!(jogador.construcao_ponto, None);
    assert_eq!(mundo.barreiras.len(), 1);
    assert_eq!(mundo.barreiras[&1].dono, 1);
    assert_eq!(mundo.barreiras[&1].posicao, ponto);
    assert_eq!(mundo.barreiras[&1].pv, PV_BARREIRA);
    assert_eq!(mundo.barreiras[&1].ticks_restantes, TICKS_DURACAO_BARREIRA);
}

#[test]
fn saldo_insuficiente_recusa_sem_debito() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    garantir_nivel(&mut mundo, 1, 2);
    let insuficiente = CUSTO_BARREIRA_MATERIAIS - 1;
    definir_materiais(&mut mundo, 1, insuficiente);
    let ponto = (0.25, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    let recusado = ValidacaoComandos::validar(&mundo, 1, "construir_barreira", 2, 2, Some(ponto));
    assert_eq!(recusado, Err(CodigoRecusa::SaldoInsuficiente));
    assert_eq!(mundo.jogadores[&1].materiais, insuficiente);

    // Um comando forjado não inicia canalização nem debita.
    mundo.agendar_comando(comando_barreira(1, 2, 2, ponto));
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.construcao_ticks, 0);
    assert_eq!(jogador.construcao_ponto, None);
    assert_eq!(jogador.materiais, insuficiente);
    assert!(mundo.barreiras.is_empty());

    // O saldo é revalidado no tick de conclusão: sem débito e sem barreira.
    definir_materiais(&mut mundo, 1, CUSTO_BARREIRA_MATERIAIS);
    for offset in 0..(TICKS_CONSTRUCAO_BARREIRA - 1) {
        let seq = 3 + offset;
        agendar_barreira(&mut mundo, 1, seq, seq, ponto);
        mundo.avancar_tick();
    }
    assert_eq!(
        mundo.jogadores[&1].construcao_ticks,
        TICKS_CONSTRUCAO_BARREIRA - 1
    );
    definir_materiais(&mut mundo, 1, insuficiente);
    let seq = 3 + TICKS_CONSTRUCAO_BARREIRA;
    mundo.agendar_comando(comando_barreira(1, seq, seq, ponto));
    mundo.avancar_tick();

    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.materiais, insuficiente);
    assert_eq!(jogador.construcao_ticks, 0);
    assert_eq!(jogador.construcao_ponto, None);
    assert!(mundo.barreiras.is_empty());
}

#[test]
fn construcao_fora_do_ponto_recusada() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    garantir_nivel(&mut mundo, 1, 2);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.25, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    let fora = ValidacaoComandos::validar(&mundo, 1, "construir_barreira", 2, 2, Some((0.75, 0.0)));
    assert_eq!(fora, Err(CodigoRecusa::DestinoInvalido));
    let sem_destino = ValidacaoComandos::validar(&mundo, 1, "construir_barreira", 2, 2, None);
    assert_eq!(sem_destino, Err(CodigoRecusa::DestinoInvalido));
    let aceito = ValidacaoComandos::validar(&mundo, 1, "construir_barreira", 2, 2, Some(ponto));
    assert!(aceito.is_ok());

    for (seq, destino) in [(2_i64, None), (3_i64, Some((0.75_f64, 0.0)))] {
        mundo.agendar_comando(Comando {
            peer_id: 1,
            acao: "construir_barreira".to_string(),
            seq,
            request_id: seq,
            destino,
        });
        mundo.avancar_tick();
    }

    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.materiais, 100);
    assert_eq!(jogador.construcao_ticks, 0);
    assert_eq!(jogador.construcao_ponto, None);
    assert!(mundo.barreiras.is_empty());
}

#[test]
fn barricada_200_pv_destruida_ao_200_dano() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.5, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();
    concluir_barreira(&mut mundo, 1, 2, ponto);

    assert_eq!(mundo.barreiras[&1].pv, PV_BARREIRA);
    assert!(!mundo.aplicar_dano_a_barreira(1, -1));
    assert_eq!(mundo.barreiras[&1].pv, PV_BARREIRA);
    assert!(!mundo.aplicar_dano_a_barreira(1, PV_BARREIRA - 1));
    assert_eq!(mundo.barreiras[&1].pv, 1);
    assert!(!mundo.aplicar_dano_a_barreira(1, 0));
    assert!(!mundo.aplicar_dano_a_barreira(99, PV_BARREIRA));
    assert!(mundo.aplicar_dano_a_barreira(1, 1));
    assert!(mundo.barreiras.is_empty());
    assert!(!mundo.aplicar_dano_a_barreira(1, 1));
}

#[test]
fn barricada_expira_apos_360_ticks() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.5, 0.25);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();
    concluir_barreira(&mut mundo, 1, 2, ponto);
    assert_eq!(mundo.barreiras[&1].ticks_restantes, TICKS_DURACAO_BARREIRA);

    // Este caso cobre só a expiração: o dono sai do ponto para que a defesa
    // não credite durante os 360 ticks e o saldo final seja verificável.
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.posicao = (0.75, 0.75);
    }

    for _ in 0..(TICKS_DURACAO_BARREIRA - 1) {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.barreiras[&1].ticks_restantes, 1);
    let defesa_ticks_antes = mundo.barreiras[&1].defesa_ticks;
    mundo.avancar_tick();
    assert!(mundo.barreiras.is_empty());
    assert_eq!(mundo.proxima_barreira_id, 2);
    assert_eq!(defesa_ticks_antes, 0);
    assert_eq!(mundo.jogadores[&1].pontos_autogestao, 0);
}

#[test]
fn defesa_nao_credita_no_tick_de_expiracao() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.25, 0.0);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.posicao = ponto;
    }
    mundo
        .barreiras
        .insert(1, barreira_teste(1, 1, ponto, PV_BARREIRA, 1));
    if let Some(barreira) = mundo.barreiras.get_mut(&1) {
        barreira.defesa_ticks = TICKS_AUTOGESTAO_POR_PONTO - 1;
    }

    mundo.avancar_tick();

    assert!(mundo.barreiras.is_empty());
    assert_eq!(mundo.jogadores[&1].pontos_autogestao, 0);
}

#[test]
fn terceira_barricada_remove_a_mais_antiga() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.25, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    concluir_barreira(&mut mundo, 1, 2, ponto);
    concluir_barreira(&mut mundo, 1, 2 + TICKS_CONSTRUCAO_BARREIRA, ponto);
    assert_eq!(mundo.barreiras.len(), MAX_BARREIRAS_POR_JOGADOR);
    assert!(mundo.barreiras.contains_key(&1));
    assert!(mundo.barreiras.contains_key(&2));
    assert_eq!(
        mundo.jogadores[&1].materiais,
        100 - 2 * CUSTO_BARREIRA_MATERIAIS
    );

    concluir_barreira(&mut mundo, 1, 2 + 2 * TICKS_CONSTRUCAO_BARREIRA, ponto);
    assert_eq!(mundo.barreiras.len(), MAX_BARREIRAS_POR_JOGADOR);
    assert!(!mundo.barreiras.contains_key(&1));
    assert!(mundo.barreiras.contains_key(&2));
    assert!(mundo.barreiras.contains_key(&3));
    assert_eq!(
        mundo.jogadores[&1].materiais,
        100 - 3 * CUSTO_BARREIRA_MATERIAIS
    );
    assert_eq!(mundo.proxima_barreira_id, 4);
}

#[test]
fn dupla_construcao_no_mesmo_tick_nao_acelera() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.25, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    let mut seq = 2_i64;
    // Dois comandos válidos por tick não podem valer dois ticks de canalização.
    for tick in 0..(TICKS_CONSTRUCAO_BARREIRA - 1) {
        agendar_barreira(&mut mundo, 1, seq, seq, ponto);
        seq += 1;
        agendar_barreira(&mut mundo, 1, seq, seq, ponto);
        seq += 1;
        mundo.avancar_tick();
        assert_eq!(
            mundo.jogadores[&1].construcao_ticks,
            1 + tick,
            "tick {tick}"
        );
        assert_eq!(mundo.jogadores[&1].materiais, 100, "débito antecipado");
        assert!(mundo.barreiras.is_empty());
    }

    agendar_barreira(&mut mundo, 1, seq, seq, ponto);
    seq += 1;
    agendar_barreira(&mut mundo, 1, seq, seq, ponto);
    mundo.avancar_tick();

    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.construcao_ticks, 0);
    assert_eq!(jogador.materiais, 100 - CUSTO_BARREIRA_MATERIAIS);
    assert_eq!(jogador.seq, seq);
    assert_eq!(jogador.request_id, seq);
    assert_eq!(mundo.barreiras.len(), 1);
}

#[test]
fn movimento_cancela_construcao() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.25, 0.0);
    let fora = (0.25, 0.5);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    for seq in 2..(2 + 10) {
        agendar_barreira(&mut mundo, 1, seq, seq, ponto);
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&1].construcao_ticks, 10);
    assert_eq!(mundo.jogadores[&1].construcao_ponto, Some(ponto));

    let seq = 2 + 10;
    agendar_mover(&mut mundo, 1, seq, seq, fora);
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.posicao, fora);
    assert_eq!(jogador.construcao_ticks, 0);
    assert_eq!(jogador.construcao_ponto, None);
    assert_eq!(jogador.materiais, 100);
    assert!(mundo.barreiras.is_empty());

    // O `mover` para o mesmo ponto apenas não cancela a canalização; o
    // segundo tick de construção vem do novo `construir_barreira` deste tick.
    let seq = 3 + 10;
    agendar_barreira(&mut mundo, 1, seq, seq, fora);
    mundo.avancar_tick();
    let seq = 4 + 10;
    agendar_mover(&mut mundo, 1, seq, seq, fora);
    let seq = 5 + 10;
    agendar_barreira(&mut mundo, 1, seq, seq, fora);
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.construcao_ticks, 2);
    assert_eq!(jogador.construcao_ponto, Some(fora));
    assert_eq!(jogador.materiais, 100);
}

#[test]
fn hash_reflete_barreiras_e_construcao() {
    let mut sem_construcao = Mundo::new();
    let mut com_construcao = Mundo::new();
    for mundo in [&mut sem_construcao, &mut com_construcao] {
        mundo.adicionar_jogador(1);
        if let Some(jogador) = mundo.jogadores.get_mut(&1) {
            jogador.seq = 7;
            jogador.request_id = 9;
            jogador.materiais = 100;
        }
    }
    assert_eq!(sem_construcao.hash_estado(), com_construcao.hash_estado());

    if let Some(jogador) = com_construcao.jogadores.get_mut(&1) {
        jogador.construcao_ticks = 3;
        jogador.construcao_ponto = Some((0.25, 0.0));
    }
    assert_ne!(sem_construcao.hash_estado(), com_construcao.hash_estado());

    if let Some(jogador) = sem_construcao.jogadores.get_mut(&1) {
        jogador.construcao_ticks = 3;
        jogador.construcao_ponto = Some((0.25, 0.0));
    }
    assert_eq!(sem_construcao.hash_estado(), com_construcao.hash_estado());

    if let Some(jogador) = sem_construcao.jogadores.get_mut(&1) {
        jogador.construcao_ponto = Some((0.75, 0.0));
    }
    assert_ne!(sem_construcao.hash_estado(), com_construcao.hash_estado());

    let mut primeira = Mundo::new();
    let mut segunda = Mundo::new();
    for mundo in [&mut primeira, &mut segunda] {
        mundo.adicionar_jogador(1);
    }
    let sem_barreiras = primeira.hash_estado();
    primeira
        .barreiras
        .insert(1, barreira_teste(1, 1, (0.5, 0.0), 200, 10));
    primeira
        .barreiras
        .insert(2, barreira_teste(2, 1, (-0.5, 0.0), 100, 20));
    // Ordem de inserção distinta: a iteração do BTreeMap é canônica.
    segunda
        .barreiras
        .insert(2, barreira_teste(2, 1, (-0.5, 0.0), 100, 20));
    segunda
        .barreiras
        .insert(1, barreira_teste(1, 1, (0.5, 0.0), 200, 10));
    assert_ne!(primeira.hash_estado(), sem_barreiras);
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(barreira) = primeira.barreiras.get_mut(&2) {
        barreira.pv -= 1;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    if let Some(barreira) = primeira.barreiras.get_mut(&2) {
        barreira.pv += 1;
        barreira.ticks_restantes -= 1;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
}

#[test]
fn determinismo_com_barreiras() {
    let mut primeira = Mundo::new();
    let mut segunda = Mundo::new();
    for mundo in [&mut primeira, &mut segunda] {
        mundo.adicionar_jogador(2);
        mundo.adicionar_jogador(1);
        definir_materiais(mundo, 1, 100);
        definir_materiais(mundo, 2, 100);
    }
    let ponto_um = (0.25, 0.0);
    let ponto_dois = (0.0, 0.25);

    agendar_mover(&mut primeira, 1, 1, 1, ponto_um);
    agendar_mover(&mut primeira, 2, 1, 1, ponto_dois);
    agendar_mover(&mut segunda, 2, 1, 1, ponto_dois);
    agendar_mover(&mut segunda, 1, 1, 1, ponto_um);
    primeira.avancar_tick();
    segunda.avancar_tick();
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    for seq in 2..(2 + TICKS_CONSTRUCAO_BARREIRA) {
        for mundo in [&mut primeira, &mut segunda] {
            agendar_barreira(mundo, 1, seq, seq, ponto_um);
            agendar_barreira(mundo, 2, seq, seq, ponto_dois);
        }
        primeira.avancar_tick();
        segunda.avancar_tick();
        assert_eq!(primeira.hash_estado(), segunda.hash_estado());
    }
    for peer_id in [1, 2] {
        let jogador = &primeira.jogadores[&peer_id];
        assert_eq!(jogador.materiais, 100 - CUSTO_BARREIRA_MATERIAIS);
        assert_eq!(jogador.construcao_ticks, 0);
        assert_eq!(jogador.construcao_ponto, None);
    }
    assert_eq!(primeira.barreiras.len(), 2);
    assert_eq!(segunda.barreiras.len(), 2);

    assert!(!primeira.aplicar_dano_a_barreira(1, PV_BARREIRA - 1));
    assert!(!segunda.aplicar_dano_a_barreira(1, PV_BARREIRA - 1));
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());
    for _ in 0..TICKS_DURACAO_BARREIRA {
        primeira.avancar_tick();
        segunda.avancar_tick();
        assert_eq!(primeira.hash_estado(), segunda.hash_estado());
    }
    assert!(primeira.barreiras.is_empty());
    assert!(segunda.barreiras.is_empty());
}

#[test]
fn defesa_gera_ponto_apos_300_ticks() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.25, 0.0);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.posicao = ponto;
    }
    mundo.barreiras.insert(
        1,
        barreira_teste(1, 1, ponto, PV_BARREIRA, TICKS_DURACAO_BARREIRA),
    );

    for _ in 0..(TICKS_AUTOGESTAO_POR_PONTO - 1) {
        mundo.avancar_tick();
    }
    assert_eq!(
        mundo.barreiras[&1].defesa_ticks,
        TICKS_AUTOGESTAO_POR_PONTO - 1
    );
    assert_eq!(mundo.jogadores[&1].pontos_autogestao, 0);

    mundo.avancar_tick();
    assert_eq!(
        mundo.jogadores[&1].pontos_autogestao,
        PONTOS_AUTOGESTAO_POR_DEFESA
    );
    assert_eq!(mundo.barreiras[&1].defesa_ticks, 0);
    assert_eq!(
        mundo.barreiras[&1].ticks_restantes,
        TICKS_DURACAO_BARREIRA - TICKS_AUTOGESTAO_POR_PONTO
    );

    // A barricada ainda tem 60 ticks de vida: não credita uma segunda vez.
    for _ in 0..(TICKS_DURACAO_BARREIRA - TICKS_AUTOGESTAO_POR_PONTO) {
        mundo.avancar_tick();
    }
    assert!(mundo.barreiras.is_empty());
    assert_eq!(
        mundo.jogadores[&1].pontos_autogestao,
        PONTOS_AUTOGESTAO_POR_DEFESA
    );
}

#[test]
fn dono_fora_do_ponto_nao_pontua() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.25, 0.0);
    let fora = (0.25, 0.25);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.posicao = ponto;
    }
    mundo.barreiras.insert(
        1,
        barreira_teste(1, 1, ponto, PV_BARREIRA, TICKS_DURACAO_BARREIRA),
    );

    for _ in 0..200 {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.barreiras[&1].defesa_ticks, 200);
    assert_eq!(mundo.jogadores[&1].pontos_autogestao, 0);

    agendar_mover(&mut mundo, 1, 1, 1, fora);
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].posicao, fora);
    assert_eq!(mundo.barreiras[&1].defesa_ticks, 0);
    assert_eq!(mundo.jogadores[&1].pontos_autogestao, 0);

    for _ in 0..10 {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.barreiras[&1].defesa_ticks, 0);
    assert_eq!(mundo.jogadores[&1].pontos_autogestao, 0);

    agendar_mover(&mut mundo, 1, 2, 2, ponto);
    mundo.avancar_tick();
    assert_eq!(mundo.barreiras[&1].defesa_ticks, 1);

    // `mover` para o próprio ponto não interrompe a defesa.
    agendar_mover(&mut mundo, 1, 3, 3, ponto);
    mundo.avancar_tick();
    assert_eq!(mundo.barreiras[&1].defesa_ticks, 2);
    assert_eq!(mundo.jogadores[&1].pontos_autogestao, 0);

    // Dono ausente também descarta o progresso sem creditar.
    if let Some(barreira) = mundo.barreiras.get_mut(&1) {
        barreira.defesa_ticks = 7;
    }
    mundo.remover_jogador(1);
    mundo.avancar_tick();
    assert_eq!(mundo.barreiras[&1].defesa_ticks, 0);
}

#[test]
fn barricada_destruida_nao_pontua() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.25, 0.0);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.posicao = ponto;
    }
    mundo.barreiras.insert(
        1,
        barreira_teste(1, 1, ponto, PV_BARREIRA, TICKS_DURACAO_BARREIRA),
    );
    if let Some(barreira) = mundo.barreiras.get_mut(&1) {
        barreira.defesa_ticks = TICKS_AUTOGESTAO_POR_PONTO - 1;
    }

    assert!(mundo.aplicar_dano_a_barreira(1, PV_BARREIRA));
    assert!(mundo.barreiras.is_empty());
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].pontos_autogestao, 0);

    // Uma barreira com `pv <= 0` que persista no mapa também não acumula.
    mundo
        .barreiras
        .insert(2, barreira_teste(2, 1, ponto, 0, TICKS_DURACAO_BARREIRA));
    mundo.avancar_tick();
    assert_eq!(mundo.barreiras[&2].defesa_ticks, 0);
    assert_eq!(mundo.jogadores[&1].pontos_autogestao, 0);
}

#[test]
fn hash_reflete_autogestao() {
    let mut primeira = Mundo::new();
    let mut segunda = Mundo::new();
    for mundo in [&mut primeira, &mut segunda] {
        mundo.adicionar_jogador(1);
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(jogador) = primeira.jogadores.get_mut(&1) {
        jogador.pontos_autogestao = PONTOS_AUTOGESTAO_POR_DEFESA;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(jogador) = segunda.jogadores.get_mut(&1) {
        jogador.pontos_autogestao = PONTOS_AUTOGESTAO_POR_DEFESA;
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    for mundo in [&mut primeira, &mut segunda] {
        mundo.barreiras.insert(
            1,
            barreira_teste(1, 1, (0.25, 0.0), PV_BARREIRA, TICKS_DURACAO_BARREIRA),
        );
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(barreira) = primeira.barreiras.get_mut(&1) {
        barreira.defesa_ticks = 7;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
}

#[test]
fn ponte_apos_45_ticks_debita_15() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.25, 0.5);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    for offset in 0..(TICKS_CONSTRUCAO_PONTE - 1) {
        let seq = 2 + offset;
        agendar_ponte(&mut mundo, 1, seq, seq, ponto);
        mundo.avancar_tick();
        let jogador = &mundo.jogadores[&1];
        assert_eq!(jogador.ponte_ticks, 1 + offset, "tick {offset}");
        assert_eq!(jogador.ponte_ponto, Some(ponto));
        assert_eq!(jogador.materiais, 100);
        assert!(mundo.pontes.is_empty());
    }

    let seq = 1 + TICKS_CONSTRUCAO_PONTE;
    agendar_ponte(&mut mundo, 1, seq, seq, ponto);
    mundo.avancar_tick();

    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.materiais, 100 - CUSTO_PONTE_MATERIAIS);
    assert_eq!(jogador.ponte_ticks, 0);
    assert_eq!(jogador.ponte_ponto, None);
    assert_eq!(mundo.pontes.len(), 1);
    let ponte_construida = &mundo.pontes[&1];
    assert_eq!(ponte_construida.id, 1);
    assert_eq!(ponte_construida.dono, 1);
    assert_eq!(ponte_construida.posicao, ponto);
    assert_eq!(ponte_construida.pv, PV_PONTE);
    assert_eq!(ponte_construida.ticks_restantes, TICKS_DURACAO_PONTE);
    assert_eq!(mundo.proxima_ponte_id, 2);
}

#[test]
fn saldo_14_nao_construi_ponte() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    garantir_nivel(&mut mundo, 1, 2);
    let insuficiente = CUSTO_PONTE_MATERIAIS - 1;
    definir_materiais(&mut mundo, 1, insuficiente);
    let ponto = (0.25, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    let recusado = ValidacaoComandos::validar(&mundo, 1, "construir_ponte", 2, 2, Some(ponto));
    assert_eq!(recusado, Err(CodigoRecusa::SaldoInsuficiente));
    assert_eq!(mundo.jogadores[&1].materiais, insuficiente);

    // Um comando forjado não inicia canalização nem debita.
    mundo.agendar_comando(comando_ponte(1, 2, 2, ponto));
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.ponte_ticks, 0);
    assert_eq!(jogador.ponte_ponto, None);
    assert_eq!(jogador.materiais, insuficiente);
    assert!(mundo.pontes.is_empty());

    // O saldo é revalidado no tick de conclusão: sem débito e sem ponte.
    definir_materiais(&mut mundo, 1, CUSTO_PONTE_MATERIAIS);
    for offset in 0..(TICKS_CONSTRUCAO_PONTE - 1) {
        let seq = 3 + offset;
        agendar_ponte(&mut mundo, 1, seq, seq, ponto);
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&1].ponte_ticks, TICKS_CONSTRUCAO_PONTE - 1);
    definir_materiais(&mut mundo, 1, insuficiente);
    let seq = 3 + TICKS_CONSTRUCAO_PONTE;
    mundo.agendar_comando(comando_ponte(1, seq, seq, ponto));
    mundo.avancar_tick();

    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.materiais, insuficiente);
    assert_eq!(jogador.ponte_ticks, 0);
    assert_eq!(jogador.ponte_ponto, None);
    assert!(mundo.pontes.is_empty());
}

#[test]
fn ponte_60_pv_destruida() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.5, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();
    concluir_ponte(&mut mundo, 1, 2, ponto);

    assert_eq!(mundo.pontes[&1].pv, PV_PONTE);
    assert!(!mundo.aplicar_dano_a_ponte(1, -1));
    assert_eq!(mundo.pontes[&1].pv, PV_PONTE);
    assert!(!mundo.aplicar_dano_a_ponte(1, PV_PONTE - 1));
    assert_eq!(mundo.pontes[&1].pv, 1);
    assert!(!mundo.aplicar_dano_a_ponte(1, 0));
    assert!(!mundo.aplicar_dano_a_ponte(99, PV_PONTE));
    assert!(mundo.aplicar_dano_a_ponte(1, 1));
    assert!(mundo.pontes.is_empty());
    assert!(!mundo.aplicar_dano_a_ponte(1, 1));
}

#[test]
fn ponte_expira_apos_1350_ticks() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.5, 0.25);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();
    concluir_ponte(&mut mundo, 1, 2, ponto);
    assert_eq!(mundo.pontes[&1].ticks_restantes, TICKS_DURACAO_PONTE);

    for _ in 0..(TICKS_DURACAO_PONTE - 1) {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.pontes[&1].ticks_restantes, 1);
    mundo.avancar_tick();
    assert!(mundo.pontes.is_empty());
    assert_eq!(mundo.proxima_ponte_id, 2);
}

#[test]
fn ponte_apos_um_unico_comando() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.25, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    // Uma única tecla P inicia a canalização; os 44 ticks seguintes são
    // automáticos, sem qualquer comando adicional na fila.
    agendar_ponte(&mut mundo, 1, 2, 2, ponto);
    for tick in 0..TICKS_CONSTRUCAO_PONTE {
        mundo.avancar_tick();
        assert!(mundo.comandos_pendentes.is_empty());
        assert!(mundo.comandos_do_tick.is_empty());
        assert_eq!(mundo.jogadores[&1].seq, 2, "tick {tick}");
        if tick < TICKS_CONSTRUCAO_PONTE - 1 {
            assert_eq!(mundo.jogadores[&1].ponte_ticks, tick + 1, "tick {tick}");
            assert_eq!(mundo.jogadores[&1].ponte_ponto, Some(ponto));
            assert_eq!(mundo.jogadores[&1].materiais, 100);
            assert!(mundo.pontes.is_empty(), "tick {tick}");
        }
    }

    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.materiais, 100 - CUSTO_PONTE_MATERIAIS);
    assert_eq!(jogador.ponte_ticks, 0);
    assert_eq!(jogador.ponte_ponto, None);
    assert_eq!(mundo.pontes.len(), 1);
    assert_eq!(mundo.pontes[&1].dono, 1);
    assert_eq!(mundo.pontes[&1].posicao, ponto);
    assert_eq!(mundo.pontes[&1].pv, PV_PONTE);
    assert_eq!(mundo.pontes[&1].ticks_restantes, TICKS_DURACAO_PONTE);
}

#[test]
fn hash_reflete_pontes() {
    let mut primeira = Mundo::new();
    let mut segunda = Mundo::new();
    for mundo in [&mut primeira, &mut segunda] {
        mundo.adicionar_jogador(1);
    }
    let sem_pontes = primeira.hash_estado();
    primeira
        .pontes
        .insert(1, ponte_teste(1, 1, (0.5, 0.0), PV_PONTE, 10));
    primeira
        .pontes
        .insert(2, ponte_teste(2, 1, (-0.5, 0.0), 30, 20));
    // Ordem de inserção distinta: a iteração do BTreeMap é canônica.
    segunda
        .pontes
        .insert(2, ponte_teste(2, 1, (-0.5, 0.0), 30, 20));
    segunda
        .pontes
        .insert(1, ponte_teste(1, 1, (0.5, 0.0), PV_PONTE, 10));
    assert_ne!(primeira.hash_estado(), sem_pontes);
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(ponte) = primeira.pontes.get_mut(&2) {
        ponte.pv -= 1;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    if let Some(ponte) = primeira.pontes.get_mut(&2) {
        ponte.pv += 1;
        ponte.ticks_restantes -= 1;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());

    let mut com_canalizacao = Mundo::new();
    com_canalizacao.adicionar_jogador(1);
    let hash_sem_canalizacao = com_canalizacao.hash_estado();
    if let Some(jogador) = com_canalizacao.jogadores.get_mut(&1) {
        jogador.ponte_ticks = 3;
        jogador.ponte_ponto = Some((0.25, 0.0));
    }
    assert_ne!(com_canalizacao.hash_estado(), hash_sem_canalizacao);
}

#[test]
fn ponte_fora_do_ponto_recusada() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    garantir_nivel(&mut mundo, 1, 2);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.25, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    let aceita = ValidacaoComandos::validar(&mundo, 1, "construir_ponte", 2, 2, Some(ponto));
    assert!(aceita.is_ok());
    let fora = ValidacaoComandos::validar(&mundo, 1, "construir_ponte", 2, 2, Some((0.75, 0.0)));
    assert_eq!(fora, Err(CodigoRecusa::DestinoInvalido));
    let sem_destino = ValidacaoComandos::validar(&mundo, 1, "construir_ponte", 2, 2, None);
    assert_eq!(sem_destino, Err(CodigoRecusa::DestinoInvalido));

    mundo.agendar_comando(comando_ponte(1, 2, 2, (0.75, 0.0)));
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.ponte_ticks, 0);
    assert_eq!(jogador.ponte_ponto, None);
    assert_eq!(jogador.materiais, 100);
    assert!(mundo.pontes.is_empty());
}

#[test]
fn movimento_cancela_ponte() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.25, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();
    agendar_ponte(&mut mundo, 1, 2, 2, ponto);
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].ponte_ticks, 1);

    agendar_mover(&mut mundo, 1, 3, 3, (0.5, 0.0));
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.ponte_ticks, 0);
    assert_eq!(jogador.ponte_ponto, None);
    assert_eq!(jogador.materiais, 100);

    for _ in 0..TICKS_CONSTRUCAO_PONTE {
        mundo.avancar_tick();
    }
    assert!(mundo.pontes.is_empty());
    assert_eq!(mundo.jogadores[&1].materiais, 100);
}

#[test]
fn dupla_construcao_ponte_no_mesmo_tick_nao_acelera() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    garantir_nivel(&mut mundo, 1, 2);
    definir_materiais(&mut mundo, 1, 100);
    let ponto = (0.25, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    mundo.agendar_comando(comando_ponte(1, 2, 2, ponto));
    mundo.agendar_comando(comando_ponte(1, 3, 3, ponto));
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].ponte_ticks, 1);

    for _ in 0..(TICKS_CONSTRUCAO_PONTE - 1) {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.pontes.len(), 1);
    assert_eq!(mundo.jogadores[&1].materiais, 100 - CUSTO_PONTE_MATERIAIS);
}

/// Monta um comando `propaganda` com os monotônicos já preenchidos.
#[must_use]
fn comando_propaganda(peer_id: u64, seq: i64, request_id: i64, ponto: (f64, f64)) -> Comando {
    Comando {
        peer_id,
        acao: "propaganda".to_string(),
        seq,
        request_id,
        destino: Some(ponto),
    }
}

/// Agenda uma propaganda somente após validá-la contra o estado corrente.
fn agendar_propaganda(
    mundo: &mut Mundo,
    peer_id: u64,
    seq: i64,
    request_id: i64,
    ponto: (f64, f64),
) {
    garantir_nivel(mundo, peer_id, 3);
    assert!(
        ValidacaoComandos::validar(mundo, peer_id, "propaganda", seq, request_id, Some(ponto),)
            .is_ok()
    );
    mundo.agendar_comando(comando_propaganda(peer_id, seq, request_id, ponto));
}

/// Converte um Súdito com uma sequência ininterrupta de comandos válidos.
fn converter_sudito(mundo: &mut Mundo, peer_id: u64, id: u64, seq_base: i64, ponto: (f64, f64)) {
    for offset in 0..TICKS_PROPAGANDA {
        let seq = seq_base + offset;
        agendar_propaganda(mundo, peer_id, seq, seq, ponto);
        mundo.avancar_tick();
    }
    assert!(mundo.suditos[&id].convertido);
}

#[test]
fn propaganda_converte_apos_120_ticks() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.25, 0.0);
    mundo.adicionar_sudito(7, ponto);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    for offset in 0..(TICKS_PROPAGANDA - 1) {
        let seq = 2 + offset;
        agendar_propaganda(&mut mundo, 1, seq, seq, ponto);
        mundo.avancar_tick();
        let sudito = &mundo.suditos[&7];
        assert_eq!(sudito.propaganda_ticks, 1 + offset, "tick {offset}");
        assert!(!sudito.convertido, "tick {offset}");
    }

    let seq = 1 + TICKS_PROPAGANDA;
    agendar_propaganda(&mut mundo, 1, seq, seq, ponto);
    mundo.avancar_tick();
    let sudito = &mundo.suditos[&7];
    assert_eq!(sudito.propaganda_ticks, TICKS_PROPAGANDA);
    assert!(sudito.convertido);
    assert_eq!(sudito.reversao_ticks, 0);
    assert_eq!(mundo.jogadores[&1].propaganda_alvo, Some(7));
}

#[test]
fn dano_reverte_em_90_ticks() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.25, 0.0);
    mundo.adicionar_sudito(7, ponto);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();
    converter_sudito(&mut mundo, 1, 7, 2, ponto);

    mundo.aplicar_dano_a_sudito(7, 1);
    assert_eq!(mundo.suditos[&7].reversao_ticks, TICKS_REVERSAO_SUDITO);
    for offset in 0..(TICKS_REVERSAO_SUDITO - 1) {
        mundo.avancar_tick();
        let sudito = &mundo.suditos[&7];
        assert!(sudito.convertido, "tick {offset}");
        assert_eq!(sudito.reversao_ticks, TICKS_REVERSAO_SUDITO - 1 - offset);
        assert_eq!(sudito.propaganda_ticks, TICKS_PROPAGANDA);
    }

    mundo.avancar_tick();
    let sudito = &mundo.suditos[&7];
    assert!(!sudito.convertido);
    assert_eq!(sudito.reversao_ticks, 0);
    assert_eq!(sudito.propaganda_ticks, 0);

    // Dano não positivo e ID inexistente não alteram o estado.
    mundo.aplicar_dano_a_sudito(7, 0);
    mundo.aplicar_dano_a_sudito(99, 10);
    assert_eq!(mundo.suditos[&7].reversao_ticks, 0);
    assert!(!mundo.suditos[&7].convertido);

    // Dano antes da conversão registra a reversão sem apagar a exposição.
    let mut outro = Mundo::new();
    outro.adicionar_sudito(9, (0.25, 0.0));
    outro.aplicar_dano_a_sudito(9, 1);
    assert_eq!(outro.suditos[&9].reversao_ticks, TICKS_REVERSAO_SUDITO);
    assert!(!outro.suditos[&9].convertido);
    assert_eq!(outro.suditos[&9].propaganda_ticks, 0);
}

#[test]
fn propaganda_apos_um_unico_comando() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.25, 0.0);
    mundo.adicionar_sudito(7, ponto);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    // Uma única tecla G inicia a exposição; os ticks seguintes são automáticos.
    agendar_propaganda(&mut mundo, 1, 2, 2, ponto);
    for tick in 0..TICKS_PROPAGANDA {
        mundo.avancar_tick();
        assert!(mundo.comandos_pendentes.is_empty());
        assert!(mundo.comandos_do_tick.is_empty());
        assert_eq!(mundo.jogadores[&1].seq, 2, "tick {tick}");
        let sudito = &mundo.suditos[&7];
        if tick < TICKS_PROPAGANDA - 1 {
            assert_eq!(sudito.propaganda_ticks, tick + 1, "tick {tick}");
            assert!(!sudito.convertido, "tick {tick}");
        }
    }

    let sudito = &mundo.suditos[&7];
    assert_eq!(sudito.propaganda_ticks, TICKS_PROPAGANDA);
    assert!(sudito.convertido);
    assert_eq!(mundo.jogadores[&1].propaganda_alvo, Some(7));
}

#[test]
fn dupla_propaganda_no_mesmo_tick_nao_acelera() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.25, 0.0);
    mundo.adicionar_sudito(7, ponto);
    garantir_nivel(&mut mundo, 1, 3);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    let mut seq = 2_i64;
    // Dois comandos válidos por tick não podem valer dois ticks de exposição.
    for tick in 0..(TICKS_PROPAGANDA - 1) {
        mundo.agendar_comando(comando_propaganda(1, seq, seq, ponto));
        seq += 1;
        mundo.agendar_comando(comando_propaganda(1, seq, seq, ponto));
        seq += 1;
        mundo.avancar_tick();
        assert_eq!(mundo.suditos[&7].propaganda_ticks, 1 + tick, "tick {tick}");
        assert!(!mundo.suditos[&7].convertido, "tick {tick}");
    }

    mundo.agendar_comando(comando_propaganda(1, seq, seq, ponto));
    seq += 1;
    mundo.agendar_comando(comando_propaganda(1, seq, seq, ponto));
    mundo.avancar_tick();

    let sudito = &mundo.suditos[&7];
    assert_eq!(sudito.propaganda_ticks, TICKS_PROPAGANDA);
    assert!(sudito.convertido);
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.seq, seq);
    assert_eq!(jogador.request_id, seq);
}

#[test]
fn hash_reflete_suditos() {
    let mut primeira = Mundo::new();
    let mut segunda = Mundo::new();
    for mundo in [&mut primeira, &mut segunda] {
        mundo.adicionar_jogador(1);
    }
    let sem_suditos = primeira.hash_estado();
    primeira.adicionar_sudito(7, (0.5, 0.0));
    primeira.adicionar_sudito(2, (-0.5, 0.0));
    // Ordem de inserção distinta: a iteração do BTreeMap é canônica.
    segunda.adicionar_sudito(2, (-0.5, 0.0));
    segunda.adicionar_sudito(7, (0.5, 0.0));
    assert_ne!(primeira.hash_estado(), sem_suditos);
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(sudito) = primeira.suditos.get_mut(&2) {
        sudito.convertido = true;
        sudito.propaganda_ticks = TICKS_PROPAGANDA;
        sudito.reversao_ticks = TICKS_REVERSAO_SUDITO;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    if let Some(sudito) = segunda.suditos.get_mut(&2) {
        sudito.convertido = true;
        sudito.propaganda_ticks = TICKS_PROPAGANDA;
        sudito.reversao_ticks = TICKS_REVERSAO_SUDITO;
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(sudito) = primeira.suditos.get_mut(&7) {
        sudito.posicao = (0.75, 0.0);
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());

    let mut com_alvo = Mundo::new();
    com_alvo.adicionar_jogador(1);
    let hash_sem_alvo = com_alvo.hash_estado();
    if let Some(jogador) = com_alvo.jogadores.get_mut(&1) {
        jogador.propaganda_alvo = Some(7);
    }
    assert_ne!(com_alvo.hash_estado(), hash_sem_alvo);
}

#[test]
fn movimento_cancela_propaganda() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.25, 0.0);
    let fora = (0.25, 0.5);
    mundo.adicionar_sudito(7, ponto);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();
    agendar_propaganda(&mut mundo, 1, 2, 2, ponto);
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].propaganda_alvo, Some(7));
    assert_eq!(mundo.suditos[&7].propaganda_ticks, 1);

    agendar_mover(&mut mundo, 1, 3, 3, fora);
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.posicao, fora);
    assert_eq!(jogador.propaganda_alvo, None);
    // O auto-avanço ocorre antes do `mover`, mas a saída zera o acúmulo do alvo.
    assert_eq!(mundo.suditos[&7].propaganda_ticks, 0);

    for _ in 0..TICKS_PROPAGANDA {
        mundo.avancar_tick();
    }
    assert!(!mundo.suditos[&7].convertido);
    assert_eq!(mundo.suditos[&7].propaganda_ticks, 0);

    // Um `mover` para o mesmo ponto não cancela o alvo recém-fixado.
    mundo.adicionar_sudito(8, fora);
    let seq = 4 + TICKS_PROPAGANDA;
    agendar_propaganda(&mut mundo, 1, seq, seq, fora);
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].propaganda_alvo, Some(8));
    let seq = seq + 1;
    agendar_mover(&mut mundo, 1, seq, seq, fora);
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.propaganda_alvo, Some(8));
    assert_eq!(mundo.suditos[&8].propaganda_ticks, 2);
}

#[test]
fn dois_jogadores_nao_aceleram_propaganda() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_jogador(2);
    let ponto = (0.25, 0.0);
    mundo.adicionar_sudito(7, ponto);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    agendar_mover(&mut mundo, 2, 1, 1, ponto);
    mundo.avancar_tick();

    // Comandos de dois pares no mesmo ponto, somados ao auto-avanço, valem 1 tick.
    for tick in 0..TICKS_PROPAGANDA {
        let seq = 2 + tick;
        agendar_propaganda(&mut mundo, 1, seq, seq, ponto);
        agendar_propaganda(&mut mundo, 2, seq, seq, ponto);
        mundo.avancar_tick();
        let sudito = &mundo.suditos[&7];
        if tick < TICKS_PROPAGANDA - 1 {
            assert_eq!(sudito.propaganda_ticks, tick + 1, "tick {tick}");
            assert!(!sudito.convertido, "tick {tick}");
        }
    }

    let sudito = &mundo.suditos[&7];
    assert_eq!(sudito.propaganda_ticks, TICKS_PROPAGANDA);
    assert!(sudito.convertido);
    assert_eq!(mundo.jogadores[&1].propaganda_alvo, Some(7));
    assert_eq!(mundo.jogadores[&2].propaganda_alvo, Some(7));
}

#[test]
fn ataque_custa_30_e_causa_10_dano() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_jogador(2);
    let ponto = (0.5, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    agendar_mover(&mut mundo, 2, 1, 1, ponto);
    mundo.avancar_tick();

    agendar_ataque(&mut mundo, 1, 2, 2, ponto);
    mundo.avancar_tick();
    assert_eq!(
        mundo.jogadores[&1].folego,
        FOLEGO_MAXIMO - CUSTO_ATAQUE_FOLEGO
    );
    assert_eq!(mundo.jogadores[&2].vida, 100 - DANO_ATAQUE);
    assert_eq!(mundo.jogadores[&1].combate_ticks, TICKS_COMBATE);
    assert_eq!(mundo.jogadores[&2].combate_ticks, TICKS_COMBATE);
    // O tick de `mover` e o do ataque acumulam 2; em combate só regenera no 6.º tick.
    assert_eq!(mundo.jogadores[&1].folego_ticks, 2);

    if let Some(alvo) = mundo.jogadores.get_mut(&2) {
        alvo.vida = 5;
    }
    if let Some(atacante) = mundo.jogadores.get_mut(&1) {
        atacante.folego = FOLEGO_MAXIMO;
    }
    agendar_ataque(&mut mundo, 1, 3, 3, ponto);
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&2].vida, 0);
    assert_eq!(
        mundo.jogadores[&1].folego,
        FOLEGO_MAXIMO - CUSTO_ATAQUE_FOLEGO
    );
}

#[test]
fn ataque_fora_de_alcance_recusado() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_jogador(2);
    let ponto = (3.0, 0.0);
    for (seq, destino) in [(1, (1.0, 0.0)), (2, (2.0, 0.0)), (3, ponto)] {
        agendar_mover(&mut mundo, 2, seq, seq, destino);
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&2].posicao, ponto);

    assert_eq!(
        ValidacaoComandos::validar(&mundo, 1, "atacar_forte", 1, 1, Some(ponto)),
        Err(CodigoRecusa::AlvoInvalido),
    );
    // Comando forjado sem validação também não altera o estado.
    mundo.agendar_comando(comando_ataque(1, 1, 1, ponto));
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].folego, FOLEGO_MAXIMO);
    assert_eq!(mundo.jogadores[&2].vida, 100);
    assert_eq!(mundo.jogadores[&1].combate_ticks, 0);
}

#[test]
fn folego_insuficiente_recusa() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_jogador(2);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.folego = CUSTO_ATAQUE_FOLEGO - 1;
    }
    assert_eq!(
        ValidacaoComandos::validar(&mundo, 1, "atacar_forte", 1, 1, Some((0.0, 0.0))),
        Err(CodigoRecusa::FolegoInsuficiente),
    );

    mundo.agendar_comando(comando_ataque(1, 1, 1, (0.0, 0.0)));
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].folego, CUSTO_ATAQUE_FOLEGO - 1);
    assert_eq!(mundo.jogadores[&2].vida, 100);
    assert_eq!(mundo.jogadores[&2].combate_ticks, 0);
}

#[test]
fn regenera_10_por_segundo_fora_de_combate() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.folego = 50;
    }

    // A 30 Hz, +1 a cada 3 ticks equivale a 10 Fôlego por segundo.
    for acumulado in 1..TICKS_REGENERACAO_FORA_COMBATE {
        mundo.avancar_tick();
        assert_eq!(mundo.jogadores[&1].folego, 50, "tick {acumulado}");
        assert_eq!(mundo.jogadores[&1].folego_ticks, acumulado);
    }
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].folego, 51);
    assert_eq!(mundo.jogadores[&1].folego_ticks, 0);

    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.folego = FOLEGO_MAXIMO;
        jogador.folego_ticks = 0;
    }
    for _ in 0..(TICKS_REGENERACAO_FORA_COMBATE * 2) {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&1].folego, FOLEGO_MAXIMO);
    assert_eq!(mundo.jogadores[&1].folego_ticks, 0);
}

#[test]
fn regenera_5_por_segundo_em_combate() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.folego = 50;
        jogador.combate_ticks = TICKS_COMBATE;
    }

    // A 30 Hz, +1 a cada 6 ticks equivale a 5 Fôlego por segundo em combate.
    for acumulado in 1..TICKS_REGENERACAO_COMBATE {
        mundo.avancar_tick();
        assert_eq!(mundo.jogadores[&1].folego, 50, "tick {acumulado}");
        assert_eq!(mundo.jogadores[&1].folego_ticks, acumulado);
        assert_eq!(mundo.jogadores[&1].combate_ticks, TICKS_COMBATE - acumulado);
    }
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].folego, 51);
    assert_eq!(mundo.jogadores[&1].folego_ticks, 0);
    assert_eq!(
        mundo.jogadores[&1].combate_ticks,
        TICKS_COMBATE - TICKS_REGENERACAO_COMBATE,
    );

    let mut decay = Mundo::new();
    decay.adicionar_jogador(1);
    if let Some(jogador) = decay.jogadores.get_mut(&1) {
        jogador.folego = 0;
        jogador.combate_ticks = TICKS_COMBATE;
    }
    for _ in 0..TICKS_COMBATE {
        decay.avancar_tick();
    }
    assert_eq!(decay.jogadores[&1].combate_ticks, 0);
    assert_eq!(
        decay.jogadores[&1].folego,
        TICKS_COMBATE / TICKS_REGENERACAO_COMBATE,
    );
}

#[test]
fn hash_reflete_folego_e_vida() {
    let mut primeira = Mundo::new();
    let mut segunda = Mundo::new();
    for mundo in [&mut primeira, &mut segunda] {
        mundo.adicionar_jogador(1);
        mundo.adicionar_jogador(2);
    }
    let mut anterior = primeira.hash_estado();
    assert_eq!(anterior, segunda.hash_estado());

    if let Some(jogador) = primeira.jogadores.get_mut(&1) {
        jogador.folego = 70;
    }
    let proximo = primeira.hash_estado();
    assert_ne!(proximo, anterior);
    anterior = proximo;

    if let Some(jogador) = primeira.jogadores.get_mut(&1) {
        jogador.folego_ticks = 2;
    }
    let proximo = primeira.hash_estado();
    assert_ne!(proximo, anterior);
    anterior = proximo;

    if let Some(jogador) = primeira.jogadores.get_mut(&2) {
        jogador.combate_ticks = TICKS_COMBATE;
    }
    let proximo = primeira.hash_estado();
    assert_ne!(proximo, anterior);
    anterior = proximo;

    if let Some(jogador) = primeira.jogadores.get_mut(&2) {
        jogador.vida = 90;
    }
    assert_ne!(primeira.hash_estado(), anterior);

    if let Some(jogador) = segunda.jogadores.get_mut(&1) {
        jogador.folego = 70;
        jogador.folego_ticks = 2;
    }
    if let Some(jogador) = segunda.jogadores.get_mut(&2) {
        jogador.combate_ticks = TICKS_COMBATE;
        jogador.vida = 90;
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());
}

#[test]
fn um_ataque_por_jogador_por_tick() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_jogador(2);
    let ponto = (0.5, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    agendar_mover(&mut mundo, 2, 1, 1, ponto);
    mundo.avancar_tick();

    agendar_ataque(&mut mundo, 1, 2, 2, ponto);
    agendar_ataque(&mut mundo, 1, 3, 3, ponto);
    mundo.avancar_tick();
    assert_eq!(
        mundo.jogadores[&1].folego,
        FOLEGO_MAXIMO - CUSTO_ATAQUE_FOLEGO
    );
    assert_eq!(mundo.jogadores[&2].vida, 100 - DANO_ATAQUE);

    let outro_ponto = (1.5, 0.0);
    let mut outro = Mundo::new();
    for peer_id in 1..=3 {
        outro.adicionar_jogador(peer_id);
    }
    agendar_mover(&mut outro, 1, 1, 1, ponto);
    agendar_mover(&mut outro, 2, 1, 1, ponto);
    agendar_mover(&mut outro, 3, 1, 1, (0.75, 0.0));
    outro.avancar_tick();
    agendar_mover(&mut outro, 3, 2, 2, outro_ponto);
    outro.avancar_tick();

    agendar_ataque(&mut outro, 1, 2, 2, outro_ponto);
    agendar_ataque(&mut outro, 2, 2, 2, outro_ponto);
    outro.avancar_tick();
    assert_eq!(
        outro.jogadores[&1].folego,
        FOLEGO_MAXIMO - CUSTO_ATAQUE_FOLEGO
    );
    assert_eq!(
        outro.jogadores[&2].folego,
        FOLEGO_MAXIMO - CUSTO_ATAQUE_FOLEGO
    );
    assert_eq!(outro.jogadores[&3].vida, 100 - 2 * DANO_ATAQUE);
    for peer_id in 1..=3 {
        assert_eq!(outro.jogadores[&peer_id].combate_ticks, TICKS_COMBATE);
    }
}

#[test]
fn comando_invalido_nao_suprime_ataque_valido_no_mesmo_tick() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_jogador(2);
    let ponto = (0.5, 0.0);
    agendar_mover(&mut mundo, 1, 1, 1, ponto);
    agendar_mover(&mut mundo, 2, 1, 1, ponto);
    mundo.avancar_tick();

    // Destino sem alvo: comando inválido, mas primeiro na fila do mesmo tick.
    mundo.agendar_comando(comando_ataque(1, 2, 2, (9.0, 9.0)));
    agendar_ataque(&mut mundo, 1, 3, 3, ponto);
    mundo.avancar_tick();

    assert_eq!(
        mundo.jogadores[&1].folego,
        FOLEGO_MAXIMO - CUSTO_ATAQUE_FOLEGO
    );
    assert_eq!(mundo.jogadores[&2].vida, 100 - DANO_ATAQUE);
    assert_eq!(mundo.jogadores[&1].combate_ticks, TICKS_COMBATE);
    assert_eq!(mundo.jogadores[&2].combate_ticks, TICKS_COMBATE);
}

#[test]
fn ilha_controlada_apos_300_ticks() {
    let mut mundo = Mundo::new();
    mundo.adicionar_sudito(7, (0.25, 0.0));
    mundo.adicionar_ilha(3);
    assert!(!mundo.ilhas[&3].controlada);
    assert_eq!(mundo.ilhas[&3].controle_ticks, 0);

    if let Some(sudito) = mundo.suditos.get_mut(&7) {
        sudito.convertido = true;
    }
    for _ in 0..(TICKS_CONTROLE_ILHA - 1) {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.ilhas[&3].controle_ticks, TICKS_CONTROLE_ILHA - 1);
    assert!(!mundo.ilhas[&3].controlada);
    assert!(mundo.sinais.is_empty());

    mundo.avancar_tick();
    assert_eq!(mundo.ilhas[&3].controle_ticks, TICKS_CONTROLE_ILHA);
    assert!(mundo.ilhas[&3].controlada);
    assert_eq!(
        mundo.sinais.iter().map(String::as_str).collect::<Vec<_>>(),
        vec!["ilha_convertida:3"]
    );

    mundo.avancar_tick();
    assert_eq!(mundo.ilhas[&3].controle_ticks, TICKS_CONTROLE_ILHA);
    assert_eq!(mundo.sinais.len(), 1);
}

#[test]
fn sem_sudito_ilha_perdida_sinal() {
    let mut mundo = Mundo::new();
    mundo.adicionar_ilha(5);
    for _ in 0..TICKS_CONTROLE_ILHA {
        mundo.avancar_tick();
    }
    assert!(!mundo.ilhas[&5].controlada);
    assert_eq!(mundo.ilhas[&5].controle_ticks, 0);
    assert!(
        mundo.sinais.is_empty(),
        "ilha nunca controlada não emite sinal"
    );

    mundo.adicionar_sudito(7, (0.25, 0.0));
    if let Some(sudito) = mundo.suditos.get_mut(&7) {
        sudito.convertido = true;
    }
    for _ in 0..10 {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.ilhas[&5].controle_ticks, 10);

    if let Some(sudito) = mundo.suditos.get_mut(&7) {
        sudito.convertido = false;
    }
    mundo.avancar_tick();
    assert_eq!(mundo.ilhas[&5].controle_ticks, 0);
    assert!(!mundo.ilhas[&5].controlada);
    assert_eq!(
        mundo.sinais.iter().map(String::as_str).collect::<Vec<_>>(),
        vec!["ilha_perdida:5"]
    );

    if let Some(sudito) = mundo.suditos.get_mut(&7) {
        sudito.convertido = true;
    }
    for _ in 0..TICKS_CONTROLE_ILHA {
        mundo.avancar_tick();
    }
    assert!(mundo.ilhas[&5].controlada);
    assert_eq!(
        mundo.sinais.iter().map(String::as_str).collect::<Vec<_>>(),
        vec!["ilha_perdida:5", "ilha_convertida:5"]
    );

    if let Some(sudito) = mundo.suditos.get_mut(&7) {
        sudito.convertido = false;
    }
    mundo.avancar_tick();
    assert!(!mundo.ilhas[&5].controlada);
    assert_eq!(mundo.ilhas[&5].controle_ticks, 0);
    assert_eq!(
        mundo.sinais.iter().map(String::as_str).collect::<Vec<_>>(),
        vec!["ilha_perdida:5", "ilha_convertida:5", "ilha_perdida:5"]
    );
    for _ in 0..5 {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.sinais.len(), 3);

    // Uma interrupção no meio zera a contagem contínua.
    let mut interrompida = Mundo::new();
    interrompida.adicionar_ilha(9);
    interrompida.adicionar_sudito(1, (0.25, 0.0));
    if let Some(sudito) = interrompida.suditos.get_mut(&1) {
        sudito.convertido = true;
    }
    for _ in 0..150 {
        interrompida.avancar_tick();
    }
    if let Some(sudito) = interrompida.suditos.get_mut(&1) {
        sudito.convertido = false;
    }
    interrompida.avancar_tick();
    if let Some(sudito) = interrompida.suditos.get_mut(&1) {
        sudito.convertido = true;
    }
    for _ in 0..150 {
        interrompida.avancar_tick();
    }
    assert_eq!(interrompida.ilhas[&9].controle_ticks, 150);
    assert!(!interrompida.ilhas[&9].controlada);
}

#[test]
fn hash_reflete_ilhas_e_sinais() {
    let mut primeira = Mundo::new();
    let mut segunda = Mundo::new();
    let sem_ilhas = primeira.hash_estado();
    assert_eq!(sem_ilhas, segunda.hash_estado());

    primeira.adicionar_ilha(2);
    primeira.adicionar_ilha(7);
    // Ordem de inserção distinta: a iteração do BTreeMap é canônica.
    segunda.adicionar_ilha(7);
    segunda.adicionar_ilha(2);
    assert_ne!(primeira.hash_estado(), sem_ilhas);
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(ilha) = primeira.ilhas.get_mut(&7) {
        ilha.controle_ticks = 123;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    if let Some(ilha) = segunda.ilhas.get_mut(&7) {
        ilha.controle_ticks = 123;
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(ilha) = primeira.ilhas.get_mut(&2) {
        ilha.controlada = true;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    if let Some(ilha) = segunda.ilhas.get_mut(&2) {
        ilha.controlada = true;
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    primeira.sinais.push_back("ilha_convertida:2".to_string());
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    segunda.sinais.push_back("ilha_convertida:2".to_string());
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    // A ordem FIFO faz parte do hash.
    primeira.sinais.push_back("ilha_perdida:7".to_string());
    segunda.sinais.push_front("ilha_perdida:7".to_string());
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    segunda.sinais.pop_front();
    segunda.sinais.push_back("ilha_perdida:7".to_string());
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    // Reiniciar uma ilha existente também altera o hash.
    primeira.adicionar_ilha(2);
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
}

#[test]
fn duas_ilhas_controle_deterministico() {
    let mut primeira = Mundo::new();
    let mut segunda = Mundo::new();
    for mundo in [&mut primeira, &mut segunda] {
        mundo.adicionar_sudito(7, (0.25, 0.0));
        if let Some(sudito) = mundo.suditos.get_mut(&7) {
            sudito.convertido = true;
        }
    }
    primeira.adicionar_ilha(2);
    primeira.adicionar_ilha(7);
    segunda.adicionar_ilha(7);
    segunda.adicionar_ilha(2);

    for _ in 0..TICKS_CONTROLE_ILHA {
        primeira.avancar_tick();
        segunda.avancar_tick();
    }
    for mundo in [&primeira, &segunda] {
        assert!(mundo.ilhas[&2].controlada);
        assert!(mundo.ilhas[&7].controlada);
        assert_eq!(mundo.ilhas[&2].controle_ticks, TICKS_CONTROLE_ILHA);
        assert_eq!(mundo.ilhas[&7].controle_ticks, TICKS_CONTROLE_ILHA);
        assert_eq!(
            mundo.sinais.iter().map(String::as_str).collect::<Vec<_>>(),
            vec!["ilha_convertida:2", "ilha_convertida:7"]
        );
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    for mundo in [&mut primeira, &mut segunda] {
        if let Some(sudito) = mundo.suditos.get_mut(&7) {
            sudito.convertido = false;
        }
        mundo.avancar_tick();
        assert_eq!(
            mundo.sinais.iter().map(String::as_str).collect::<Vec<_>>(),
            vec![
                "ilha_convertida:2",
                "ilha_convertida:7",
                "ilha_perdida:2",
                "ilha_perdida:7"
            ]
        );
        assert!(!mundo.ilhas[&2].controlada);
        assert!(!mundo.ilhas[&7].controlada);
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());
}

/// Posições fixas dos Nós do protótipo (`[HIPÓTESE]`), usadas pelos testes.
#[must_use]
fn posicoes_nos() -> [(u64, (f64, f64)); 3] {
    [(1, (0.5, 0.0)), (2, (1.5, 0.0)), (3, (2.5, 0.0))]
}

/// Monta um comando de Nó com os monotônicos já preenchidos.
#[must_use]
fn comando_no(peer_id: u64, acao: &str, seq: i64, request_id: i64, ponto: (f64, f64)) -> Comando {
    Comando {
        peer_id,
        acao: acao.to_string(),
        seq,
        request_id,
        destino: Some(ponto),
    }
}

/// Agenda uma ação de Nó somente após validá-la contra o estado corrente.
fn agendar_no(
    mundo: &mut Mundo,
    peer_id: u64,
    acao: &str,
    seq: i64,
    request_id: i64,
    ponto: (f64, f64),
) {
    garantir_nivel(mundo, peer_id, 6);
    assert!(ValidacaoComandos::validar(mundo, peer_id, acao, seq, request_id, Some(ponto)).is_ok());
    mundo.agendar_comando(comando_no(peer_id, acao, seq, request_id, ponto));
}

/// Move um jogador até `destino` em passos válidos de no máximo 1 metro.
fn posicionar_jogador(mundo: &mut Mundo, peer_id: u64, seq_inicial: i64, destino: (f64, f64)) {
    let (mut x, mut y) = mundo.jogadores[&peer_id].posicao;
    let mut seq = seq_inicial;
    while (x - destino.0).abs() > 1e-12 || (y - destino.1).abs() > 1e-12 {
        if (x - destino.0).abs() > 1e-12 {
            x = if destino.0 > x {
                (x + 1.0).min(destino.0)
            } else {
                (x - 1.0).max(destino.0)
            };
        } else {
            y = if destino.1 > y {
                (y + 1.0).min(destino.1)
            } else {
                (y - 1.0).max(destino.1)
            };
        }
        agendar_mover(mundo, peer_id, seq, seq, (x, y));
        mundo.avancar_tick();
        seq += 1;
    }
}

#[test]
fn no_convertido_apos_600_ticks() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_no(1, (0.5, 0.0));
    let seq = 1;
    posicionar_jogador(&mut mundo, 1, seq, (0.5, 0.0));

    agendar_no(&mut mundo, 1, "propaganda_no", 10, 10, (0.5, 0.0));
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].no_alvo, Some(1));
    assert_eq!(mundo.nos[&1].propaganda_ticks, 1);
    assert!(!mundo.nos[&1].convertido);

    for _ in 1..TICKS_PROPAGANDA_NO {
        mundo.avancar_tick();
    }
    let no = &mundo.nos[&1];
    assert_eq!(no.propaganda_ticks, TICKS_PROPAGANDA_NO);
    assert!(no.convertido);
    assert_eq!(
        mundo.resistencia_ideologica,
        RESISTENCIA_INICIAL - CONTRIBUICAO_NO
    );
    assert!(mundo.resultado.is_none());

    for _ in 0..3 {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.nos[&1].propaganda_ticks, TICKS_PROPAGANDA_NO);
    assert_eq!(
        mundo.resistencia_ideologica,
        RESISTENCIA_INICIAL - CONTRIBUICAO_NO
    );
}

#[test]
fn dano_nao_converte_no_e_respeita_teto() {
    let mut mundo = Mundo::new();
    for (id, _) in posicoes_nos() {
        mundo.adicionar_jogador(id);
    }
    for (id, posicao) in posicoes_nos() {
        mundo.adicionar_no(id, posicao);
    }
    for (id, posicao) in posicoes_nos() {
        posicionar_jogador(&mut mundo, id, 1, posicao);
    }
    // Sem progresso prévio de Propaganda, nenhum auto-avanço converte um Nó antes
    // dos ataques. O teto de 20 é global: só o primeiro Nó atacado sofre dano.

    for (id, posicao) in posicoes_nos() {
        agendar_no(&mut mundo, id, "atacar_no", 10, 10, posicao);
    }
    mundo.avancar_tick();

    assert_eq!(
        mundo.resistencia_ideologica,
        RESISTENCIA_INICIAL - DANO_MAXIMO_NOS
    );
    // O primeiro Nó atacado (menor id) recebe o dano e 600 ticks de inatividade;
    // os demais encontram a Resistência no piso e não entram em inatividade.
    let primeiro_id = posicoes_nos()[0].0;
    for (id, _) in posicoes_nos() {
        assert!(!mundo.nos[&id].convertido);
        assert_eq!(mundo.nos[&id].propaganda_ticks, 0);
        let inatividade = if id == primeiro_id {
            TICKS_INATIVIDADE_NO
        } else {
            0
        };
        assert_eq!(mundo.nos[&id].inativo_ticks, inatividade);
    }

    for _ in 0..TICKS_INATIVIDADE_NO {
        mundo.avancar_tick();
    }
    for (id, _) in posicoes_nos() {
        assert_eq!(mundo.nos[&id].inativo_ticks, 0);
    }
    assert_eq!(mundo.nos[&1].propaganda_ticks, 0);
    assert!(mundo.resultado.is_none());

    // Com a Resistência no piso derivado, o ataque tem dano zero: não converte
    // Nó nem reinicia a inatividade.
    for (id, posicao) in posicoes_nos() {
        agendar_no(&mut mundo, id, "atacar_no", 11, 11, posicao);
    }
    mundo.avancar_tick();
    assert_eq!(
        mundo.resistencia_ideologica,
        RESISTENCIA_INICIAL - DANO_MAXIMO_NOS
    );
    for (id, _) in posicoes_nos() {
        assert!(!mundo.nos[&id].convertido);
        assert_eq!(mundo.nos[&id].inativo_ticks, 0);
    }
}

#[test]
fn tres_nos_geram_vitoria() {
    let mut mundo = Mundo::new();
    for (id, _) in posicoes_nos() {
        mundo.adicionar_jogador(id);
    }
    for (id, posicao) in posicoes_nos() {
        mundo.adicionar_no(id, posicao);
    }
    for (id, posicao) in posicoes_nos() {
        posicionar_jogador(&mut mundo, id, 1, posicao);
    }
    for (id, posicao) in posicoes_nos() {
        agendar_no(&mut mundo, id, "propaganda_no", 10, 10, posicao);
    }
    mundo.avancar_tick();

    for _ in 1..TICKS_PROPAGANDA_NO {
        mundo.avancar_tick();
    }
    for (id, _) in posicoes_nos() {
        assert!(mundo.nos[&id].convertido);
    }
    assert_eq!(mundo.resistencia_ideologica, 10);
    assert!(mundo.resultado.is_none());

    mundo.avancar_tick();
    assert_eq!(mundo.resistencia_ideologica, 0);
    assert_eq!(mundo.resultado, Some(ResultadoPartida::Vitoria));

    for _ in 0..3 {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.resistencia_ideologica, 0);
    assert_eq!(mundo.resultado, Some(ResultadoPartida::Vitoria));
}

#[test]
fn dano_nao_bloqueia_vitoria() {
    let mut mundo = Mundo::new();
    for (id, _) in posicoes_nos() {
        mundo.adicionar_jogador(id);
    }
    for (id, posicao) in posicoes_nos() {
        mundo.adicionar_no(id, posicao);
    }
    for (id, posicao) in posicoes_nos() {
        posicionar_jogador(&mut mundo, id, 1, posicao);
    }

    // Dano prévio em todos os Nós: a Resistência para no teto derivado de 20.
    for (id, posicao) in posicoes_nos() {
        agendar_no(&mut mundo, id, "atacar_no", 10, 10, posicao);
    }
    mundo.avancar_tick();
    assert_eq!(
        mundo.resistencia_ideologica,
        RESISTENCIA_INICIAL - DANO_MAXIMO_NOS
    );
    for _ in 0..TICKS_INATIVIDADE_NO {
        mundo.avancar_tick();
    }

    for (id, posicao) in posicoes_nos() {
        agendar_no(&mut mundo, id, "propaganda_no", 20, 20, posicao);
    }
    mundo.avancar_tick();
    for _ in 1..TICKS_PROPAGANDA_NO {
        mundo.avancar_tick();
    }
    for (id, _) in posicoes_nos() {
        assert!(mundo.nos[&id].convertido);
    }
    // As duas primeiras conversões respeitam o piso; a última deixa a barra em 10.
    assert_eq!(mundo.resistencia_ideologica, 10);
    assert!(mundo.resultado.is_none());

    mundo.avancar_tick();
    assert_eq!(mundo.resistencia_ideologica, 0);
    assert_eq!(mundo.resultado, Some(ResultadoPartida::Vitoria));
}

#[test]
fn hash_inclui_acao_no() {
    let mut propaganda = Mundo::new();
    let mut ataque = Mundo::new();
    for mundo in [&mut propaganda, &mut ataque] {
        mundo.adicionar_jogador(1);
        mundo.adicionar_no(1, (0.5, 0.0));
        if let Some(no) = mundo.nos.get_mut(&1) {
            no.convertido = true;
        }
        posicionar_jogador(mundo, 1, 1, (0.5, 0.0));
    }
    assert_eq!(propaganda.hash_estado(), ataque.hash_estado());

    agendar_no(&mut propaganda, 1, "propaganda_no", 10, 10, (0.5, 0.0));
    propaganda.avancar_tick();
    agendar_no(&mut ataque, 1, "atacar_no", 10, 10, (0.5, 0.0));
    ataque.avancar_tick();
    assert_eq!(
        propaganda.jogadores[&1].no_alvo,
        ataque.jogadores[&1].no_alvo
    );
    assert_ne!(propaganda.hash_estado(), ataque.hash_estado());

    for mundo in [&mut propaganda, &mut ataque] {
        agendar_no(mundo, 1, "atacar_no", 11, 11, (0.5, 0.0));
        mundo.avancar_tick();
    }
    assert_eq!(propaganda.hash_estado(), ataque.hash_estado());
}

#[test]
fn hash_reflete_nos_e_resistencia() {
    let mut primeira = Mundo::new();
    let mut segunda = Mundo::new();
    let sem_nos = primeira.hash_estado();
    assert_eq!(sem_nos, segunda.hash_estado());

    for (id, posicao) in posicoes_nos() {
        primeira.adicionar_no(id, posicao);
    }
    for (id, posicao) in posicoes_nos().into_iter().rev() {
        segunda.adicionar_no(id, posicao);
    }
    assert_ne!(primeira.hash_estado(), sem_nos);
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    primeira.resistencia_ideologica -= CONTRIBUICAO_NO;
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    segunda.resistencia_ideologica -= CONTRIBUICAO_NO;
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(no) = primeira.nos.get_mut(&2) {
        no.convertido = true;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    if let Some(no) = segunda.nos.get_mut(&2) {
        no.convertido = true;
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(no) = primeira.nos.get_mut(&2) {
        no.propaganda_ticks = 123;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    if let Some(no) = segunda.nos.get_mut(&2) {
        no.propaganda_ticks = 123;
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    if let Some(no) = primeira.nos.get_mut(&3) {
        no.inativo_ticks = TICKS_INATIVIDADE_NO;
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    if let Some(no) = segunda.nos.get_mut(&3) {
        no.inativo_ticks = TICKS_INATIVIDADE_NO;
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    primeira.resultado = Some(ResultadoPartida::Vitoria);
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    segunda.resultado = Some(ResultadoPartida::Vitoria);
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());

    primeira.adicionar_jogador(1);
    if let Some(jogador) = primeira.jogadores.get_mut(&1) {
        jogador.no_alvo = Some(1);
    }
    assert_ne!(primeira.hash_estado(), segunda.hash_estado());
    segunda.adicionar_jogador(1);
    if let Some(jogador) = segunda.jogadores.get_mut(&1) {
        jogador.no_alvo = Some(1);
    }
    assert_eq!(primeira.hash_estado(), segunda.hash_estado());
}

#[test]
fn atacar_no_inativo_recusado() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_no(1, (0.5, 0.0));
    posicionar_jogador(&mut mundo, 1, 1, (0.5, 0.0));

    agendar_no(&mut mundo, 1, "atacar_no", 10, 10, (0.5, 0.0));
    mundo.avancar_tick();
    assert_eq!(
        mundo.resistencia_ideologica,
        RESISTENCIA_INICIAL - DANO_MAXIMO_NOS
    );
    assert_eq!(mundo.nos[&1].inativo_ticks, TICKS_INATIVIDADE_NO);
    assert_eq!(mundo.jogadores[&1].no_alvo, Some(1));

    let recusado = ValidacaoComandos::validar(&mundo, 1, "atacar_no", 11, 11, Some((0.5, 0.0)));
    assert_eq!(recusado, Err(CodigoRecusa::DestinoInvalido));

    for _ in 0..TICKS_INATIVIDADE_NO {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.nos[&1].inativo_ticks, 0);
    assert!(ValidacaoComandos::validar(&mundo, 1, "atacar_no", 12, 12, Some((0.5, 0.0))).is_ok());
    agendar_no(&mut mundo, 1, "atacar_no", 12, 12, (0.5, 0.0));
    mundo.avancar_tick();
    assert_eq!(
        mundo.resistencia_ideologica,
        RESISTENCIA_INICIAL - DANO_MAXIMO_NOS
    );
    assert!(!mundo.nos[&1].convertido);
}

#[test]
fn derrota_por_timeout() {
    let mut antes = Mundo::new();
    antes.tick = TICKS_LIMITE_PARTIDA - 2;
    antes.avancar_tick();
    assert_eq!(antes.tick, TICKS_LIMITE_PARTIDA - 1);
    assert!(antes.resultado.is_none());

    let mut mundo = Mundo::new();
    mundo.tick = TICKS_LIMITE_PARTIDA - 1;
    mundo.avancar_tick();
    assert_eq!(mundo.tick, TICKS_LIMITE_PARTIDA);
    assert_eq!(mundo.resultado, Some(ResultadoPartida::DerrotaPorTempo));

    for _ in 0..3 {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.resultado, Some(ResultadoPartida::DerrotaPorTempo));
}

#[test]
fn derrota_apos_tres_reconquistas() {
    let mut mundo = Mundo::new();
    for id in [1_u64, 2] {
        mundo.adicionar_ilha(id);
        if let Some(ilha) = mundo.ilhas.get_mut(&id) {
            ilha.controlada = true;
        }
    }
    mundo.avancar_tick();
    assert_eq!(
        mundo
            .ilhas_reconquistadas
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert!(mundo.resultado.is_none());

    mundo.adicionar_ilha(3);
    if let Some(ilha) = mundo.ilhas.get_mut(&3) {
        ilha.controlada = true;
    }
    mundo.avancar_tick();
    assert_eq!(
        mundo
            .ilhas_reconquistadas
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
    assert_eq!(
        mundo.resultado,
        Some(ResultadoPartida::DerrotaPorReconquista)
    );
}

#[test]
fn vitoria_precede_derrota() {
    let mut mundo = Mundo::new();
    for (id, posicao) in posicoes_nos() {
        mundo.adicionar_no(id, posicao);
        if let Some(no) = mundo.nos.get_mut(&id) {
            no.convertido = true;
        }
    }
    for id in [7_u64, 2, 5] {
        mundo.adicionar_ilha(id);
        if let Some(ilha) = mundo.ilhas.get_mut(&id) {
            ilha.controlada = true;
        }
    }
    mundo.resistencia_ideologica = RESISTENCIA_INICIAL - 3 * CONTRIBUICAO_NO;
    mundo.tick = TICKS_LIMITE_PARTIDA - 1;

    mundo.avancar_tick();
    assert_eq!(mundo.tick, TICKS_LIMITE_PARTIDA);
    assert_eq!(mundo.ilhas_reconquistadas.len(), MAX_RECONQUISTAS);
    assert_eq!(mundo.resultado, Some(ResultadoPartida::Vitoria));

    for _ in 0..2 {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.resultado, Some(ResultadoPartida::Vitoria));
}

#[test]
fn hash_reflete_derrota() {
    let mut primeiro = Mundo::new();
    let mut segundo = Mundo::new();
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    primeiro.ilhas_reconquistadas.insert(7);
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());
    segundo.ilhas_reconquistadas.insert(7);
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    // A serialização em ordem crescente não depende da ordem de inserção.
    let mut ordem_a = Mundo::new();
    let mut ordem_b = Mundo::new();
    for id in [7, 2, 5] {
        ordem_a.ilhas_reconquistadas.insert(id);
    }
    for id in [5, 7, 2] {
        ordem_b.ilhas_reconquistadas.insert(id);
    }
    assert_eq!(ordem_a.hash_estado(), ordem_b.hash_estado());
    ordem_a.ilhas_reconquistadas.insert(9);
    assert_ne!(ordem_a.hash_estado(), ordem_b.hash_estado());

    primeiro.resultado = Some(ResultadoPartida::DerrotaPorTempo);
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());
    segundo.resultado = Some(ResultadoPartida::DerrotaPorTempo);
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());
    segundo.resultado = Some(ResultadoPartida::DerrotaPorReconquista);
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());
}

#[test]
fn reconquista_duplicada_nao_conta() {
    let mut mundo = Mundo::new();
    mundo.adicionar_ilha(5);
    if let Some(ilha) = mundo.ilhas.get_mut(&5) {
        ilha.controlada = true;
    }
    mundo.avancar_tick();
    assert_eq!(
        mundo
            .ilhas_reconquistadas
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        vec![5]
    );
    assert!(mundo.resultado.is_none());

    if let Some(ilha) = mundo.ilhas.get_mut(&5) {
        ilha.controlada = true;
    }
    mundo.avancar_tick();
    assert_eq!(
        mundo.sinais.iter().map(String::as_str).collect::<Vec<_>>(),
        vec!["ilha_perdida:5", "ilha_perdida:5"]
    );
    assert_eq!(
        mundo
            .ilhas_reconquistadas
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        vec![5]
    );
    assert!(mundo.resultado.is_none());

    // Ilha latente sem progresso não emite sinal nem entra no conjunto.
    mundo.adicionar_ilha(9);
    for _ in 0..5 {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.sinais.len(), 2);
    assert_eq!(
        mundo
            .ilhas_reconquistadas
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        vec![5]
    );
    assert!(mundo.resultado.is_none());
}

/// A progressão usa somente o acumulador determinístico de cada jogador.
#[test]
fn nivel_sobe_apos_900_ticks() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    assert_eq!(mundo.jogadores[&1].funcao, Funcao::Artesao);
    assert_eq!(mundo.jogadores[&1].nivel, 1);
    assert_eq!(mundo.jogadores[&1].nivel_ticks, 0);

    for _ in 0..(TICKS_POR_NIVEL - 1) {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&1].nivel, 1);
    assert_eq!(mundo.jogadores[&1].nivel_ticks, TICKS_POR_NIVEL - 1);

    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].nivel, 2);
    assert_eq!(mundo.jogadores[&1].nivel_ticks, 0);

    for _ in 0..TICKS_POR_NIVEL {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&1].nivel, 3);
    assert_eq!(mundo.jogadores[&1].nivel_ticks, 0);
}

/// O teto satura e congela o acumulador.
#[test]
fn nivel_teto_12() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    for _ in 0..(TICKS_POR_NIVEL * i64::from(NIVEL_MAXIMO - 1)) {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&1].nivel, NIVEL_MAXIMO);
    assert_eq!(mundo.jogadores[&1].nivel_ticks, 0);

    for _ in 0..(3 * TICKS_POR_NIVEL) {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&1].nivel, NIVEL_MAXIMO);
    assert_eq!(mundo.jogadores[&1].nivel_ticks, 0);
}

/// `AcaoBloqueada` precede as recusas específicas, mas não as guardas iniciais.
#[test]
fn acao_bloqueada_por_nivel() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_sudito(7, (0.0, 0.0));
    mundo.adicionar_no(1, (0.0, 0.0));
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.materiais = 100;
        jogador.folego = FOLEGO_MAXIMO;
    }
    let ponto = (0.0, 0.0);

    for acao in [
        "construir_barreira",
        "construir_ponte",
        "propaganda",
        "propaganda_no",
        "atacar_no",
    ] {
        let bloqueado = ValidacaoComandos::validar(&mundo, 1, acao, 1, 1, Some(ponto));
        assert_eq!(bloqueado, Err(CodigoRecusa::AcaoBloqueada), "{acao}");
        let sem_destino = ValidacaoComandos::validar(&mundo, 1, acao, 1, 1, None);
        assert_eq!(sem_destino, Err(CodigoRecusa::AcaoBloqueada), "{acao}");
    }

    let sem_peer = ValidacaoComandos::validar(&mundo, 99, "atacar_no", 1, 1, None);
    assert_eq!(sem_peer, Err(CodigoRecusa::PeerNaoAutorizado));
    let replay = ValidacaoComandos::validar(&mundo, 1, "mover", 0, 0, Some((0.25, 0.0)));
    assert_eq!(replay, Err(CodigoRecusa::SeqOuRequestReplay));
    let desconhecida = ValidacaoComandos::validar(&mundo, 1, "voar", 1, 1, None);
    assert_eq!(desconhecida, Err(CodigoRecusa::AcaoDesconhecida));

    // Q e as ações sem slot nunca são bloqueadas por nível.
    let alvo = ValidacaoComandos::validar(&mundo, 1, "atacar_forte", 1, 1, None);
    assert_eq!(alvo, Err(CodigoRecusa::AlvoInvalido));
    let mover = ValidacaoComandos::validar(&mundo, 1, "mover", 1, 1, Some((0.25, 0.0)));
    assert!(mover.is_ok());

    garantir_nivel(&mut mundo, 1, 2);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.materiais = 0;
    }
    let saldo = ValidacaoComandos::validar(&mundo, 1, "construir_barreira", 1, 1, Some(ponto));
    assert_eq!(saldo, Err(CodigoRecusa::SaldoInsuficiente));
    let destino = ValidacaoComandos::validar(&mundo, 1, "construir_ponte", 1, 1, Some((0.75, 0.0)));
    assert_eq!(destino, Err(CodigoRecusa::DestinoInvalido));
}

/// Mapa Q1/W2/E3/R6 sobre ações existentes, sem novas teclas ou habilidades.
#[test]
fn desbloqueio_q_w_e_r() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_jogador(2);
    if let Some(alvo) = mundo.jogadores.get_mut(&2) {
        alvo.posicao = (0.0, 0.0);
    }
    mundo.adicionar_sudito(7, (0.0, 0.0));
    mundo.adicionar_no(1, (0.0, 0.0));
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.materiais = 100;
        jogador.folego = FOLEGO_MAXIMO;
    }
    let ponto = (0.0, 0.0);

    // Q (atacar_forte) e as ações livres já valem no nível 1.
    for acao in ["sinalizar_sincronia", "sinalizar_alavanca"] {
        assert!(ValidacaoComandos::validar(&mundo, 1, acao, 1, 1, None).is_ok());
    }
    for acao in ["atacar_forte", "coletar_materiais"] {
        assert!(ValidacaoComandos::validar(&mundo, 1, acao, 1, 1, Some(ponto)).is_ok());
    }
    assert!(ValidacaoComandos::validar(&mundo, 1, "mover", 1, 1, Some((0.25, 0.0))).is_ok());

    for (acao, nivel) in [
        ("construir_barreira", 2_u8),
        ("construir_ponte", 2),
        ("propaganda", 3),
        ("propaganda_no", 3),
        ("atacar_no", 6),
    ] {
        if let Some(jogador) = mundo.jogadores.get_mut(&1) {
            jogador.nivel = nivel - 1;
            jogador.nivel_ticks = 0;
        }
        let bloqueado = ValidacaoComandos::validar(&mundo, 1, acao, 1, 1, Some(ponto));
        assert_eq!(bloqueado, Err(CodigoRecusa::AcaoBloqueada), "{acao}");
        garantir_nivel(&mut mundo, 1, nivel);
        let liberado = ValidacaoComandos::validar(&mundo, 1, acao, 1, 1, Some(ponto));
        assert!(liberado.is_ok(), "{acao} no nivel {nivel}");
    }
}

/// O gate de nível do núcleo recusa comandos enfileirados diretamente: o
/// comando recusado não executa, não atualiza os monotônicos e é observável.
#[test]
fn acao_bloqueada_nao_executa_no_core() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (0.25, 0.0);
    definir_materiais(&mut mundo, 1, CUSTO_BARREIRA_MATERIAIS);
    assert_eq!(mundo.jogadores[&1].nivel, 1);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.posicao = ponto;
    }

    // Um chamador pode enfileirar sem passar pelo validador isolado.
    assert_eq!(
        ValidacaoComandos::validar(&mundo, 1, "construir_barreira", 1, 1, Some(ponto)),
        Err(CodigoRecusa::AcaoBloqueada)
    );
    mundo.agendar_comando(comando_barreira(1, 1, 1, ponto));
    mundo.avancar_tick();

    assert_eq!(mundo.ultima_recusa, Some(CodigoRecusa::AcaoBloqueada));
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.seq, 0);
    assert_eq!(jogador.request_id, 0);
    assert_eq!(jogador.construcao_ticks, 0);
    assert_eq!(jogador.construcao_ponto, None);
    assert_eq!(jogador.materiais, CUSTO_BARREIRA_MATERIAIS);
    assert!(mundo.barreiras.is_empty());

    // A recusa observável entra no hash determinístico do estado.
    let com_recusa = mundo.hash_estado();
    mundo.ultima_recusa = None;
    assert_ne!(mundo.hash_estado(), com_recusa);

    // No nível 2 o mesmo comando inicia a canalização e avança os monotônicos.
    garantir_nivel(&mut mundo, 1, 2);
    mundo.agendar_comando(comando_barreira(1, 2, 2, ponto));
    mundo.avancar_tick();
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.seq, 2);
    assert_eq!(jogador.request_id, 2);
    assert_eq!(jogador.construcao_ticks, 1);
    assert_eq!(jogador.construcao_ponto, Some(ponto));
    assert_eq!(jogador.materiais, CUSTO_BARREIRA_MATERIAIS);
    assert!(mundo.barreiras.is_empty());

    // A canalização restante é automática e conclui com débito atômico.
    for _ in 1..TICKS_CONSTRUCAO_BARREIRA {
        mundo.avancar_tick();
    }
    let jogador = &mundo.jogadores[&1];
    assert_eq!(jogador.construcao_ticks, 0);
    assert_eq!(jogador.construcao_ponto, None);
    assert_eq!(jogador.materiais, 0);
    assert_eq!(mundo.barreiras.len(), 1);
}

/// Papel, nível e acumulador entram no hash em ordem estável.
#[test]
fn hash_reflete_funcao_e_nivel() {
    let mut primeiro = Mundo::new();
    let mut segundo = Mundo::new();
    for mundo in [&mut primeiro, &mut segundo] {
        mundo.adicionar_jogador(1);
    }
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    if let Some(jogador) = primeiro.jogadores.get_mut(&1) {
        jogador.funcao = Funcao::Agitador;
    }
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());
    if let Some(jogador) = segundo.jogadores.get_mut(&1) {
        jogador.funcao = Funcao::Agitador;
    }
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    if let Some(jogador) = primeiro.jogadores.get_mut(&1) {
        jogador.nivel = 6;
    }
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());
    if let Some(jogador) = segundo.jogadores.get_mut(&1) {
        jogador.nivel = 6;
    }
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    if let Some(jogador) = primeiro.jogadores.get_mut(&1) {
        jogador.nivel_ticks = 17;
    }
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());
    if let Some(jogador) = segundo.jogadores.get_mut(&1) {
        jogador.nivel_ticks = 17;
    }
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());
}

/// PNJs ativos atacam o jogador mais próximo em alcance a cada 90 ticks.
#[test]
fn pnj_ataca_jogador_em_alcance() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_jogador(2);
    mundo.adicionar_jogador(3);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.posicao = (1.0, 0.0);
    }
    if let Some(jogador) = mundo.jogadores.get_mut(&2) {
        jogador.posicao = (1.5, 0.0);
    }
    if let Some(jogador) = mundo.jogadores.get_mut(&3) {
        jogador.posicao = (9.0, 0.0);
    }
    assert!((ALCANCE_ATAQUE_PNJ_M - 2.0).abs() < 1e-12);
    mundo.adicionar_pnj(1, (0.0, 0.0));
    if let Some(pnj) = mundo.pnjs.get_mut(&1) {
        pnj.ataque_ticks = TICKS_ATAQUE_PNJ - 1;
    }

    mundo.avancar_tick();

    // O jogador mais próximo leva o dano; os demais permanecem intactos.
    assert_eq!(mundo.jogadores[&1].vida, 100 - DANO_ATAQUE_PNJ);
    assert_eq!(mundo.jogadores[&2].vida, 100);
    assert_eq!(mundo.jogadores[&3].vida, 100);
    assert_eq!(mundo.pnjs[&1].ataque_ticks, 0);

    // Empate de distância é decidido pelo menor `peer_id`.
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.posicao = (1.0, 0.0);
        jogador.vida = 100;
    }
    if let Some(jogador) = mundo.jogadores.get_mut(&2) {
        jogador.posicao = (-1.0, 0.0);
        jogador.vida = 100;
    }
    if let Some(pnj) = mundo.pnjs.get_mut(&1) {
        pnj.ataque_ticks = TICKS_ATAQUE_PNJ - 1;
    }
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].vida, 100 - DANO_ATAQUE_PNJ);
    assert_eq!(mundo.jogadores[&2].vida, 100);

    // Nenhum dano no tick anterior ao limiar.
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.vida = 100;
    }
    if let Some(pnj) = mundo.pnjs.get_mut(&1) {
        pnj.ataque_ticks = TICKS_ATAQUE_PNJ - 2;
    }
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].vida, 100);
    assert_eq!(mundo.pnjs[&1].ataque_ticks, TICKS_ATAQUE_PNJ - 1);
}

/// Dano do jogador reduz os PV até zero, inativando sem remover a entrada.
#[test]
fn dano_mata_pnj() {
    let mut mundo = Mundo::new();
    mundo.adicionar_pnj(1, (0.0, 0.0));
    assert_eq!(TOTAL_PNJS_PADRAO, 4);
    assert_eq!(PV_PNJ, 24);
    assert_eq!(DANO_ATAQUE, 10);
    assert_eq!(mundo.pnjs[&1].pv, PV_PNJ);
    assert!(mundo.pnjs[&1].ativo);

    assert!(!mundo.aplicar_dano_a_pnj(1, 0));
    assert!(!mundo.aplicar_dano_a_pnj(1, -1));
    assert!(!mundo.aplicar_dano_a_pnj(99, DANO_ATAQUE));
    assert_eq!(mundo.pnjs[&1].pv, PV_PNJ);

    assert!(!mundo.aplicar_dano_a_pnj(1, DANO_ATAQUE));
    assert_eq!(mundo.pnjs[&1].pv, 14);
    assert!(mundo.pnjs[&1].ativo);
    assert!(mundo.aplicar_dano_a_pnj(1, DANO_ATAQUE + 4));
    assert_eq!(mundo.pnjs[&1].pv, 0);
    assert!(!mundo.pnjs[&1].ativo);
    assert!(mundo.pnjs.contains_key(&1));
    assert!(!mundo.aplicar_dano_a_pnj(1, DANO_ATAQUE));
    assert_eq!(mundo.pnjs[&1].pv, 0);

    mundo.adicionar_pnj(1, (0.0, 0.0));
    assert_eq!(mundo.pnjs[&1].pv, PV_PNJ);
    assert!(mundo.pnjs[&1].ativo);
    assert_eq!(mundo.pnjs[&1].ataque_ticks, 0);
}

/// `atacar_pnj` exige destino no ponto do jogador e PNJ ativo nesse ponto.
#[test]
fn atacar_pnj_respeita_alcance() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    let ponto = (5.0, 0.0);
    mundo.adicionar_pnj(1, ponto);

    assert_eq!(
        ValidacaoComandos::validar(&mundo, 1, "atacar_pnj", 1, 1, Some(ponto)),
        Err(CodigoRecusa::DestinoInvalido)
    );
    assert_eq!(
        ValidacaoComandos::validar(&mundo, 1, "atacar_pnj", 1, 1, Some((0.0, 0.0))),
        Err(CodigoRecusa::DestinoInvalido)
    );
    assert_eq!(
        ValidacaoComandos::validar(&mundo, 1, "atacar_pnj", 1, 1, None),
        Err(CodigoRecusa::DestinoInvalido)
    );

    // Comando recusado não atualiza os monotônicos.
    mundo.agendar_comando(comando_ataque_pnj(1, 1, 1, ponto));
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].seq, 0);
    assert_eq!(mundo.jogadores[&1].request_id, 0);
    assert_eq!(mundo.pnjs[&1].pv, PV_PNJ);

    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.posicao = ponto;
    }
    assert!(ValidacaoComandos::validar(&mundo, 1, "atacar_pnj", 2, 2, Some(ponto)).is_ok());

    // Dois comandos no mesmo PNJ e tick: apenas o primeiro é aceito.
    mundo.agendar_comando(comando_ataque_pnj(1, 2, 2, ponto));
    mundo.agendar_comando(comando_ataque_pnj(1, 3, 3, ponto));
    mundo.avancar_tick();
    assert_eq!(mundo.pnjs[&1].pv, PV_PNJ - DANO_ATAQUE);
    assert_eq!(mundo.jogadores[&1].seq, 2);
    assert_eq!(mundo.jogadores[&1].request_id, 2);

    assert!(mundo.aplicar_dano_a_pnj(1, PV_PNJ));
    assert_eq!(
        ValidacaoComandos::validar(&mundo, 1, "atacar_pnj", 4, 4, Some(ponto)),
        Err(CodigoRecusa::DestinoInvalido)
    );
}

/// O hash cobre criação, dano, inatividade, cadência e ordem dos PNJs.
#[test]
fn hash_reflete_pnjs() {
    let mut primeiro = Mundo::new();
    let mut segundo = Mundo::new();
    for mundo in [&mut primeiro, &mut segundo] {
        mundo.adicionar_jogador(1);
    }
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    primeiro.adicionar_pnj(1, (1.0, 0.0));
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());
    segundo.adicionar_pnj(1, (1.0, 0.0));
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    if let Some(pnj) = primeiro.pnjs.get_mut(&1) {
        pnj.pv = 14;
    }
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());
    if let Some(pnj) = segundo.pnjs.get_mut(&1) {
        pnj.pv = 14;
    }
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    if let Some(pnj) = primeiro.pnjs.get_mut(&1) {
        pnj.ativo = false;
    }
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());
    if let Some(pnj) = segundo.pnjs.get_mut(&1) {
        pnj.ativo = false;
    }
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    if let Some(pnj) = primeiro.pnjs.get_mut(&1) {
        pnj.ataque_ticks = 89;
    }
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());
    if let Some(pnj) = segundo.pnjs.get_mut(&1) {
        pnj.ataque_ticks = 89;
    }
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    primeiro.adicionar_pnj(2, (2.0, 0.0));
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());
    segundo.adicionar_pnj(2, (2.0, 0.0));
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());
}

/// PNJs inativos não atacam e, sem alvo em alcance, o contador zera sem dano.
#[test]
fn pnj_inativo_nao_ataca() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_pnj(1, (0.0, 0.0));
    mundo.adicionar_pnj(2, (0.0, 0.0));
    assert!(mundo.aplicar_dano_a_pnj(1, PV_PNJ));
    for pnj in mundo.pnjs.values_mut() {
        pnj.ataque_ticks = TICKS_ATAQUE_PNJ - 1;
    }

    mundo.avancar_tick();

    assert!(!mundo.pnjs[&1].ativo);
    assert_eq!(mundo.pnjs[&1].ataque_ticks, TICKS_ATAQUE_PNJ - 1);
    assert_eq!(mundo.pnjs[&2].ataque_ticks, 0);
    assert_eq!(mundo.jogadores[&1].vida, 100 - DANO_ATAQUE_PNJ);

    let mut sem_alvo = Mundo::new();
    sem_alvo.adicionar_pnj(1, (10.0, 0.0));
    if let Some(pnj) = sem_alvo.pnjs.get_mut(&1) {
        pnj.ataque_ticks = TICKS_ATAQUE_PNJ - 1;
    }
    sem_alvo.avancar_tick();
    assert_eq!(sem_alvo.pnjs[&1].ataque_ticks, 0);
}

/// `atacar_pnj` vale a 1,5 m do guarda, sem exigir co-localização exata.
#[test]
fn atacar_pnj_a_1_5m_sem_colocalizacao() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    // Distância hipotética de protótipo, dentro do alcance de contato de 2 m.
    let ponto_pnj = (1.5, 0.0);
    mundo.adicionar_pnj(1, ponto_pnj);
    assert!(ValidacaoComandos::validar(&mundo, 1, "atacar_pnj", 1, 1, Some(ponto_pnj)).is_ok());

    mundo.agendar_comando(comando_ataque_pnj(1, 1, 1, ponto_pnj));
    mundo.avancar_tick();

    assert_eq!(mundo.pnjs[&1].pv, PV_PNJ - DANO_ATAQUE);
    assert_eq!(mundo.jogadores[&1].seq, 1);

    // O mesmo destino é recusado quando o jogador está fora do alcance.
    mundo.adicionar_pnj(2, (5.0, 0.0));
    assert_eq!(
        ValidacaoComandos::validar(&mundo, 1, "atacar_pnj", 2, 2, Some((5.0, 0.0))),
        Err(CodigoRecusa::DestinoInvalido)
    );
}

/// A cota de `atacar_pnj` é por atacante: dois atacantes somam dano, um não.
#[test]
fn cota_de_ataque_pnj_e_por_atacante() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_jogador(2);
    let ponto = (0.0, 0.0);
    mundo.adicionar_pnj(1, ponto);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.posicao = (0.5, 0.0);
    }
    if let Some(jogador) = mundo.jogadores.get_mut(&2) {
        jogador.posicao = (1.0, 0.0);
    }
    assert!(ValidacaoComandos::validar(&mundo, 1, "atacar_pnj", 1, 1, Some(ponto)).is_ok());
    assert!(ValidacaoComandos::validar(&mundo, 2, "atacar_pnj", 1, 1, Some(ponto)).is_ok());

    mundo.agendar_comando(comando_ataque_pnj(1, 1, 1, ponto));
    mundo.agendar_comando(comando_ataque_pnj(2, 1, 1, ponto));
    mundo.avancar_tick();

    assert_eq!(mundo.pnjs[&1].pv, PV_PNJ - 2 * DANO_ATAQUE);
    assert_eq!(mundo.jogadores[&1].seq, 1);
    assert_eq!(mundo.jogadores[&2].seq, 1);

    let mut outro = Mundo::new();
    outro.adicionar_jogador(1);
    outro.adicionar_pnj(1, (0.0, 0.0));
    outro.adicionar_pnj(2, (1.0, 0.0));
    if let Some(jogador) = outro.jogadores.get_mut(&1) {
        jogador.posicao = (0.5, 0.0);
    }
    outro.agendar_comando(comando_ataque_pnj(1, 1, 1, (0.0, 0.0)));
    outro.agendar_comando(comando_ataque_pnj(1, 2, 2, (1.0, 0.0)));
    outro.avancar_tick();

    let dano_total = (PV_PNJ - outro.pnjs[&1].pv) + (PV_PNJ - outro.pnjs[&2].pv);
    assert_eq!(dano_total, DANO_ATAQUE);
    assert_eq!(outro.jogadores[&1].seq, 1);
}

/// O contra-ataque do PNJ recai no menor `peer_id`, mesmo sendo o mais distante.
#[test]
fn pnj_ataca_menor_peer_id_em_alcance() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_jogador(2);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.posicao = (1.9, 0.0);
    }
    if let Some(jogador) = mundo.jogadores.get_mut(&2) {
        jogador.posicao = (0.1, 0.0);
    }
    mundo.adicionar_pnj(1, (0.0, 0.0));
    if let Some(pnj) = mundo.pnjs.get_mut(&1) {
        pnj.ataque_ticks = TICKS_ATAQUE_PNJ - 1;
    }

    mundo.avancar_tick();

    assert_eq!(mundo.jogadores[&1].vida, 100 - DANO_ATAQUE_PNJ);
    assert_eq!(mundo.jogadores[&2].vida, 100);
}

/// A marca autoriza o bônus inteiro de 25%: 10 de dano passa a 12.
#[test]
fn analise_marca_e_bonus_de_dano() {
    let mut mundo = mundo_para_analise();
    let ponto = (0.0, 0.0);
    mundo.adicionar_pnj(2, (10.0, 0.0));
    assert_eq!(mundo.pnjs[&1].marca_ticks, 0);

    agendar_analise(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();

    assert_eq!(mundo.pnjs[&1].marca_ticks, TICKS_ANALISE_MATERIALISTA);
    assert_eq!(
        mundo.jogadores[&1].analise_recarga_ticks,
        RECARGA_ANALISE_MATERIALISTA_TICKS
    );
    assert_eq!(
        mundo.jogadores[&1].folego,
        FOLEGO_MAXIMO - CUSTO_ANALISE_MATERIALISTA
    );
    assert_eq!(mundo.jogadores[&1].seq, 1);
    assert_eq!(mundo.jogadores[&1].request_id, 1);

    let dano_esperado = DANO_ATAQUE + DANO_ATAQUE * BONUS_DANO_ANALISE_PERCENT / 100;
    assert_eq!(dano_esperado, 12);
    assert!(!mundo.aplicar_dano_a_pnj(2, DANO_ATAQUE));
    mundo.aplicar_dano_a_pnj(1, DANO_ATAQUE);
    assert_eq!(mundo.pnjs[&1].pv, PV_PNJ - dano_esperado);
    assert_eq!(mundo.pnjs[&2].pv, PV_PNJ - DANO_ATAQUE);

    // O piso zero e a inativação permanecem com o bônus aplicado.
    assert!(mundo.aplicar_dano_a_pnj(1, PV_PNJ));
    assert_eq!(mundo.pnjs[&1].pv, 0);
    assert!(!mundo.pnjs[&1].ativo);
}

/// Duas marcas no mesmo PNJ não somam: a reaplicação reinicia 150 ticks.
#[test]
fn analise_nao_acumula() {
    let mut mundo = mundo_para_analise();
    mundo.adicionar_jogador(2);
    garantir_nivel(&mut mundo, 2, 3);
    if let Some(jogador) = mundo.jogadores.get_mut(&2) {
        jogador.posicao = (1.0, 0.0);
    }
    let ponto = (0.0, 0.0);

    agendar_analise(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();
    assert_eq!(mundo.pnjs[&1].marca_ticks, TICKS_ANALISE_MATERIALISTA);

    for _ in 0..30 {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.pnjs[&1].marca_ticks, TICKS_ANALISE_MATERIALISTA - 30);

    agendar_analise(&mut mundo, 2, 1, 1, ponto);
    mundo.avancar_tick();
    assert_eq!(mundo.pnjs[&1].marca_ticks, TICKS_ANALISE_MATERIALISTA);
    assert_eq!(
        mundo.jogadores[&2].analise_recarga_ticks,
        RECARGA_ANALISE_MATERIALISTA_TICKS
    );

    // Após a recarga completa, o mesmo atacante também reinicia sem somar.
    for _ in 0..(RECARGA_ANALISE_MATERIALISTA_TICKS - 1) {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&1].analise_recarga_ticks, 0);
    assert_eq!(mundo.pnjs[&1].marca_ticks, 0);
    agendar_analise(&mut mundo, 1, 2, 2, ponto);
    mundo.avancar_tick();
    assert_eq!(mundo.pnjs[&1].marca_ticks, TICKS_ANALISE_MATERIALISTA);
}

/// A recarga de 450 ticks bloqueia nova análise sem debitar Fôlego.
#[test]
fn analise_respeita_recarga() {
    let mut mundo = mundo_para_analise();
    let ponto = (0.0, 0.0);
    agendar_analise(&mut mundo, 1, 1, 1, ponto);
    mundo.avancar_tick();
    assert_eq!(
        mundo.jogadores[&1].analise_recarga_ticks,
        RECARGA_ANALISE_MATERIALISTA_TICKS
    );

    let em_recarga =
        ValidacaoComandos::validar(&mundo, 1, "analise_materialista", 2, 2, Some(ponto));
    assert_eq!(em_recarga, Err(CodigoRecusa::AnaliseEmRecarga));

    // Comando forjado durante a recarga: sem efeito e sem monotônicos.
    mundo.agendar_comando(comando_analise(1, 2, 2, ponto));
    mundo.avancar_tick();
    assert_eq!(
        mundo.jogadores[&1].analise_recarga_ticks,
        RECARGA_ANALISE_MATERIALISTA_TICKS - 1
    );
    assert_eq!(
        mundo.jogadores[&1].folego,
        FOLEGO_MAXIMO - CUSTO_ANALISE_MATERIALISTA
    );
    assert_eq!(mundo.jogadores[&1].seq, 1);
    assert_eq!(mundo.jogadores[&1].request_id, 1);
    assert_eq!(mundo.pnjs[&1].marca_ticks, TICKS_ANALISE_MATERIALISTA - 1);

    for _ in 0..(RECARGA_ANALISE_MATERIALISTA_TICKS - 1) {
        mundo.avancar_tick();
    }
    assert_eq!(mundo.jogadores[&1].analise_recarga_ticks, 0);
    if let Some(jogador) = mundo.jogadores.get_mut(&1) {
        jogador.folego = 50;
        jogador.folego_ticks = 0;
    }
    agendar_analise(&mut mundo, 1, 3, 3, ponto);
    mundo.avancar_tick();
    assert_eq!(mundo.jogadores[&1].folego, 50 - CUSTO_ANALISE_MATERIALISTA);
    assert_eq!(
        mundo.jogadores[&1].analise_recarga_ticks,
        RECARGA_ANALISE_MATERIALISTA_TICKS
    );
    assert_eq!(mundo.jogadores[&1].seq, 3);
}

/// O alcance é de 12 m, inclusive no limite; além dele nada é debitado.
#[test]
fn analise_respeita_alcance() {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    garantir_nivel(&mut mundo, 1, 3);
    let no_limite = (ALCANCE_ANALISE_MATERIALISTA_M, 0.0);
    mundo.adicionar_pnj(1, no_limite);
    agendar_analise(&mut mundo, 1, 1, 1, no_limite);
    mundo.avancar_tick();
    assert_eq!(mundo.pnjs[&1].marca_ticks, TICKS_ANALISE_MATERIALISTA);
    assert_eq!(
        mundo.jogadores[&1].folego,
        FOLEGO_MAXIMO - CUSTO_ANALISE_MATERIALISTA
    );

    let fora = (ALCANCE_ANALISE_MATERIALISTA_M + 0.5, 0.0);
    let mut outro = Mundo::new();
    outro.adicionar_jogador(1);
    garantir_nivel(&mut outro, 1, 3);
    outro.adicionar_pnj(1, fora);
    let recusado = ValidacaoComandos::validar(&outro, 1, "analise_materialista", 1, 1, Some(fora));
    assert_eq!(recusado, Err(CodigoRecusa::DestinoInvalido));
    outro.agendar_comando(comando_analise(1, 1, 1, fora));
    outro.avancar_tick();
    assert_eq!(outro.pnjs[&1].marca_ticks, 0);
    assert_eq!(outro.jogadores[&1].folego, FOLEGO_MAXIMO);
    assert_eq!(outro.jogadores[&1].seq, 0);
}

/// Marca e recarga entram no hash canônico, em ordem estável de identificadores.
#[test]
fn hash_reflete_analise_materialista() {
    let mut primeiro = mundo_para_analise();
    let mut segundo = mundo_para_analise();
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    agendar_analise(&mut primeiro, 1, 1, 1, (0.0, 0.0));
    primeiro.avancar_tick();
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());

    agendar_analise(&mut segundo, 1, 1, 1, (0.0, 0.0));
    segundo.avancar_tick();
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());

    if let Some(pnj) = primeiro.pnjs.get_mut(&1) {
        pnj.marca_ticks = 0;
    }
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());

    if let Some(jogador) = segundo.jogadores.get_mut(&1) {
        jogador.analise_recarga_ticks = 0;
    }
    assert_ne!(primeiro.hash_estado(), segundo.hash_estado());

    if let Some(pnj) = primeiro.pnjs.get_mut(&1) {
        pnj.marca_ticks = TICKS_ANALISE_MATERIALISTA;
    }
    if let Some(jogador) = segundo.jogadores.get_mut(&1) {
        jogador.analise_recarga_ticks = RECARGA_ANALISE_MATERIALISTA_TICKS;
    }
    assert_eq!(primeiro.hash_estado(), segundo.hash_estado());
}
