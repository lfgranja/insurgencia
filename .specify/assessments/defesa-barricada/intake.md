# Intake — Defesa de barricada (autogestão)

**Data**: 2026-11-23T05:47Z · **Slug**: `defesa-barricada` · **Sessão**: `session_id=20261123T054700Z-defesa-barricada`, `session_title=insurgencia - defesa de barricada`.

## Ideia

Capturar, sem tocar em código, a fatia 80/20 do ciclo 044: a **defesa determinística de barricada** no Núcleo Rust, convertendo a barricada já entregue no ciclo 040 (200 PV, 360 ticks) em recompensa observável de autogestão. A regra canônica do GDD §7.1:306 — **+1 ponto de autogestão a cada 10 s de barricada intacta** — é implementada como **300 ticks a 1/30 s**: o campo `defesa_ticks: i64` em `Barreira` avança 1 por tick enquanto a barricada está intacta e o dono permanece no mesmo ponto; ao completar 300, credita +1 em `Jogador.pontos_autogestao: i64` e zera o contador. Se o dono sai do ponto, a barricada é destruída (`pv == 0`) ou expira (`ticks_restantes == 0`), o acúmulo zera e **não credita**. O HUD passa a exibir `Autogestão`.

**Motivação**: tornar verificável a condição de vitória "autogestão, não demolição" (GDD §6.2:273) e a fase Defender do loop de 30 s (§7.1:306), reaproveitando a barricada 040 sem mudança de `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`, e sem Lightyear/async/sockets.

**Simplificação de escopo registrada**: o protótipo usa o **dono da barricada no mesmo ponto** como defensor. Os papéis distintos do GDD (Guerreiro na barricada, Artesão reforçando — §7.1:306) ficam registrados como simplificação, sem implementação neste ciclo.

## Contexto do ciclo (observado no contrato da sessão e em STATE.md; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/defesa-barricada-044/`, branch `044-defesa-barricada`, base `b6d2f36`.
- Alvo de especificação `specs/044-defesa-barricada`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012.
- Constituição v1.2.2: R1 `go`, R4 antes de R7, `analyze` READ-ONLY depois de `tasks`; **R6 N/A** — removida da sequência executável pela emenda de 23/11 (`.specify/memory/constitution.md:79-80`).
- Decisão 80/20 já registrada em `.specify/memory/STATE.md:304-315` (2026-11-23T05:47Z), anterior à implementação.

## 80/20 (decisão de escopo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Defesa de barricada → +1 autogestão/10 s (escolhida) | 4×4×4×4 = 256/625 | **256/625** |
| Ponte provisória | 3×3×3×4 = 108/625 | 108/625 |
| Súditos/Propaganda | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (quarto fator = risco invertido/segurança), escala 1–5, denominador **625**.

Escolhe-se a defesa de barricada: é a única opção com recompensa **canônica** (GDD §7.1:306) apoiada em uma mecânica **já entregue** (barricada 040). "Ponte provisória" é `[SUPOSIÇÃO]` (§10.3:574) e não tem recompensa própria; "Súditos/Propaganda" é majoritariamente `[HIPÓTESE]` (§6.2:275) e exige conversão de Nós; "Rede" tem impacto alto, mas confiança 2 e facilidade 1, porque está **bloqueada por ADR-004 até decisão da SE** (Lightyear declarado na ADR, ausente de `Cargo.toml`/`Cargo.lock`; nenhuma troca adicional de engine sem novo ADR).

## Evidências de entrada (coladas; não reexecutadas)

- GDD §7.1:306 — fase "4. Defender | Guerreiro na barricada, Artesão reforça | segurar o ponto ou perseguir a emboscada | **+1 ponto de autogestão a cada 10 s de barricada intacta** | 8 s".
- GDD §6.2:273 — "A condição de vitória é **estabelecer a autogestão na Ilha Central**. A ilha não é explodida; é convertida."
- GDD §10.3:574,577-579 — barricada: **200 PV**, **12 s = 360 ticks** de duração, "Materiais, canalização, pontos de vida e duração são canônicos"; `[HIPÓTESE]` apenas Fôlego, alcance, recarga e dimensões; ponte provisória marcada `[SUPOSIÇÃO]`.
- `.specify/memory/STATE.md:310-315` — scorecard 256/108/48/10 com I/C/F/R (4/4/4/4, 3/3/3/4, 4/2/2/3, 5/2/1/1) e escolha `defesa-barricada`; protótipo com dono no mesmo ponto; R6 N/A.
- `.specify/memory/constitution.md:48-50,79-80,84` — R1 `go`; R4 antes de R7; R6 removida da sequência executável na v1.2.2; versão 1.2.2 ratificada 19/11, emendada 23/11.
- `docs/adr/004-migracao-bevy-lightyear.md:15-24,53` — stack ADR-004 Accepted; "nenhuma troca adicional de engine é permitida sem novo ADR"; rede não entra neste ciclo.
- API Rust prevista (contrato da sessão, imutável): `Jogador.pontos_autogestao: i64`, `Barreira.defesa_ticks: i64`, fase determinística de defesa, HUD `Autogestão`; ambos os campos cobertos por `hash_estado`.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/defesa-barricada/`. Não executa gates, build, testes ou formatadores; não faz commit/push/PR; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` nem código-fonte. Tudo o que depende de execução permanece **[NV]**.
