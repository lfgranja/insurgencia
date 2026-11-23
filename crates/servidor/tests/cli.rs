#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Testes de integração do binário `insurgencia-servidor`.
//!
//! Cada caso executa o binário real com stdin controlada, sem arquivos
//! temporários, estado compartilhado ou dependência de ordem entre testes.

use std::io::Write;
use std::process::{Command, Output, Stdio};

/// Quantidade de ticks usada nos testes de contagem.
const TICKS: usize = 4;

/// Entrada válida: linhas em branco, três peers e ações com e sem coordenadas.
const ENTRADA_VALIDA: &str = concat!(
    "\n7 mover 1 1 0.25 0.0\n",
    "8 sinalizar_sincronia 1 1\n",
    "9 coletar_materiais 1 1 0.5 0.5\n",
    "\n",
);

/// Executa o binário com `args` e `entrada` controlados.
fn executar(args: &[&str], entrada: &str) -> Result<Output, String> {
    let mut filho = Command::new(env!("CARGO_BIN_EXE_insurgencia-servidor"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|erro| format!("falha ao iniciar o binário: {erro}"))?;

    let mut stdin = filho
        .stdin
        .take()
        .ok_or_else(|| String::from("stdin do processo filho indisponível"))?;
    stdin
        .write_all(entrada.as_bytes())
        .map_err(|erro| format!("falha ao escrever na stdin: {erro}"))?;
    drop(stdin);

    filho
        .wait_with_output()
        .map_err(|erro| format!("falha ao aguardar o binário: {erro}"))
}

/// Executa o binário e falha o teste somente se o processo não puder ser iniciado.
fn executar_ou_falhar(args: &[&str], entrada: &str) -> Output {
    match executar(args, entrada) {
        Ok(saida) => saida,
        Err(erro) => panic!("{erro}"),
    }
}

/// Decodifica bytes de saída preservando qualquer conteúdo inesperado.
fn texto(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn emite_uma_linha_por_tick_no_formato_canonical() {
    let ticks = TICKS.to_string();
    let saida = executar_ou_falhar(&["--ticks", ticks.as_str()], ENTRADA_VALIDA);
    assert!(
        saida.status.success(),
        "execução falhou: {}",
        texto(&saida.stderr)
    );

    let stdout = texto(&saida.stdout);
    let linhas: Vec<&str> = stdout.lines().collect();
    assert_eq!(linhas.len(), TICKS, "esperadas {TICKS} linhas: {stdout}");

    for (indice, linha) in linhas.iter().enumerate() {
        let campos: Vec<&str> = linha.split('\t').collect();
        assert_eq!(campos.len(), 2, "linha {indice} sem dois campos: {linha}");
        assert_eq!(
            campos[0],
            indice.to_string().as_str(),
            "tick inesperado: {linha}"
        );
        let hash = campos[1];
        assert_eq!(hash.len(), 64, "hash com tamanho inesperado: {hash}");
        assert!(
            hash.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "hash não hexadecimal: {hash}"
        );
    }
    assert!(
        saida.stderr.is_empty(),
        "stderr inesperada: {}",
        texto(&saida.stderr)
    );
}

#[test]
fn duas_execucoes_com_mesma_entrada_sao_identicas() {
    let primeira = executar_ou_falhar(&["--ticks", "3"], ENTRADA_VALIDA);
    let segunda = executar_ou_falhar(&["--ticks", "3"], ENTRADA_VALIDA);

    assert!(
        primeira.status.success() && segunda.status.success(),
        "pelo menos uma execução falhou"
    );
    assert_eq!(
        primeira.stdout, segunda.stdout,
        "stdout divergente entre execuções idênticas"
    );
    assert_eq!(
        primeira.stderr, segunda.stderr,
        "stderr divergente entre execuções idênticas"
    );
}

#[test]
fn entrada_malformada_retorna_exit_2_sem_saida() {
    const CASOS: &[&str] = &[
        "7 voar 1 1\n",
        "7 mover 1 1\n",
        "7 sinalizar_sincronia 1 1 0.5 0.0\n",
        "x mover 1 1 0.5 0.0\n",
        "7 mover 1 1 NaN 0.0\n",
        "\n\n7 mover 1\n",
    ];

    for entrada in CASOS {
        let saida = executar_ou_falhar(&["--ticks", "2"], entrada);
        assert_eq!(
            saida.status.code(),
            Some(2),
            "caso {entrada:?}: código de saída inesperado"
        );
        assert!(
            saida.stdout.is_empty(),
            "caso {entrada:?}: stdout deveria estar vazia"
        );
        assert!(
            !saida.stderr.is_empty(),
            "caso {entrada:?}: erro determinístico ausente na stderr"
        );
    }
}

#[test]
fn argumentos_invalidos_retornam_exit_2_sem_saida() {
    const CASOS: &[&[&str]] = &[
        &[],
        &["--ticks", "0"],
        &["--ticks", "-1"],
        &["--ticks", "abc"],
        &["--ticks", "2", "--desconhecido"],
    ];

    for args in CASOS {
        let saida = executar_ou_falhar(args, "");
        assert_eq!(
            saida.status.code(),
            Some(2),
            "args {args:?}: código de saída inesperado"
        );
        assert!(
            saida.stdout.is_empty(),
            "args {args:?}: stdout deveria estar vazia"
        );
        assert!(
            !saida.stderr.is_empty(),
            "args {args:?}: erro determinístico ausente na stderr"
        );
    }
}
