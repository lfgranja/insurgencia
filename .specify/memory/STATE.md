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

## 2026-11-23T04:30Z — decisão 80/20 do ciclo 042 (antes da implementação)

**Sessão**: `20261123T043000Z-servidor-headless` · **Título**: `insurgencia - servidor headless deterministico`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Servidor headless determinístico (stdin → ticks → hash) | 4 | 4 | 4 | 4 | **256/625** |
| Dívida documental restante | 2 | 5 | 5 | 5 | 250/625 |
| Combate/Fôlego (`[HIPÓTESE]`) | 4 | 2 | 2 | 3 | 48/625 |
| Rede com Lightyear (dep ausente) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `servidor-headless`. É o maior ganho de autoridade sem dependência externa: um binário que possui `Mundo`, consome comandos de entrada determinística e emite hash por tick. Prepara o caminho para rede sem introduzir Lightyear, async ou sockets neste ciclo. R6 permanece **N/A** por constituição v1.2.2.

## 2026-11-23T05:47Z — decisão 80/20 do ciclo 044 (antes da implementação)

**Sessão**: `20261123T054700Z-defesa-barricada` · **Título**: `insurgencia - defesa de barricada`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Defesa de barricada → +1 autogestão/10 s | 4 | 4 | 4 | 4 | **256/625** |
| Súditos/Propaganda (maioria `[HIPÓTESE]`) | 4 | 2 | 2 | 3 | 48/625 |
| Ponte provisória | 3 | 3 | 3 | 4 | 108/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `defesa-barricada`. GDD §7.1:306 dá recompensa canônica de +1 ponto de autogestão a cada 10 s de barricada intacta, e o ciclo 040 já entregou a barricada. Nenhuma mudança de stack, dependência ou protocolo. Protótipo usa o dono da barricada como defensor no mesmo ponto; papéis Guerreiro/Artesão ficam registrados como simplificação escopo. R6 permanece **N/A** por constituição v1.2.2.

## 2026-11-23T06:25Z — decisão 80/20 do ciclo 045 (antes da implementação)

**Sessão**: `20261123T062500Z-ponte-provisoria` · **Título**: `insurgencia - ponte provisoria deterministica`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Ponte provisória (15 Materiais, 60 PV, 45 s) | 3 | 3 | 3 | 4 | **108/625** |
| Súditos/Propaganda | 4 | 2 | 2 | 3 | 48/625 |
| Combate/Fôlego | 4 | 2 | 2 | 3 | 48/625 |
| Rede (bloqueada por ADR-004) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `ponte-provisoria`. É o próximo recurso canônico implementável sem nova stack: GDD §9.3:475,483 fixa custo 15 Materiais e §8.4:407 fixa 60 PV e 45 s de duração; o atalho de 8 s depende de mapa/ilhas e fica fora de escopo, registrado como limitação. R6 continua **N/A**.

## 2026-11-23T06:58Z — decisão 80/20 do ciclo 046 (antes da implementação)

**Sessão**: `20261123T065800Z-suditos` · **Título**: `insurgencia - suditos e propaganda`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Súditos/Propaganda (4 s de exposição, 3 s de reversão) | 4 | 2 | 2 | 3 | **48/625** |
| Combate/Fôlego | 4 | 2 | 2 | 3 | 48/625 |
| Rede (bloqueada por ADR-004) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `suditos`. A regra de aceitação GDD §9.2:464 (4 s de Propaganda interrompe ataque a estruturas; dano de jogador reverte em 3 s) é verificável mesmo com spawn/controle de ilha ainda `[HIPÓTESE]`. Valores de spawn, raio e função do Súdito ficam fora de escopo e registrados como hipóteses. R6 continua **N/A**.

## 2026-11-23T07:35Z — decisão 80/20 do ciclo 047 (antes da implementação)

**Sessão**: `20261123T073500Z-combate-folego` · **Título**: `insurgencia - combate e folego`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Combate/Fôlego (`atacar_forte`) | 4 | 2 | 2 | 3 | **48/625** |
| Rede (bloqueada por ADR-004) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `combate-folego`. Única frente de produto restante sem mudança de stack: GDD §10.1 define Fôlego 100, regeneração 10/s fora de combate e 5/s em combate, todos rotulados `[HIPÓTESE]`; a spec deve registrá-los como escolhas de protótipo, não dogmas canônicos. Dano e alcance também hipotéticos. R6 permanece **N/A**.

## 2026-11-23T08:18Z — decisão 80/20 do ciclo 048 (antes da implementação)

**Sessão**: `20261123T081800Z-alvo-treino` · **Título**: `insurgencia - alvo de treino jogavel`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Alvo de treino no cliente standalone | 3 | 4 | 4 | 5 | **240/625** |
| Ilhas/condição de vitória | 4 | 2 | 2 | 3 | 48/625 |
| Rede (bloqueada por ADR-004) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `alvo-treino`. A ACR do ciclo 047 apontou que a tecla `Q` gera um comando válido no core, mas o cliente standalone só tem `PEER_ID=1` e nenhum alvo, deixando o ataque inerte e a dica enganosa. Um alvo de treino determinístico resolve o achado sem mudar stack, protocolo ou regra canônica; é scaffolding de protótipo, explicitamente não-canônico. R6 permanece **N/A**.

