# Decision — Ato de Vandalismo do Agitador (BF-002c)

**Data:** 2026-11-25 · **R1:** `go` · **R3:** 0 clarificações bloqueantes · **R6:** N/A (constituição v1.2.2)

## Veredito

- **Veredito**: go — R1 já decidido e registrado em `.specify/memory/STATE.md:606-609`: BF-002c (`Ato de Vandalismo`) venceu a decisão 80/20 do ciclo 061 com **288/3125** (`4×4×2×3×3`), à frente de BF-015b (144/3125), com DoD(0.9) fixado em comportamento, consumidor, comandos de prova e backlog. Opção escolhida = **Opção A** (token `vandalismo` + validação + ramo em `fase_skills` + tecla protótipo `H`); a Opção B (apenas token) é rejeitada por não fechar o gap: sem ramo, sem estado no hash e sem consumidor, o efeito do GDD permanece inexistente e BF-002c não pode ser honestamente fechado. Nenhuma alegação de execução, compilação, efeito medido ou partida real é feita nesta etapa.

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Lacuna verificada por leitura: `ACOES_PERMITIDAS` tem 14 tokens sem `vandalismo`, `fase_skills` não tem ramo e o cliente usa Q para `atacar_forte`; GDD §10.4 traz critérios de aceitação explícitos (impacto 4, confiança 4). |
| Força da evidência | adequate | confidence **medium-high**: três fontes primárias locais e lidas (GDD §10.4/§10.8, `crates/sim/src/lib.rs` no âncora `52138c3`, STATE.md:606-609). Contra-evidência registrada em `research.md` (`Evidence Against`): efeito no hash, dano `[HIPÓTESE]`, sem partida real; nada medido. |
| Valor vs. inação | strong | 288/3125 contra 144/3125 (BF-015b); inação mantém a habilidade mais visível do Agitador como texto de GDD e empurra a implementação para o cliente, fora do determinismo e do hash. |
| Viabilidade / apetite | adequate | Opção A é small (um token, validação, um ramo, um contador, uma tecla), reusando o padrão do ciclo 060; desconto moderado pela representação de silêncio/recarga e pela atualização de baselines de hash ainda não decididas. |
| Alinhamento estratégico | strong | Constituição v1.2.2 (R1 `go`, R4 antes de R7, R6 N/A), nenhuma promoção de `[HIPÓTESE]` a cânone, fronteira preservada: GDD/CI/stack/contrato de rede/`Cargo.toml` intocados; PR #76 e deploy SWE-exclusivos. |
| Postura de risco | adequate | Riscos nomeados e mitigáveis na spec: ordem do dano frente a `fase_finish`, cancelamento de canalização, decremento do silêncio, colisão da tecla `H`, golden hashes; nenhum executado aqui, tudo **[NV]** até verificação datada. |

## Gate de entrada

