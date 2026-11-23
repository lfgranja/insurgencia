# Concept — Coleta determinística de materiais

**Data**: 2026-11-23T01:54Z

## Opções
1. **Escrever os 14 artefatos SDD da coleta determinística (recomendada)**: fixar a regra canônica do GDD §7.1 e o contrato de implementação no sim, com roteiro de 10 passos antes do PR, sem tocar código. Impacto 5 × confiança 4 × facilidade 3 × segurança 3 ÷ 625 = **180/625**. Fila documental esgotada e menor incremento com valor canônico.
2. Repetir documentação pós-v1.2.2 (375/625 na ocasião) — rejeitada: fila esgotada no ciclo 038; não gera produto novo.
3. Implementar coleta diretamente em `crates/sim/src/lib.rs` neste agente — rejeitada: fora da fronteira SDD; código, gates e PR pertencem ao SWE/orquestrador e ficam **[NV]**.
4. Criar entidade "ponto neutro" no contrato ou usar nova mensagem — rejeitada: `ACOES_PERMITIDAS` já contém `coletar_materiais` e `Comando.destino` serve como ID do ponto; mudança de `PROTOCOL_VERSION` fora de escopo.
5. Implementar em GDScript/GUI/rede ou acrescentar dano, barricada e interrupção por 1 s — rejeitada: fora de escopo; o incremento deve permanecer smallest viable no sim autoritativo.
6. Ativar R6/criar issues — rejeitada: R6 suspensa por decisão ratificada; PARE se invocada, nunca simular issue.

## Decisão (R1: go)
Opção 1. 5×4×3×3 ÷ 625 = **180/625** (contra 375/625 da documentação, cuja fila está esgotada). R3: 0 clarificações — regra canônica e contrato técnico já auditado/orientado pelo orquestrador, com fonte no GDD §7.1 e no `crates/sim/src/lib.rs` lidos; a única ressalva é de execução: este agente escreve os 14 artefatos SDD e nada mais. Handoff: SWE/orquestrador implementa `lib.rs`, executa gates e roteiro e decide PR; este agente não executa nada.

## Salvaguardas
- Regra canônica registrada com fonte (GDD §7.1, linhas ~299–314): 180 ticks a 1/30 s, +3 Suprimentos/+2 Materiais, saída antes zera parcial.
- Contrato registrado sem inventar protocolo: `destino` como ID do ponto; validação `DestinoInvalido`; crédito atômico em `fase_skills`; `hash_estado` inclui os quatro campos novos; `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml` intactos.
- Constituição v1.2.2 preservada: `tasks` antes de `analyze` (READ-ONLY), R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only; R6 fora do fluxo, PARE se invocada, nenhuma issue simulada.
- Fronteira exata de 14 artefatos; nenhum build/gate/formatador/teste/commit/push/PR/merge/deploy; `.specify/memory/STATE.md` e a alteração preexistente em `.specify/feature.json` preservados; pendências **[NV]**.
