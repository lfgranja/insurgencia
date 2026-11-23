# STATE.md — Insurgência

## Correção de contagens — 2026-11-22T23:05Z (snapshot 22:58–23:01Z superado)

- **Motivo**: o snapshot 22:58–23:01Z (36 specs, 238 assessments, 36 validations, 36 releases, 0 bugs, 13 research) misturou a árvore versionada do worktree com untracked históricos do checkout compartilhado. A nota imediatamente abaixo permanece como histórico, não apagada.
- **Contagens VERIFICADAS (2026-11-22T23:05:02Z, árvore versionada a partir de `origin/dev`)**: **37 specs, 65 assessments, 37 validations, 37 releases, 0 bugs, 10 research**.
- **Comandos (saídas verificadas)**: `find specs -maxdepth 1 -type d -name '[0-9][0-9][0-9]-*' | wc -l` → 37; `find .specify/assessments -mindepth 1 -maxdepth 1 -type d | wc -l` → 65; `find .specify/validations -mindepth 1 -maxdepth 1 -type d | wc -l` → 37; `find .specify/releases -mindepth 1 -maxdepth 1 -type d | wc -l` → 37; `find docs/research -maxdepth 1 -type f -name '*.md' | wc -l` → 10; `find docs/research -type f -name '*.md' | wc -l` → 10.
- **Por que o checkout compartilhado apresenta 238/13**: ele inclui 173 diretórios untracked históricos (`ast-*`, `cor-*`, `doc-*`, `srv-*`, `ev-*`) e 3 notas untracked em `docs/research/2026-11-20-*`, preservados por decisão registrada — não são versionados. O 36 de specs/validations/releases era o estado antes da criação do ciclo 037; agora são 37 cada (incluindo o ciclo 037 em curso).
- **Inalterado**: âncora `efb4d4b`/`c126b6e`, PR #45 escalado ao SWE, `./verify.sh` 41/41, escalonamento e decisão de governança/R6. Nota append-only; nenhum gate/build/format/commit/push/PR/deploy executado; 0 linhas removidas.

## Escalonamento ao SWE — 2026-11-22T23:01Z (sessão "insurgencia - estado verificado e escalonamento 45")

- **Sessão/pedido**: `insurgencia - estado verificado e escalonamento 45`; `session_id=20261122T225813Z-estado-escalonamento`. Escalonamento didático do estado verificado ao SWE — nada executado além destas notas append-only.
- **Âncora (Bloco 0.a)**: HEAD/`dev` `efb4d4b`; `main=c126b6e`. PR #44 mesclado em `dev` às 21:44:38Z, merge commit `efb4d4b`. Autoridade do alvo = `./verify.sh` + 5 gates Python; CI GitHub Actions; CLI `gh`; factory apenas fonte. Divergência v4.6 (prompt pede constituição v1.2.0/tasks antes de analyze/R6; alvo adota v1.0.0/analyze antes de tasks, sem R6, e `resolve-tasks.sh` ausente) registrada como **decisão de governança do alvo**, não como dívida executada (Bloco 0.a).
- **80/20 registrado**: critério interno impacto × confiança × facilidade × segurança ÷ 625 = **375/625** — documentação/escalonamento (375) sobre execução (250). Nenhuma mudança de código.
- **Checks/roteiro/evidência (observados pela orquestração, não reexecutados nesta nota)**: `./verify.sh` `41/41` (`ALL STAGES OK`); 5 gates Python PASS; contagens reais da auditoria **36 specs, 238 assessments, 36 validations, 36 releases, 0 bugs, 13 research**. Roteiro de 11 passos **11/11 PASS** do ciclo 036, evidência em `.specify/validations/hud-acento-fonte-livre/evidence.md`.
- **PRs**: #44 `dev` — **mesclado** 21:44:38Z (`efb4d4b`). #45 `dev → main` — **OPEN**, head `efb4d4b`, `mergeable`, `autoMergeRequest=null`, checks `gates`/`rust` success (runs `45073795205`/`45073823941`), ACR sem blockers (comentário `7086327685`); escalado ao SWE em https://github.com/lfgranja/insurgencia/pull/45. **Não mesclar `main`**: promoção/deploy é competência exclusiva do SWE.
- **Dívidas/gaps em aberto**: (1) deploy em `main` ainda sem job no workflow `ci-gates.yml` — gap previamente escalado, segue aberto; (2) R6 não ratificado no alvo (divergência v4.6, governança registrada, não executada).
- **[NV] remanescentes**: decisão do SWE sobre o PR #45, estado remoto após esta nota, deploy/publicação, efeitos em produção e assinatura GPG. Nota append-only; nenhum gate/build/format/commit/push/PR/deploy executado.

## HUD com acento e fonte livre — 2026-11-22T21:17Z (ciclo 036 em curso)

- **Sessão/pedido**: `insurgencia - HUD com acento e fonte livre`; `session_id=20261122T211548Z-hud-acento-fonte`. Pedido explícito do SWE: restaurar `Ê`, `—` e `·` no HUD.
- **Âncora/80-20**: ciclo **036** em curso, branch `036-hud-acento-fonte-livre`, base `7c5c0f3`. 80/20 registrado — a escolha do usuário tem prioridade; critério interno impacto 5 × confiança 4 × facilidade 3 × segurança 4 ÷ 625 = **240/625**.
- **Superação**: o paliativo ASCII do ciclo 029 (`INSURGENCIA - low-poly`, `/` no lugar de `·`) é superado por esta mudança. O histórico do ciclo 029 permanece intacto (append-only).
- **Fonte**: Noto Sans Regular, 621572 bytes, OFL 1.1, cobertura de U+00CA/U+2014/U+00B7 confirmada por `fc-match`; licença e proveniência em `assets/fonts/OFL.txt` e `docs/governance/font-provenance.md` (escritos pelo agente de código).
- **PR #40 mesclado pelo SWE** às 20:52:35Z, merge commit `c126b6e`; `origin/main=c126b6e`. Isso **encerra o [NV]** de promoção `dev → main` das sessões anteriores — registrado como fato observado, não como pendência.
- **Evidência visual**: captura anterior (ASCII, sem tofu) em `docs/evidencias/hud-confirmacao-visual-2026-11-22.png`; a nova captura com acento será executada pelo orquestrador em Xvfb isolado, sem janela no desktop.
- **[NV] remanescentes**: CI/merge do ciclo 036, deploy, assinatura GPG e qualquer efeito de produção. Nota append-only; nenhum gate/build/format/commit/push/PR executado.

## Escalonamento ao SWE — 2026-11-22T19:06Z (sessão "insurgencia - fechamento de advisories ACR e rastreabilidade")

