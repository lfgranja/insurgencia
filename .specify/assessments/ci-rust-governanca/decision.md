# Decision — CI Rust e governança alinhada

**Data**: 2026-11-21T04:26Z · **R1**: `go`

## Scorecard
| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | CI verde sem cobrir código, constituição exige CI confiável para merge autônomo; evidência executada (verify.sh OK vs YAML sem cargo). |
| Força da evidência | strong | Comandos executados nesta sessão + leitura direta de YAML/Cargo/AGENTS; auditoria independente concorda. |
| Valor vs. inação | strong | Sem correção, qualquer PR com Rust quebrado pode ser mesclado automaticamente. |
| Viabilidade / apetite | adequate | 1 arquivo YAML + 3 documentos; risco de install do nextest mitigado por fallback do próprio verify.sh. |
| Alinhamento estratégico | strong | Constituição art. CI/CD; ADR-004 Rust; Soberania (verificação honesta). |
| Postura de risco | adequate | Riscos conhecidos (tempo de CI, duplicação, autoridade documental) com mitigação na Opção 1. |

## Veredito
**go** (R1). Opção 1 — job `rust` no CI chamando `./verify.sh` (uma fonte de verdade), toolchain lida de `rust-toolchain.toml`, `cargo-nextest` via action com fallback do script. Documentação alinhada no mesmo ciclo (AGENTS.md, verify.sh:2, GDD:1627). Nenhuma afirmação de CI remoto verde após a mudança sem execução ao vivo **[NV]**.

## Handoff para specify
- Problema: CI não cobre código; autoridade documental obsoleta.
- Incluído: job Rust no CI; AGENTS.md; verify.sh header; GDD:1627.
- Excluído: provider, CLI, deploy, credenciais, itens 5-6, GPG, factory.
- Sucesso: CI roda verify.sh; docs coerentes; 5/5 gates + verify.sh PASS.
- Perguntas em aberto: nenhuma (R3 não necessário).

## Gate de entrada
HEAD `2587647`, branch `feature/028-ci-rust-governanca`, feature.json → `specs/028-ci-rust-governanca`. Gates: verify.sh ALL STAGES OK; 5/5 SDD PASS; pytest exit 5 (N/A). Factory fonte `b1deab15` não transplantada.
