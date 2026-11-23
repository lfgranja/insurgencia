# Intake — Súditos e Propaganda

**Data**: 2026-11-23T06:58Z · **Slug**: `suditos` · **Sessão**: `session_id=20261123T065800Z-suditos`, `session_title=insurgencia - suditos e propaganda`.

## Ideia

Capturar, sem tocar em código, a fatia 80/20 do ciclo 046: a **conversão determinística de Súdito inimigo por Propaganda** no Núcleo Rust. A regra canônica do GDD §9.2:464 é a âncora: um Súdito inimigo exposto **4 s (120 ticks)** à Propaganda para de atacar estruturas e passa a panfletar; se levar dano de jogador, **reverte em 3 s (90 ticks)**. O que o GDD chama de "[HIPÓTESE] A cada pulso de 30 s, cada ilha convertida libera 2 Súditos", o **raio de 6 m** e as **funções trabalhador/propagandista/miliciano** ficam registrados como `[HIPÓTESE]` e **fora de escopo**.

Implementação prevista no contrato (sem mudança em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`): ação `propaganda` em `ACOES_PERMITIDAS` da sim; struct pública `Sudito { id, posicao, convertido, propaganda_ticks, reversao_ticks }` em `BTreeMap<u64, Sudito>` no `Mundo`; API `adicionar_sudito(id, posicao)` para injeção determinística; `aplicar_dano_a_sudito(id)`; auto-avanço por **um** comando com `BTreeSet<u64>` (no máximo um avanço por súdito por tick); `hash_estado` refletivo; HUD com contagem de convertidos e tecla `G`.

**Motivação**: tornar verificável a única regra de Súditos que o GDD traz como critério de aceitação sem depender de spawn, controle de ilha ou sistemas ainda hipotéticos — preservando determinismo e a fronteira do Núcleo.

## Contexto do ciclo (observado no contrato da sessão; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/suditos-046/`, branch `046-suditos`, base `d410bd6`.
- Alvo de especificação `specs/046-suditos`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012.
- R1 `go`; R3 = 0 clarificações; R4 antes de R7 (`1.0.0-local`); **R6 N/A** por constituição v1.2.2 (fora da sequência executável até existir adaptador de issues).
- Constituição v1.2.2, GDD como âncora de valores canônicos; nenhuma troca de stack, dependência, protocolo ou engine.

## 80/20 (decisão de escopo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Súditos/Propaganda (escolhida) | 4×2×2×3 = 48/625 | **48/625** |
| Combate/Fôlego | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (quarto fator = risco invertido/segurança), escala 1–5, denominador **625**.

Escolhe-se `suditos`: a regra de aceitação GDD §9.2:464 é verificável mesmo com spawn/controle de ilha ainda `[HIPÓTESE]`. Há empate numérico com Combate/Fôlego (ambos 48/625); o desempate é do contrato da sessão e do lastro canônico — a conversão por Propaganda tem regra escrita no GDD, enquanto Fôlego, alcance, recarga e dimensões permanecem hipóteses (§10.3:574-579) e o MVP prova jogabilidade por desafios de ilha, "em vez de combate puro" (§16.3:1509). A rede, embora de impacto 5, está bloqueada por ADR-004 (Lightyear ausente de `Cargo.toml`/`Cargo.lock`). Contraste rastreável: Súditos/Propaganda foi **rejeitada** como alternativa (48/625) em `.specify/assessments/defesa-barricada/decision.md` e `.specify/assessments/ponte-provisoria/decision.md`; aqui é o objeto dedicado do ciclo.

## Evidências de entrada (coladas; não reexecutadas)

- GDD §9.2:457-459 — "Os Súditos não são soldados irracionais: são grupos de trabalhadores, propagandistas e milicianos que avançam pelas rotas representando o avanço material e ideológico da Comuna. Um Intelectual próximo pode inspirá-los: eles param de atacar estruturas e passam a panfletar, convertendo passivamente as defesas da ilha."
- GDD §9.2:461 — "[HIPÓTESE] A cada pulso de 30 s, cada ilha convertida libera 2 Súditos (1 trabalhador e 1 propagandista); ilhas da Rota Central liberam 1 miliciano adicional. ... Propaganda do Intelectual tem área de 6 m, exige 4 s de exposição e converte até 3 Súditos inimigos; convertidos conservam a função original e permanecem assim por 60 s."
- GDD §9.2:463-467 — critérios de aceitação: "Um Súdito inimigo exposto 4 s à Propaganda para de atacar estruturas e passa a panfletar; se levar dano de jogador, reverte em 3 s." Os dois critérios seguintes (renda do trabalhador por 60 s; 2 Súditos por pulso, ou 3 na Rota Central) dependem da hipótese de spawn/função e ficam fora do recorte.
- `.specify/memory/STATE.md` — decisão 80/20 do ciclo 046 (2026-11-23T06:58Z): I/C/F/R = 4/2/2/3 → 48/625 para Súditos/Propaganda; 4/2/2/3 → 48/625 para Combate/Fôlego; 5/2/1/1 → 10/625 para rede; escolha `suditos`; spawn/raio/função fora de escopo; R6 N/A.
- API Rust prevista no contrato da sessão: constantes de 120 e 90 ticks; `Sudito { id, posicao, convertido, propaganda_ticks, reversao_ticks }`; `Mundo.suditos: BTreeMap<u64, Sudito>`; `adicionar_sudito(id, posicao)`; `aplicar_dano_a_sudito(id)`; `propaganda` em `ACOES_PERMITIDAS` com `BTreeSet<u64>`; `hash_estado`; HUD com convertidos e tecla `G`.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/suditos/`. Não executa gates, build, testes ou formatadores; não faz commit/push/PR; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` nem código-fonte. Tudo o que depende de execução permanece **[NV]**.
