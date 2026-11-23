# Decision — Coleta determinística de materiais

**Data**: 2026-11-23T01:55Z · **R1**: `go` · **R3**: 0 clarificações

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | GDD §7.1 define +3/+2 em 6 s e saída zera parcial, mas o sim autoritativo não implementa coleta nem a reflete no hash. |
| Força da evidência | strong | GDD §7.1 e `crates/sim/src/lib.rs` lidos; contrato técnico auditado pelo orquestrador; constituição v1.2.2 lida. |
| Valor vs. inação | strong | 180/625: fila documental esgotada; menor incremento de produto com valor canônico e regra verificável. |
| Viabilidade | strong | Somente 14 artefatos documentais neste agente; sem dependência de ambiente, rede ou credencial. |
| Alinhamento estratégico | strong | Fase 1 do pulso de 30 s e Pilar de Ação Coletiva/Materialismo Histórico; determinismo preservado. |
| Postura de risco | strong | Sem mudança de protocolo e sem dano/barricada/GUI; riscos mitigados por validação, crédito atômico e hash refletivo no contrato registrado. |

**Veredito: go** — Opção 1 (artefatos SDD da coleta determinística). 80/20: 5×4×3×3 = 180/625, contra 375/625 da documentação já esgotada. R1 `go` com scorecard 6/6 strong e R3 = 0.

Condições: (1) regra canônica e contrato registrados com fonte (GDD §7.1; `lib.rs`; auditoria do orquestrador), com **[NV]** para qualquer execução; (2) fronteira restrita a `crates/sim/src/lib.rs` na implementação futura — nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, GDScript, GUI, rede, dano ou barricada; `Comando.destino` como ID do ponto; (3) constituição v1.2.2 aplicada: `... 09 checklist → revisão → 10 tasks → 11 analyze (READ-ONLY) → 12 implement → 13 converge → 27 validate → 26 release`, R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only, **R6 suspensa por decisão ratificada** — PARE se invocada, nenhuma issue simulada; (4) `test-roadmap.md` antes do PR com 10 passos, capturas de terminal, coluna `automatizável?`, rollback e os seis casos exigidos (180 ticks, saída antes, recusa sem destino, hash divergente, ausência de consumo parcial, determinismo); (5) exatamente os 14 artefatos da fronteira, sem tocar `.specify/memory/STATE.md`, `converge.md` alheio ou a alteração preexistente em `.specify/feature.json`; (6) nenhum build/gate/formatador/teste/commit/push/PR/merge/deploy executado por este agente.
