# Decision — Vitória local observável no espelho do cliente

- **Slug**: `mirror-vitoria-local`
- **Decided**: 2026-11-24
- **Score**: **270/3125** (`5×3×3×2×3`)
- **Consumer**: jogador do cliente Bevy + testes de integração da sim
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6/taskstoissues**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## DoD do ciclo (referência única; idêntico nos cinco artefatos)

- **DoD-1**: espelho `Mundo` do cliente semeado com **4 ilhas (ids 1..=4)**, **1 súdito (id 1 em `(0.0, 236.0)`)** e **3 nós**.
- **DoD-2**: vitória local alcançável convertendo os **3 nós via `propaganda_no`** — mesma rota de `crates/sim/tests/simulacao.rs:2558-2593`.
- **DoD-3**: `hash_estado` **muda** ao semear o espelho (baseline: sem ilhas/súditos/nós).
- **DoD-4**: **BF-003a fechado**; **BF-003b** (mapa completo de 12 ilhas) **permanece aberto**.

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | `crates/cliente/src/main.rs:528-605` registra só jogadores/PNJs no `Mundo`; as ilhas são só cena Bevy, não estado. HUD `Nós: 0/3` e `propaganda_no` sem alvo → vitória testada na sim, inalcançável no cliente. |
| Força da evidência | strong | `lib.rs:719-746` (APIs `adicionar_ilha`/`adicionar_no`), `simulacao.rs:2404-2407,2558-2593` (três nós → `Vitoria`), `lib.rs:2158-2206` (`hash_estado` serializa nós/resistência/resultado), GDD:275 (`[HIPÓTESE]`), contrato da sessão e constituição v1.2.2 lidos. |
| Valor vs. inação | strong | **270/3125** contra 120/3125 (semeadura duplicada) e 75/3125 (HUD/hash sem semear); o mesmo 270/3125 já era o valor de BF-003 no ciclo 057, agora com corte fechado. Inação mantém `Nós: 0/3` e convida à duplicação. |
| Viabilidade / appetite | strong | `small`: um helper público na sim (`semear_espelho_local` / `POSICOES_NOS_LOCAIS`), uma chamada em `configurar`, um teste de integração e evidência/validação/release — sem contrato, protocolo, Cargo, CI, stack ou rede. |
| Alinhamento estratégico | strong | Não promove a `[HIPÓTESE]` do GDD:275 a cânone, não abre BF-003b (DoD-4), respeita R1 `go`, R4 antes de R7 `1.0.0-local` e `analyze` READ-ONLY. |
| Postura de risco | strong | Risco invertido 3: semente estática e determinística; rota de vitória reaproveitada de `tres_nos_geram_vitoria`; hash muda de forma previsível (DoD-3); fronteira intacta. Efeito na HUD permanece **[NV]** até verificação. R6 registrada como N/A, sem issue simulada nem `gh` improvisado. |

## 80/20 (scorecard único do ciclo)

| Opção | Impacto × Confiança × Fechamento × Esforço⁻¹ × Risco⁻¹ ÷ 3125 | Resultado | Consumidor |
|:--|:--|:--|:--|
| Helper público na sim + chamada no cliente (BF-003a, escolhida) | 5×3×3×2×3 = **270/3125** | **270/3125** | jogador do cliente Bevy + testes de integração da sim |
| Semeadura duplicada dentro do cliente | 5×3×2×2×2 = 120/3125 | 120/3125 | jogador do cliente Bevy |
| Apenas HUD/`hash_estado` (sem semear) | 1×3×1×5×5 = 75/3125 | 75/3125 | diagnóstico da HUD |

Critério do contrato — **impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹**, escala 1–5, denominador **5⁵ = 3125**; o quarto e o quinto fatores são inversos de esforço e de risco (5 = menor esforço/risco, 1 = maior). Impacto **5** porque a vitória passa de inalcançável a jogável no cliente; confiança **3** porque os números do GDD:275 são `[HIPÓTESE]` e o helper ainda não foi executado; fechamento **3** porque consumidor e teste de referência estão nomeados, mas ids/posições são escolha de protótipo; esforço⁻¹ **2** porque exige helper na sim, chamada no cliente e verificação de hash; risco invertido **3** porque a fronteira fica intacta, embora o efeito final na HUD seja **[NV]**. Este é o único scorecard do assessment; os mesmos valores aparecem em `intake.md`, `research.md`, `define.md` e `shape.md`, sem pontuação paralela.

A opção escolhida vence porque é a única que satisfaz **todos os quatro DoDs** com uma só verdade de estado: o helper vive na sim, o cliente apenas o chama e o teste de integração cobre a rota de vitória e o hash. A duplicação (120/3125) cria divergência entre sim e espelho; a HUD diagnóstica (75/3125) não cria nós e deixa `Nós: 0/3`. Nenhuma alternativa abre BF-003b nem promove número a cânone.

## Veredito e justificativa

