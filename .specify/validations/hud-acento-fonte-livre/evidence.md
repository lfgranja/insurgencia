# Evidência — Roteiro de Testes 036 (execução autônoma)

**Data/hora**: 2026-11-22T21:23–21:28Z · **Sessão**: `insurgencia - HUD com acento e fonte livre` (`session_id=20261122T211548Z-hud-acento-fonte`) · **Worktree**: `.specify/queue/worktrees/hud-acento-036/`, branch `036-hud-acento-fonte-livre`, base `7c5c0f3`. **Superfície**: CLI + captura visual em display virtual isolado (`xvfb-run` + lavapipe/Vulkan por software) — nenhuma janela foi aberta no desktop da SE.

| Passo | Ação | Saída observada | Resultado | Referência |
|:--|:--|:--|:--|:--|
| 1 | `sha256sum`/`stat` da fonte | 621572 bytes; `478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823` | PASS | saída colada no converge |
| 2 | Licença e proveniência | `assets/fonts/OFL.txt` com corpo integral da SIL OFL 1.1; `docs/governance/font-provenance.md` com hash, URL e cobertura | PASS | leitura dos arquivos |
| 3 | `fc-match` dos glifos | U+00CA (`Ê`), U+2014 (`—`) e U+00B7 (`·`) resolvem para NotoSans-Regular.ttf | PASS | saída de 21:16Z |
| 4 | Código do cliente | `configurar` recebe `Res<AssetServer>`; `fontes.load("fonts/NotoSans-Regular.ttf")` aplicado ao `TextFont`; strings restauradas | PASS | `main.rs:458,555` |
| 5 | `./verify.sh` (21:23:11Z) | `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0` | PASS | saída do gate |
| 6 | Captura visual isolada | `INSURGÊNCIA — low-poly`, `Limites: X +/-580 · Y +/-310`, sem `▯`, sem `Ê` ausente | PASS | `docs/evidencias/hud-acento-confirmado-2026-11-22.png` (SHA-256 `8d353895…70ce`) |
| 7 | Gates Python | SDD `36/0`; DAG `350 tarefas/51 arestas/0 ciclos`; task-id `0 duplicatas`; hub-docs `0 erros`; converge `19/19` após bootstrap | PASS | saídas coladas no converge |
| 8 | Rastreabilidade SDD | 14 artefatos; `analyze.md` antes de `tasks.md`; R4 aprovado antes de R7 `1.0.0`; roteiro escrito antes do PR | PASS | leitura dos artefatos |
| 9 | Isolamento da SE | nenhuma janela no desktop; `xvfb-run` com `BEVY_ASSET_ROOT` apontando para o worktree; 0 processos do jogo ao fim | PASS | comandos executados |
| 10 | Fronteira | nenhum `Cargo.toml`/lockfile/gate/workflow alterado;fonte e licença versionadas como assets | PASS | `git status --short` no worktree |
| 11 | Captura sem `BEVY_ASSET_ROOT` (blocker P1 da ACR) | `env -u BEVY_ASSET_ROOT` em Xvfb/lavapipe: HUD `INSURGÊNCIA — low-poly` / `·` sem `▯`; log sem `TextError::NoSuchFont`; cena 3D renderizada | PASS | `docs/evidencias/hud-acento-confirmado-2026-11-22.png` (SHA-256 `e1769d91…8fa1`) |

**Resultado**: 11/11 PASS. `evidence_status: passed`. Rollback do roteiro: remover a captura temporária em `/tmp`; a fonte e a licença são artefatos do ciclo e não do roteiro.

**Observação (final 2026-11-22T21:37Z)**: a primeira captura falhou com `Path not found: .../target/debug/assets/fonts/...` porque, sem `BEVY_ASSET_ROOT`, o Bevy 0.20 usa o diretório do executável como base. Isso era **defeito de configuração de caminho no código entregue**, não comportamento inevitável da engine; corrigido em `crates/cliente/src/main.rs` pela função `caminho_assets()` com `AssetPlugin { file_path: caminho_assets(), ..default() }`. A captura do passo 6 usou `BEVY_ASSET_ROOT` e permanece como registro histórico; a captura final do passo 11, **sem env externo**, confirma a correção e a cena 3D completa.