- **Contexto**: o ciclo 80/20 de fechamento de advisories de ACR e deriva documental terminou com o merge do PR #42 em `dev` (`87c8099`, 19:05:43Z). A promoção `dev → main` é a continuação normativa do ciclo (Bloco 7b), mas a decisão final e o deploy pertencem exclusivamente ao SWE — nenhum agente mesclou `main`.
- **Evidência**: PR #42 — `gates pass` (6s, run `45058283735`) e `rust pass` (4m37s, mesmo run), ACR sem blockers (comentário `7084068822`), roteiro 10/10 PASS com capturas em `/tmp/tmp.FAhaahvF3D/capturas/` e tabela em `.specify/validations/advisories-acr-rastreabilidade/evidence.md`. `./verify.sh` no worktree: `ALL STAGES OK`, 41/41; gates Python: SDD `35/0`, DAG `332/34/0`, task-id `0`, converge `18/18`, hub-docs `0`. PR #40 (`dev → main`) permanece `OPEN`; seu head será reobservado no SHA final antes do escalonamento.
- **O que resta ao SWE**: decidir a promoção de `dev` para `main` no PR #40 e, se aprovada, disparar o deploy em produção conforme a configuração de release do repositório. O agente não executa nenhum desses passos, não habilita auto-merge e não altera proteções.
- **Riscos, limitações e rollback**: blast radius limitado à promoção documental/canônica já validada; rollback do ciclo 035 é por reversão do merge commit `87c8099` em `dev` (ou do merge da promoção em `main`, se ela ocorrer). **[NV]**: estado final do PR #40, deploy, efeitos em produção, confirmação visual da SE (PR #35) e assinatura GPG.
- **Issues relacionadas**: Refs #39, #40, #41, #011; PRs #42 (mergeado em `dev`) e #40 (escalado).

## Estado verificado — 2026-11-22T18:34Z (sessão "insurgencia - fechamento de advisories ACR e rastreabilidade"; ciclo 035 em curso)

- **Sessão**: `session_title=insurgencia - fechamento de advisories ACR e rastreabilidade`; `session_id=20261122T183153Z-acr-advisories-trace`. Decisão 80/20 (impacto × confiança × facilidade × segurança ÷ 625) = fechar advisories ACR + corrigir deriva documental, **400/625**.
- **Âncora**: HEAD `f9d87e0` em `dev`; `.specify/feature.json → specs/034-gate-manifesto-cli`. Ciclo **035** (`035-advisories-acr-rastreabilidade`, base `f9d87e0`) em curso neste worktree isolado. Autoridade do alvo = `./verify.sh` + gates Python; CI GitHub Actions; CLI `gh`; factory apenas fonte (Bloco 0.a). **[NV]**: estado remoto após esta nota.
- **Contagens reais (auditoria 18:31–18:34Z)**: **34 specs, 237 assessments, 34 validations, 34 releases, 0 bugs, 13 research**. A nota de 16:59Z imediatamente abaixo (33/236/33/33, âncora `specs/033`) é **snapshot pré-034** — preservada como histórico e **superada** por esta.
- **Gates (18:32:17Z)**: `./verify.sh` → `ALL STAGES OK`, `41 tests run: 41 passed`; SDD `34/0`; DAG `314/17/0`; task-id `0 duplicatas`; converge `17/17`; hub-docs `0 erros`. Observado pela orquestração via `gh`/`git`; não reexecutado nesta nota.
- **Proteção de `dev`**: `contexts=["gates","rust"]`, `strict=true`, `enforce_admins=false` (verificado 21/11, `docs/PROGRESSO.md`). Avisos antigos ("rust não obrigatório", `dev=3709c56`) estão superados.
- **PR #40 (`dev→main`, head `f9d87e0`)**: `OPEN` na fila do SWE; checks `gates pass` (run `45050452465`) e `rust pass` (run `45050458892`). **Sem merge de `main`** nesta sessão — deploy/publicação é competência do SWE, não autorizada a agente. Estado do PR após esta nota: **[NV]**.
- **Dívidas documentais corrigidas nesta nota**: contagens, âncora e proteção em `STATE.md`, `docs/PROGRESSO.md`, `OBJETIVO.md`, `ROADMAP.md` e `docs/research/index.md` — notas datadas, append-only; nenhum gate/build/format/commit/push/PR executado.
- **[NV]/DEFER remanescentes**: confirmação visual final da SE, deploy/publicação e assinatura GPG.

## Estado verificado — 2026-11-22T16:59Z (sessão "insurgencia - gate CLI de manifesto")

- **Autoridade (Bloco 0.a)**: gate do alvo = `./verify.sh` + 5 gates Python; CI GitHub Actions; CLI `gh`; contexts obrigatórios em `dev` = `gates` + `rust` (`enforce_admins=false`, sem approvers). Factory `b1deab1` (origin/dev `d5d796f` capturado ao vivo) é apenas fonte: Forgejo/`fj`/`check_production_gate.py` **não** são importados. Superfície de teste = CLI/HTTP (sem UI de produto utilizável para `browser-use` nesta sessão — capturas do cliente **[NV]**).
- **Drenagem (0.0)**: 0 PRs abertos para `dev`/`main`; 1 worktree, 0 stashes, 193 paths untracked alheios preservados; ledger criado nesta sessão (`.specify/queue/concurrency.jsonl`, agora gitignorado). Nada a herdar.
- **Gates (16:54:17Z)**: `./verify.sh` → `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0`; 5 gates PASS (SDD `33/0`, DAG `296/0`, task-id `0 duplicatas`, converge `16/16`, hub-docs `0 erros`). Tentativa anterior estourou 900 s por `cargo` concorrente — **não** foi RED. CI remota no head exato `7329c53`: run `44952738240` `success`.
- **Contagens**: 33 specs, 236 assessments, 33 validations, 33 releases, 0 bugs, 13 research. Scripts de docs atualizados em 21/11; nota de 22/11 registrada em `docs/PROGRESSO.md`.
- **80/20 escolhido (Bloco 5)**: critério `impacto × confiança × facilidade × segurança / 625`, escala 1–5. A = **gate CLI de manifesto** 4×5×4×4 = **320/625 = 0,512**; B = proveniência de assets 2×4×5×5 = 200/625 = 0,320; C = `avancar_tick` no cliente 5×3×3×2 = 90/625 = 0,144. Escolhido A: converte o validador de biblioteca do ciclo 033 em consumidor executável determinístico, sem dependências novas.
- **Gap escalado (0.7)**: workflow `ci-gates.yml` não tem job de deploy em push `main` — configuração de deploy é competência do SWE; registrado, não alterado.

