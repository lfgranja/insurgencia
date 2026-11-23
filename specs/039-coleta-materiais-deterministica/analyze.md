# Analyze (READ-ONLY) — 039-coleta-materiais-deterministica

**Data:** 2026-11-23T01:56Z · **Escopo:** spec, plano e checklist — nenhuma edição de artefato, código, GDD, baseline ou documento-fonte nesta etapa. **Posição:** executa **DEPOIS** de `tasks`, conforme a constituição v1.2.2 do alvo (Princípio III).

## Achados

| # | Achado | Severidade | Correção na fonte |
|:--|:--|:--|:--|
| A1 | A regra do GDD poderia ser diluída para "algum tempo" ou recompensa aproximada, perdendo o critério canônico | alta | FR-001/US1: 6 s = 180 ticks a 1/30 s, exatamente +3 Suprimentos/+2 Materiais, com fonte GDD §7.1 |
| A2 | Crédito parcial ou acúmulo sobrevivente na saída antecipada violaria "sair antes zera o acúmulo parcial" | alta | FR-001/FR-002: `coleta_ticks` reiniciado sem crédito; passo 3 do roteiro |
| A3 | `coletar_materiais` sem `destino` ou com destino ≠ posição poderia ser aceito, acumulando fora do ponto neutro | alta | FR-002: `ValidacaoComandos::validar` recusa com `CodigoRecusa::DestinoInvalido`; passo 4 do roteiro |
| A4 | Novos campos ausentes de `hash_estado` dariam o mesmo resumo a estados com estoques distintos, quebrando determinismo | alta | FR-002: serialização inclui `materiais`/`suprimentos`/`coleta_ticks`/`coleta_ponto`; caso "hash divergente" no roteiro |
| A5 | Implementação poderia debitar recursos parciais ou estender-se a dano/barricada, GUI, rede e GDScript | alta | FR-003/plano: escopo restrito a `lib.rs`, apenas crédito atômico ao completar 180 ticks; caso "ausência de consumo parcial" |
| A6 | Tentação de criar entidade "ponto neutro" no contrato ou alterar `PROTOCOL_VERSION`/`Cargo.toml` | alta | FR-002/FR-003: `Comando.destino` como ID do ponto; nenhuma mensagem nova, nenhum bump de protocolo |
| A7 | Risco de inverter a ordem por hábito (`analyze` antes de `tasks`), antecipar R7 ou ativar R6 sem ferramenta | média | FR-004/plano/DAG: `tasks` (T012) antes de `analyze` (T013), R4 (T017) antes de R7 (T018); R6 suspensa, PARE se invocada, nenhuma issue simulada |
| A8 | Risco de afirmar verde sem execução, escrever o roteiro depois do PR ou tocar `STATE.md`/`feature.json`/`converge` alheio | média | FR-006/FR-007/FR-008: roteiro de 10 passos pré-PR com os seis casos; pendências **[NV]**; fronteira de 14 artefatos; arquivos alheios preservados |

## Convergência
Não há conflito entre spec, plano e checklist: a regra canônica, o contrato técnico auditado, a fronteira de código, a governança v1.2.2 e o roteiro pré-PR estão alinhados em FR-001..FR-008 e SC-001..SC-006. Nenhum marcador `[NEEDS CLARIFICATION]` em aberto (R3 = 0). R1 `go` presente com scorecard 6/6 strong e 80/20 = 180/625; a documentação (375/625) foi executada no ciclo 038 e sua fila está esgotada. A constituição **v1.2.2** do alvo (ratificada 2026-11-19, última emenda 2026-11-23) coloca `tasks` antes de `analyze` READ-ONLY e R4 antes de R7, coerente com a DAG de `tasks.md`. A suspensão de R6 é decisão ratificada, não dívida nem não-ratificação. A implementação em `lib.rs`, gates e CI são trabalho posterior do SWE/orquestrador, marcado **[NV]** — nenhum resultado é afirmado como executado. `.specify/memory/STATE.md` não é versionado nem editado; a alteração preexistente em `.specify/feature.json` pertence ao usuário.

## Veredito
Prosseguir para o roteiro, o `converge.md` (primeira entrada append-only), `validate.md` (R4) e `release.md` (R7), com A1–A8 tratados como requisitos de regra canônica, determinismo, fronteira e governança. Código, gates, roteiro executado, CI e PR permanecem **[NV]** até o SWE/orquestrador. R6 suspensa por decisão ratificada — PARE se invocada, nenhuma issue simulada. PASS.
