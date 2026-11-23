# Intake — Ponte provisória determinística

**Data**: 2026-11-23T06:25Z · **Slug**: `ponte-provisoria` · **Sessão**: `session_id=20261123T062500Z-ponte-provisoria`, `session_title=insurgencia - ponte provisoria deterministica`.

## Ideia

Capturar, sem tocar em código, a fatia 80/20 do ciclo 045: a **ponte provisória determinística** no Núcleo Rust, convertendo as regras canônicas do GDD em estado observável e auditável. O GDD §9.3:475,483 fixa o custo de **15 Materiais** e a recusa sem saldo **sem consumo parcial**; o GDD §8.4:407 fixa **60 PV** e **45 s de duração** — **1350 ticks a 1/30 s**. A implementação prevista adiciona a ação `construir_ponte` em `ACOES_PERMITIDAS` da sim (nenhuma mudança em `crates/contratos` nem em `PROTOCOL_VERSION`), a struct pública `Ponte { id, dono, posicao, pv, ticks_restantes }` em `BTreeMap<u64, Ponte>` em `Mundo`, as constantes `CUSTO_PONTE_MATERIAIS`, `PV_PONTE`, `TICKS_DURACAO_PONTE` e `TICKS_CONSTRUCAO_PONTE`, validação `destino == posicao` e saldo, débito atômico, cobertura em `hash_estado`, expiração determinística e HUD com contagem de Pontes e tecla `P`.

**Motivação**: entregar o próximo recurso canônico implementável sem nova stack — a ponte do Delta-3 (Ilha da Barca, §8.4:407) — fechando o caminho entre coleta de Materiais, construção e objeto destrutível com duração finita, com determinismo por tick e hash estável, sem Lightyear/async/sockets.

**Escopo registrado (não reaberto)**: o **atalho de 8 s entre ilhas** depende de mapa/ilhas e fica **fora de escopo**, registrado como limitação; o **tempo de construção de 45 ticks** é escolha de **protótipo**, não valor canônico. Nenhum dos dois deve ser tratado como regra do GDD neste ciclo.

## Contexto do ciclo (observado no contrato da sessão e em STATE.md; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/ponte-provisoria-045/`, branch `045-ponte-provisoria`, base `fa33052`.
- Alvo de especificação `specs/045-ponte-provisoria`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012.
- Constituição v1.2.2: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — removida da sequência executável pela emenda de 23/11 (`.specify/memory/constitution.md:79-80`).
- Decisão 80/20 já registrada em `.specify/memory/STATE.md:317-328` (2026-11-23T06:25Z), anterior à implementação.

## 80/20 (decisão de escopo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Ponte provisória (15 Materiais, 60 PV, 45 s; escolhida) | 3×3×3×4 = 108/625 | **108/625** |
| Súditos/Propaganda | 4×2×2×3 = 48/625 | 48/625 |
| Combate/Fôlego | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (quarto fator = risco invertido/segurança), escala 1–5, denominador **625**.

Escolhe-se a ponte provisória: é o próximo recurso com valores **canônicos fechados** — custo 15 Materiais (§9.3:475,483), 60 PV e 45 s de duração (§8.4:407) — implementável reaproveitando o padrão de coleta/construção já entregue, sem nova stack. Confiança 3 porque o uso da ponte em emboscadas é `[SUPOSIÇÃO]` (§10.3:574) e o atalho de 8 s fica de fora. Súditos/Propaganda e Combate/Fôlego são majoritariamente `[HIPÓTESE]` e não têm recompensa canônica própria neste recorte; a rede, embora de impacto máximo, está **bloqueada por ADR-004 até decisão da SE** (Lightyear declarado na ADR, ausente de `Cargo.toml`/`Cargo.lock`; nenhuma troca adicional de engine sem novo ADR).

## Evidências de entrada (coladas; não reexecutadas)

- GDD §9.3:475 — tabela de recursos: `Materiais` são gastos em "barricada (20), **ponte provisória (15)**, sabotagem".
- GDD §9.3:483 — critério de aceitação: "Uma barricada custa exatamente 20 Materiais e **uma ponte provisória 15**; **sem saldo, a ação é recusada sem consumo parcial**."
- GDD §8.4:407 — Delta-3 (Ilha da Barca): "Artesão constrói Ponte Provisória | Delta-2 | **atalho de 8 s entre ilhas por 45 s** | **ponte destrutível com 60 PV**."
- GDD §10.3:574 — "na Rota Inferior / Delta do Comércio, a mesma mecânica gera a Ponte Provisória usada em emboscadas. `[SUPOSIÇÃO]`" — o uso em emboscadas é suposição; custo, PV e duração são canônicos.
- `.specify/memory/STATE.md:317-328` — scorecard 108/48/48/10 com I/C/F/R (3/3/3/4, 4/2/2/3, 4/2/2/3, 5/2/1/1) e escolha `ponte-provisoria`; 45 s = 1350 ticks a 1/30 s; atalho de 8 s fora de escopo; R6 N/A.
- `.specify/memory/constitution.md:48-50,79-80,84` — R1 `go`; R4 antes de R7 (`1.0.0-local`); R6 removida da sequência executável na v1.2.2; versão 1.2.2 ratificada 19/11, emendada 23/11.
- `docs/adr/004-migracao-bevy-lightyear.md:15-24,53` — stack ADR-004 Accepted; "nenhuma troca adicional de engine é permitida sem novo ADR"; rede não entra neste ciclo.
- API Rust prevista (contrato da sessão, imutável): ação `construir_ponte` em `ACOES_PERMITIDAS`; struct pública `Ponte { id, dono, posicao, pv, ticks_restantes }` em `BTreeMap<u64, Ponte>` em `Mundo`; constantes `CUSTO_PONTE_MATERIAIS`, `PV_PONTE`, `TICKS_DURACAO_PONTE`, `TICKS_CONSTRUCAO_PONTE`; validação `destino == posicao` e saldo ≥ 15; débito atômico; expiração determinística; `hash_estado`; HUD com contagem de Pontes e tecla `P`.
- Regras Rust do contrato: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem async/rede/Lightyear.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/ponte-provisoria/`. Não executa gates, build, testes ou formatadores; não faz commit/push/PR; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` nem código-fonte. Tudo o que depende de execução permanece **[NV]**.
