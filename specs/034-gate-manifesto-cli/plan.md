# Implementation Plan — 034-gate-manifesto-cli

Ordem normativa: assess.intake → research → define → shape → decide (`go`) → specify → clarify → plan → checklist → revisar checklist → analyze (READ-ONLY) → tasks → implement → converge (append-only) → validate (R4) → release (R7).

1. Confirmar R1 `go` em `.specify/assessments/gate-manifesto-cli/decision.md` (scorecard 6/6 strong), com âncora em `specs/011`, ciclo 033 e GDD §1/§14.2.
2. Registrar intake, research, problem, concept e decision na pasta de assess, preservando as evidências datadas de `2026-11-22T16:54:17Z` e a CI run `44952738240` (`success`) no head `7329c53`.
3. Substituir o stub `spec.md` por FR-001..FR-006 e SC-001..SC-006; registrar as clarificações (manifesto com campos ausentes vs. caminho inexistente; regras ausentes como condição de domínio) e os pressupostos de execução pelo orquestrador.
4. Escrever `checklists/requirements.md` com um item por FR e a seção "Revisão" (checklist `[x]` = revisão de requisitos, não implementação).
5. Produzir `analyze.md` **READ-ONLY**, apontando conflitos e sua correção na fonte — em especial o mapeamento `NotFound`→`None`→`REGRAS_AUSENTES` em FR-002 e a necessidade de bootstrap do baseline de converge pelo orquestrador.
6. Escrever `tasks.md` com T-IDs únicos e arestas `Bloqueado por:` acíclicas, cobrindo todas as etapas normativas e o arquivo de código.
7. Implementar `crates/sim/examples/verificar_manifesto.rs` como adaptador: `args_os()` sem pânico, `fs::read_to_string`/`fs::read`, `Sha256::digest`, hex minúsculo manual, chamada a `conferir`, `main() -> ExitCode`, saídas `COMPAT_OK`/`COMPAT_FALHA` e exit codes 0/1/2. Sem tocar biblioteca, `Cargo.toml`, `Cargo.lock` ou qualquer outro arquivo de código.
8. Escrever `test-roadmap.md` **antes do PR**: escopo FR/T, pré-condições, passos numerados ação → resultado esperado, superfície CLI, plano de capturas de terminal/JSON (sem UI), coluna `automatizável?` e rollback. Cobrir os 8 casos exigidos, inclusive 3 execuções idênticas com `diff`.
9. Registrar `converge.md` como primeira entrada append-only datada `2026-11-22`, com evidências observadas, atendimento FR/SC e rollback documentado (nunca executado).
10. Aprovar R4 em `.specify/validations/gate-manifesto-cli/validate.md`, referenciando o roteiro e marcando **[NV]** tudo o que depende de execução pelo orquestrador; R4 precede R7.
11. Emitir R7 em `.specify/releases/gate-manifesto-cli/release.md` com versão `1.0.0`, changelog FR/T, rastreabilidade e rollback.
12. Encerrar sem build, clippy, teste, gate, commit, push ou PR; sem tocar factory `b1deab1`, `docs/GDD*`, baselines, `.gitignore`, `STATE.md`, `PROGRESSO.md`, ledger, CI, janelas, credenciais ou 193 paths untracked alheios.
