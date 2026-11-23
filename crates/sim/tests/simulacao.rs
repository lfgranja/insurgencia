#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use insurgencia_sim::{
    Barreira, CodigoRecusa, Comando, Mundo, Ponte, ValidacaoComandos, CUSTO_BARREIRA_MATERIAIS,
    CUSTO_PONTE_MATERIAIS, MAX_BARREIRAS_POR_JOGADOR, PONTOS_AUTOGESTAO_POR_DEFESA, PV_BARREIRA,
    PV_PONTE, TICKS_AUTOGESTAO_POR_PONTO, TICKS_COLETA, TICKS_CONSTRUCAO_BARREIRA,
    TICKS_CONSTRUCAO_PONTE, TICKS_DURACAO_BARREIRA, TICKS_DURACAO_PONTE,
};

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
