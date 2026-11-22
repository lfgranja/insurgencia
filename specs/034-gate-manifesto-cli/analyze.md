# Analyze (READ-ONLY) — 034-gate-manifesto-cli

**Data:** 2026-11-22T16:54:17Z · **Escopo:** spec, plano, checklist e tasks — nenhuma edição de artefato ou código nesta etapa.

## Achados

| # | Achado | Severidade | Correção na fonte |
|:--|:--|:--|:--|
| A1 | Risco de duplicar a validação no CLI, criando segunda autoridade para os 5 códigos e a ordem de 011 | alta | FR-004: `conferir` é a única autoridade; o adaptador apenas traduz `ErroCompatibilidade::codigo()` |
| A2 | Tentação de calcular o hash dentro da biblioteca (como faz o GDScript) violaria a Constituição II | alta | FR-002/FR-003: SHA-256 e hex ficam no exemplo; `hash_regras: Option<&str>` continua na fronteira do contrato |
| A3 | Conflito aparente: "leitura falha → exit 2" versus "arquivo de regras ausente → REGRAS_AUSENTES" | alta | FR-002 + Clarificações: `NotFound` nas regras é condição de domínio mapeada a `None`; demais falhas de leitura são exit 2 |
| A4 | "Manifesto ausente" pode significar arquivo inexistente ou campos ausentes no JSON | média | Clarificações + roteiro: JSON malformado/campos ausentes → `MANIFEST_INVALID`; caminho inexistente → exit 2 |
| A5 | Novo `converge.md` não está no baseline append-only (16 arquivos em `16:54:17Z`) | média | Fora da fronteira: orquestrador deve bootstrapar o baseline; registrado como **[NV]** em validate/converge, sem reexecução de gate aqui |
| A6 | Uso de `.unwrap()`/`.expect()` em conversão de argumentos causaria pânico | alta | FR-001/FR-005: `args_os()`, `into_string()`, `parse::<u8>()` com `map_err`, `main() -> ExitCode` |
| A7 | Novo arquivo de exemplo poderia arrastar dependências (`clap`, `hex`) | média | FR-003/FR-005: apenas `sha2`, `thiserror` e `insurgencia-contratos`, todos já fixados em `crates/sim/Cargo.toml` |
| A8 | Evidências locais e remotas poderiam ser reexecutadas por iniciativa do agente | média | Pressupostos: colar `ALL STAGES OK`/41 de 41, 5 gates PASS e CI run `44952738240` como observados; **[NV]** para o roteiro |

## Convergência

Não há conflito entre spec, plano, checklist e tasks após as correções A1–A4 registradas nas fontes. Nenhum marcador `[NEEDS CLARIFICATION]` em aberto. R1 `go` presente com scorecard 6/6 strong. As evidências de `2026-11-22T16:54:17Z` e a CI run `44952738240` são suficientes e não devem ser reexecutadas. R4 precede R7; release `1.0.0` com rollback documentado (nunca executado). Nenhuma mudança de código foi feita nesta etapa (READ-ONLY). O único item pendente fora de fronteira — bootstrap do baseline de converge — é responsabilidade do orquestrador.

## Veredito

Prosseguir para `tasks` e, depois, para `implement`, com A1–A8 tratados como requisitos de adaptador fino e governança de fronteira. Factory `b1deab1` permanece somente fonte; `docs/GDD*`, baselines, `.gitignore`, `STATE.md`, `PROGRESSO.md`, ledger, CI e 193 paths untracked alheios intactos. `./verify.sh` e o roteiro de testes permanecem **[NV]** até execução observada. PASS.
