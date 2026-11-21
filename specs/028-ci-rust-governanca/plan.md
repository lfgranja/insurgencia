# Implementation Plan — 028-ci-rust-governanca

1. Editar `.github/workflows/ci-gates.yml`: adicionar job `rust` com checkout, `dtolnay/rust-toolchain@master` com `toolchain: '1.99.1'` (input obrigatório — action NÃO lê `rust-toolchain.toml` automaticamente; descoberto por CI RED em 2026-11-21T04:29Z), componentes `clippy, rustfmt`, `./verify.sh`.
2. Reescrever `AGENTS.md` preservando governança e refletindo Rust/Bevy/34 testes/verify.sh. Regra da janela reforçada pela SE às 04:33Z: agente SEMPRE `--minimizado`; nunca janela normal durante trabalho de agente.
3. Corrigir `verify.sh:2` — "Insurgência" em vez de "casv-rust".
4. Atualizar `docs/GDD - Insurgência.md:1627` com nota datada.
5. Rodar local: `./verify.sh`, 5 gates SDD, `pytest -q` (N/A), validar YAML com `yaml.safe_load`.
6. converge (READ-ONLY, append-only), validate (R4), release (R7).
7. Commit atômico em pt-BR; PR para `dev` via `gh`; **CI RED no job `rust` por input de toolchain ausente** → corrigir, reenviar, observar verde ao vivo; mesclar conforme constituição.

## Comandos
```bash
./verify.sh
python3 scripts/ci/*.py
python3 -c 'import yaml; print(list(yaml.safe_load(open(".github/workflows/ci-gates.yml"))["jobs"]))'
```
