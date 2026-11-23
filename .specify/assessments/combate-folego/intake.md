# Intake — Combate e Fôlego

**Data**: 2026-11-23T07:35Z · **Slug**: `combate-folego` · **Sessão**: `20261123T073500Z-combate-folego`, `session_title=insurgencia - combate e folego`.

## Ideia

Capturar, sem tocar em código, a fatia 80/20 do ciclo 047: dar **efeito determinístico** à ação `atacar_forte` — já presente em `ACOES_PERMITIDAS` — e tornar o **Fôlego** um recurso observável no Núcleo Rust. O `Jogador` passa a carregar `folego`, `folego_ticks` e `combate_ticks`; um ataque valida Fôlego e alvo no ponto/alcance, debita custo e aplica dano com `saturating_sub`; a regeneração ocorre em `fase_finish` com acumulador, mais lenta enquanto o combate está ativo; o cliente ganha a tecla `Q` e um HUD com `Fôlego`/`Vida`.

**Todos os valores de combate deste recorte são `[HIPÓTESE]` ou escolha de protótipo — nenhum é cânone.** O GDD §10.1:517-523 registra Fôlego máximo **100**, regeneração **10/s fora de combate** e **5/s em combate** e alcance contato **2 m**, todos marcados `[HIPÓTESE]`. O contrato da sessão fixa como **escolhas de protótipo** o custo de ataque **30**, o dano **10** e a duração de combate **90 ticks** (3 s na base de 30 ticks/s). A regeneração é derivada dessas hipóteses com acumulador: **+1 a cada 3 ticks fora de combate** (10/s) e **+1 a cada 6 ticks em combate** (5/s). O GDD §10.6:680 chama "Ataque Forte" de **nome canônico do protótipo**; os números de efeito daquela seção também são `[HIPÓTESE]` e **não** são reaproveitados nesta fatia.

Implementação prevista no contrato (sem mudança em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`): em `crates/sim/src/lib.rs`, a ação `atacar_forte` ganha efeito; `Jogador` ganha `folego`, `folego_ticks` e `combate_ticks`, incluídos no `hash_estado`; constantes documentadas com os rótulos acima; validação de alvo no ponto/alcance e de Fôlego; dano `saturating_sub`; no máximo **um** ataque por jogador por tick via `BTreeSet<u64>`; regeneração em `fase_finish` com acumulador; `CodigoRecusa::FolegoInsuficiente` e `CodigoRecusa::AlvoInvalido` conforme necessário. No cliente, a tecla `Q` agenda `atacar_forte` e o HUD exibe `Fôlego`/`Vida`.

**Motivação**: combate e Fôlego são a última frente de produto sem mudança de stack (`STATE.md:351`). Colocar o efeito em uma ação **já existente** e tornar o recurso auditável por `hash_estado` produz jogabilidade verificável sem IA, projéteis, colisão ou rede — e sem transformar nenhuma hipótese de balanceamento em dogma.

## Contexto do ciclo (observado no contrato da sessão; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/combate-047/`, branch `047-combate-folego`, base `0ac4c9f`.
- Alvo de especificação `specs/047-combate-folego`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012.
- R1 `go`; R3 = 0 clarificações de design no contrato; R4 antes de R7 (`1.0.0-local`); **R6 N/A** por constituição v1.2.2 (fora da sequência executável até existir adaptador de issues).
- Constituição v1.2.2, GDD como âncora documental; nenhuma troca de stack, dependência, protocolo ou engine.
- Regras Rust obrigatórias: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem `async`/rede/Lightyear.

## 80/20 (decisão de escopo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Combate/Fôlego — `atacar_forte` com efeito (escolhida) | 4×2×2×3 = 48/625 | **48/625** |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (quarto fator = risco invertido/segurança), escala 1–5, denominador **625**.

Escolhe-se `combate-folego` porque é a única frente de produto restante que cabe em `crates/sim/src/lib.rs` sem tocar em contrato, protocolo, `Cargo.toml` ou engine (`STATE.md:348-351`). Confiança **2** e facilidade **2** registram com honestidade que todos os números de combate são `[HIPÓTESE]`/`escolha de protótipo` e que o efeito ainda não existe no Núcleo; risco invertido **3** reflete a disciplina necessária (débito atômico, um ataque por tick, acumulador de regeneração e hash refletivo). A rede tem impacto 5, mas permanece bloqueada por ADR-004 (Lightyear ausente de `Cargo.toml`/`Cargo.lock`) e sincronizaria um estado que ainda não é determinístico. Súditos/Propaganda, escolhida no ciclo 046 com o mesmo 48/625, não é reaberta neste ciclo.

## Evidências de entrada (coladas; não reexecutadas)

- GDD §10.1:517 — "Toda habilidade consome **Fôlego**, reserva física que se regenera por tempo — não é mana (energia mágica) nem recurso sobrenatural. Os valores abaixo são chutes de design e se aplicam a todas as funções."
- GDD §10.1:519-523 — tabela: "Fôlego máximo | 100 | ... `[HIPÓTESE]`"; "Regeneração | 10/s fora de combate; 5/s em combate | Punir o gasto em combate sem travar o jogador. `[HIPÓTESE]`"; "Alcance contato | 2 m | Corpo a corpo. `[HIPÓTESE]`".
- GDD §10.6:680 — "**Q — Ataque Forte.** Golpe concentrado corpo a corpo; nome canônico do protótipo, efeito detalhado aqui." Números de custo/alcance/dano da seção são `[HIPÓTESE]` e ficam fora do recorte.
- GDD §16.3:1509 — "A validação objetiva dessa intenção está nas métricas da seção 17", antecedido por "jogabilidade baseada em desafios de ilha, em vez de combate puro"; é a objeção histórica registrada, não uma renegociação do contrato 047.
- `.specify/memory/STATE.md:342-351` — decisão 80/20 do ciclo 047 (2026-11-23T07:35Z): Combate/Fôlego 4/2/2/3 → **48/625**; rede 5/2/1/1 → 10/625; escolha `combate-folego`; Fôlego 100, 10/s e 5/s rotulados `[HIPÓTESE]`, a spec deve registrá-los como escolhas de protótipo; dano e alcance também hipotéticos; R6 N/A.
- Contrato da sessão `20261123T073500Z-combate-folego` (imutável): custo 30, dano 10, 90 ticks de combate como escolhas de protótipo; API prevista, constantes, validação, `BTreeSet<u64>`, `fase_finish` e HUD.
- `specs/047-combate-folego/spec.md` no momento da leitura: apenas o template inicial; os rastros FR/SC/T deste assessment são antevisões do handoff, não requisitos já ratificados.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/combate-folego/`. Não executa gates, build, testes ou formatadores; não faz commit/push/PR; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` nem código-fonte. Tudo o que depende de execução permanece **[NV]**.
