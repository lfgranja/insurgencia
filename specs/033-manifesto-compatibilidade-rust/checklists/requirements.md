# Checklist de Requisitos — 033-manifesto-compatibilidade-rust

- [x] FR-001 âncora registrada: 6 campos/valores de `manifesto.json` + `specs/011` FR-001..FR-006 + GDScript (GDD §1/§14.2); nenhum esquema novo
- [x] FR-002 porte em `manifesto.rs`: struct com 6 campos (`protocol_version: u8`), `ErroCompatibilidade` com 5 variantes + `codigo()`, `REGRAS_AUSENTES`, `pub mod manifesto;` adicionado apenas
- [x] FR-003 `validar_formato` exige 6 campos/tipos → `MANIFEST_INVALID`; não checa tamanho do hash
- [x] FR-004 `conferir` na ordem 011 (formato → versão → módulo → 64 hex → regras → hash), `Option<&str>`, `eq_ignore_ascii_case`, sem I/O/pânico
- [x] FR-005 higiene Rust: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`, `thiserror`, `BTreeMap`, sem dependências novas nem alteração em `Cargo.toml`
- [x] FR-006 7 testes espelhando `tests/test_manifesto.gd` + SDD ordenado R1 → R4 → R7, `converge.md` append-only, `analyze.md` READ-ONLY, fronteiras disjuntas, rollback documentado
- [x] Evidências: 5 gates PASS em `21:24:08Z`; `./verify.sh` **[NV]** por contenção de `cargo`; factory `b1deab1` somente fonte

## Revisão
Requisitos verificáveis por inspeção das âncoras literais e pelas saídas reportadas. Checklist `[x]` = revisão de requisitos, não implementação; nenhum build/gate/commit neste ciclo. Fronteira A restrita a 13 artefatos e Fronteira B a 3 arquivos de código, sem sobrescrita; nenhum código da Fronteira B, factory, `docs/GDD*`, baseline, gate, CI, janela, credencial ou 194 path untracked alheio tocado por esta revisão. Aprovado para tasks.
