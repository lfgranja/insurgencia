# Release (R7) — baseline-governanca

- **Versão**: 1.0.0 (SemVer)
- **Data**: 2026-11-19
- **Feature**: `specs/003-baseline-governanca`
- **R4**: registrado em `.specify/validations/baseline-governanca/validate.md` antes deste release
- **Branch**: `feature/003-baseline-governanca` (HEAD base `a5e4131`) — sem merge, sem auto-merge, sem force-push
- **Remote/CI/PR/proteções**: `[NV]` até confirmação ao vivo via `gh`

## Changelog (1.0.0)

| ID | Mudança | Origem |
|---|---|---|
| FR-001 | Baseline de convergência materializado em `docs/governance/converge-append-only-baseline.json` (`schema_version="087.1.0"`, `enforce_unbaselined=true`, `files={}`) | `spec.md`; contrato §2.1 |
| FR-002 | Baseline de completude materializado em `docs/governance/sdd-completeness-baseline.json` (`allow_missing={}`) derivado do inventário local | `spec.md`; contrato §2.2 |
| FR-003 | Baseline de unicidade materializado em `docs/governance/task-id-uniqueness-baseline.json` (`allow_duplicates={}`) derivado do inventário local | `spec.md`; contrato §2.3 |
| FR-004 | Campo `_provenance` com origem `alvo-local`, método, data e `factory_utilizada=false` nos três JSON, ignorado pelos gates | os três JSON; `data-model.md` |
| FR-005 | Zero chaves de spec da factory, zero chaves órfãs nos três baselines | verificação T025/T026 |
| FR-006–FR-008 | Três gates retornam PASS (exit 0) com zero regressões e zero `stale_baseline` | contagens/exit codes em `## Evidência final` de R4 |
| FR-009–FR-010 | Mapas vazios honestos aceitos; gap legítimo restrito a portaria não executada, sem entradas inventadas | `spec.md` §Clarifications; contrato §4.3 |
| FR-011 | Nenhuma lógica, limiar, schema, caminho padrão ou regex dos três scripts alterado | `sha256sum` vs. contrato §8.1 |
| FR-012 | Saída executada com data/hora colada em R4; efeito remoto rotulado `[NV]` | `.specify/validations/baseline-governanca/validate.md` |
| T001–T036 | 36 tarefas de materialização, bootstrap, verificação e auditoria concluídas (`[X]`) | `specs/003-baseline-governanca/tasks.md` |
| SC-001–SC-008 | Conformidade registrada no R4; RG-2/RG-3/RG-4 executáveis, RG-1/RG-5 CONDICIONAIS não executados | R4 `## Conformidade verificada` e `## Ressalvas` |
| DEC-R1, DIV-1 | Veredito `go` (opção A) e dívida 1 rastreáveis na proveniência e nos registros locais | `.specify/assessments/baseline-governanca/decision.md`; `.specify/memory/STATE.md` |
| BUG-### | Nenhum bug reportado neste ciclo (formato reservado no contrato §5.2/§5.3) | `contracts/baseline-bootstrap-contract.md` |

## Entregáveis versionados

- `docs/governance/sdd-completeness-baseline.json` — `allow_missing={}`, `_provenance` alvo-local.
- `docs/governance/converge-append-only-baseline.json` — `schema_version="087.1.0"`, `enforce_unbaselined=true`, `files={}`, `_provenance` alvo-local.
- `docs/governance/task-id-uniqueness-baseline.json` — `allow_duplicates={}`, `_provenance` alvo-local.
- `.specify/validations/baseline-governanca/validate.md` — R4 com `## Evidência final`.

## Plano de rollback (documental)

Procedimento do contrato §5.3, aplicável exclusivamente aos três JSON versionados:

1. Registrar o achado com IDs FR e, quando existirem, `T###` e `BUG-###` (gatilhos do §5.3:
   chave da factory/órfã, regressão não recusada, `stale` persistente, alteração indevida de script,
   dívida convertida em anuência sem registro datado ou afirmação remota sem confirmação ao vivo).
2. Criar novo commit que restaure o JSON anterior do baseline afetado, com nova versão SemVer e
   data; nunca apagar ou reescrever o histórico.
3. Reconfirmar os outros dois baselines intactos e os três scripts inalterados (comparação textual
   contra os digests do contrato §8.1).
4. Reexecutar o gate afetado e conferir zero achados na saída colada em R4.
5. Registrar entrada no changelog com versão, data e IDs FR/`T###`/`BUG-###`.

**Exemplo de reversão**: restauração de `docs/governance/sdd-completeness-baseline.json` à versão
anterior, preservando o histórico e a versão atual como registro auditável.

**Proibido**: apagar histórico, `force-push`, merge/auto-merge, alterar scripts, copiar da factory,
reescrever `converge.md` ou afirmar CI/merge/aprovação obtidos. **Não aplicável**: rollback de
runtime, banco, migração, binário, configuração de CI ou serviço externo — a feature produz somente
três JSON versionados.

## Restrições

- Nenhum merge, auto-merge, force-push ou deleção de branch executada.
- Nenhuma afirmação de CI/check verde remoto; `[NV]` até confirmação ao vivo.
- Nenhum código, dependência, engine, serviço externo ou emenda da norma externa introduzido.
- Os três scripts de `scripts/ci/` permanecem byte-idênticos; nenhum `converge.md` foi reescrito.
