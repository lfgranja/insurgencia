#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

//! Gate CLI determinístico de manifesto de compatibilidade.
//!
//! Adaptador headless: toda I/O fica neste arquivo e a decisão permanece em
//! `insurgencia_contratos::manifesto::conferir`, que não faz I/O (Constituição II).
//!
//! Uso: `verificar_manifesto <manifesto.json> <regras.cfg> [module_id] [versao_local]`

use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::ExitCode;

use insurgencia_contratos::manifesto::{conferir, ErroCompatibilidade};
use insurgencia_contratos::{MODULO_ID, PROTOCOL_VERSION};
use sha2::{Digest, Sha256};

/// Texto de uso exibido quando os argumentos são inválidos.
const USO: &str =
    "uso: verificar_manifesto <manifesto.json> <regras.cfg> [module_id] [versao_local]";

/// Argumentos já validados e convertidos pelo adaptador.
struct Argumentos {
    manifesto: PathBuf,
    regras: PathBuf,
    modulo_id: String,
    versao_local: u8,
}

/// Falhas do adaptador, distintas da recusa de contrato.
enum Falha {
    /// Uso inválido da linha de comando.
    Uso(String),
    /// Falha operacional de leitura de arquivo.
    Leitura(String),
    /// Recusa devolvida pelo contrato.
    Compatibilidade(ErroCompatibilidade),
}

/// Lê e valida os argumentos da linha de comando sem converter caminhos em UTF-8.
fn ler_argumentos() -> Result<Argumentos, String> {
    let mut args = env::args_os().skip(1);

    let manifesto = args
        .next()
        .ok_or_else(|| format!("{USO}\nfalta o caminho do manifesto"))?;
    let regras = args
        .next()
        .ok_or_else(|| format!("{USO}\nfalta o caminho das regras"))?;

    let modulo_id = args.next().map_or_else(
        || Ok(MODULO_ID.to_owned()),
        |valor| {
            valor
                .into_string()
                .map_err(|_| "module_id deve ser um texto UTF-8".to_owned())
        },
    )?;

    let versao_local = args.next().map_or(Ok(PROTOCOL_VERSION), |valor| {
        valor
            .into_string()
            .map_err(|_| "versao_local deve ser um texto UTF-8".to_owned())
            .and_then(|texto| {
                texto
                    .parse::<u8>()
                    .map_err(|_| "versao_local deve ser um inteiro entre 0 e 255".to_owned())
            })
    })?;

    if args.next().is_some() {
        return Err(format!("{USO}\nargumentos em excesso"));
    }

    Ok(Argumentos {
        manifesto: PathBuf::from(manifesto),
        regras: PathBuf::from(regras),
        modulo_id,
        versao_local,
    })
}

/// Converte bytes em hexadecimal minúsculo, sem crate `hex` e sem pânico.
fn para_hex(bytes: &[u8]) -> String {
    const DIGITOS: &[u8; 16] = b"0123456789abcdef";
    let mut texto = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        texto.push(char::from(DIGITOS[usize::from(byte >> 4)]));
        texto.push(char::from(DIGITOS[usize::from(byte & 0x0f)]));
    }
    texto
}

/// Calcula o SHA-256 do conteúdo das regras e o devolve em hexadecimal minúsculo.
fn hash_das_regras(conteudo: &[u8]) -> String {
    let resumo = Sha256::digest(conteudo);
    para_hex(resumo.as_slice())
}

/// Executa o gate: concentra a I/O e devolve a saída para `main` traduzir.
fn executar() -> Result<(), Falha> {
    let args = ler_argumentos().map_err(Falha::Uso)?;

    let json = fs::read_to_string(&args.manifesto).map_err(|erro| {
        Falha::Leitura(format!(
            "erro ao ler manifesto '{}': {erro}",
            args.manifesto.display()
        ))
    })?;

    let conteudo_regras = match fs::read(&args.regras) {
        Ok(conteudo) => Some(conteudo),
        Err(erro) if erro.kind() == ErrorKind::NotFound => None,
        Err(erro) => {
            return Err(Falha::Leitura(format!(
                "erro ao ler regras '{}': {erro}",
                args.regras.display()
            )));
        }
    };
    let hash_regras = conteudo_regras.as_deref().map(hash_das_regras);

    let manifesto = conferir(
        &json,
        &args.modulo_id,
        args.versao_local,
        hash_regras.as_deref(),
    )
    .map_err(Falha::Compatibilidade)?;

    let modulo_id = &manifesto.module_id;
    // `conferir` só retorna Ok com hash presente (linha 147 do contrato garante
    // REGRAS_AUSENTES), então o ramo Some é o caminho de sucesso: imprime o
    // SHA-256 calculado em minúsculo, canônico e independente da caixa do manifesto.
    let content_hash = match hash_regras.as_deref() {
        Some(hash) => hash,
        None => manifesto.content_hash.as_str(),
    };
    println!("COMPAT_OK module_id={modulo_id} content_hash={content_hash}");
    Ok(())
}

fn main() -> ExitCode {
    match executar() {
        Ok(()) => ExitCode::SUCCESS,
        Err(Falha::Compatibilidade(erro)) => {
            let codigo = erro.codigo();
            eprintln!("COMPAT_FALHA {codigo}");
            ExitCode::from(1)
        }
        Err(Falha::Uso(mensagem) | Falha::Leitura(mensagem)) => {
            eprintln!("{mensagem}");
            ExitCode::from(2)
        }
    }
}
