#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Servidor headless determinístico do Núcleo Insurgência.
//!
//! Lê comandos da `stdin`, alimenta o `insurgencia-sim` e emite o hash canônico
//! do estado a cada tick, sem relógio, rede, aleatoriedade ou janela.

use std::collections::BTreeSet;
use std::env;
use std::io::{self, BufRead, BufWriter, Write};
use std::process::ExitCode;

use insurgencia_sim::{Comando, Mundo};

/// Ações conhecidas do contrato, espelhando `ACOES_PERMITIDAS` do núcleo.
const ACOES_PERMITIDAS: &[&str] = &[
    "mover",
    "sinalizar_sincronia",
    "sinalizar_alavanca",
    "coletar_materiais",
    "construir_barreira",
    "atacar_forte",
];

/// Comando validado na fronteira da `stdin`, antes do agendamento no `Mundo`.
#[derive(Clone, Debug, PartialEq)]
struct LinhaComando {
    /// Identificador do par que enviou o comando.
    peer_id: u64,
    /// Ação solicitada.
    acao: String,
    /// Sequencial monotônico do comando.
    seq: i64,
    /// Identificador de requisição monotônico do comando.
    request_id: i64,
    /// Destino opcional, exigido por ações que dependem de posição.
    destino: Option<(f64, f64)>,
}

/// Analisa os argumentos da linha de comando.
///
/// # Errors
///
/// Retorna uma mensagem estável quando a forma não é exatamente `--ticks N`
/// ou quando `N` não é um inteiro positivo.
fn analisar_argumentos(args: &[String]) -> Result<usize, String> {
    if args.len() != 2 || args[0] != "--ticks" {
        return Err("uso: insurgencia-servidor --ticks N (N > 0)".to_string());
    }
    let ticks: usize = args[1]
        .parse()
        .map_err(|_| "argumento --ticks inválido: N deve ser positivo".to_string())?;
    if ticks == 0 {
        return Err("argumento --ticks inválido: N deve ser maior que zero".to_string());
    }
    Ok(ticks)
}

/// Analisa uma coordenada `f64` finita.
///
/// # Errors
///
/// Retorna uma mensagem com o número da linha quando o texto não representa
/// um número finito.
fn analisar_coordenada(texto: &str, campo: &str, numero: usize) -> Result<f64, String> {
    let valor: f64 = texto
        .parse()
        .map_err(|_| format!("linha {numero}: {campo} inválido"))?;
    if valor.is_finite() {
        Ok(valor)
    } else {
        Err(format!("linha {numero}: {campo} inválido"))
    }
}

/// Analisa uma linha da `stdin` em um comando, ignorando linhas em branco.
///
/// # Errors
///
/// Retorna uma mensagem estável com o número da linha, 1-based, quando a
/// aridade, os números ou o nome da ação são inválidos.
fn analisar_linha(linha: &str, numero: usize) -> Result<Option<LinhaComando>, String> {
    let partes: Vec<&str> = linha.split_whitespace().collect();
    if partes.is_empty() {
        return Ok(None);
    }
    let erro = |mensagem: &str| format!("linha {numero}: {mensagem}");
    if partes.len() != 4 && partes.len() != 6 {
        return Err(erro("esperado 'peer_id acao seq request_id [x y]'"));
    }
    let peer_id: u64 = partes[0].parse().map_err(|_| erro("peer_id inválido"))?;
    let acao = partes[1];
    if !ACOES_PERMITIDAS.contains(&acao) {
        return Err(erro(&format!("ação desconhecida: {acao}")));
    }
    let seq: i64 = partes[2].parse().map_err(|_| erro("seq inválido"))?;
    let request_id: i64 = partes[3].parse().map_err(|_| erro("request_id inválido"))?;
    let exige_destino = matches!(acao, "mover" | "coletar_materiais" | "construir_barreira");
    let destino = match (exige_destino, partes.len()) {
        (true, 6) => {
            let x = analisar_coordenada(partes[4], "x", numero)?;
            let y = analisar_coordenada(partes[5], "y", numero)?;
            Some((x, y))
        }
        (true, _) => return Err(erro("ação exige coordenadas x e y")),
        (false, 4) => None,
        (false, _) => return Err(erro("ação não aceita coordenadas")),
    };
    Ok(Some(LinhaComando {
        peer_id,
        acao: acao.to_string(),
        seq,
        request_id,
        destino,
    }))
}

/// Lê toda a `stdin` e analisa os comandos na ordem das linhas.
///
/// # Errors
///
/// Retorna uma mensagem estável se a leitura falhar ou se alguma linha for
/// inválida; nenhum comando é entregue ao `Mundo` nesse caso.
fn ler_comandos() -> Result<Vec<LinhaComando>, String> {
    let entrada = io::stdin();
    let mut comandos = Vec::new();
    for (indice, linha) in entrada.lock().lines().enumerate() {
        let linha = linha.map_err(|erro| format!("falha ao ler stdin: {erro}"))?;
        if let Some(comando) = analisar_linha(&linha, indice + 1)? {
            comandos.push(comando);
        }
    }
    Ok(comandos)
}