## Estado vivo — 2026-11-21T21:31Z (ciclo 033: manifesto de compatibilidade em Rust)

- **80/20 escolhido**: portar para Rust o contrato de manifesto **já existente** (`manifesto.json` 6 campos, `specs/011` FR-001..FR-006, ordem formato→versão→módulo→64 hex→regras→hash), não criar segundo esquema. GDD §1/§14.2 fundamenta a camada de framework; `contratos` só tinha constantes e `REJEICAO_MANIFESTO`, sem validador.
- **Entrega**: `crates/contratos/src/manifesto.rs` (`ManifestoCompatibilidade`, `ErroCompatibilidade` com `codigo()`, `validar_formato`, `conferir` sem I/O), `pub mod manifesto;` em `lib.rs`, `crates/contratos/tests/manifesto.rs` com 7 testes espelhando `tests/test_manifesto.gd`. Sem dependências novas, sem `unsafe`/`unwrap`/`expect`.
- **Correções reais registradas no converge (append-only)**: `clippy::double_must_use` (atributos redundantes removidos) e rustfmt aplicado (`cargo fmt --all`). Nenhum `allow` adicionado.
- **Verificação (21:31:15Z)**: `./verify.sh` → `ALL STAGES OK`, `41 tests run: 41 passed` (antes 34), `VERIFY_EXIT=0`; 5 gates PASS — SDD `33/0`, DAG `296/0`, task-id `0 duplicatas`, converge `16/16`, hub-docs `0 erros`. Baseline de converge reatualizado por bootstrap.
- Factory fonte `b1deab1`, nada transplantado; 194 paths untracked alheios preservados. **[NV]**: CI/merge do ciclo 033, confirmação visual da SE, deploy/GPG (DEFER).

## Estado vivo — 2026-11-21T20:25Z (PR #36 mesclado, proteção de `dev` endurecida)

- **Merge autorizado**: `gh pr merge 36 --merge` às 20:25:32Z → `MERGED` em `2026-11-21T20:25:35Z`, merge commit `ce2d1f0`; `git ls-remote origin refs/heads/dev` = `ce2d1f0`; 0 PRs abertos. Precedido de `gh pr checks 36`: `gates pass 9s`, `rust pass 3m48s` e `./verify.sh ALL STAGES OK (34/34)` às 20:25:17Z, mais 5/5 gates Python PASS.
- **Endurecimento autorizado e verificado (20:25:48Z)**: proteção de `dev` passou de `contexts=["gates"]` para `contexts=["gates","rust"]`, preservando `strict=true`, `enforce_admins=false`, `reviews=null`, `restrictions=null`. PUT + GET via `gh api` com respostas coladas em `docs/PROGRESSO.md`. Risco de confiança corrigido.
- **Ciclo 032 em curso** (`032-endurecimento-check-rust`, base `ce2d1f0`, `feature.json → specs/032-endurecimento-check-rust`): artefatos SDD registram a mutação remota, R4/R7 `1.0.0` com rollback documentado.
- **[NV]/DEFER**: confirmação visual final da SE; deploy/publicação e assinatura GPG (DEFER, sem autorização); nenhum outro status remoto afirmado sem comando executado. Factory lida como fonte em `b1deab1`, nada transplantado.

## Estado vivo — 2026-11-21T13:28Z (ciclo 031, PR #36 aberto, gates verdes)

- **Âncora**: branch `031-fechamento-rastreabilidade`, HEAD `887e042`, `.specify/feature.json` → `specs/031-fechamento-rastreabilidade`; factory lida como fonte em `b1deab1`, nada transplantado. Gate autoritativo do alvo = `./verify.sh` + 5 gates Python; CI GitHub Actions; CLI `gh`. Bloco 0.a: divergência de provider/CLI/gate da factory classificada como decisão de governança do alvo; `check_production_gate.py` ausente → exit 2 = N/A, não RED.
- **80/20 escolhido (I×C×F÷R = 2×3×3÷1 = 18)**: fechar exatidão documental e tarefas herdadas. Ciclo 031 completo: assess `go` → spec/plan/tasks T001–T012/checklist/analyze READ-ONLY/converge append-only → R4 → R7 `1.0.0`. Fechadas `028-T011` e `029-T004..T008` com notas datadas; errata de `verify.sh --all-targets` corrigida em 8 artefatos; `AGENTS.md` esclarece Lightyear (ADR-004, ausente das manifestações).
- **Entrega**: commit `887e042` (DCO OK), push e **PR #36 aberto** para `dev` às 13:27:18Z (`state=OPEN`, `mergeable=MERGEABLE`). Sem merge, sem auto-merge, sem deleção de branches. 194 paths untracked alheios (`ast-*`, `cor-*`, `doc-*`, `srv-*`, `ev-*`) preservados e não staged.
- **Verificação final (13:28:34Z)**: `./verify.sh` → `ALL STAGES OK`, `34 tests run: 34 passed`, `VERIFY_EXIT=0`; `pytest -q` → `no tests ran` (exit 5 = N/A); 5 gates PASS — hub-docs `0 erros`, SDD `spec_dirs=31 gaps=0`, converge `14/14`, task-id `0 duplicatas`, DAG `274 tarefas/0 ciclos`.
- **Remoto ao vivo (13:19:29Z, `gh`)**: repo público, default `main`; 5 runs `deterministic-gates` `success`; proteção de `dev` exige **somente** o check `gates` (job `rust` não obrigatório), `enforce_admins=false`, sem approvers; `main` desprotegido (404). Endurecer `rust` exige mutação remota e autorização explícita da SE — escalado, não executado.
- **CI do PR #36 verificado ao vivo (13:35Z)**: run `44909322956` — `gates` **pass** (7s) e `rust` **pass** (3m30s); `state=OPEN`, `mergeable=MERGEABLE`. **Sem merge**: Bloco 7 exige aprovador distinto e revisão deliberada de `docs/governance/**` (baseline de converge alterado) — não disponível nesta sessão. **[NV] remanescentes**: merge, confirmação visual final da SE, deploy/publicação, assinatura GPG.

## Estado vivo — 2026-11-21T13:09Z (assess Slotbound 28/28 + ciclo 80/20 demo headless)

