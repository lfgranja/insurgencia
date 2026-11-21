#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use insurgencia_metricas::{
    correr_partidas, ErroMetricas, EventoNucleo, Relatorio, TipoCliente, LIMITE_CONCLUSAO_PCT,
    LIMITE_FORKS, LIMITE_MEDIANA_CAPTURA_S, LIMITE_MODULOS_12M, LIMITE_MODULOS_3M,
    LIMITE_P95_CAPTURA_S, LIMITE_RESPOSTA_POSITIVA_PCT, LIMITE_RTT_COMUNITARIO_MS,
    LIMITE_RTT_MEDIANO_MS, LIMITE_RTT_P95_MS, LIMITE_SERVIDORES_12M, LIMITE_SERVIDORES_3M,
    LIMITE_SINCRONIA_MAX_PCT, LIMITE_SINCRONIA_MIN_PCT,
};

/// Obtém um relatório por casamento de padrão: erro é falha explícita do teste.
fn relatorio(n: usize, cliente_modificado: bool) -> Relatorio {
    match correr_partidas(n, cliente_modificado) {
        Ok(value) => value,
        Err(erro) => panic!("correr_partidas({n}, {cliente_modificado}) falhou: {erro}"),
    }
}

#[test]
fn vinte_partidas_consecutivas_instrumentadas() {
    let relatorio = relatorio(20, false);

    assert_eq!(relatorio.registros.len(), 20);
    assert_eq!(relatorio.registros[0].id_partida, 1);
    assert_eq!(relatorio.registros[19].id_partida, 20);

    for (indice, registro) in relatorio.registros.iter().enumerate() {
        assert_eq!(registro.id_partida, indice + 1);
        assert_eq!(
            registro.eventos.first(),
            Some(&EventoNucleo::PartidaIniciada)
        );
        assert!(registro.eventos.contains(&EventoNucleo::CapturaIlha));
        assert_ne!(registro.tentativas_sincronia, 0);
        assert_eq!(registro.cliente_modificado, TipoCliente::Padrao);
    }
}

#[test]
fn duas_partidas_consecutivas_com_cliente_modificado() {
    let relatorio_modificado = relatorio(2, true);

    assert_eq!(relatorio_modificado.registros.len(), 2);
    assert_eq!(relatorio_modificado.registros[0].id_partida, 1);
    assert_eq!(relatorio_modificado.registros[1].id_partida, 2);
    assert_eq!(
        relatorio_modificado.registros[0].cliente_modificado,
        TipoCliente::Padrao
    );
    assert_eq!(
        relatorio_modificado.registros[1].cliente_modificado,
        TipoCliente::Modificado
    );
    assert_eq!(
        relatorio_modificado.registros[0].divergencias_cliente_servidor,
        0
    );
    assert_eq!(
        relatorio_modificado.registros[1].divergencias_cliente_servidor,
        1
    );
    assert_eq!(
        relatorio_modificado.registros[1].tempo_ate_primeira_captura_s,
        80
    );
    assert_eq!(relatorio_modificado.registros[1].rtt_cliente_ms, 85);

    let canonico = relatorio(2, false);
    assert_eq!(
        canonico
            .registros
            .iter()
            .filter(|registro| registro.cliente_modificado == TipoCliente::Modificado)
            .count(),
        0
    );
}

#[test]
fn cinco_metricas_calculadas() {
    let relatorio = relatorio(20, false);

    assert_eq!(relatorio.mediana_tempo_captura(), 78);
    assert_eq!(relatorio.p95_tempo_captura(), 118);
    assert_eq!(relatorio.taxa_conclusao_sincronia(), 71);
    assert_eq!(relatorio.rtt_mediano_cliente(), 69);
    assert_eq!(relatorio.rtt_p95_cliente(), 95);
    assert_eq!(relatorio.taxa_resposta_sem_atraso(), 100);
    assert_eq!(relatorio.servidores_comunitarios_ativos(), 5);
    assert_eq!(relatorio.modulos_publicados_distintos(), 8);
    assert_eq!(relatorio.forks_publicados(), 2);
    assert_eq!(relatorio.taxa_partidas_concluidas(), 75);
}

#[test]
fn determinismo_entre_execucoes() {
    let primeiro = relatorio(20, false);
    let segundo = relatorio(20, false);
    let modificado = relatorio(20, true);

    assert_eq!(primeiro, segundo);
    assert_ne!(primeiro, modificado);
    assert_eq!(primeiro.registros.len(), modificado.registros.len());
    assert_eq!(
        primeiro.mediana_tempo_captura(),
        segundo.mediana_tempo_captura()
    );
}

#[test]
fn divergencia_cliente_servidor_registrada_no_nucleo() {
    let canonico = relatorio(20, false);
    let modificado = relatorio(20, true);
    let duas = relatorio(2, true);
    let ultimo = &duas.registros[1];

    assert_eq!(canonico.total_divergencias(), 0);
    assert_eq!(
        canonico.contagem_eventos(EventoNucleo::DivergenciaClienteServidor),
        0
    );
    assert_eq!(modificado.total_divergencias(), 3);
    assert_eq!(
        modificado.contagem_eventos(EventoNucleo::DivergenciaClienteServidor),
        3
    );
    assert_eq!(duas.total_divergencias(), 1);
    assert_eq!(ultimo.divergencias_cliente_servidor, 1);
    assert_eq!(
        ultimo
            .eventos
            .iter()
            .filter(|evento| **evento == EventoNucleo::DivergenciaClienteServidor)
            .count(),
        1
    );
}