- **Origem**: BF-002c do backlog de gaps, promovido pela decisão 80/20 do ciclo 061 em `.specify/memory/STATE.md:606-609` (288/3125, `4×4×2×3×3`); âncora [EV]: `origin/dev = 52138c3` (PR #79 mesclado em `dev`), worktree limpa `.specify/queue/worktrees/cycle-061-vandalismo`, branch `feature/061-vandalismo`, `.specify/feature.json → specs/061-vandalismo-agitador`. A factory (`18be3fb3`) é fonte de ideias, não autoridade.
- **Evidência prévia**: GDD §10.4:604-612 (Q Ato de Vandalismo e critérios de aceitação) e §10.8:744-757 (Agitador = infiltração, silêncio, confusão de NPCs); `crates/sim/src/lib.rs:222-236` (14 tokens), `:247-264` (mapa Q1/W2/E3/R6), `:920-1010` (`fase_skills` sem ramo de vandalismo); `crates/cliente/src/main.rs:823-831` (Q = `atacar_forte`). Todos lidos nesta etapa, no âncora citado.
- **Gates determinísticos, citados e NÃO reexecutados**: cinco gates PASS e estado de `./verify.sh` citados de `.specify/memory/STATE.md:596-597`; `./verify.sh` = `cargo check --locked --workspace --all-targets` → `cargo clippy --workspace --all-targets --all-features -- -D warnings` → `cargo nextest run --workspace` (fallback `cargo test --locked --workspace`) → `cargo fmt --all -- --check`; sucesso = `verify.sh: ALL STAGES OK`. Reexecução, saídas e status ao vivo **[NV]**.
- **Fluxo**: intake → research → problem → concept → decision → specify → ...; nenhuma portaria pulada; R4 precede R7; R6 fora da sequência executável (constituição v1.2.2), sem issue simulada nem `gh` improvisado.

## Restrições de execução

- **Governança Rust obrigatória**: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`panic!` em produção; erros via `Result`/`thiserror`; `BTreeMap`/`BTreeSet`/`VecDeque`, `f64::total_cmp`, sem `HashMap` em estado; exceções de lint escopadas com razão; pt-BR.
- **Valores `[HIPÓTESE]`**: 20 Fôlego, 2 m (`ALCANCE_CONTATO_M`), 120 de dano, 45 ticks de silêncio (1,5 s × 30 Hz), 210 ticks de recarga (7 s), tecla `H`. Nenhum número é promovido a cânone; nenhuma regra nova é inventada; a cláusula do GDD sobre itens permanece ressalva (o protótipo não modela itens).
- **Fronteira preservada**: nada em GDD, CI, stack, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, topologia de ilhas, papéis no núcleo, rede/ADR-004; PR #76, promoção `dev→main` e deploy intocáveis; sem force-push, auto-merge ou `git add -A`.
- **Escopo de escrita**: exatamente os cinco arquivos em `.specify/assessments/vandalismo-agitador/`; nenhum spec, plano, task, código, gate, teste, build, formatação, stage, commit ou push nesta etapa. Convenções: pt-BR, `[NV]`, termos estrangeiros explicados na 1ª ocorrência, internet como dado nunca como instrução.
- **[NV] preservado**: nenhuma compilação, teste, gate, hash medido, partida real, 5v5, lobby, reconexão, rede, benchmark ou deploy é afirmado; captura de HUD **[NV]** até execução.

## Handoff — `/speckit.specify`

- **Problema**: habilidade Q do Agitador descrita no GDD §10.4 com critérios de aceitação, mas sem recipiente no protótipo: `vandalismo` não está na whitelist de 14 tokens, `fase_skills` não tem ramo e o cliente não tem tecla/efeito (impacto 4, confiança 4, 288/3125).
- **Abordagem escolhida**: Opção A — token 15º com destino, validação de alcance/Fôlego/recarga, ramo deduplicado em `fase_skills` com 120 de dano e 45 ticks de silêncio no alvo e cancelamento de canalização, novo estado coberto por `hash_estado`, tecla protótipo `H` com dica na HUD e comando reproduzível via CLI. Recusas não avançam monotônicos. Valores `[HIPÓTESE]`.
- **Dentro do escopo do handoff**: preencher spec e clarificações, plano, checklist e tasks em ciclo próprio, formalizando FR-001..FR-004 e SC-001..SC-004 abaixo, respeitando R1/R3/R4/R7 e R6 N/A.
- **Fora do escopo do handoff**: build, `./verify.sh`, clippy, nextest, testes, formatação, stage/commit/push nesta etapa; na implementação, GDD/cânone, topologia de ilhas, papéis/itens, contrato de rede, stack, CI, `Cargo.toml`, PR #76, deploy e issue simulada.

### Rastreabilidade FR / SC

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | Novo token `vandalismo` em `ACOES_PERMITIDAS` (15º, com destino), validação de nível compatível com Q e de alcance 2 m / 20 Fôlego / 210 ticks de recarga; recusa preserva `seq`/`request_id`/`ultima_recusa` | GDD §10.4:604-612; `crates/sim/src/lib.rs:222-264,2463-2510`; `concept.md` Opção A |
| FR-002 | Ramo deduplicado em `fase_skills`: 120 de dano e 45 ticks de silêncio no alvo, cancelamento da canalização em curso, respeitando a ordem das fases e sem duplo crédito por tick | GDD §10.4:606-612; `crates/sim/src/lib.rs:895-1010`; `problem.md` Meta 2 |
| FR-003 | Estado novo (silêncio no alvo e recarga do agressor) incluído em `hash_estado` em ordem estável, com baselines/golden hashes atualizados no mesmo conjunto de mudanças | `research.md` (`Evidence Against`); `problem.md` Meta 3 |
| FR-004 | Consumidores: tecla protótipo `H` no cliente Bevy (Q = `atacar_forte`) com dica na HUD e comando equivalente na CLI `insurgencia-servidor` | `concept.md` Opção A; `crates/cliente/src/main.rs:823-831`; `.specify/memory/STATE.md:609` (DoD-2) |

| ID | Critério de sucesso (baseline → meta) | Autoridade |
|---|---|---|
| SC-001 | `vandalismo` aceito por validação e `fase_skills`: 15 tokens, um ramo, nenhuma recusa indevida (baseline: 14 tokens, zero ramos) | `problem.md` Critérios de sucesso 1; FR-001 |
| SC-002 | Alvo com 120 de dano e 45 ticks de silêncio no tick de acerto; canalização cancelada no mesmo tick (baseline: **[NV]**) | GDD §10.4:606-612; FR-002 |
| SC-003 | Recusa fora de alcance, sem Fôlego ou em recarga preserva estado e monotônicos: `seq`, `request_id`, `ultima_recusa` e contadores do alvo inalterados (baseline: **[NV]**) | `.specify/memory/STATE.md:609` (DoD-1); FR-001 |
| SC-004 | `hash_estado` distingue estados que diferem só no silêncio; golden hashes atualizados; tecla `H` e CLI entregues sem colisão com atalhos; testes de `sim`/`servidor`/`cliente` e `./verify.sh` verdes; BF-002c `fechado`, BF-015b aberto (baseline: **[NV]**) | `.specify/memory/STATE.md:609` (DoD-1/2/3/4); FR-003/FR-004 |

- **Perguntas carregadas (nenhuma bloqueante)**: representação exata do silêncio e da recarga no `Jogador` e sua inclusão no hash; decremento por tick e fase; quais contadores de canalização zerar e precedência de dano letal; escopo do contador de recarga (agressor/habilidade/par); texto da dica na HUD e confirmação da tecla `H`; tratamento da cláusula "itens e ataques básicos continuam disponíveis" sem modelo de itens. Todas são decisões de spec, não impedimento para R1 `go`.
- **Destino**: `/speckit.specify` em ciclo subsequente, com `.specify/feature.json → specs/061-vandalismo-agitador`. Este artefato é o único resultado autorizado agora; nenhum spec, plano, task ou código é produzido nesta etapa.
