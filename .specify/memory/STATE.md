# STATE.md — Insurgência

## Estado vivo — 2026-11-20T10:58Z (diretrizes da SE)

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