**Veredito: go** — Opção A (helper público na sim + chamada no cliente, BF-003a), scorecard 6/6 strong, **270/3125** (5×3×3×2×3), R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, **R6/taskstoissues N/A**. Consumidor: jogador do cliente Bevy + testes de integração da sim. DoD-1..DoD-4 são os quatro listados no topo deste arquivo e repetidos em `intake.md`, `research.md`, `define.md` e `shape.md`. Nenhum spec, plano, tarefa, código, gate, teste, commit, push, PR, promoção `dev → main` ou deploy foi produzido ou executado neste assessment. **PR #76 pertence a outro owner ativo e não é tocado.**

## Condições (R1)

1. **DoD-1 / semente**: helper público `semear_espelho_local` com `POSICOES_NOS_LOCAIS` na sim semeia o espelho com **4 ilhas (ids 1..=4)**, **1 súdito (id 1 em `(0.0, 236.0)`)** e **3 nós**, reaproveitando `adicionar_ilha`/`adicionar_no` (`crates/sim/src/lib.rs:719-746`). Valores rotulados `[HIPÓTESE]` (GDD:275); nada é promovido a cânone.
2. **DoD-2 / vitória**: `configurar` (`crates/cliente/src/main.rs:528-605`) chama o helper após registrar jogadores e PNJs; `propaganda_no` nos 3 nós leva a `resistencia_ideologica == 0` e `resultado == Some(ResultadoPartida::Vitoria)`, estável nos ticks seguintes, como em `crates/sim/tests/simulacao.rs:2558-2593`.
3. **DoD-3 / hash**: `hash_estado()` do espelho semeado difere do baseline sem semente, de forma determinística (a serialização em `lib.rs:2158-2206` já inclui nós, resistência e resultado).
4. **DoD-4 / fronteira de pendências**: BF-003a é fechado no handoff; BF-003b (mapa de 12 ilhas) permanece aberto — nenhuma topologia de 12 ilhas é implementada.
5. **HUD**: o cliente deixa de mostrar `Nós: 0/3` e passa a ler os 3 nós do espelho; efeito renderizado **[NV]** até execução.
6. **Fronteira técnica**: nenhuma mudança em regra canônica, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, jobs `gates`/`rust` do CI, `.github/workflows`, stack, Lightyear (ADR-004) ou rede; cobertura determinística existente preservada; estilo Rust `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`.
7. **Verificação**: gate de destino é `./verify.sh`; CI = jobs `gates`/`rust` do GitHub Actions; CLI de PR = `gh`. Nenhum foi executado por este agente; tudo que depende de execução é **[NV]**.
8. **Constituição v1.2.2**: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6/taskstoissues N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado. `dev → main` e deploy são exclusivos da SE; este agente não promove nem deploya.

## Riscos registrados

- **Semente como conteúdo canônico**: mitigado pelo rótulo explicitamente prototípico — ids/posições vêm do contrato, não do GDD como regra (GDD:275 permanece `[HIPÓTESE]`). Se a spec promover os números a cânone, é violação deste assessment.
- **Divergência sim ↔ cliente**: mitigado pela opção A — verdade única no helper da sim, teste de integração na sim e uma chamada no cliente; a duplicação é o risco rejeitado em `research.md`.
- **Determinismo/hash**: a semente é estática, sem aleatoriedade; `hash_estado` muda de modo previsível (DoD-3) e a cobertura determinística não é reescrita. Permanece **[NV]** até execução.
- **Verificação visual/HUD**: efeito renderizado depende de execução; não prometer captura real sem surface automatizável — **[NV]**.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Governança**: PR #76 não é tocado; `dev → main` e deploy são exclusivos da SE; nenhum comando de gate, build, git remoto ou GitHub foi executado.

## Handoff — `/speckit.specify`

- **Problem**: espelho `Mundo` do cliente contém só jogadores/PNJs (`main.rs:528-605`); ilhas são cena, não estado; `Nós: 0/3` e `propaganda_no` sem alvo tornam a vitória inalcançável embora testada em `simulacao.rs:2558-2593`.
- **Chosen approach**: Opção A — helper `semear_espelho_local` / `POSICOES_NOS_LOCAIS` na sim, chamada em `configurar`, teste `espelho_local_seed_viabiliza_vitoria_e_reflete_hash`, evidência/validação/release. **270/3125**.
- **In scope**: DoD-1..DoD-4 (4 ilhas ids 1..=4; 1 súdito id 1 em `(0.0, 236.0)`; 3 nós; vitória por `propaganda_no`; `hash_estado` muda; BF-003a fechado e BF-003b aberto).
- **Out of scope**: mapa de 12 ilhas (BF-003b), promoção da `[HIPÓTESE]` do GDD:275 a cânone, duplicação no cliente, HUD diagnóstica sem semear, mudanças em contrato/protocolo/Cargo/CI/stack/rede, gates/build/testes/commits/PRs neste assessment, **R6 (N/A)**, `dev → main`/deploy e PR #76.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: API exata de súditos e ordem de inserção no helper; posições dos nós (possivelmente `posicoes_nos()`, `simulacao.rs:2404-2407`); `(0.0, 236.0)` dentro do terreno e serializado no hash; efeito na HUD — validar na spec e no gate `./verify.sh`.

