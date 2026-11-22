# Convergência — 034-gate-manifesto-cli

**Data**: 2026-11-22 · **Modo**: READ-ONLY, append-only. Primeira entrada do ciclo 034.

## Evidências (observadas — leitura de fontes locais e relatórios do orquestrador)

- `2026-11-22` — âncora lida: `specs/011-compatibilidade-manifesto/spec.md` FR-001..FR-006 define os 6 campos, a ordem formato → versão → módulo → 64 hex → regras → hash, os 5 códigos literais e as verificações de referência; `manifesto.json` traz `protocol_version=1`, `module_id="insurgencia.exemplo"` e `content_hash` de 64 hex; GDD §1 (soberania das regras) e §14.2 (handshake com versão, `module_id` e hash SHA-256) fundamentam o gate.
- `2026-11-22` — contrato disponível: `crates/contratos/src/manifesto.rs` expõe `validar_formato`, `conferir(json, modulo_id, versao_local, Option<&str>)`, `ErroCompatibilidade::codigo()` e `REGRAS_AUSENTES`; `lib.rs` expõe `MODULO_ID`, `PROTOCOL_VERSION` e os `REJEICAO_*`. O ciclo 033 entregou esse contrato sem I/O — esta é a autoridade que o CLI reutiliza.
- `2026-11-22T16:54:17Z` — `./verify.sh`: `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0`. Gates Python no mesmo instante: SDD `33/0`, DAG `296/0`, task-id `0 duplicatas`, converge `16/16`, hub-docs `0 erros` — reportados, não reexecutados.
- `2026-11-22` — CI GitHub Actions (contextos `gates` + `rust`) no head `7329c53`: run `44952738240`, `conclusion=success`, via `gh` — observada, não reexecutada.
- `2026-11-22` — factory `mdmv-dark-factory` HEAD `b1deab1` (origin/dev `d5d796f`) é **apenas fonte**, não autoridade; nada transplantado. 193 paths untracked alheios, `docs/GDD*`, baselines, `.gitignore`, `STATE.md`, `PROGRESSO.md`, ledger, CI, janelas e credenciais intactos.

## Atendimento
FR-001..FR-006 e SC-001..SC-006 atendidos na escrita: o adaptador `crates/sim/examples/verificar_manifesto.rs` concentra I/O, SHA-256 e saídas, enquanto `conferir` permanece puro, sem alterações em `crates/contratos`, `Cargo.toml`/`Cargo.lock` ou `manifesto.json`. `NotFound` nas regras é traduzido a `None` → `REGRAS_AUSENTES`; demais leituras e uso inválido são exit 2; recusas são exit 1 com o literal oficial. SDD ordenado R1 go (scorecard 6/6 strong) → R4 aprovado antes de R7 `1.0.0`; `analyze.md` READ-ONLY; checklist revisado; `test-roadmap.md` escrito antes do PR com 8 casos, capturas de terminal/JSON sem UI e 3 execuções idênticas por `diff`. Nenhum build, teste, gate, commit, push ou PR executado. O roteiro e qualquer verificação pós-mudança permanecem **[NV]** até o orquestrador, que também deve bootstrapar o baseline deste arquivo (nova entrada, fora da fronteira deste agente). Rollback documentado em `release.md`, nunca executado.

## Atualização append-only — 2026-11-22 (reservada ao orquestrador)
Nenhuma entrada adicional nesta entrega. Resultados do roteiro, `./verify.sh` pós-mudança, gates reexecutados e bootstrap do baseline devem ser **anexados** aqui pelo orquestrador, sem reescrever o prefixo acima.

### Execução pelo orquestrador — 2026-11-22T17:22Z

- **RED real corrigido**: `./verify.sh` às 17:06:29Z falhou em `crates/sim/examples/verificar_manifesto.rs` com `-D unused-imports` (`use std::ffi::OsString;`). Import removido na fonte; nenhum `allow` acrescentado.
- **Gate verde**: `./verify.sh` (17:12:43Z) → `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0`.
- **Roteiro 100% autônomo**: 10/10 PASS (17:21:57–58Z), capturas em `/tmp/tmp.t9J9ly2h8z/capturas/`, relatório em `.specify/validations/gate-manifesto-cli/evidence.md`. `evidence_status: passed`; nenhuma UI fingida.
- **Gates Python**: SDD `34/0`; DAG `314 tarefas/17 arestas/0 ciclos`; task-id `0 duplicatas`; hub-docs `0 erros`. Converge baselinado por bootstrap após esta entrada.
