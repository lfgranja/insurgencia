# Idea Intake: Baselines locais de governança

- **Slug**: baseline-governanca
- **Created**: 2026-11-19
- **Source**: repo path — `.specify/memory/STATE.md` (dívida 1), saídas dos gates em `.specify/memory/STATE.md`/`docs/PROGRESSO.md` e formatos em `scripts/ci/check_{sdd_completeness,converge_append_only,task_id_uniqueness}.py`
- **Type**: improvement

## Idea (as captured)

> "Materializar baselines LOCAIS de governança (sdd-completeness, converge-append-only, task-id-uniqueness) a partir do estado atual do alvo — nunca copiar da factory — para que os gates deixem de ser 'verdes vazios' (baselined=0) e passem a detectar regressão. Evidência: dívida 1 do STATE, saídas dos gates com baselined=0, formatos nos scripts locais."

Nenhuma URL foi fornecida ou buscada nesta captura; nenhum segredo apareceu na entrada.

## Restated

Propõe-se que os três checks determinísticos de governança do alvo passem a operar com baselines derivados do próprio estado local, em vez de rodarem sem baseline aprovado. A finalidade declarada é tornar a saída "PASS" dos gates uma afirmação auditável de não-regressão, e não apenas um resultado vazio.

## Origin & Context

- **Raised by**: registrado como dívida 1 em `.specify/memory/STATE.md:50` (`docs/governance/sdd-completeness-baseline.json` não materializado, com a ressalva de evitar chaves de specs da factory). Reaparece como candidato 3 no ranqueamento 80/20 de 2026-11-19T22:11:07Z (score 30,0) — `docs/governance/80-20-ranking-2026-11-19.md:33`. [NEEDS CLARIFICATION: autoria humana original da dívida 1]
- **Trigger**: os quatro checkers SDD do alvo registram PASS com `baselined=0` e nenhum arquivo de baseline existe em `docs/governance/`; `docs/PROGRESSO.md:96` classifica "verdes com baselined=0/edges=0/files=0" como parte do que **não** funciona na auditoria pós-ciclo-001.

## First-Glance Unknowns

- [NEEDS CLARIFICATION: qual formato de baseline cada um dos três scripts consome e quais caminhos são considerados autoritativos no alvo?]
- [NEEDS CLARIFICATION: `baselined=0` decorre de arquivo de baseline ausente, de estado local sem gaps/arquivos/duplicatas, ou de ambos?]
- [NEEDS CLARIFICATION: o que exatamente conta como "detectar regressão" para cada gate e como isso será evidenciado antes de R4?]
- [NEEDS CLARIFICATION: como derivar o baseline do alvo sem herdar chaves de specs da factory, que é fonte mas não autoridade?]
- [NEEDS CLARIFICATION: se o estado atual estiver limpo, um baseline com mapas vazios é aceitável como ponto de partida?]
