# STATE.md — Insurgência

## Estado vivo — 2026-11-20T01:14:03Z (autorizações executadas até o limite seguro)

- **Atualizado em**: 2026-11-20T01:14:03Z — PRs #1–#11 mesclados em `dev`; HEAD remoto `05d38b8`. Âncora: `.specify/feature.json` → `specs/007-postgres-local-validacao`.
- **Repositório**: `visibility=PUBLIC` (00:26:47Z); CI `.github/workflows/ci-gates.yml` com execuções `success` e check `gates`; proteção de `dev` aplicada (`required_checks=["gates"]`, `strict=true`, sem force-push/deleção).
- **Godot 4.7.2**: instalada da release oficial com SHA512 de 128 hex idêntico ao `SHA512-SUMS.txt` (`CHECKSUM_OK`, 00:43:52Z); binário `4.7.2.stable.official.ed1daf0bf`. Benchmark headless sintético 600 ticks × 64 entidades: mediana 37,48 µs/tick em 7 amostras (mín 22,71; máx 42,95) — carga sintética, **não** 5v5 real.
- **Postgres**: validado localmente em container descartável `postgres:16-alpine` → 16.15; JSONB/GIN e privilégio mínimo provados; UPDATE negado ao papel da aplicação; container removido (0 remanescentes). Remote/Coolify/Zitadel/`pgmq` permanecem `[NV]`.
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
