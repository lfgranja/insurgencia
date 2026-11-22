#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Demonstração determinística e headless da simulação autoritativa.
//!
//! Nenhuma janela, plugin de engine, relógio, rede ou aleatoriedade é usado:
//! a saída depende apenas do estado inicial e da sequência fixa de comandos.

use insurgencia_sim::{Comando, Mundo, PASSO_SEGUNDOS};

/// Número de ticks fixos executados pela demonstração.
const TICKS: i32 = 30;

/// Monta um comando `mover` com os monotônicos já preenchidos.
#[must_use]
fn comando_mover(peer_id: u64, seq: i64, request_id: i64, destino: (f64, f64)) -> Comando {
    Comando {
        peer_id,
        acao: String::from("mover"),
        seq,
        request_id,
        destino: Some(destino),
    }
}

/// Executa a sequência determinística e devolve o hash final do estado.
#[must_use]
fn executar() -> String {
    let mut mundo = Mundo::new();
    mundo.adicionar_jogador(1);
    mundo.adicionar_jogador(2);

    for tick in 1..=TICKS {
        let monotônico = i64::from(tick);
        let deslocamento = f64::from(tick % 5) / 100.0;

        mundo.agendar_comando(comando_mover(
            1,
            monotônico,
            monotônico,
            (deslocamento, 0.0),
        ));
        mundo.agendar_comando(comando_mover(
            2,
            monotônico,
            monotônico,
            (0.0, -deslocamento),
        ));
        mundo.avancar_tick();

        let hash = mundo.hash_estado();
        println!("tick={tick} hash={hash}");
    }

    mundo.hash_estado()
}

fn main() {
    let hash_final = executar();
    let segundos = f64::from(TICKS) * PASSO_SEGUNDOS;
    println!("resumo: ticks={TICKS} segundos={segundos} jogadores=2 hash_final={hash_final}");
}