### Rastreabilidade FR / SC / T (mapeamento canônico da spec 058)

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | helper `semear_espelho_local` / `POSICOES_NOS_LOCAIS` na sim: 4 ilhas (ids 1..=4), 1 súdito (id 1 em `(0.0, 236.0)`) e 3 nós — DoD-1 | contrato da sessão; `crates/sim/src/lib.rs:719-746`; `intake.md`; `shape.md` · Opção A |
| FR-002 | chamada do helper no `configurar` do cliente — DoD-2 | `crates/cliente/src/main.rs:528-605`; `define.md`; `shape.md` · Opção A |
| FR-003 | teste de integração `espelho_local_seed_viabiliza_vitoria_e_reflete_hash`: vitória por `propaganda_no` nos 3 nós e `hash_estado` alterado — DoD-2 e DoD-3 | `crates/sim/tests/simulacao.rs:2404-2407,2558-2593`; `crates/sim/src/lib.rs:2158-2206` |
| FR-004 | evidência/validação/release: BF-003a fechado, BF-003b aberto, fronteira intacta e constituição v1.2.2 — DoD-4 | contrato da sessão; `./verify.sh`; jobs `gates`/`rust`; `.specify/memory/constitution.md` |
| SC-001 | 4 ilhas, 1 súdito em `(0.0, 236.0)` e 3 nós no espelho | `define.md` · Success Metrics 1 |
| SC-002 | `propaganda_no` nos 3 nós → `resistencia_ideologica == 0` e `Vitoria`, estável | `define.md` · Success Metrics 2 |
| SC-003 | `hash_estado()` semeado ≠ baseline sem semente, determinístico | `define.md` · Success Metrics 3 |
| SC-004 | BF-003a fechado; BF-003b permanece aberto | `define.md` · Success Metrics 4 |
| SC-005 | HUD deixa `Nós: 0/3` e lê os 3 nós do espelho | `define.md` · Success Metrics 5 |
| SC-006 | nenhuma mudança em regra/contratos/protocolo/Cargo/CI/stack/rede; cobertura válida; estilo Rust | `define.md` · Success Metrics 6 |
| T001 | helper `semear_espelho_local` e `POSICOES_NOS_LOCAIS` na sim | `shape.md` · Opção A / FR-001 |
| T002 | chamada do helper em `configurar` no cliente | `shape.md` · Opção A / FR-002 |
| T003 | teste `espelho_local_seed_viabiliza_vitoria_e_reflete_hash` | `shape.md` · Assumptions / FR-003 |
| T004 | evidência/validação/release; `./verify.sh`, jobs `gates`/`rust` | contrato; `shape.md` · Por que não há mudança de CI / FR-004 |
| T005 | rastros e governança: R1 `go`, R4 antes de R7 `1.0.0-local`, **R6/taskstoissues N/A**; não-execução, sem promoção/deploy, PR #76 intocado | `.specify/memory/constitution.md`; `intake.md` |
| T006 | 4 ilhas com ids 1..=4 inseridas no helper via `adicionar_ilha` | `define.md` · SC-001 / FR-001 |
| T007 | 1 súdito com id 1 em `(0.0, 236.0)` inserido no helper | `define.md` · SC-001 / FR-001 |
| T008 | 3 nós via `POSICOES_NOS_LOCAIS` / `adicionar_no` | `simulacao.rs:2404-2407`; `shape.md` · Assumptions / FR-001 |
| T009 | rota `propaganda_no` por `TICKS_PROPAGANDA_NO` até `Vitoria`, sem mudança de regra | `simulacao.rs:2558-2593`; `define.md` · SC-002 / FR-002 |
| T010 | HUD deixa `Nós: 0/3` e passa a ler os 3 nós do espelho | `define.md` · SC-005 / FR-002 |
| T011 | verificação determinística: `hash_estado()` semeado difere do baseline | `lib.rs:2158-2206`; `define.md` · SC-003 / FR-003 |
| T012 | roteiro de validação (`./verify.sh`, jobs `gates`/`rust`) e release com BF-003a fechado / BF-003b aberto; **[NV]** neste assessment, sem promoção/deploy pelo agente | contrato; `define.md` · SC-004 / FR-004 |

Os nomes definitivos de SC-001..SC-006 e T001..T012 serão os de `specs/058-mirror-vitoria-local/` (em construção); a coluna "Eco" apenas rastreia a cobertura deste assessment. FR-001..FR-004 seguem o **mapeamento canônico da spec** e são idênticos em `intake.md`, `research.md`, `define.md` e `shape.md`. Todo número de vitória citado é `[HIPÓTESE]` (GDD:275); nada aqui é especificação, plano ou implementação.

Veredito: go