/// Executa o ciclo headless completo, da argumentação ao hash final.
///
/// # Errors
///
/// Propaga falhas de argumentos, de leitura da `stdin` e de escrita na `stdout`.
fn executar() -> Result<(), String> {
    let args = env::args_os()
        .skip(1)
        .map(|arg| {
            arg.into_string()
                .map_err(|_| "argumento inválido: texto não UTF-8".to_string())
        })
        .collect::<Result<Vec<String>, String>>()?;
    let ticks = analisar_argumentos(&args)?;
    let comandos = ler_comandos()?;
    let mut mundo = Mundo::new();

    let peers: BTreeSet<u64> = comandos.iter().map(|comando| comando.peer_id).collect();
    for peer_id in peers {
        mundo.adicionar_jogador(peer_id);
    }
    for comando in comandos {
        mundo.agendar_comando(Comando {
            peer_id: comando.peer_id,
            acao: comando.acao,
            seq: comando.seq,
            request_id: comando.request_id,
            destino: comando.destino,
        });
    }

    let stdout = io::stdout();
    let mut saida = BufWriter::new(stdout.lock());
    for tick in 0..ticks {
        writeln!(saida, "{tick}\t{}", mundo.hash_estado())
            .map_err(|erro| format!("falha ao escrever stdout: {erro}"))?;
        mundo.avancar_tick();
    }
    saida
        .flush()
        .map_err(|erro| format!("falha ao escrever stdout: {erro}"))?;
    Ok(())
}

fn main() -> ExitCode {
    match executar() {
        Ok(()) => ExitCode::SUCCESS,
        Err(mensagem) => {
            eprintln!("{mensagem}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod testes {
    use super::{analisar_argumentos, analisar_linha, LinhaComando};

    #[test]
    fn argumentos_validos() {
        let args = vec!["--ticks".to_string(), "3".to_string()];
        assert_eq!(analisar_argumentos(&args), Ok(3));
    }

    #[test]
    fn argumentos_invalidos() {
        let ausente: Vec<String> = Vec::new();
        assert!(analisar_argumentos(&ausente).is_err());
        let zero = vec!["--ticks".to_string(), "0".to_string()];
        assert!(analisar_argumentos(&zero).is_err());
        let negativo = vec!["--ticks".to_string(), "-1".to_string()];
        assert!(analisar_argumentos(&negativo).is_err());
        let nao_inteiro = vec!["--ticks".to_string(), "abc".to_string()];
        assert!(analisar_argumentos(&nao_inteiro).is_err());
        let extra = vec!["--ticks".to_string(), "1".to_string(), "extra".to_string()];
        assert!(analisar_argumentos(&extra).is_err());
        let desconhecido = vec!["--ticks=1".to_string()];
        assert!(analisar_argumentos(&desconhecido).is_err());
    }

    #[test]
    fn linha_valida_com_coordenadas() {
        let esperado = Ok(Some(LinhaComando {
            peer_id: 7,
            acao: "mover".to_string(),
            seq: 1,
            request_id: 2,
            destino: Some((3.5, -4.0)),
        }));
        assert_eq!(analisar_linha("7 mover 1 2 3.5 -4", 1), esperado);
    }

    #[test]
    fn linha_valida_sem_coordenadas() {
        let esperado = Ok(Some(LinhaComando {
            peer_id: 8,
            acao: "atacar_forte".to_string(),
            seq: 4,
            request_id: 5,
            destino: None,
        }));
        assert_eq!(analisar_linha("8 atacar_forte 4 5", 2), esperado);
    }

    #[test]
    fn linha_em_branco_ignorada() {
        assert_eq!(analisar_linha(" \t  ", 1), Ok(None));
    }

    #[test]
    fn linhas_invalidas() {
        assert!(analisar_linha("7 voar 1 2", 1).is_err());
        assert!(analisar_linha("x mover 1 2 3 4", 1).is_err());
        assert!(analisar_linha("7 mover a 2 3 4", 1).is_err());
        assert!(analisar_linha("7 mover 1 2", 1).is_err());
        assert!(analisar_linha("7 coletar_materiais 1 2 3", 1).is_err());
        assert!(analisar_linha("7 sinalizar_sincronia 1 2 3 4", 1).is_err());
        assert!(analisar_linha("7 mover 1 2 3 4 5", 1).is_err());
        assert!(analisar_linha("7 mover 1 2 NaN 4", 1).is_err());
        assert!(analisar_linha("7 mover 1 2 inf 4", 1).is_err());
    }
}
