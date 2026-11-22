# Checklist de Requisitos — 034-gate-manifesto-cli

- [x] FR-001 assinatura `verificar_manifesto <manifesto.json> <regras.cfg> [module_id] [versao_local]`, defaults de `MODULO_ID`/`PROTOCOL_VERSION`, uso inválido → exit 2
- [x] FR-002 I/O apenas no adaptador; `NotFound` nas regras → `None` → `REGRAS_AUSENTES`; demais falhas de leitura → exit 2; biblioteca intacta
- [x] FR-003 SHA-256 com `sha2 = "=0.10.9"`, hex minúsculo manual, sem crate `hex`, sem dependência nova, `Cargo.toml`/`Cargo.lock` intactos
- [x] FR-004 `conferir` é a única autoridade; `COMPAT_OK` stdout/exit 0, `COMPAT_FALHA <codigo>` stderr/exit 1, ordem do contrato preservada
- [x] FR-005 higiene Rust: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`unsafe`/`async`, `max_width=100`, sem allow amplo
- [x] FR-006 SDD ordenado, 14 artefatos + 1 arquivo de código, roteiro com 8 casos, `analyze.md` READ-ONLY, `converge.md` append-only, R4 antes de R7 `1.0.0`, rollback documentado, **[NV]** onde não houve execução

## Revisão
Requisitos verificáveis por inspeção das âncoras e do roteiro de testes. Checklist `[x]` = revisão de requisitos, **não** execução de build/gate/teste. Conflito identificado e corrigido na fonte: "leitura falha → exit 2" versus "arquivo de regras ausente → `REGRAS_AUSENTES`" resolve-se em FR-002 com o mapeamento explícito de `NotFound` para `None`. O baseline de converge do novo arquivo não foi atualizado (fora de fronteira): cabe ao orquestrador fazê-lo. Aprovado para `analyze` e, depois, para `tasks`. Factory `b1deab1` somente fonte; `docs/GDD*`, baselines, CI e 193 paths untracked alheios intactos. Nenhum build/gate/commit executado.
