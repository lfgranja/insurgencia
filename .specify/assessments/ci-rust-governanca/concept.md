# Concept — CI Rust e governança alinhada

**Data**: 2026-11-21T04:26Z

## Opções consideradas

### Opção 1 — Adicionar job `rust` no `ci-gates.yml` chamando `./verify.sh` (recomendada)
- Novo job `rust` em `.github/workflows/ci-gates.yml`: checkout → `dtolnay/rust-toolchain@stable` com `toolchain: 1.99.1` + componentes `clippy`, `rustfmt` → instala `cargo-nextest` → `./verify.sh`.
- `verify.sh` é a única fonte de verdade local; chamar o script evita divergência entre CI e local.
- Mantém job `gates` (docs) existente.
- Prós: coerência, baixa duplicação, usa gate já validado. Contras: tempo de CI; `cargo-nextest` installation precisa ser pinada/fiável.

### Opção 2 — Chamar `cargo check/clippy/test/fmt` diretamente no YAML
- Duplica os 4 estágios do `verify.sh` em YAML.
- Contras: duplicação de fonte de verdade; risco de divergência futura. Rejeitada.

### Opção 3 — Apenas corrigir documentação, sem CI
- Não resolve o risco de merge com código quebrado. Rejeitada.

## Escopo recomendado (Opção 1)
1. `.github/workflows/ci-gates.yml`: job `rust` com toolchain fixada e `./verify.sh`.
2. `AGENTS.md`: reescrever seções para refletir Rust/Bevy/verify.sh/34 testes/regra da janela (preservando governança SDD, gate autoritativo, append-only, 80/20, untrusted content).
3. `verify.sh:2`: trocar "casv-rust" por "Insurgência".
4. `docs/GDD - Insurgência.md:1627`: marcar CI remoto como materializado, com check `gates SUCCESS` observado em 2026-11-21T02:57Z; histórico preservado.

## Riscos e mitigação
- `cargo-nextest` ausente no CI → `verify.sh` tem fallback `cargo test`; mas para gate completo, instalar via `taiki-e/install-action@nextest` (ação community) ou `cargo install cargo-nextest --locked`. Preferir `taiki-e/install-action@nextest` por velocidade; se falhar, fallback documentado.
- Toolchain: usar `dtolnay/rust-toolchain@master` com `toolchain: 1.99.1` (existe `rust-toolchain.toml` com essa versão; `dtolnay/rust-toolchain@stable` lê o arquivo se `toolchain` omitido). **Decisão**: omitir `toolchain` e deixar a ação ler `rust-toolchain.toml` — uma fonte de verdade.
- `AGENTS.md` é autoridade: alteração revisada contra constituição/STATE/PROGRESSO/ADR-004.
