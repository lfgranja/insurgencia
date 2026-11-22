# Implementation Plan — 035-advisories-acr-rastreabilidade

Ordem normativa: assess.intake → research → define → shape → decide (`go`) → specify → clarify → plan → checklist → revisar checklist → analyze (READ-ONLY) → tasks → implement → converge (append-only) → validate (R4) → release (R7).

1. Confirmar R1 `go` em `.specify/assessments/advisories-acr-rastreabilidade/decision.md` (scorecard 6/6 strong; 80/20 = 4×5×4×5 = 400/625 = 0,64), com âncora nos PRs #39/#40/#41, no ciclo 034 e no 80/20 registrado.
2. Registrar intake, research, problem, concept e decision na pasta de assess, preservando as evidências datadas (`18:31Z–18:34Z`), o `./verify.sh` de `18:32:17Z` e o PR #40 `OPEN` sem merge.
3. Substituir o stub `spec.md` por FR-001..FR-008 e SC-001..SC-005; registrar as clarificações (fechar ≠ redefinir contrato; `.gitignore` legítimo; hash canônico minúsculo) e os pressupostos de execução pelo orquestrador.
4. Escrever `checklists/requirements.md` com um item por FR e a seção "Revisão" (checklist `[x]` = revisão de requisitos, não implementação).
5. Produzir `analyze.md` **READ-ONLY**, apontando conflitos e sua correção na fonte — em especial a ordem `analyze` antes de `tasks`, a distinção "aceitar maiúsculas" vs. "imprimir minúsculas" (FR-005) e as quatro divergências factuais.
6. Escrever `tasks.md` com T-IDs únicos e arestas `Bloqueado por:` acíclicas, cobrindo todas as etapas normativas e separando claramente os artefatos SDD (neste agente) das correções na fonte (agente de código/orquestrador).
7. Escrever `test-roadmap.md` **antes do PR**: escopo FR/T, pré-condições, passos numerados ação → resultado esperado, superfície CLI/docs, plano de capturas de terminal/JSON (sem UI), coluna `automatizável?` e rollback. Incluir passo explícito: manifesto com `content_hash` maiúsculo aceito e hash minúsculo canônico impresso.
8. Registrar `converge.md` como primeira entrada append-only datada `2026-11-22`, com evidências observadas, atendimento FR/SC e rollback documentado (nunca executado); não editar `converge.md` de ciclos anteriores.
9. Aprovar R4 em `.specify/validations/advisories-acr-rastreabilidade/validate.md`, referenciando o roteiro e marcando **[NV]** tudo o que depende de execução pelo orquestrador; R4 precede R7.
10. Emitir R7 em `.specify/releases/advisories-acr-rastreabilidade/release.md` com versão `1.0.0`, changelog FR/T, rastreabilidade e rollback.
11. Encerrar sem build, clippy, teste, gate, commit, push ou PR; sem mesclar PR #40 em `main`; sem tocar factory `b1deab1`, `docs/GDD*`, `.gitignore`, `STATE.md`, `PROGRESSO.md`, ledger, CI, baselines pré-existentes, janelas, credenciais ou 194 paths untracked alheios.
12. Registrar como **[NV]** e de responsabilidade do orquestrador/agente de código: aplicar as quatro correções na fonte (STATE.md, evidence.md, afirmação de `.gitignore` em 034 e linha de saída do CLI), executar o roteiro e rodar gates/CI. Nada disso é feito neste ciclo.
