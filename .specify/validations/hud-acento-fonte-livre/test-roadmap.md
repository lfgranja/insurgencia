# Roteiro de Testes — HUD com acento e fonte livre (ciclo 036)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador · **Evidências aqui**: **[NV]** (nada executado neste ciclo pelo agente SDD)

## Escopo e rastreabilidade

| Item | Cobertura |
|---|---|
| FR-001 | passo 3 — `Text::new("INSURGÊNCIA — low-poly")`; passos 5–6 — captura do título |
| FR-002 | passo 3 — `format!` com `·`; passos 5–6 — captura dos limites |
| FR-003 | passos 1 e 3 — TTF embarcado, `AssetServer`/`Handle<Font>`/`font`, sem deps novas |
| FR-004 | passo 2 — licença OFL 1.1 integral, hash e proveniência |
| FR-005 | passos 1, 5 e 6 — cobertura dos três glifos e ausência de `▯` |
| FR-006 | passos 4–7 — `--minimizado`, Xvfb isolado, sem janela no desktop da SE |
| FR-007 | passo 9 — 14 artefatos, `analyze` antes de `tasks`, R4 antes de R7 |
| FR-008 | passos 1–10 e plano de capturas abaixo |
| FR-009 | passos 9–10 — fronteira, não-execução e rollback |
| SC-001..SC-006 | passos 1–10 |

T-IDs relacionados: T001–T014 (SDD), T015 (converge), T016 (R4), T017 (R7) e T018 (implementação/execução, **[NV]**).

## Pré-condições

1. Worktree `.specify/queue/worktrees/hud-acento-036/`, branch `036-hud-acento-fonte-livre`, base `7c5c0f3`. PR #40 já mesclado às 20:52:35Z (`c126b6e`) — nada a promover neste roteiro.
2. Toolchain fixada em `rust-toolchain.toml`; Bevy `=0.20.1` já resolvido em `Cargo.lock`. Nenhum download de dependência nova.
3. Implementação do agente de código presente: `main.rs` com as duas strings unicode e `font` no `TextFont`; `assets/fonts/NotoSans-Regular.ttf`, `assets/fonts/OFL.txt`, `docs/governance/font-provenance.md`.
4. Ambiente headless: `xvfb-run -a` + Mesa lavapipe (`LIBGL_ALWAYS_SOFTWARE=1`, `GALLIUM_DRIVER=llvmpipe` quando necessário). Nenhuma janela normal aberta no desktop da SE; cliente sempre com `--minimizado`.
5. `TMP=$(mktemp -d)` para capturas e arquivos de apoio, fora do worktree; `mkdir -p "$TMP/capturas"`.
6. Construir uma vez: `cargo build -p insurgencia-cliente --locked` (use `CARGO_TARGET_DIR` próprio se houver contenção). Registrar `$?` após cada comando relevante.
7. Nenhum gate, build, capture, commit, push ou PR executado por este agente; os passos abaixo são instruções para o orquestrador.