#[test]
fn limites_hipotese_da_secao_17() {
    assert_eq!(LIMITE_MEDIANA_CAPTURA_S, 90);
    assert_eq!(LIMITE_P95_CAPTURA_S, 150);
    assert_eq!(LIMITE_SINCRONIA_MIN_PCT, 60);
    assert_eq!(LIMITE_SINCRONIA_MAX_PCT, 90);
    assert_eq!(LIMITE_RTT_MEDIANO_MS, 80);
    assert_eq!(LIMITE_RTT_COMUNITARIO_MS, 150);
    assert_eq!(LIMITE_RTT_P95_MS, 250);
    assert_eq!(LIMITE_RESPOSTA_POSITIVA_PCT, 90);
    assert_eq!(LIMITE_SERVIDORES_3M, 5);
    assert_eq!(LIMITE_SERVIDORES_12M, 20);
    assert_eq!(LIMITE_MODULOS_3M, 3);
    assert_eq!(LIMITE_MODULOS_12M, 10);
    assert_eq!(LIMITE_FORKS, 2);
    assert_eq!(LIMITE_CONCLUSAO_PCT, 50);

    for cliente_modificado in [false, true] {
        let relatorio = relatorio(20, cliente_modificado);
        assert!(relatorio.mediana_tempo_captura() <= LIMITE_MEDIANA_CAPTURA_S);
        assert!(relatorio.p95_tempo_captura() <= LIMITE_P95_CAPTURA_S);
        assert!(relatorio.taxa_conclusao_sincronia() >= LIMITE_SINCRONIA_MIN_PCT);
        assert!(relatorio.taxa_conclusao_sincronia() <= LIMITE_SINCRONIA_MAX_PCT);
        assert!(relatorio.rtt_mediano_cliente() <= LIMITE_RTT_MEDIANO_MS);
        assert!(relatorio.rtt_mediano_cliente() <= LIMITE_RTT_COMUNITARIO_MS);
        assert!(relatorio.rtt_p95_cliente() <= LIMITE_RTT_P95_MS);
        assert!(relatorio.taxa_resposta_sem_atraso() >= LIMITE_RESPOSTA_POSITIVA_PCT);
        assert!(relatorio.servidores_comunitarios_ativos() >= LIMITE_SERVIDORES_3M);
        assert!(relatorio.modulos_publicados_distintos() >= LIMITE_MODULOS_3M);
        assert!(relatorio.forks_publicados() >= LIMITE_FORKS);
        assert!(relatorio.taxa_partidas_concluidas() >= LIMITE_CONCLUSAO_PCT);
    }
}

#[test]
fn amostra_vazia_retorna_erro_tipado() {
    assert_eq!(correr_partidas(0, false), Err(ErroMetricas::SemPartidas));
    assert_eq!(correr_partidas(0, true), Err(ErroMetricas::SemPartidas));

    let unica = relatorio(1, false);
    assert_eq!(unica.registros.len(), 1);
}

#[test]
fn agregacoes_de_amostra_unitaria_e_vazia() {
    let unica = relatorio(1, false);
    let registro = &unica.registros[0];

    assert_eq!(registro.tempo_ate_primeira_captura_s, 47);
    assert_eq!(registro.sincronias_concluidas, 17);
    assert_eq!(registro.tentativas_sincronia, 20);
    assert_eq!(unica.mediana_tempo_captura(), 47);
    assert_eq!(unica.p95_tempo_captura(), 47);
    assert_eq!(unica.rtt_mediano_cliente(), registro.rtt_cliente_ms);
    assert_eq!(unica.rtt_p95_cliente(), registro.rtt_cliente_ms);
    assert_eq!(unica.taxa_conclusao_sincronia(), 85);
    assert_eq!(unica.taxa_partidas_concluidas(), 100);

    let vazia = Relatorio {
        registros: Vec::new(),
    };
    assert_eq!(vazia.mediana_tempo_captura(), 0);
    assert_eq!(vazia.p95_tempo_captura(), 0);
    assert_eq!(vazia.taxa_conclusao_sincronia(), 0);
    assert_eq!(vazia.rtt_mediano_cliente(), 0);
    assert_eq!(vazia.rtt_p95_cliente(), 0);
    assert_eq!(vazia.taxa_resposta_sem_atraso(), 0);
    assert_eq!(vazia.servidores_comunitarios_ativos(), 0);
    assert_eq!(vazia.modulos_publicados_distintos(), 0);
    assert_eq!(vazia.forks_publicados(), 0);
    assert_eq!(vazia.taxa_partidas_concluidas(), 0);
    assert_eq!(vazia.total_divergencias(), 0);
    assert_eq!(vazia.contagem_eventos(EventoNucleo::Vitoria), 0);
}
