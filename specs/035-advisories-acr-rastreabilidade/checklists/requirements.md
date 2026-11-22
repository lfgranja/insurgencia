# Checklist de Requisitos — 035-advisories-acr-rastreabilidade

- [x] FR-001 10 advisories ACR (#39/#40/#41, comentários `7082836571`/`7082907820`/`7082916777`) rastreados e fechados, sem blockers; contrato 011/033 intacto
- [x] FR-002 `STATE.md` com 34 specs / 237 assessments / 34 validations / 34 releases / 0 bugs / 13 research e âncora coerente com `feature.json`
- [x] FR-003 `evidence.md` com cinco execuções/capturas e converge baselinado `17/17` (não "pendente")
- [x] FR-004 artefatos 034 registram `.gitignore:5-6` com `/.specify/queue/` (regra legítima preservada)
- [x] FR-005 CLI imprime hash minúsculo calculado (`hash_regras`) em `COMPAT_OK`; aceita `content_hash` maiúsculo via `eq_ignore_ascii_case`; exit 0/1/2, higiene Rust, `crates/contratos` intacto
- [x] FR-006 14 artefatos SDD, `analyze` READ-ONLY antes de `tasks`, `converge` append-only, R4 antes de R7 `1.0.0`, changelog FR/T, rollback
- [x] FR-007 `test-roadmap.md` antes do PR com escopo/FR/T, pré-condições, passos numerados, superfície CLI/docs, capturas, coluna `automatizável?`, rollback e passo do `content_hash` maiúsculo → hash minúsculo canônico
- [x] FR-008 fronteira respeitada: nenhum build/gate/commit/push/PR, nenhum merge em `main`, factory fonte, 194 paths untracked e CI intactos, **[NV]** preservado

## Revisão
Requisitos verificáveis por inspeção das âncoras e do roteiro de testes. Checklist `[x]` = revisão de requisitos, **não** execução de build/gate/teste nem aplicação das correções na fonte. Conflito identificado e corrigido na fonte: "aceitar `content_hash` maiúsculo" (contrato, `eq_ignore_ascii_case`) versus "imprimir hash minúsculo canônico" (adaptador, `hash_regras`) resolve-se em FR-005 sem alterar `conferir`. As quatro divergências são factuais e foram confirmadas por leitura direta em `18:31–18:34Z`. A aplicação das correções e o bootstrap de baselines pertencem ao orquestrador/agente de código (fora da fronteira SDD). Aprovado para `analyze` e, depois, para `tasks`. Factory `b1deab1` somente fonte; `docs/GDD*`, `.gitignore` (regra preservada), ledger e CI intactos. Nenhum build/gate/commit executado; PR #40 permanece `OPEN` na fila SWE.