- **Assess concluído**: 28 propostas do vídeo `Z4ayMTXnkoQ` com `intake.md`/`research.md`/`problem.md`/`concept.md`/`decision.md` em `.specify/assessments/sb-*`. Vereditos: 3 ADOPT · 19 ADAPT · 4 DEFER · 2 AVOID · 0 REJECT integral (`sb-26` tem subparte REJECT). Síntese e 9 notas em `docs/research/slotbound-2026-11-21-*.md`.
- **Correções factuais**: o salto de wishlists veio da **demo**, não do Next Fest; Valve documenta que wishlists "mostly aren't a factor"; voz/stills IA e o suposto "walk-back" são inferências do youtuber, não evidência primária; percentuais regionais `[NV]`, 79% global confirmado (1.598 reviews em 21/11).
- **Ciclo 80/20 executado (`sb-16`)**: novo `crates/sim/examples/demo_deterministica.rs` — 30 ticks, 2 jogadores, `hash_estado` por tick, sem Bevy/janela/rede; `./verify.sh` (que executa `--all-targets` internamente) cobre o exemplo automaticamente. Verificado: `rustc --edition 2021 -D warnings` PASS; `clippy-driver -D warnings -W clippy::pedantic` `CLIPPY_EXIT=0`; duas execuções idênticas (`DETERMINISTIC_OK`), `hash_final=6fcc2f9e…`, `segundos=1`.
- **[NV]**: `verify.sh` completo e CI/merge remotos — duas tentativas estouraram 180 s e 600 s por processos `cargo` concorrentes de outros repositórios na máquina. Nenhum verde afirmado sem saída observada (constituição VI).
- 14 subagentes em segundo plano; 5 skills lidas via `npx skills use` (research, competitive-brief, novelty-assessment, research-intake, kickstart-assess), nenhuma instalada. Trabalho alheio untracked (`ast-*`, `cor-*`, `doc-*`, `srv-*`, `ev-*`) preservado.
- **Ciclo 030 fechado**: `specs/030-demo-deterministica-headless/` (spec, plan, tasks T001–T011, checklist, converge), validação R4 e release R7 `1.0.0`, assessment equivalente em `.specify/assessments/demo-deterministica-headless/`; `.specify/feature.json` aponta para o ciclo 030. **5/5 gates Python PASS**: SDD `spec_dirs=30 gaps=0`, converge `13/13` (baseline atualizado), DAG `262 tarefas/0 ciclos`, task-id `0 duplicatas`, hub-docs `0 erros`.

## Estado vivo — 2026-11-21T05:12Z (HUD ASCII e AGENTS sintetizado, PR #35 mesclado)

- **PR #35 mesclado** (ciclo 029, merge `970af4e`, 05:11:50Z): total 35 PRs, 0 abertos. HUD com ASCII seguro (`INSURGENCIA - low-poly`, `/`), eliminando "tofu" da fonte padrão do Bevy (imagem da SE 04:33Z); 2 inserções/2 remoções, verificador isolado PASS. `AGENTS.md` sintetizado de 4 pesquisas paralelas (core src, tests, configs/build, scripts/docs) — corrige afirmação obsoleta sobre `Makefile`, reforça regra da janela. CI `gates` + `rust` SUCCESS (run `44869231266`, 05:11Z). Inspeção visual final **[NV]** até SE.
- **PR #34 mesclado** (ciclo 028, merge `d2b23e4`, 04:57:38Z): total 34 PRs, 0 abertos. CI `ci-gates.yml` agora tem job `rust` (`./verify.sh`) além de `gates`; toolchain 1.99.1 explícita e libs de sistema (Wayland/ALSA/X11/GL) instaladas no runner. Check `rust` SUCCESS observado ao vivo (run `44868073016`, 04:56:22Z) após 3 iterações de correção (C6/C7/C8). `AGENTS.md` reescrito com realidade Rust; `verify.sh:2` e GDD:1627 corrigidos. 5/5 gates + verify.sh PASS.
- **PR #33 mesclado** (ciclo 027), total 33 PRs, 0 abertos. Correção do report da SE ("continua greybox"). `main.rs` reescrito com terreno irregular (320 triângulos), ilhas com 32 triângulos cada, 3 árvores + 2 rochas por ilha e avatar com corpo/cabeça icosaédrica. `lib.rs` intacto (5 testes preservados).
- **Correção B0004**: `Visibility::default()` no pai do avatar — sem isso os filhos ficavam invisíveis conforme doc oficial Bevy. `Transform::IDENTITY` nos spawns sem transform.
- **Inspeção visual da SE confirmada (2026-11-21T04:33Z)**: imagem anexada mostra terreno irregular triangulado, 4 ilhas com taludos/cores, árvores/rochas e avatar low-poly com HUD. Resolve o `[NV]` de visual do ciclo 027 (SC-005). Captura de agente via `spectacle -b -n -f` funciona em Wayland (não mais "imagem preta" via Xwayland).
- **Regra da janela reforçada pela SE (2026-11-21T04:33Z)**: agente SEMPRE usa `--minimizado` em dev/debug/teste/validação/segundo plano. **NUNCA** abrir janela normal enquanto a SE trabalha no notebook; janela normal só sob pedido explícito e momentâneo. Registrada em `AGENTS.md` §Development Commands/§Testing & QA.

## Anteriores — 2026-11-21T01:26Z (4 itens da SE entregues)

