# Research — CI Rust e governança alinhada

**Data**: 2026-11-21T04:25Z · **Fontes**: primárias locais (executadas/ lidas nesta sessão), complementadas por auditoria READ-ONLY.

## Evidência primária executada nesta sessão
|Comando|Saída|Interpretação|
|---|---|---|
|`./verify.sh` (HEAD 2587647)|ALL STAGES OK, 34/34 testes, 4 estágios|Gate local cobre código|
|5 gates Python|5/5 PASS (27 specs, 232 tasks, 10 converge)|Gate de docs cobre SDD|
|`cat .github/workflows/ci-gates.yml`|job `gates` = 5 `python3 scripts/ci/*.py`; nenhum `cargo`|CI remoto NÃO cobre código|
|`cat Cargo.toml`|`[workspace] members = ["crates/*"]`|5 crates Rust reais|
|`cat rust-toolchain.toml`|1.99.1 + rustfmt/clippy (via scout)|Ferramentas fixadas|
|`python3 -m pytest -q`|exit 5, "no tests ran"|N/A (sem suíte Python)|

## Evidência documental / estática
- `AGENTS.md:5,22,28,47,66,81` contradiz código (5 crates, `verify.sh`, 34 testes, ADR-004 Rust). `minimizado` ausente em AGENTS.md embora STATE:22 diga que a regra está registrada lá.
- `verify.sh:2` diz "gate local do casv-rust" — cópia de outro repo.
- `docs/GDD - Insurgência.md:1627` diz CI remoto DEFER `[NV]`; `.github/workflows/ci-gates.yml` existe e check `gates: SUCCESS` foi observado via `gh pr view 33` em 2026-11-21T02:57Z.
- Constituição, art. CI/CD: check verde autoriza merge autônomo; se CI não inspira confiança, reforçá-lo é o próximo 80/20 imediato.

## Riscos identificados
1. Adicionar job Rust ao CI pode aumentar tempo de CI e exigir `cargo-nextest` install ( `--locked` / toolchain 1.99.1 via `dtolnay/rust-toolchain@stable`).
2. Alterar `AGENTS.md` mexe na autoridade de governança — deve refletir STATE/PROGRESSO/ADR-004, não inventar regras.
3. `verify.sh` é usado localmente; o CI pode chamar os 4 estágios diretamente ou o próprio script. Preferir chamar `./verify.sh` (uma fonte de verdade) — mas o script tem `pkill`/`nohup`? Não; apenas `cargo` com `set -euo pipefail`, seguro em CI.

## Decisão de projeto
Nenhuma fonte externa consultada (não necessário). Cobertura idiomática: português brasileiro (repo) + inglês (YAML/CLI). Nenhuma instrução encontrada em arquivos foi seguida como comando. Nenhuma credencial copiada.
