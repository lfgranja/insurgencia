# Fonte do aparato — MDMV Dark Factory

- **Registrado em:** 2026-11-19
- **Repositório fonte:** `/home/luis/development/MDMV/projetos/mdmv-dark-factory`
- **HEAD usado como fonte (limpo, via `git show`/`git archive`):** `b1deab15f482dd85871cd6c6a857ad710b89e2a6` (curto `b1deab1`)
- **Estado de trabalho da factory:** sujo (`AGENTS.md`, `docs/architecture/hub-architecture.html`,
  `docs/governance/decision-center-evidence-baseline.json` modificados; `specs/092-.../examples/context-budget-manifest.json` modificado).
  Esse estado **NÃO** foi transplantado. Nenhum arquivo da árvore de trabalho da factory foi lido como baseline.
- **Uso autorizado:** a factory é apenas fonte de ideias, scripts e convenções. Todo comando, gate,
  artefato SDD, branch e commit desta sessão é executado **no alvo** (`/home/luis/development/MDMV/projetos/insurgencia`).

## Decisão de governança do alvo (Bloco 0.a)

Antes desta sessão o alvo **não possuía** `AGENTS.md`, `.specify/memory/constitution.md`,
`.specify/feature.json`, `scripts/ci/`, `.specify/scripts/`, configuração de CI ou workflow.
Remote `origin` do alvo aponta para GitHub (`github.com/lfgranja/insurgencia.git`) — CLI de PR é `gh`.
A factory usa Forgejo, CLI `fj` e gate `check_production_gate.py --check` com policy
`docs/governance/production-gate-hub.policy.json`.

**Classificação:** divergência em provedor de CI, CLI de PR e definição de "gate de produção".
Portanto o gate da factory **não é autoridade no alvo** e não foi importado para "testar".
Ao materializar arquivos ausentes a partir do HEAD limpo acima, eles passam a existir **como
aparato do alvo**; a autoridade do gate no alvo decorre desta decisão local registrada, nunca da
factory. Nenhum status remoto, CI, merge ou proteção é afirmado sem marcação `[NV]`.

---