## 2026-11-23T11:15Z — decisão 80/20 do ciclo 049 (antes da implementação)

**Sessão**: `20261123T111200Z-controle-ilha` · **Título**: `insurgencia - controle de ilha por sudito`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Controle de ilha por Súdito (10 s contínuos) | 4 | 3 | 3 | 3 | **108/625** |
| Nós de Controle + vitória completa | 4 | 2 | 2 | 3 | 48/625 |
| Papéis W/E/R + níveis | 4 | 2 | 2 | 3 | 48/625 |
| Rede (bloqueada por ADR-004) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `ilhas-vitoria` — subset **controle de ilha**: GDD §9.4:489,501 fixa 10 s contínuos de Súdito da Comuna e sinal `ilha_perdida` quando falta; sem mapa/rede, o protótipo trata Súditos convertidos como presentes globalmente e registra a simplificação. Nós de Controle e Resistência Ideológica ficam para o próximo ciclo. O refresh de `AGENTS.md` solicitado pelo usuário entra neste mesmo commit. R6 permanece **N/A**.

## 2026-11-23T12:50Z — decisão 80/20 do ciclo 050 (antes da implementação)

**Sessão**: `20261123T125000Z-nos-vitoria` · **Título**: `insurgencia - nos de controle e vitoria`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Nós de Controle + Resistência + vitória | 5 | 2 | 2 | 3 | **60/625** |
| Papéis W/E/R + níveis | 4 | 2 | 2 | 3 | 48/625 |
| PVE/PNJs | 4 | 2 | 2 | 3 | 48/625 |
| Rede (bloqueada por ADR-004) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `nos-vitoria`. É a condição de vitória do GDD, agora factível após controle de ilha e Súditos. Todos os números (3 Nós, 100 Resistência, −30/Nó, pulso final −10, dano cap −20, 20 s por Nó) são `[HIPÓTESE]` no GDD e devem ser rotulados como protótipo. R6 permanece **N/A**.

## 2026-11-23T14:25Z — decisão 80/20 do ciclo 051 (antes da implementação)

**Sessão**: `20261123T142500Z-derrota` · **Título**: `insurgencia - derrota por timeout e reconquista`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Derrota por timeout/reconquista | 4 | 3 | 4 | 4 | **192/625** |
| Papéis W/E/R + níveis | 4 | 2 | 2 | 3 | 48/625 |
| PVE/PNJs | 4 | 2 | 2 | 3 | 48/625 |
| Rede (bloqueada por ADR-004) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `derrota`. GDD §6.2:277 define derrota por relógio de 20 min ou 3 reconquistas; §9.4:489,501 já fornece perda de ilha e sinal `ilha_perdida`. A implementação reusa esses sinais e o contador de ilhas, sem mapa/rede. Números permanecem `[HIPÓTESE]`/protótipo. R6 permanece **N/A**.

## 2026-11-23T15:08Z — decisão 80/20 do ciclo 052 (antes da implementação)

**Sessão**: `20261123T150800Z-papeis-niveis` · **Título**: `insurgencia - papeis e niveis`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Papéis + níveis + desbloqueio Q/W/E/R | 4 | 2 | 2 | 3 | **48/625** |
| PVE/PNJs | 4 | 2 | 2 | 3 | 48/625 |
| Rede (bloqueada por ADR-004) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `papeis-niveis`. GDD §10.1 fixa papéis materiais, nível 1–12 e desbloqueio Q no 1, W no 2, E no 3, R no 6 — todos rotulados `[HIPÓTESE]`. Implementação mínima: campo `funcao`, nível por pulso, desbloqueio de ações existentes por nível e HUD; novas habilidades W/E/R ficam para ciclo posterior. R6 permanece **N/A**.

## 2026-11-23T15:53Z — decisão 80/20 do ciclo 053 (antes da implementação)

**Sessão**: `20261123T155300Z-pnj-defensores` · **Título**: `insurgencia - pnjs defensores`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| PNJs defensores (PV + dano determinístico) | 4 | 2 | 2 | 3 | **48/625** |
| Novas habilidades W/E/R | 4 | 2 | 2 | 3 | 48/625 |
| Rede (bloqueada por ADR-004) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `pnj-defensores`. GDD §8.4/§8.5 descreve guardas/patrulhas com dano e PV como `[HIPÓTESE]`; o ciclo implementa entidades determinísticas com PV, ataque em alcance e reação a dano do jogador, sem mapa/rede. Novas habilidades W/E/R ficam para o próximo ciclo. R6 permanece **N/A**.

## 2026-11-23T16:11Z — fechamento do ciclo 053

**Sessão**: `20261123T155300Z-pnj-defensores` · **Título**: `insurgencia - pnjs defensores`. Roteiro **10/10 PASS** (`.specify/validations/pnj-defensores/evidence.md`); `./verify.sh` **ALL STAGES OK**, **128 testes run: 128 passed, 0 skipped**, `VERIFY_EXIT=0`; cinco gates locais PASS após bootstrap do baseline de `specs/053-pnj-defensores/converge.md` (`files=35 baselined=35`). Implementação: `Pnj`/`Mundo.pnjs`/`adicionar_pnj`/`aplicar_dano_a_pnj` com piso zero/`atacar_pnj` nível 1/`fase_ataque_pnjs` 8 dmg a cada 90 ticks em 2 m/hash/HUD `PNJs: N`/tecla `T`. Três RED corrigidos na fonte (não em expectativa): `texto_hud` <100 linhas, piso zero do dano e asserção da transição letal. R4 PASS, R7 `1.0.0-local` pronto, R6 N/A. CI/merge/promoção pendentes nesta entrada; captura visual real **[NV]**.