## Superfície isolada (visual/CLI)
- Binário: `cargo run -p insurgencia-cliente --locked -- --minimizado`, sob `xvfb-run -a`.
- Inspetores CLI: `stat`, `sha256sum`, `fc-match`, `grep`, `git diff --name-only`.
- Capturas: `import -window root "$TMP/capturas/hud-036-<caso>.png"` (ou `scrot -o`), dentro do Xvfb; nunca no display da SE.

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `stat -c '%s' assets/fonts/NotoSans-Regular.ttf`; `sha256sum assets/fonts/NotoSans-Regular.ttf`; comparar com `docs/governance/font-provenance.md` e `fc-match -f '%{family} %{file}\n' 'charset:00ca'`, idem `charset:2014` e `charset:00b7` | 621572 bytes; SHA-256 `478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823`; as três consultas resolvem para `Noto Sans` em `/usr/share/fonts/google-noto/NotoSans-Regular.ttf` (origem registrada) | sim |
| 2 | `grep -n 'SIL OPEN FONT LICENSE Version 1.1' assets/fonts/OFL.txt`; `sha256sum assets/fonts/OFL.txt`; conferir URL primária em `font-provenance.md`; comparar o corpo da licença com `/usr/share/doc/anicraft/OFL-noto-sans-arabic.txt` | texto integral da SIL OFL 1.1 presente; hash `4b0e3cf405082dc00cc396e53af77cd6ba6cfe02537e19786e5d113c9158d107`; URL https://github.com/notofonts/noto-fonts/blob/main/LICENSE registrada; corpo da licença idêntico ao canônico (cabeçalho de copyright difere, conforme esperado) | sim (comparação de corpo com revisão humana do cabeçalho) |
| 3 | `grep -nE 'INSURGÊNCIA — low-poly|· Y \+/-|\.load\("fonts/NotoSans-Regular.ttf"\)|font: fonte\.into\(\)' crates/cliente/src/main.rs`; `git diff --name-only -- Cargo.toml Cargo.lock` | duas strings unicode presentes (`Ê`, `—`, `·`), carregamento e aplicação da fonte presentes, nenhum `▯` no arquivo; diff vazio para `Cargo.toml`/`Cargo.lock` | sim |
| 4 | Iniciar o cliente sob Xvfb: `LIBGL_ALWAYS_SOFTWARE=1 xvfb-run -a cargo run -p insurgencia-cliente --locked -- --minimizado`; aguardar estabilizar; `echo $?` | processo ativo sem erro de asset/font; nenhuma janela aparece no desktop da SE; exit 0 ao encerrar | parcial (execução automatizável; ausência de janela no desktop é observação da SE) |
| 5 | Capturar a tela do Xvfb: `import -window root "$TMP/capturas/hud-036-titulo.png"` e `...-limites.png`; registrar `date -u` em `$TMP/capturas/carimbo.txt` | dois PNGs legíveis no diretório temporário, com carimbo de data/hora; nada escrito na árvore principal da SE | sim |
| 6 | **Verificar os três glifos na captura e ausência de `▯`**: inspecionar os PNGs do passo 5 (visualmente e/ou por OCR) conferindo `INSURGÊNCIA — low-poly` (U+00CA e U+2014) e `Limites: X +/-580 · Y +/-310` (U+00B7); buscar qualquer `▯` | `Ê`, `—` e `·` visíveis nas posições esperadas; nenhum caractere de substituição/tofu em nenhuma linha; título e limites legíveis | parcial (captura/OCR automatizáveis; leitura final pela SE) |
| 7 | Repetir os passos 4–6 uma segunda vez, salvando `-2.png`; comparar as duas capturas quanto à presença dos glifos e ausência de `▯` | duas execuções estáveis: grafia correta nas duas, sem tofu; nenhuma diferença de caractere relevante | parcial |
| 8 | Rodar `./verify.sh` e os 5 gates Python (`check_sdd_completeness.py`, `check_work_order_dag.py`, `check_task_ids.py`, `check_converge_append_only.py`, `hub_docs_gate.py`) | `ALL STAGES OK`, 41/41 testes; SDD `36/0`, DAG sem ciclos/órfãos, task-id `0 duplicatas`, converge append-only (bootstrap da entrada 036 pelo orquestrador), hub-docs `0 erros` | sim |
| 9 | Inspecionar os 14 artefatos e `git diff --name-only` do trabalho deste agente; conferir que `analyze.md` precede `tasks.md`, `validate.md` (R4) precede `release.md` (R7 `1.0.0`), `converge.md` tem só a 1ª entrada | 14 artefatos presentes, ordem normativa respeitada, R4 antes de R7, rollback documentado; diff do agente SDD limitado à fronteira (alterações em `STATE.md`, `PROGRESSO.md`, `feature.json`, `main.rs`, `assets/` e proveniência pertencem a outros agentes) | parcial (inspeção automatizada + revisão humana) |
| 10 | Conferir que nenhum comando deste roteiro tocou `main`, `docs/GDD*`, baselines, ledger, CI ou paths untracked alheios; planejar rollback sem executá-lo | nada alterado fora de `$TMP` e dos artefatos/código do ciclo; rollback documentado em `release.md` permanece não executado | sim |

## Plano de capturas (Xvfb — sem janela no desktop da SE)
- `$TMP/capturas/hud-036-titulo.png`, `hud-036-limites.png`, `hud-036-titulo-2.png`, `hud-036-limites-2.png` — saídas cruas do display virtual.
- `$TMP/capturas/carimbo.txt` — data/hora UTC de cada captura; `$TMP/capturas/glyph-check.txt` — resultado do passo 6 (glifos presentes / nenhum `▯`).
- `$TMP/capturas/verify.txt` e `$TMP/capturas/gates.txt` — saídas do passo 8; `$TMP/capturas/source-hash.txt` e `license-hash.txt` — saídas dos passos 1–2.
- Toda captura é rotulada **[NV]** até o orquestrador executar o passo correspondente; nenhuma captura existe neste ciclo. A captura prévia `docs/evidencias/hud-confirmacao-visual-2026-11-22.png` (na árvore principal) documenta apenas o HUD ASCII anterior.

## Rollback do roteiro
`rm -rf "$TMP"` remove capturas e arquivos de apoio; encerrar o cliente Xvfb e não persistir nada fora do worktree. Se algum passo falhar, registrar a saída obtida e corrigir a **fonte** (código, fonte ou spec), nunca a expectativa do teste. Não executar `git clean`, `git reset`, force-push, remoção de artefatos do ciclo, reexecução de gates por este agente nem abertura de janela normal. Para reverter a mudança em si, seguir `release.md` (rollback documentado, nunca executado neste ciclo).