- **Ciclos 023–025 mesclados** (PRs #29, #30, #31). Stack Bevy 0.20.1 + Lightyear 0.31 (ADR-004), Rust 1.99.1, 5 crates. Total 31 PRs mesclados, 0 abertos.
- **Item 1** — cliente greybox jogável: `Camera2d`, avatar WASD, 4 ilhas geométricas, painel de texto; janela inicia **minizada** por pedido da SE. Binário executado 8s com GPU Vulkan e sem erro; inspeção visual do conteúdo **[NV]** (Wayland/KDE, captura da ponte Xwayland preta).
- **Item 2** — duas partidas consecutivas com cliente modificado (GDD:1567) no corredor determinístico, com 3 divergências registradas no Núcleo.
- **Item 3** — instrumentação no Núcleo + 20 partidas com métricas da §17; relatório em `docs/evidencias/metricas-20-partidas-2026-11-21.md`. Números de simulação determinística, não de partidas humanas.
- **Item 4** — exportação versionada, `COPYRIGHT.txt` MIT e DCO (`Signed-off-by`) no commit. **Assinatura GPG [NV]**: não há chave no ambiente; gerar exige autorização da SE.
- **DEFER (itens 5/6)** — dois servidores com Módulos diferentes/queda do servidor de referência e servidores comunitários/forks publicados.


**Validação visual confirmada pela SE (2026-11-21T01:57Z):** o cliente greybox abre, movimenta e, ao passar sobre as ilhas, identifica a região no painel. Observação direta da SE — não reexecutada nem inferida por agente. Resolve o `[NV]` anterior sobre inspeção visual do greybox (capturas de agente via ponte Xwayland continuam limitations técnicas, não do jogo).
**Regra operacional da janela (SE, 2026-11-21):** rodar o cliente **normal** (sem flag) somente quando a SE solicitar; durante inspecao, debug, testes e validacao de agente, iniciar com `--minimizado`. Registrada em `AGENTS.md` §Development Commands e §Testing & QA.
- **Verificação:** 5/5 gates do alvo PASS, `verify.sh` ALL STAGES OK, 34 testes nextest, 31 CI `success` (verificado em 01:26Z).

## Anteriores — 2026-11-21T00:22Z (prioridade da SE redefinida)

- **Alvo agora:** MVP jogável **single-player** que prove a tese: (1) cliente greybox jogável (main scene, câmera, ilhas geométricas); (2) duas partidas consecutivas, uma com cliente modificado (GDD:1567); (3) instrumentação no Núcleo + 20 partidas com as métricas da §17; (4) exportação de conta/personagem, `COPYRIGHT.txt` e commits assinados (GDD §15.9).
- **DEFER explícito:** (5) dois servidores com Módulos diferentes e teste de queda do servidor de referência (GDD:50–53); (6) servidores comunitários e forks publicados — dependem de outras pessoas, não de código.
- **Reorientação:** o ciclo 023 anterior (servidor Lightyear) foi cancelado antes do commit; seus artefatos untracked foram removidos. Gates restaurados: 5/5 PASS e `verify.sh` ALL STAGES OK. Histórico Git preservado até o ciclo 022 (PR #28).
- **Stack mantida:** Bevy 0.20.1 + Lightyear 0.31 (ADR-004); rust-toolchain 1.99.1; crates `contratos` e `sim` mesclados. Cliente greybox entra como próximo ciclo, não servidor isolado.

## Anteriores — 2026-11-20T23:52Z (ciclo 020 mesclado; PR #26)

- **Âncora:** `.specify/feature.json` → `specs/020-engines-rust`; branch `feature/020-engines-rust`; PR #26 `MERGED` em `dev` às 23:52:56Z, check `gates` SUCCESS observado ao vivo. Total 26 PRs mesclados, 0 abertos.
- **Entrega:** `docs/AVALIACAO-ENGINES-RUST.md` (10 opções, 10 critérios C1–C10 com Σ=100%, matriz Score/Fase 0, 32 linhas de evidência, spikes A/B/C) e 5 relatórios em `docs/avaliacao-engines/pesquisa/`. Godot 82,6 ADOPT inalterado; Bevy+Lightyear 77,1 ADAPT só como padrões em GDScript; Fyrox/macroquad/ggez/Rapier REJECT; Ambient/gdext DEFER. ADR-001 não reaberta.
- **Protocolo:** factory HEAD `b1deab15` lido como fonte; specs 019/040; hosts restritos à allowlist (github.com); cobertura idiomática real = inglês; `[UNVERIFIED — fetch skipped]` para sites externos. Correções registradas no converge: Fyrox 65,2 vs 62,6, O10 derivada, C6 proxy, cobertura linguística.
- **Verificação:** 5/5 gates PASS (`spec_dirs=20`, `files=20`, `converge files=3 baselined=3` via bootstrap do próprio gate); `make check` e `make test-rede-e2e` permanecem exit 0 (estado técnico prévio, sem alteração de código).
- **[NV] remanescentes:** benchmark pareado Godot vs Bevy+Lightyear, reconexão Rust, migração de toolchain, sites fora da allowlist, conexão UDP externa/NAT, lobby, 5v5, deploy e serviços remotos.

## Anteriores — 2026-11-20T11:52Z (ciclo 019 mesclado; MVP fechado)

- **Âncora:** `.specify/feature.json` → `specs/019-tela-resultado`; branch `feature/019-tela-resultado`; PR #25 `MERGED` em `dev` às 11:52:26Z, check `gates` SUCCESS observado ao vivo. Total 25 PRs mesclados, 0 abertos.
- **Entrega:** `nucleo/apresentacao/tela_resultado.gd` — componente `Control` programático, sem `.tscn` e sem `main scene`; estado inicial `AGUARDANDO` e `VITORIA` somente se `evento="vitoria_ilha_central"` e `declarada_por="servidor"`. `tests/test_tela_resultado.gd` com 19 `PASS`.
- **Ciclo completo:** intake → research → problem → concept → decision (`go`) → specify (4 clarifications) → plan → checklist → tasks → analyze READ-ONLY → implement → converge append-only → R4 → R7 `1.0.0`, sem fase pulada. Três corretivas registradas no converge.
- **[NV] IANA resolvido:** CSV oficial consultado em 2026-11-20T11:38:06Z; nenhuma entrada `7777/tcp` ou `7777/udp`. Porta livre no registro — marcador informativo fechado, sem afirmar disponibilidade em rede.
- **Verificação:** 19 `PASS` tela + 167 `PASS` em 12 suítes, 0 `FAIL`; `make check` exit 0; 5/5 gates PASS (`spec_dirs=19`, `converge files=2 baselined=2`); bootstrap do baseline executado pelo próprio gate.
- **[NV] restantes:** NAT/Internet, lobby, 5v5/escala de produção, renderização visual inspecionada, deploy, Postgres/Coolify remotos, `pgmq`, federação, IAM remoto, chaves do jogador e aprovador humano (não exigido pela regra atual).

## Anteriores — 2026-11-20T11:34Z (ciclo 018 mesclado; PR #24)

- **Âncora:** `.specify/feature.json` → `specs/018-iam-adapter-local`; branch `feature/018-iam-adapter-local`; PR #24 `MERGED` em `dev` às 11:34:04Z, check `gates` SUCCESS observado ao vivo. Total 24 PRs mesclados, 0 abertos.
- **Entrega:** `nucleo/iam/iam_interface.gd` (4 métodos) e `nucleo/iam/iam_adapter_local.gd` (determinístico, em processo, sem servidor/segredo/credencial) + `tests/test_iam_adapter_local.gd` com 36 `PASS`. ADR-003 e contrato de arquitetura inalterados.
- **Ciclo completo:** intake → research → problem → concept → decision (`go`) → specify (4 clarifications) → plan → checklist → tasks → analyze READ-ONLY → implement → converge append-only → R4 → R7 `1.0.0`. Nenhuma fase pulada.
- **Verificação:** 36 `PASS` IAM + 131 `PASS` em regressão, 0 `FAIL`; `make check` exit 0; 5/5 gates PASS (`spec_dirs=18`, `files=18`, `converge files=1 baselined=1`). Bootstrap do baseline de converge executado pelo próprio gate (`--bootstrap`).
- **[NV] restantes:** IAM remoto, login obrigatório, chaves do jogador, NAT/Internet, lobby, 5v5, escala de produção, UI de resultado, deploy, Postgres/Coolify remotos, `pgmq`, federação e aprovador humano (não mais necessário para merge sob a nova regra).

## Anteriores — 2026-11-20T10:58Z (diretrizes da SE)

- **Governança:** check CI/CD verde observado autoriza merge autônomo em `dev`/`main` via `gh`, **sem aprovação humana** (`.specify/memory/constitution.md:22`, `AGENTS.md:60`). PRs #13–#22 aprovados retroativamente pela autorização da SE. Se o CI não inspirar confiança, o próximo 80/20 **imediato** é reforçá-lo.
- **Identidade:** Zitadel substituído por **IAMAdapter / IAMInterface** (ADR-003 + contrato em `docs/architecture/iam-interface.md`). Nenhum SDK concreto no domínio; adaptador local para solo; provedor real remoto **[NV]**.
- **Escopo:** **solo/single** confirmado pela SE — sem 5v5, sem lobby e sem NAT/Internet nesta fase. Validação do MVP pela SE com `docs/ROTEIRO-TESTE-SOLO.md` e `tests/roteiro_solo.sh`.
- **Técnico (10:05:42Z):** 39/39 caminhos, 18/18 critérios MVP, 5 gates PASS, `make check` e `make test-rede-e2e` exit 0, 131 `PASS`, 22 PRs mesclados, 15 CI `success`.
- **[NV] restantes:** NAT/clientes externos, lobby, 5v5/escala de produção, UI de resultado, deploy, Postgres/Coolify remotos, IAMAdapter remoto, `pgmq`, federação forte e registro IANA da porta 7777 (informativo).

## Anteriores — 2026-11-20T10:05Z (MVP completo; 22 PRs mesclados)

- **Ciclos:** 017/017 com R1 `go`, R4 aprovado, R7 `1.0.0`. PRs #13–#22 mesclados em `dev`; total 22 PRs `MERGED`, 0 abertos; 15/15 execuções CI `success` (verificado ao vivo via `gh`).
- **Critérios MVP §16.4:** 18/18 marcados no GDD `16.7`, incluindo UDP real entre dois processos (ciclo 017). 39/39 caminhos canônicos presentes.
- **Verificação final (10:05:42Z):** 5/5 gates PASS (`spec_dirs=17`, `tasks=222`, `cycles=0`); `make check GODOT=<binário>` exit 0 (5 `OK` + `COMPAT_OK`); `make test-rede-e2e` exit 0 (`SERVIDOR_PRONTO`, `join_accept`/`COMANDO_ACEITO` para 2 clientes); 131 `PASS` em 11 suítes, 0 `FAIL`.
- **Evidência:** `docs/progress/evidence.csv` — `EV-20261120-008` UDP real agora `PASS`; `EV-20261120-009` mantém **[NV]** de NAT/clientes externos.
- **[NV] remanescentes:** NAT traversal e clientes na Internet pública; lobby/matchmaking; partidas 5v5 e escala de produção (6 ilhas/20 s); cena de UI de resultado; deploy/publicação; Postgres/Coolify remotos, Zitadel e `pgmq`; federação forte; aprovador humano distinto.

## Anteriores — 2026-11-20T03:21Z (fatia 80/20 CE-1..CE-5 concluída)

- **Ciclos:** 012/012 com R1 `go`, R4 aprovado e R7 `1.0.0`. PRs #13 (008), #14 (009), #15 (010), #16 (011) e #17 (012) `MERGED` em `dev`; total 17 PRs mesclados, 0 abertos; 12/12 execuções CI `success` (verificado ao vivo via `gh`).
- **Entrega da fatia:** contrato versionado, servidor headless com snapshots e validação, Ilha Meio, gate de compatibilidade e `make check` com evidência auditável. `make check GODOT=<binário real>` exit 0: 5 `OK` + `COMPAT_OK`, 0 `FAIL`.
- **Gates (03:21:31Z):** 5/5 PASS (`spec_dirs=12`, `tasks=222`, `cycles=0`, `0 erros`). Inventário: 12 specs, 12 validations, 12 releases, 47 docs Markdown.
- **Evidência:** `docs/progress/evidence.csv` com 9 colunas, 8 linhas (7 `PASS` + 1 `NAO_VERIFICADO` para UDP real); registro datado em `docs/progress/2026-11-19-ciclo-80-20.md`.
- **GDD:** `### 16.7 Checklist de implementação` com CE-1..CE-5 marcados; critérios MVP restantes (Ilha Topo/Inferior, funções, vitória Central) permanecem `[ ]`.
- **[NV] remanescentes:** conexão UDP real entre clientes; Ilha Topo/Inferior, Artesão/Guerreiro e vitória da Ilha Central; deploy/publicação; Postgres/Coolify remotos, Zitadel, `pgmq`; federação forte; aprovação humana distinta. Instalação/execução do Godot e status remoto/CI/merge **não são mais [NV]** — verificados ao vivo.

## Anteriores — 2026-11-20T03:09:54Z (ciclo 009 mesclado; PR #14)

- **Âncora:** `.specify/feature.json` → `specs/009-servidor-validacao` (pré-merge); branch `feature/009-servidor-validacao`; commit `b918515` (20 arquivos, 579 inserções), PR #14 `MERGED` em `dev` às 03:09:54Z, check `gates` SUCCESS observado ao vivo.
- **Entrega:** `Mundo` 30 Hz, `ServidorAutoritativo` ENet/UDP com handshake, snapshots 20 Hz, `ValidacaoComandos`, `handle_command`, `hash_estado`; 4 suites Godot = 39 `PASS`, 0 `FAIL`; `SERVIDOR_PRONTO porta=7777`.
- **Correções reais ao plano:** `sha256_string()` inexistente → `HashingContext`; acumulador 30→20 com deriva → tolerância; arranque headless → `Engine.get_main_loop().get_multiplayer()`. Registradas no R4.
- **Gates (03:08:52Z):** 5/5 PASS (`spec_dirs=9`, `tasks=222`, `cycles=0`); `pytest -q` = exit 5 (N/A); `check_production_gate.py` N/A por 0.a.
- **GDD:** `### 16.7 Checklist de implementação` inserida após 16.6, com marcação conservadora (CE-1 concluído; CE-2 parcial — conexão UDP real **[NV]**; CE-3..CE-5 não implementados).
- **Dívidas:** 20/39 caminhos canônicos presentes; Ilha Meio, manifesto, `Makefile`, `evidence.csv` pendentes; deploy/serviços remotos/federação **[NV]**.

## Anteriores — 2026-11-20T02:02Z (ciclo 008 concluído, PR #13 aberto)

- **Âncora:** `.specify/feature.json` → `specs/008-nucleo-contrato`; branch `feature/008-nucleo-contrato`; HEAD `f5d9634` (commit `feat(nucleo): cria contrato versionado e Modulo de exemplo`, 20 arquivos, 489 inserções).
- **Entrega:** Núcleo (`Protocolo`, `CarregadorModulo`), Módulo de exemplo (11 campos, hash `b2c11e6d…d746`, 218 bytes) e `tests/test_contrato.gd` — 16 `PASS`, `OK test_contrato`, exit 0 com Godot `4.7.2.stable.official.ed1daf0bf` executado localmente.
- **R1/R4/R7:** `go` em `.specify/assessments/nucleo-contrato/decision.md`; R4 aprovado; R7 `1.0.0`. Analyze READ-ONLY corrigiu 3 divergências de escopo (39→14 caminhos) na fonte.
- **Gates (02:02:49Z):** 5/5 PASS (`spec_dirs=8`, `tasks=222`, `cycles=0`, `0 erros`); `pytest -q` = exit 5 (N/A); `check_production_gate.py` N/A por Bloco 0.a.
- **Remoto verificado ao vivo:** PR #13 `OPEN` para `dev`, check `gates` `SUCCESS` (`44657956734`, 02:02:53Z). **Merge não realizado** — exige aprovador distinto e revisão deliberada (Bloco 7). PRs #1–#12 permanecem `MERGED`.
- **Dívidas:** 25/39 caminhos canônicos pendentes das Tasks 2–6 (`make check`, `manifesto.json`, servidor, Ilha, `evidence.csv`); deploy/serviços remotos/federação **[NV]**.

## Anteriores — 2026-11-20T01:42Z (auditoria com verificação remota ao vivo)

- **Atualizado em**: 2026-11-20T01:42Z — `date -u` no início da sessão; HEAD local `2af7537`, branch `docs/estado-autorizacoes-2026-11-20`.
- **Remoto verificado ao vivo (`gh`, somente leitura)**: `visibility=PUBLIC`, default branch `main`; 12 PRs `MERGED` (#1–#12), 0 abertos; 10/10 execuções `deterministic-gates` `completed/success`; proteção de `dev` com `strict=true`, `contexts=["gates"]`.
- **Gates locais reexecutados**: 5/5 PASS (`spec_dirs=7`, `tasks=222`, `cycles=0`, `0 erros`); `pytest -q` = `no tests ran` (exit 5, N/A); `command -v godot` = AUSENTE no PATH nesta sessão (instalação registrada em ciclo 006 permanece documental **[NV]** quanto à execução agora).
- **Progresso apurado**: 7/7 ciclos com R4/R7; 89 FRs; 222/222 tarefas `[X]`; 63 SCs; ciclo 80/20 com 0/39 caminhos canônicos, 0 testes, 0 `Makefile`/`manifesto.json`/`evidence.csv`. Nenhum percentual de conclusão é declarado (portaria 80/20).
- **Pesquisa externa datada**: três notas com fontes primárias em `docs/research/` (Godot 4.7.2 confirmado por fonte oficial; ADR-001 mantida; envelope de fio próprio; correções P0/P1 registradas em `ROADMAP.md` §2A).
- **Bloqueios remanescentes**: publicação/deploy externo não autorizado; Postgres/Coolify remotos, Zitadel e `pgmq` **[NV]**; federação forte **[NV]**. Ver `OBJETIVO.md` §§2A–2B e `ROADMAP.md` §5.

## Anteriores — 2026-11-20T01:14:03Z (autorizações executadas até o limite seguro)

- **Atualizado em**: 2026-11-20T01:14:03Z — PRs #1–#11 mesclados em `dev`; HEAD remoto `05d38b8`. Âncora: `.specify/feature.json` → `specs/007-postgres-local-validacao`.
- **Repositório**: `visibility=PUBLIC` (00:26:47Z); CI `.github/workflows/ci-gates.yml` com execuções `success` e check `gates`; proteção de `dev` aplicada (`required_checks=["gates"]`, `strict=true`, sem force-push/deleção).
- **Godot 4.7.2**: instalada da release oficial com SHA512 de 128 hex idêntico ao `SHA512-SUMS.txt` (`CHECKSUM_OK`, 00:43:52Z); binário `4.7.2.stable.official.ed1daf0bf`. Benchmark headless sintético 600 ticks × 64 entidades: mediana 37,48 µs/tick em 7 amostras (mín 22,71; máx 42,95) — carga sintética, **não** 5v5 real.
- **Postgres**: validado localmente em container descartável `postgres:16-alpine` → 16.15; JSONB/GIN e privilégio mínimo provados; UPDATE negado ao papel da aplicação; container removido (0 remanescentes). Remote/Coolify/Zitadel/`pgmq` permanecem **[NV]**.
- **Ciclos concluídos**: 001, 002, NV, 003, 004, 005 (CI/proteção), 006 (Godot/benchmark), 007 (Postgres local) — todos com R1 go, R4 aprovado, R7 1.0.0 e PR mesclado.
- **Gates locais**: `sdd-completeness PASS (7/0/0)`, `work_order_dag PASS (222 tarefas/7 features)`, `task-id-uniqueness PASS`, `converge-append-only PASS`, `hub_docs_gate PASS`. `pytest`/`check_production_gate` N/A por Bloco 0.a.

### [NV] remanescentes — bloqueio real

- **Publicação/deploy externo**: não autorizado nesta rodada — permanece **[NV]**.
- **Postgres/Coolify remotos e Zitadel**: autorizados, mas credenciais e instância ficam fora do alvo (`MDMV/.tokens/` e topologia do Gru). Nenhuma credencial foi lida/copiada nesta sessão; sem acesso remoto verificável no alvo. **[NV]** — exige intervenção/fornecimento de credencial no escopo do projeto.
- **`pgmq`**: extensão não verificada no ambiente local; permanece **[NV]**.

### Próximo 80/20 (vivo)

1. ~~Corrigir documentalmente o GDD (N3 e contradição CI)~~ — **CONCLUÍDO no ciclo 002**.
2. Instalar Godot 4.7.2 e validar a suíte headless — exige download e autorização de dependência (não executado).
3. Gate CI remoto (GitHub Actions) — nenhum check reportado nos PRs #1–#3; exige configuração e verificação ao vivo.

## Anteriores — registros pré-resolução (Ciclos 001 e 002, até 2026-11-19T22:38:57Z)

- **Atualizado em**: 2026-11-19T22:38:57Z — Ciclo 002 (`emenda-gdd-adrs`) concluído; branch `feature/002-emenda-gdd-adrs`, HEADbase `723ef49` (PR #2 ainda aberto, sem merge).
- **Âncora atual**: `.specify/feature.json` → `specs/002-emenda-gdd-adrs`. Ciclo 001 permanece fechado e rastreável.
- **Data original do registro**: 2026-11-19T21:46:45Z
- **Âncora anterior**: `specs/001-excecao-linguagem-licenca-nucleo`
- **Ciclo anterior**: 001 — exceção de linguagem e licença MIT-only do Núcleo (R1 `go`, R4 aprovado, R7 1.0.0)
- **Branch anterior**: `feature/001-excecao-linguagem-licenca-nucleo` (sem merge)
- **HEAD base original**: `6e25758` (`main`); factory HEAD como fonte: `b1deab15f482dd85871cd6c6a857ad710b89e2a6`
- **Gate autoritativo do alvo**: `scripts/ci/` local; `check_production_gate.py --check` é N/A por decisão de governança do Bloco 0.a (factory é fonte, não autoridade)

## Gates locais (execução final)

| Comando | Saída |
|---|---|
| `python3 scripts/ci/check_sdd_completeness.py` | PASS (spec_dirs=1, gaps=0, baselined=0) |
| `python3 scripts/ci/check_work_order_dag.py` | PASS (files=1, tasks=35, cycles=0) |
| `python3 scripts/ci/check_task_id_uniqueness.py` | PASS (duplicates=0) |
| `python3 scripts/ci/check_converge_append_only.py` | PASS (files=0) |
| `python3 scripts/ci/hub_docs_gate.py` | PASS (0 erros, 0 avisos) |
| `python3 -m pytest -q` | N/A — `no tests ran` (exit 5); sem suíte no alvo |
| `python3 scripts/ci/check_production_gate.py --check` | N/A — arquivo ausente por decisão 0.a (exit 2 ≠ RED) |

## Dívidas / pendências

1. `docs/governance/sdd-completeness-baseline.json` não materializado (evitar chaves de specs da factory).
2. Aprovação humana dos ADR-001/ADR-002 pendente; estado dos ADRs é `Pendente de aprovação`.
3. `docs/specs/2026-11-19-ciclo-80-20-omoba.md` e `docs/plans/...` permanecem como ciclo de produto; o ciclo 001 é de governança e não os altera.
4. `docs/GDD - Insurgência.md` ainda registra N3 (MIT vs AGPL) e "CI desde o dia 1"; ADR-001/002 decidem localmente, mas a correção documental do GDD fica para ciclo posterior.
5. `checklists/adr-quality.md` tem 40 itens `[ ]` sob revisão humana de requisitos (não bloqueia a implementação conforme portaria reconciliada).
6. Nenhuma engine, código, CI, telemetria ou banco materializado neste ciclo — por escopo, não por omissão.

## [NV] — não verificado ao vivo

- PR #2 **observado ao vivo** via `gh pr view` em 2026-11-19T21:48:26Z: `state=OPEN`, `base=dev`, `head=feature/001-excecao-linguagem-licenca-nucleo`, `mergeable=MERGEABLE`, `isDraft=false`, `statusCheckRollup=[]` (nenhum check reportado). URL: https://github.com/lfgranja/insurgencia/pull/2. Nenhum merge realizado.
- PR #3 **observado ao vivo** via `gh pr view` em 2026-11-19T22:41:43Z: `state=OPEN`, `base=feature/001-excecao-linguagem-licenca-nucleo` (empilhado; redirecionar para `dev` após o merge do PR #2), `head=feature/002-emenda-gdd-adrs`, `mergeable=MERGEABLE`, `statusCheckRollup=[]`. URL: https://github.com/lfgranja/insurgencia/pull/3. Nenhum merge realizado.
- CI checks remotos, merge, proteções de branch e aprovações no GitHub: **[NV]** — nenhuma verificação ao vivo além do estado do PR acima.
- Estado da árvore de trabalho da factory (registrada apenas como fonte, nunca modificada).

## Próximo 80/20 (fora deste ciclo)

1. ~~Corrigir documentalmente o GDD (N3 e contradição CI) com base nos ADRs~~ — **CONCLUÍDO no ciclo 002** (7 passagens emendadas); mantido como histórico.
2. Instalar Godot 4.7.2 e validar a suíte headless — exige download e autorização de dependência.
3. Gate CI remoto (GitHub Actions) — exige autorização e verificação ao vivo.

## 2026-11-23T02:45Z — decisão 80/20 do ciclo 040 (antes da implementação)

**Sessão**: `20261123T024300Z-barricada-deterministica` · **Título**: `insurgencia - barricada deterministica de materiais`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Construir barricada (custo 20, 45 ticks, 200 PV, 360 ticks, máx 2) | 5 | 5 | 4 | 4 | **400/625** |
| Ponte provisória | 3 | 3 | 3 | 4 | 108/625 |
| Combate/Fôlego (valores `[HIPÓTESE]`) | 4 | 2 | 2 | 3 | 48/625 |
| Servidor + Lightyear (dep ausente, docs.rs 0.31 falha) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `construir_barreira`. É o primeiro destino canônico dos Materiais introduzidos no ciclo 039, tem valores tabelados no GDD (§9.3, §10.3), não exige protocolo/Cargo novo e produz efeito verificável no core determinístico e no HUD. Netcode fica habilitado, mas não é o nº 1. R6 permanece **N/A** por constituição v1.2.2 (dívida de automação registrada).

## 2026-11-23T03:54Z — decisão 80/20 do ciclo 041 (antes da implementação)

**Sessão**: `20261123T035400Z-coleta-jogavel` · **Título**: `insurgencia - coleta jogavel no cliente`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Coleta jogável no cliente (tecla C + auto-avanço) | 5 | 5 | 5 | 4 | **500/625** |
| Corrigir comentário de `Comando.destino` | 2 | 5 | 5 | 5 | 250/625 |
| Ponte provisória | 3 | 3 | 3 | 4 | 108/625 |
| Servidor + Lightyear | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `coleta-jogavel`. O ciclo 040 entregou barricada e HUD, mas o espelho local começa com 0 Materiais e não tem como coletar: a cadeia jogável está incompleta. A correção do comentário de `destino` entra no mesmo ciclo por ser trivial e adjacente. R6 continua **N/A** por constituição v1.2.2.
