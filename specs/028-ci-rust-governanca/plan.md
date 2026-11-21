# Implementation Plan — 028-ci-rust-governanca

1. Editar `.github/workflows/ci-gates.yml`: adicionar job `rust` com checkout, toolchain via `dtolnay/rust-toolchain@master` (lê `rust-toolchain.toml`), `taiki-e/install-action@nextest`, `./verify.sh`.
2. Reescrever `AGENTS.md` preservando governança (Bloco 0.a, SDD, append-only, 80/20, untrusted) e refletindo Rust/Bevy/5 crates/34 testes/verify.sh/regra da janela.
3. Corrigir `verify.sh:2` — "Insurgência" em vez de "casv-rust".
4. Atualizar `docs/GDD - Insurgência.md:1627` com nota datada de CI materializado, preservando histórico.
5. Rodar local: `./verify.sh`, 5 gates SDD, `pytest -q` (N/A), validar YAML com `python3 -c yaml.safe_load` ou actionlint se disponível.
6. converge (READ-ONLY), validate (R4), release (R7).
7. Commit atômico em pt-BR; PR para `dev` via `gh`; aguardar check verde observado; mesclar conforme constituição (autorização do prompt).

## Comandos
```bash
./verify.sh                       # gate autoritativo (local)
python3 scripts/ci/*.py           # 5 gates SDD
python3 -c 'import yaml,sys; yaml.safe_load(open(".github/workflows/ci-gates.yml"))'
```
