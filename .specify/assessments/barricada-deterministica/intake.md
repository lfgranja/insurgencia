# Intake — Barricada determinística de materiais

**Data**: 2026-11-23T02:43Z · **Slug**: `barricada-deterministica` · **Sessão**: `session_id=20261123T024300Z-barricada-deterministica`, `session_title=insurgencia - barricada deterministica de materiais`.

## Ideia

Capturar, sem tocar em código, a fatia 80/20 do ciclo 040: a **construção determinística de barricada** no Núcleo Rust. O recorte usa os valores canônicos do GDD — **20 Materiais**, **1,5 s = 45 ticks** de canalização, **200 PV**, **12 s = 360 ticks** de duração e **máx. 2 barricadas por jogador** (a 3ª remove a mais antiga) —, recusa por saldo insuficiente **sem consumo parcial**, canalização avançada no máximo **uma por jogador por tick**, e reflexo de todos os campos e barreiras em `hash_estado`. A entrada do jogador é a tecla `B` (just pressed) no cliente, com `seq`/`request_id` monotônicos via recurso local, e o HUD passa a exibir Materiais, Barreiras e Construindo.

**Motivação**: entregar a primeira habilidade ativa do Artesão como mecânica verificável e determinística, preservando os valores já ancorados no GDD e sem alterar `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`.

## Contexto do ciclo (observado no contrato da sessão; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/barricada-040/`, branch `040-barricada-deterministica`, base `6b913f0`.
- Alvo de especificação `specs/040-barricada-deterministica`; rastros FR-001..FR-006, SC-001..SC-006, T001..T014.
- Constituição v1.2.2: R1 `go`, R4 antes de R7, `analyze` READ-ONLY depois de `tasks`; **R6 fora da sequência executável** até existir adaptador de issues.

## 80/20 (decisão de escopo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Construir barricada (escolhida) | 5×5×4×4 = 400/625 | **400/625** |
| Ponte provisória | 3×3×3×4 = 108/625 | 108/625 |
| Combate/Fôlego | 4×2×2×3 = 48/625 | 48/625 |
| Servidor + Lightyear | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 625.

Escolhe-se a barricada determinística: impacta os pilares Materialismo Histórico e Ação Coletiva com valores já canônicos e verificáveis por hash. "Combatar primeiro" contradiz a intenção do MVP (GDD §16.3:1509); "adiar para netcode" não produz fatia jogável verificável; "ponte provisória" é `[SUPOSIÇÃO]` (GDD §10.3:574), sem valores canônicos próprios. Contraste rastreável: em `.specify/assessments/estado-escalonamento-45/decision.md:14`, `construir_barreira` foi classificada como alternativa fora de escopo (60/625); aqui, com contrato dedicado e valores canônicos, é o objeto do ciclo.

## Evidências de entrada (coladas; não reexecutadas)

- GDD §9.3:475 — Materiais financiam "barricada (20), ponte provisória (15), sabotagem"; §9.3:483 — "Uma barricada custa exatamente 20 Materiais ... sem saldo, a ação é recusada sem consumo parcial".
- GDD §10.3:574 — "Q — Construir Barricada": "1,5 s de canalização", "200 pontos de vida", "12 s de duração"; o próprio texto declara "Materiais, canalização, pontos de vida e duração são canônicos" e marca `[HIPÓTESE]` apenas Fôlego, alcance, recarga e dimensões.
- GDD §10.3:577-579 — interdita corredor por 12 s e é destruída com 200 de dano; projéteis colidem; "Cada Artesão mantém no máximo duas barricadas ativas; a terceira remove a mais antiga".
- GDD §16.3:1505 e §20.2/H4:1678 — os mesmos números reaparecem rotulados `[HIPÓTESE]`: **conflito de rótulo a registrar como risco, não reaberto** (prevalecem §9.3/§10.3 como canônicos).
- API Rust do contrato: constantes públicas, `Jogador.construcao_ticks`/`construcao_ponto`, `Mundo.barreiras`/`proxima_barreira_id`, struct `Barreira`, `CodigoRecusa::SaldoInsuficiente`, `Comando.destino` como ponto de construção.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/barricada-deterministica/`. Não executa gates, build, testes ou formatadores; não faz commit/push/PR; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` nem código-fonte. Tudo o que depende de execução permanece **[NV]**.