## 2026-11-23T16:55Z — decisão 80/20 do ciclo 054 (antes da implementação)

**Sessão**: `20261123T165500Z-analise-materialista` · **Título**: `insurgencia - análise materialista`. Critério `impacto × confiança × facilidade ÷ risco`, escala 1–5, denominador **625**.

| Candidato | I | C | F | R | Pontuação |
|:--|--:|--:|--:|--:|--:|
| Análise Materialista (E do Intelectual, marca em PNJ) | 4 | 2 | 2 | 3 | **48/625** |
| Demais W/E/R (Escudo, Carga, Oficina, Fuga etc.) | 4 | 2 | 1 | 3 | 24/625 |
| Rede (bloqueada por ADR-004) | 5 | 2 | 1 | 1 | 10/625 |

**Escolha**: `analise-materialista`. GDD §10.2:554-559 fixa E do Intelectual com nível 3, 20 Fôlego, 12 m, -25% de resistência por 5 s e não-acumulação; o corte implementa apenas a marca determinística em PNJs vivos, reaproveitando `atacar_pnj`, `DANO_ATAQUE`, `BTreeMap` e o relógio por tick, sem magia, RNG, canalização, visibilidade, mapa, protocolo ou rede. Todos os números são `[HIPÓTESE]`/protótipo. Novas habilidades W/R e as demais E ficam para ciclos posteriores. R6 permanece **N/A**.

## 2026-11-23T17:20Z — fechamento do ciclo 054

**Sessão**: `20261123T165500Z-analise-materialista` · **Título**: `insurgencia - análise materialista`. Roteiro **10/10 PASS** (`.specify/validations/analise-materialista/evidence.md`); `./verify.sh` **ALL STAGES OK**, **136 testes run: 136 passed, 0 skipped**, `VERIFY_EXIT=0`; cinco gates locais PASS após bootstrap do baseline (`files=36 baselined=36`). Implementação: `marca_ticks`/`analise_recarga_ticks`, cinco constantes `[HIPÓTESE]`, ação nível 3 com 20 Fôlego/12 m/150 ticks/450 ticks, bônus 10→12, hash, HUD `Análise: N` e tecla `E`. Três RED corrigidos na fonte (hash, helpers de validação e gate R6). R4 PASS, R7 `1.0.0-local`, R6 N/A. CI/merge/promoção pendentes nesta entrada; captura visual real **[NV]**.

## 2026-11-23T21:58Z — auditoria da realidade e backlog de fechamento (ciclo 055)

**Sessão**: `20261123T215500Z-escudo-milicia` · **Título**: `insurgencia - escudo de milicia`. **FONTE_FACTORY** `b1deab1` (`origin/dev` `1791705` [NV] capturado via `.git`; 31 paths sujos na factory — nada transplantado). **ALVO** `origin/dev=3f3b01c`, `origin/main=2acf9e5`, `HEAD` local sujo `990ab06` preservado. Worktree nova `feature/055-escudo-milicia` a partir de `origin/dev`.

**[EV] Autoridade**: `AGENTS.md:42,60,71` e constituição v1.2.2 `:66-68` fixam `./verify.sh` como gate, GitHub Actions (`gates`/`rust`, `strict=true`) como CI, `gh` como CLI e merge autônomo apenas em `dev`; promoção `dev→main` e deploy são SWE-exclusivos. Sem job de deploy em `.github/workflows/ci-gates.yml`. Superfície CLI/headless; UI Bevy sempre `--minimizado`. `check_production_gate.py`/Forgejo/`fj` da factory são N/A no ALVO.

**[EV] Fila 0.0**: `gh pr list --state open` → 0 PRs abertos; nenhum PR para `dev`/`main` a drenar ou babysitar. Worktrees/branches dos ciclos 053/054 estavam mergeados (PRs #65/#66/#68/#69 MERGED via `gh pr view`) e foram removidos com manifesto no ledger às 21:56Z. Stashes: 0. Ledger sem `active` alheio; único trabalho preservado é o checkout compartilhado sujo (31 paths não versionados/alterados), intocável.

**[EV] Gates locais no baseline** (21:58Z, HEAD `3f3b01c`): SDD completeness PASS (`spec_dirs=53 gaps=0`), DAG PASS (`files=53 tasks=576 edges=260 cycles=0`), task-id PASS (`tasks_files=53 duplicates=0`), converge PASS (`files=36 baselined=36`), hub docs PASS (`0 errors, 0 warnings`). Matriz SDD: 53 specs, 38/38 spec/plan/tasks/checklist no checkout antigo; 53 validations no head vigente; `analyze` 13, `converge` 21; `evidence.md` só nos ciclos finais. `feature.json` corrente aponta `specs/055-escudo-milicia`.

### backlog_de_fechamento (baseline único do ciclo 055)

| ID | Descrição | Tipo | Evidência | Próximo passo |
|:--|:--|:--|:--|:--|
| BF-001 | Rede autoritativa real (Lightyear ou emenda de ADR-004); sem isso não há partida entre clientes | autorização | ADR-004; sem `lightyear` em Cargo; ledger ciclo 043 `blocked` | SWE decide stack; este agente não implementa |
| BF-002a | Escudo de Milícia (W do Guerreiro): absorção determinística de 250 por 5 s, custo 25, alcance 3 m, nível 2 | implementação | GDD §10.6:687-692; nenhuma ação `escudo_milicia` em `ACOES_PERMITIDAS` | Implementar no Núcleo + cliente + 5 testes neste ciclo |
| BF-002b..v | Demais habilidades Q/W/E/R ainda não nomeadas (21 restantes) | implementação | GDD §10.2–§10.8; apenas Q, Propaganda e E da Intelectual existem | Ciclos dedicados sequenciais |
| BF-003 | PVE com mapa/rotas e spawn por ilha (hoje 4 PNJs em posições fixas) | implementação | specs/053 `Out of scope` | Módulo de mapa/rotas determinístico |
| BF-004 | Duas partidas consecutivas na stack Rust, uma com cliente modificado | capacidade | GDD §16.6; evidência só Godot legada [NV] | Orquestrador/SWE executar roteiro ao vivo |
| BF-005 | Recalibrar métricas com 20 partidas reais (as atuais são sintéticas) | implementação | `docs/evidencias/metricas-20-partidas-2026-11-21.md` | Coletar eventos reais e revisar metas |
| BF-006 | Deploy/publicação configurado e disparado por `main` | capacidade+autorização | CI sem job de deploy; SWE-exclusivo | SWE configura infra e dispara |
| BF-007 | R6 executável (adaptador/resolve-tasks) | capacidade | constituição v1.2.2 R6 fora da sequência | Criar adaptador e ratificar emenda; PARE se invocada antes |
| BF-008 | Rastreabilidade 053/054 fechada (tasks marcadas, release promovida) | implementação | ledger PRs #65/#66/#69/#70 merged | Já corrigido em `d7f4103`; manter como fechado |
| BF-009 | Infra remota/federação (Postgres/Coolify/IAM/pgmq) | autorização | OBJETIVO/ROADMAP; credenciais [NV] | SWE fornece topologia e credenciais |
| BF-010 | Assinatura GPG e Fase 5 pós-MVP | autorização+implementação | PROGRESSO; GDD §16.6 | SWE autoriza chave; ciclo documental depois |

**[NV]**: CI remoto em tempo real, deploy, credenciais, partidas jogadas, captura visual real. **[INF]**: consequências de 5v5/lobby/NAT e contagem exata das 21 habilidades restantes deduzidas do GDD.

## 2026-11-23T22:00Z — decisão 80/20 e DoD do ciclo 055 (antes da implementação)

**Sessão**: `20261123T215500Z-escudo-milicia` · **Título**: `insurgencia - escudo de milicia`. Critério normativo: `impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹`, escala 1–5, denominador **3125**. Filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo.

| Candidato | I | C | Fch | E⁻¹ | R⁻¹ | Pontuação | Consumidor |
|:--|--:|--:|--:|--:|--:|--:|:--|
| `escudo-milicia` (BF-002a) | 4 | 4 | 5 | 3 | 4 | **960/3125** | jogador humano no cliente + sim determinístico |
| Mapa/rotas de PVE (BF-003) | 4 | 3 | 3 | 2 | 3 | 216/3125 | jogador/servidor |
| Rede real (BF-001) | 5 | 2 | 5 | 1 | 1 | 50/3125 — **inelegível** (autorização ADR-004) | SWE decide |
| Rastreabilidade 053/054 (BF-008) | 2 | 5 | 0 | 5 | 5 | 0 — **inelegível** (já fechado; governança) | nenhum |

**Escolha**: `escudo-milicia`. Âncora GDD §10.6:687-692 (W do Guerreiro, `[HIPÓTESE]`): 25 Fôlego, 3 m, 250 de absorção por 5 s (150 ticks), recarga 14 s (420 ticks), um alvo. O consumidor é o jogador do cliente standalone, que passa a ver `Escudo: N` e a sobreviver ao dano de PNJ/PvP de forma observável; o segundo consumidor é o próprio Núcleo determinístico, que passa a ter uma rota única de dano ao jogador com absorção. Nenhuma mudança de stack, protocolo, CI ou rede.

**DoD (0.9)**: (1) comportamento final — ação `escudo` nível 2, custo 25 Fôlego, alcance 3 m, absorção de até 250 em PvP e dano de PNJ por 150 ticks, recarga 420 ticks, sem acumulação em dois alvos, campos no hash e linha `Escudo:` no HUD; (2) consumidores nomeados — cliente Bevy (`--minimizado`) e testes de integração da sim; (3) comandos que provam — `./verify.sh` (esperado **141 testes**), cinco gates locais, roteiro 10/10 em `.specify/validations/escudo-milicia/`, CI `gates`+`rust` no head do PR; (4) backlog — **BF-002a passa a `fechado`** com evidência, sem reabrir sem regressão provada.

**R6**: `taskstoissues` permanece **N/A/bloqueado** pela constituição v1.2.2 (sem `resolve-tasks.sh`/adaptador). Não será invocado, simulado nem improvisado com `gh`; registrado como BF-007 e entregue ao SWE. **Promoção `dev→main` e deploy também são SWE-exclusivos**; este ciclo termina no escalonamento, não em merge em `main`.

## 2026-11-24T10:29Z — auditoria, backlog e decisão 80/20 do ciclo 056

**Sessão**: `20261124T102734Z-servidor-validacao` · **Título**: `insurgencia - validacao do servidor headless`. **FONTE_FACTORY** `b1deab1` (`origin/dev` `6bd3978` [EV]); ALVO `origin/dev=0bc2c25`, `origin/main=2acf9e5`. Fila 0.0: apenas PR #72 → `main`, GREEN em `gates`/`rust` no head `0bc2c25`, **não mesclado**; babysitter marcado `orphaned` por TTL vencido, escalonamento já entregue ao SWE. Worktrees 2 (compartilhada suja + esta), stashes 0; trabalho alheio untracked preservado. Gate autoritativo `./verify.sh`; CI GitHub Actions `gates`+`rust`/`strict=true`; CLI `gh`; superfície CLI/headless, sem browser-use.

**[EV] Auditoria**: 6 crates, 141 testes ancorados em `0bc2c25`. Núcleo com 13 ações em `ACOES_PERMITIDAS` (`sim:188-202`, constante privada), `ValidacaoComandos::validar` em `sim:2087` não chamada por nenhum binário, mapa de níveis em `sim:212-220`. Servidor headless (`servidor/src/main.rs:16-24`) com whitelist própria de 6 ações, parser aridade 4/6 e `exige_destino` hardcoded para 3 ações (`:83-116`), sem `ValidacaoComandos`; emite `<tick>\t<sha256>` antes de avançar (`:169-175`). `ACOES_PERMITIDAS` privada impede reuso direto. Riscos de compatibilidade mapeados: fixture `cli.rs:19` usa `coletar_materiais 1 1 0.5 0.5` inválido para o Núcleo; `mover` para `(3.5,-4)` excede 1 m; `ultima_recusa` entra no hash, então validação na fronteira não pode mutar estado.

### backlog_de_fechamento (baseline único do ciclo 056)

| ID | Descrição | Tipo | Evidência | Próximo passo |
|:--|:--|:--|:--|:--|
| BF-001 | Rede autoritativa real (ADR-004/Lightyear ausente) | autorização | ADR-004; sem `lightyear` em Cargo | SWE decide stack |
| BF-002a | Escudo de Milícia | implementação | PR #71 merged em `dev` | **fechado** |
| BF-002b..v | Demais habilidades Q/W/E/R | implementação | GDD §10.2–§10.8 | ciclos dedicados |
| BF-003 | Mapa/rotas e spawn por ilha | implementação | specs/053 `Out of scope` | módulo determinístico |
| BF-004 | Duas partidas consecutivas na stack Rust | capacidade | GDD §16.6 | executar roteiro ao vivo |
| BF-005 | Calibração com 20 partidas reais | implementação | métricas sintéticas atuais | coletar eventos reais |
| BF-006 | Deploy/publicação | capacidade+autorização | CI sem job de deploy | SWE configura |
| BF-007 | R6 executável | capacidade | constituição v1.2.2 | adaptador + emenda |
| BF-008 | Rastreabilidade 053/054 | implementação | PRs #65/#66/#69/#70 | **fechado** |
| BF-009 | Infra remota/federação | autorização | OBJETIVO/ROADMAP | SWE fornece topologia |
| BF-010 | GPG e Fase 5 | autorização+implementação | PROGRESSO/GDD §16.6 | SWE autoriza |
| **BF-011** | **Paridade de whitelist/validação do servidor headless** | **implementação/correção** | whitelist 6 vs 13; validador não chamado | **escolhido neste ciclo** |
| BF-012 | Cliente agenda `mover` em vez de injetar posição | implementação/correção | `sincronizar_sim` bypass | ciclo posterior (unit mismatch) |
| BF-013 | `contratos` consumido por binários | implementação | módulo desconectado | ciclo posterior |
| BF-014 | HUD republica `hash_estado` | implementação | cliente não chama hash | ciclo posterior |
| BF-015 | `sinalizar_*` com efeito em `fase_skills` | implementação | aceitos sem ramo | ciclo posterior |

### Decisão 80/20 — 2026-11-24T10:29Z

Critério `impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹`, denominador **3125**. Filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo.

| Candidato | I | C | Fch | E⁻¹ | R⁻¹ | Pontuação | Consumidor |
|:--|--:|--:|--:|--:|--:|--:|:--|
| `servidor-validacao` (BF-011) | 4 | 5 | 4 | 4 | 4 | **1280/3125** | usuário/automação do CLI headless |
| HUD `hash_estado` (BF-014) | 3 | 5 | 3 | 5 | 5 | 1125/3125 | jogador no cliente |
| Cliente `mover` (BF-012) | 5 | 3 | 4 | 2 | 2 | 240/3125 | jogador no cliente |
| Contratos consumidos (BF-013) | 5 | 2 | 5 | 1 | 2 | 100/3125 | protocolo/servidor |
| Rede (BF-001) | 5 | 2 | 5 | 1 | 1 | 50/3125 — **inelegível** (autorização) | SWE decide |

**Escolha**: `servidor-validacao`. O CLI headless passa a aceitar as 13 ações canônicas, deriva a exigência de `destino` do Núcleo e chama `ValidacaoComandos::validar` antes de agendar, com recusa determinística sem mutar `ultima_recusa`. Nenhuma mudança de protocolo, Cargo, CI, stack ou rede.

**DoD (0.9)**: (1) comportamento final — whitelist única derivada de `ACOES_PERMITIDAS`, `acao_exige_destino` pública no sim, validação no servidor antes do agendamento, recusa `linha N: <CodigoRecusa>` com exit 2/stdout vazia, formato `<tick>\t<sha256>` e hash inicial preservados; (2) consumidores nomeados — usuário/automação do CLI `insurgencia-servidor` e testes `crates/servidor/tests/cli.rs`; (3) comandos que provam — `cargo test -p insurgencia-servidor --test cli`, sondas `printf ... | cargo run -p insurgencia-servidor -- --ticks N`, `./verify.sh` remoto `gates`+`rust`; (4) backlog — **BF-011 passa a `fechado`**.

**R6**: N/A/bloqueado (BF-007); não será invocada, simulada nem improvisada com `gh`. Promoção `dev→main` e deploy permanecem SWE-exclusivos.

## 2026-11-24T10:49Z — fechamento do ciclo 056

**Sessão**: `20261124T102734Z-servidor-validacao` · **Título**: `insurgencia - validacao do servidor headless`. Roteiro **10/10 PASS** (`.specify/validations/servidor-validacao/evidence.md`); `cargo fmt --all` OK, `cargo check -p insurgencia-sim --tests` EXIT 0, `cargo test -p insurgencia-sim --test simulacao` **97/97**, `cargo check -p insurgencia-servidor --tests` EXIT 0, `cargo test -p insurgencia-servidor --test cli` **10/10**, inventário **146**. Cinco gates locais PASS; converge `files=38 baselined=38`. Implementação: `ACOES_PERMITIDAS` pública e `acao_exige_destino` no sim; servidor com whitelist derivada, parser por 11/2, validação antes de agendar, recusa `linha N: <CodigoRecusa>` exit 2/stdout vazia, formato tick/hash preservado. Dois placeholders corrigidos e dois artefatos históricos restaurados byte a byte. R4 PASS, R7 `1.0.0-local`, R6 N/A/bloqueado. CI/merge em `dev` e promoção/deploy **[NV]**; promoção/deploy SWE-exclusivos.

## 2026-11-24T12:15Z — auditoria, backlog e decisão 80/20 do ciclo 057

**Sessão**: `20261124T121311Z-armadilha-improvisada` · **Título**: `insurgencia - armadilha improvisada`. **FONTE_FACTORY** `b1deab1` (`origin/dev` `daa21eb` [EV]); ALVO `origin/dev=661890f`, `origin/main=2acf9e5`, checkout local sujo `990ab06`. Fila 0.0: apenas PR #72 → `main`, `gates`/`rust` success no head `661890f`, **não mesclado**; babysitter anterior marcado `orphaned` por TTL. Worktrees 2 (compartilhada + esta), stashes 0. Gate `./verify.sh`, CI GitHub Actions `gates`/`rust`/`strict=true`, CLI `gh`, superfície CLI/headless. R6 bloqueada; promoção/deploy SWE-exclusivos.

**[EV] Auditoria**: 6 crates, 146 testes. Núcleo com 13 ações, 11 com destino, hash SHA-256 canônico e validação isolada no sim; servidor valida antes de agendar; cliente não chama validador e não agenda `mover`. Backlog vigente: BF-002a/BF-008/BF-011 fechados; BF-001/BF-006/BF-007/BF-009/BF-010 inelegíveis por autorização/capacidade; BF-004 depende de execução externa. BF-014 (HUD hash) teve scoring anterior 1125/3125, mas é diagnóstico, não muda comportamento de jogo; BF-012 (mover) tem risco de unidade e é correção de plumbing; BF-003 (mapa/rotas) é amplo. BF-002b é o primeiro incremento nomeado do elenco de habilidades com âncora canônica em GDD §10.3:588-593.

### Decisão 80/20 — 2026-11-24T12:15Z

Critério `impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹`, denominador **3125**. Filtro: comportamento observável + consumidor fora dos artefatos do ciclo.

| Candidato | I | C | Fch | E⁻¹ | R⁻¹ | Pontuação | Consumidor |
|:--|--:|--:|--:|--:|--:|--:|:--|
| **BF-002b `armadilha-improvisada`** | 5 | 4 | 4 | 3 | 3 | **720/3125** | jogador do cliente Bevy (Artesão) |
| BF-003 mapa/rotas | 5 | 3 | 3 | 2 | 3 | 270/3125 | jogador/servidor |
| BF-014 HUD `hash_estado` | 2 | 5 | 1 | 5 | 5 | 250/3125 | jogador (diagnóstico) |
| BF-012 cliente `mover` | 4 | 3 | 3 | 2 | 2 | 144/3125 | jogador (correção de plumbing) |
| BF-013 contratos consumidos | 5 | 2 | 5 | 1 | 2 | 100/3125 | protocolo/servidor |
| BF-001 rede | 5 | 2 | 5 | 1 | 1 | 50/3125 — **inelegível** | SWE decide |

**Escolha**: `armadilha-improvisada`. Reescalonamento de BF-014 de 1125 para 250 reflete que exibir hash não muda o jogo; BF-002b entrega habilidade material de controle de área, com efeito visível no cliente e consumidor jogador. Nenhuma mudança de stack, contrato, Cargo, CI, rede ou GDD canônico.

**DoD (0.9)**: (1) comportamento final — ação `armadilha` nível 3, custo 25 Fôlego + 5 Materiais, alcance 3 m, arme 30 ticks, raio 1 m, imobilização 60 ticks, duração 1800 ticks, máximo 3 por jogador, aliados imunes por `equipe`, hash cobrindo campos/estado, HUD `Armadilhas: N`, tecla `E` com despacho por `funcao` e bloqueio de movimento local enquanto imobilizado; (2) consumidor — jogador do Artesão no cliente Bevy e testes de integração da sim; (3) comandos que provam — `cargo test -p insurgencia-sim --test simulacao`, `cargo test -p insurgencia-cliente --lib`, `./verify.sh` remoto `gates`+`rust`, roteiro 10/10; (4) backlog — **BF-002b passa a `fechado`**.

**R6**: N/A/bloqueado (BF-007); não será invocada, simulada nem improvisada com `gh`. Promoção `dev→main` e deploy permanecem SWE-exclusivos.

## 2026-11-24T12:55Z — fechamento do ciclo 057

**Sessão**: `20261124T121311Z-armadilha-improvisada` · **Título**: `insurgencia - armadilha improvisada`. Roteiro **10/10 PASS** (`.specify/validations/armadilha-improvisada/evidence.md`); `cargo fmt --all` OK, `cargo check -p insurgencia-sim --tests` EXIT 0, `cargo test -p insurgencia-sim --test simulacao` **102/102**, `cargo check -p insurgencia-cliente` EXIT 0, inventário **151**. Cinco gates locais PASS; converge `files=39 baselined=39`. Implementação: 9 constantes, `Armadilha`, `Mundo.armadilhas`, `Jogador.equipe/recarga/imobilização`, ação `armadilha` nível 3, validação, cota de 3, arme 30, imobilização 60, duração 1800, bloqueio de `mover`, hash e HUD `Armadilhas: N` com `E` por `funcao`. Três RED corrigidos na fonte (hash, import e tipo do helper). R4 PASS, R7 `1.0.0-local`, R6 N/A/bloqueado. CI/merge em `dev` e promoção/deploy **[NV]**; promoção/deploy SWE-exclusivos.

## 2026-11-24 — ciclo 058 (mirror vitoria local): implementação e evidência

- **Sessão/escopo**: `insurgencia - mirror vitoria local`, BF-003a. Worktree `.specify/queue/worktrees/impl-058-mirror-vitoria/`, branch `feature/058-mirror-vitoria-local`, âncora `HEAD = origin/dev = 1855e5c`, constituição **v1.2.2**, R1 `go`. Nota **append-only**: nenhuma seção anterior reescrita, renumerada ou removida.
- **Decisão 80/20**: **270/3125** (`5×3×3×2×3`, impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵) para o espelho local BF-003a, contra 120/3125 (semente duplicada no cliente) e 75/3125 (HUD/`hash_estado` sem semear). Consumidores: jogador do cliente Bevy e testes de integração da sim. GDD:275 permanece **`[HIPÓTESE]`** (3 nós, 100, −30, −10, `TICKS_PROPAGANDA_NO = 600`); nenhuma regra canônica promovida ou reescrita.
- **DoD-1..DoD-4**: (1) espelho com **4 ilhas (ids 1..=4)**, **1 súdito (id 1 em `(0.0, 236.0)`)** e **3 nós**; (2) vitória local por conversão dos 3 nós via `propaganda_no`, pela rota de `tres_nos_geram_vitoria`; (3) `hash_estado()` muda ao semear e é determinístico; (4) **BF-003a fechado** e **BF-003b (mapa completo de 12 ilhas, topologia/spawning/rotas) aberto**.
- **Arquivos implementados (4)**: `crates/sim/src/lib.rs:567-585` (`POSICOES_NOS_LOCAIS`, `semear_espelho_local`), `crates/cliente/src/main.rs:597` (chamada única em `configurar` antes de `insert_resource(Simulacao(sim))`), `crates/sim/tests/simulacao.rs:4233` (`espelho_local_seed_viabiliza_vitoria_e_reflete_hash`; `tres_nos_geram_vitoria` em `:2558` preservado) e `.specify/feature.json`. Nenhuma mudança em contratos, `PROTOCOL_VERSION`, Cargo, GDD, CI, stack ou rede.
- **Comandos/saídas observados pelo orquestrador (verbatim)**: `cargo test --locked -p insurgencia-sim --test simulacao` → **`103 passed (1 suite, 0.28s)`**; `cargo test --locked -p insurgencia-servidor --test cli` → **`10 passed (1 suite, 0.11s)`**; `cargo check --locked -p insurgencia-sim --tests` → **`Finished dev profile ... in 14.09s`**; gates `work-order-dag PASS (57 files, 622 tasks, 302 edges, 0 cycles)`, `task-id PASS (57, 0 duplicates)`, `converge PASS (39 baselined/39)`, `hub-docs PASS (0 errors, 0 warnings)`. `check_sdd_completeness` falhava apenas pela ausência de `.specify/releases/mirror-vitoria-local/release.md`, agora escrito. `cargo fmt --all` executado pelo orquestrador; `--check` pendente de reexecução.
- **Artefatos do ciclo**: roteiro de **14 passos** em `.specify/validations/mirror-vitoria-local/test-roadmap.md`, saídas em `.specify/validations/mirror-vitoria-local/evidence.md`, **R4 aprovado condicionado** em `.specify/validations/mirror-vitoria-local/validate.md` (**antes** de R7), release local `1.0.0-local` em `.specify/releases/mirror-vitoria-local/release.md` e `specs/058-mirror-vitoria-local/converge.md` como **1ª entrada append-only** (0 `missing`/`contradicts`/`unrequested`; 2 `partial` de evidência). Rastreabilidade FR-001..FR-004 ↔ SC-001..SC-004 ↔ T001..T010 (cadeia canônica de `tasks.md`), `tasks` T008 antes de `analyze` T009 (READ-ONLY).
- **Governança**: **R6/taskstoissues N/A** sob v1.2.2 — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado. **PR #76 pertence a outro owner e não foi tocado**. Nenhum merge `dev→main`, promoção a `1.0.0` ou deploy realizado; rollback documentado = revert do único commit de feature, devolvendo HEAD a `1855e5c`, **não executado** por este agente.
- **[NV] remanescentes**: jobs remotos `gates`/`rust` no head do PR (somente o SWE confirma SHA/run; não afirmar verde sem leitura remota); `./verify.sh` completo e `cargo fmt --all -- --check`; recheck do SDD e inclusão do novo `converge.md` no baseline do gate; captura visual do HUD Bevy (build excedeu **900 s/15 min** e não há janela gráfica automatizável — SWE deve capturar em X11/GPU e anexar screenshot + carimbo a `evidence.md`); execução do rollback. Nenhum teste, gate, formatador, build, commit, push, PR, merge ou deploy executado por este agente de documentação.

## 2026-11-25T00:38:55Z — decisão 80/20 do ciclo 059 (mover validado no cliente)

**Sessão/título**: `insurgencia - mover validado no cliente`. Âncora [EV]: `origin/dev = 737b876` (PR #77 mesclado em `dev`), worktree limpa `.specify/queue/worktrees/cycle-059`, cinco gates PASS (`sdd 57/0`, `dag 57/622/302/0`, `task-id 0 dup`, `converge 40/40`, `hub 0 erros`). Fila `base=dev` vazia; único PR aberto é #76 (`dev→main`), de outro dono com babysitter ativo — intocável. FONTE_FACTORY capturada em `18be3fb3`; factory é fonte de ideias, não autoridade.

**Critério**: `impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹`, denominador 3125; filtro eliminatório exige comportamento observável e consumidor fora dos artefatos do ciclo. Ranking: **BF-012 144/3125** (`4×3×3×2×2`) — elegível, consumidor jogador/validação; BF-003b — o 270/3125 registrado incluía o BF-003a já fechado, e o restante depende de decisão canônica de topologia no GDD, portanto não elegível neste ciclo sem autorização; BF-013 100/3125 — **diagnóstico**, sem comportamento observável, eliminado; BF-014 250/3125 — eliminado por "exibir hash não muda o jogo".

**Escolha**: **BF-012 — WASD passa a gerar comandos `mover` validados no espelho `Simulacao(Mundo)`**, com conversão explícita pixels↔metros (`PIXELS_POR_METRO = 100`, protótipo `[HIPÓTESE]`, nenhuma regra canônica alterada). `ValidacaoComandos::validar` decide aceite antes de `agendar_comando`; `seq`/`request_id` avançam somente no aceite; `sincronizar_sim` passa a derivar a posição local do espelho, não injetar posição. Escala 260 px/s ÷ 100 = 2,6 m/s ≈ 0,087 m/tick, abaixo de `LIMITE_MOVIMENTO_M = 1.0`.

**DoD (0.9)**: (1) comportamento — avatar/HUD só avançam quando o núcleo aceita `mover`; recusa preserva posição e monotônicos; imobilização bloqueia movimento autoritativo; (2) consumidor — jogador do cliente Bevy e validação da sim/CLI; (3) comandos que provam — `cargo test --locked -p insurgencia-cliente --lib`, `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo check --locked -p insurgencia-cliente --all-targets`, `./verify.sh` remoto (`gates`/`rust`); (4) backlog — **BF-012 passa a `fechado`**; BF-003b e BF-013 permanecem abertos.

**Governança/R6**: constituição v1.2.2 mantém R6 fora da sequência executável até existir adaptador; nenhuma issue será simulada. PR #76 e deploy são SWE-exclusivos. Captura visual Bevy permanece **[NV]** (sem janela automatizável); venue para compilação pesada é GitHub Actions.
