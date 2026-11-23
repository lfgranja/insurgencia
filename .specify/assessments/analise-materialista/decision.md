# Decision — Análise Materialista (E do Intelectual)

- **Slug**: `analise-materialista`
- **Decided**: 2026-11-23T16:55Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | O GDD §10.2:554-559 descreve a habilidade E do Intelectual e o ciclo 052 abriu o slot E3, mas `Pnj` não tem marca, `Jogador` não tem recarga de análise, `ACOES_PERMITIDAS` não tem `analise_materialista` e `aplicar_dano_a_pnj` não aplica bônus: a regra existe só como linha `[HIPÓTESE]`. |
| Força da evidência | strong | `STATE.md:432-442` lido com o 80/20 (48/625, 24/625, 10/625); GDD §10.2:536-559 lido com todos os rótulos `[HIPÓTESE]`; `crates/sim/src/lib.rs` verificado (`ACOES_PERMITIDAS`, `nivel_minimo_para_acao`, `Jogador`, `Pnj`, `aplicar_dano_a_pnj`, `hash_estado`, `ValidacaoComandos`); ciclos 052/053 e constituição v1.2.2 lidos. |
| Valor vs. inação | strong | 48/625 à frente de demais W/E/R (24/625) e rede (10/625); sem o recorte, o slot E3 e os PNJs do ciclo 053 seguem desconectados, sem custo, janela, recarga ou bônus auditável por `hash_estado`. |
| Viabilidade / appetite | strong | `small`: cinco constantes, dois campos, uma ação e um nível na tabela, validação reaproveitada, bônus local em `aplicar_dano_a_pnj`, dois decrementos, hash, tecla `E`, linha `Análise: N` e cinco testes; sem `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, stack ou rede. |
| Alinhamento estratégico | strong | Preserva os rótulos: 20/12 m/15 s/−25%/5 s permanecem `[HIPÓTESE]` e 150/450 ticks e o bônus 25% são `escolha de protótipo`; respeita o corte em PNJs ativos, a constituição v1.2.2 (R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, R6 N/A) e ADR-004; W/R e demais E ficam para ciclos posteriores. |
| Postura de risco | strong | Riscos nomeados e mitigados por decisão: não-acumulação por reinício, cota de um por atacante/tick, `seq`/`request_id` só em aceito, piso zero preservado, decrementos sem `Instant`, hash em ordem estável, 5 testes novos com 131 preservados; R6 N/A sem issue simulada nem `gh` improvisado. |

## 80/20 (scorecard único do ciclo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Análise Materialista (E do Intelectual, marca em PNJ — escolhida) | 4×2×2×3 = 48/625 | **48/625** |
| Demais W/E/R (Escudo, Carga, Oficina, Fuga etc.) | 4×2×1×3 = 24/625 | 24/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério do contrato — impacto × confiança × facilidade ÷ risco, escala 1–5, denominador **625** (5⁴); o quarto fator é o risco invertido (segurança): 5 = risco mínimo, 1 = risco máximo. Este é o único scorecard do assessment; os mesmos valores aparecem em `intake.md`, `research.md` e `shape.md`, sem pontuação paralela.

A Análise Materialista vence porque é a única opção com **regra escrita no GDD e fronteira fechada no contrato simultaneamente**: o §10.2:554-559 fixou nível 3, 20 Fôlego, 12 m, −25%/5 s e não-acumulação, e o contrato 054 fixou a conversão para ticks, a API, o reaproveitamento de `atacar_pnj`/`aplicar_dano_a_pnj`/`hash_estado` e o corte em PNJs ativos, sem tocar em contrato, protocolo ou stack. Impacto **4** registra uma habilidade real que conecta o slot E3 do ciclo 052 aos PNJs do ciclo 053; confiança **2** e facilidade **2** registram honestamente que todos os números são `[HIPÓTESE]`/`escolha de protótipo`, que parte do efeito do GDD (jogadores, estruturas, visão por 10 s, execução 0,3 s) fica fora e que dois campos, uma ação, validação, tick, bônus e cliente são necessários; risco invertido **3** reflete a disciplina de tick, a não-acumulação, o hash e os testes ainda por verificar. Demais W/E/R (24/625) têm impacto e risco iguais, mas facilidade 1 por falta de corte fechado e de reaproveitamento imediato; rede (10/625) inverte a dependência determinismo → sincronização e está bloqueada por ADR-004.

## Veredito e justificativa

**Veredito: go** — Opção A (Análise Materialista: marca determinística em PNJs ativos), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 (`1.0.0-local`), R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, formatação, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. **Constantes rotuladas**: `CUSTO_ANALISE_MATERIALISTA = 20`, `ALCANCE_ANALISE_MATERIALISTA_M = 12.0`, `TICKS_ANALISE_MATERIALISTA = 150` (5 s a 30 Hz), `RECARGA_ANALISE_MATERIALISTA_TICKS = 450` (15 s) e `BONUS_DANO_ANALISE_PERCENT = 25` documentados como `[HIPÓTESE]`/`escolha de protótipo`, derivados de GDD §10.2:554-559; nenhum número é promovido a cânone e o contrato 054 não é reaberto.
2. **Campos e hash**: `Pnj.marca_ticks: i64` (0 = sem marca) e `Jogador.analise_recarga_ticks: i64` acrescentados e serializados por `hash_estado` em ordem estável, sem `Instant`/relógio externo.
3. **Ação e nível**: `"analise_materialista"` em `ACOES_PERMITIDAS` e nível 3 em `nivel_minimo_para_acao` (slot E3), reaproveitando o mapa Q1/W2/E3/R6 do ciclo 052.
4. **Validação**: `destino` na posição de PNJ **ativo**, distância jogador↔PNJ `<= 12 m`, Fôlego `>= 20` e recarga 0; recusa sem dano, sem débito e sem avanço de `seq`/`request_id` nos demais casos.
5. **Aplicação atômica e não-acumulação**: em comando aceito, debita 20, marca 150 ticks, inicia recarga 450 e avança `seq`/`request_id`; cota de um por atacante por tick; reaplicar **reinicia** para 150 ticks, nunca soma nem estende.
6. **Dano e piso zero**: `aplicar_dano_a_pnj` aplica `dano + dano * BONUS_DANO_ANALISE_PERCENT / 100` quando `marca_ticks > 0` (10 → 12), preservando `saturating_sub(...).max(0)`, a recusa de dano não positivo e a transição para inativo.
7. **Cliente e HUD**: tecla `E` (livre; `W` é movimento) agenda `analise_materialista` no PNJ ativo mais próximo dentro de 12 m, **sem comando sem alvo**; HUD acrescenta `Análise: N` (PNJs marcados ativos) e preserva todas as linhas/teclas existentes; nenhuma janela normal — cliente só com `--minimizado`.
8. **Testes e estilo**: 5 novos testes em `crates/sim/tests/simulacao.rs` (marca e bônus 10→12, não-acumulação, recarga 450, alcance 12 m, hash), 131 existentes preservados; `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`.
9. **Fronteira preservada**: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, stack, rede, Lightyear, sockets ou habilidades W/R; nenhuma mudança em `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`.
10. **Constituição v1.2.2**: R1 `go`, R3 0 clarificações, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Número promovido a cânone (GDD §10.2:554-559)**: 20/12 m/15 s/−25%/5 s são `[HIPÓTESE]`; 150/450 ticks e o bônus 25% são escolhas de protótipo. Se a spec promover qualquer valor a dogma, é violação deste assessment — não reabrir o contrato.
- **Acumulação indevida**: §10.2:559 veta marcas acumuladas; mitigado por decisão — reaplicar reinicia para 150 ticks, sem soma nem extensão; o teste de não-acumulação permanece **[NV]** até execução.
- **Disciplina de tick e hash**: risco invertido 3 (não 5) — decrementos em ordem determinística sem `Instant`, cota de um por atacante/tick e serialização estável de ambos os campos em `hash_estado` exigem verificação; permanecem **[NV]**.
- **Dano e piso zero**: o bônus 10 → 12 entra em `aplicar_dano_a_pnj` sem alterar `saturating_sub(...).max(0)` nem a recusa de dano não positivo; **[NV]** até execução.
- **Comando órfão**: a tecla `E` só agenda com PNJ ativo dentro de 12 m; nenhum débito sem alvo e nenhum avanço de `seq`/`request_id` em comando recusado.
- **Corte parcial do GDD**: jogadores, estruturas, visão por 10 s e execução de 0,3 s ficam fora; se a spec os incluir, viola a fronteira e o appetite `small`.
- **Demais W/E/R e rede**: não são pontuação a renegociar (24/625 e 10/625 no único scorecard); nenhum rastro FR/SC/T lhes é atribuído neste ciclo.
- **Verificação visual**: cliente somente com `--minimizado` e ausência de surface automatizável de captura tornam a linha `Análise: N` renderizada **[NV]**; a prova é estrutural. Não prometer screenshot real.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação geral**: gates, build, testes, formatadores, R4 e R7 (`1.0.0-local`) permanecem **[NV]** — nenhum foi executado por este agente; nenhum commit, push ou PR foi feito.

## Handoff — `/speckit.specify`

- **Problem**: o slot E3 aberto no ciclo 052 e os PNJs entregues no ciclo 053 permanecem desconectados — o GDD §10.2:554-559 descreve a habilidade, mas o Núcleo não tem marca, recarga, ação nem bônus de dano; a regra segue `[HIPÓTESE]` sem verdade observável por `hash_estado`.
- **Chosen approach**: Opção A — constantes rotuladas (20, 12 m, 150, 450, 25), `Pnj.marca_ticks` e `Jogador.analise_recarga_ticks` em `hash_estado`, `"analise_materialista"` em `ACOES_PERMITIDAS` com nível 3, validação reaproveitada (PNJ ativo, `<= 12 m`, Fôlego `>= 20`, recarga 0), aplicação atômica com um por atacante/tick e não-acumulação, bônus `dano + dano * 25 / 100` (10 → 12), decrementos determinísticos, tecla `E` e HUD `Análise: N`. Corte em **PNJs ativos**; todo número `[HIPÓTESE]`/`escolha de protótipo`.
- **In scope**: constantes e rótulos, dois campos e hash, ação e nível, validação e recusas, débito e marca atômicos, bônus de dano com piso zero, tick determinístico, cliente/HUD e 5 testes novos com 131 preservados. **Out of scope**: jogadores/estruturas/Nós, visão por 10 s, execução 0,3 s, magia, RNG, `Instant`, canalização, mapa, demais W/E/R, netcode/ADR-004, contratos/protocolo/Cargo/stack e **R6 (N/A)**.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: ordem exata dos decrementos e ponto de serialização no `hash_estado`; reaproveitamento integral do predicado de PNJ ativo e do cálculo de distância de `atacar_pnj`; critério determinístico de "mais próximo" no cliente em empate; conversão 5 s/15 s → 150/450 ticks a 30 Hz — validar na especificação e no roteiro de verificação.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | constantes rotuladas: 20 Fôlego, 12 m, 150 ticks (5 s), 450 ticks (15 s), bônus 25% | GDD §10.2:554-559; contrato da sessão; `intake.md` |
| FR-002 | `Pnj.marca_ticks` (0 = sem marca) e `Jogador.analise_recarga_ticks`, ambos em `hash_estado` | contrato; `crates/sim/src/lib.rs:359,232,1654`; `define.md` |
| FR-003 | `"analise_materialista"` em `ACOES_PERMITIDAS`, nível 3 em `nivel_minimo_para_acao`, validação PNJ ativo/`<= 12 m`/Fôlego `>= 20`/recarga 0 | `crates/sim/src/lib.rs:152-182,1725-1776`; `shape.md` · Opção A |
| FR-004 | débito 20, marca 150, recarga 450, `seq`/`request_id` só em aceito, um por atacante/tick, não-acumulação por reinício | contrato; `shape.md` · Opção A |
| FR-005 | `aplicar_dano_a_pnj`: `dano + dano * 25 / 100` com `marca_ticks > 0` (10 → 12), piso zero preservado | `crates/sim/src/lib.rs:546-556`; contrato; `shape.md` · Opção A |
| FR-006 | cliente (tecla `E`, PNJ ativo mais próximo, sem comando sem alvo, HUD `Análise: N`), 5 testes novos + 131 preservados, fronteira e governança | contrato; `.specify/memory/constitution.md`; `intake.md` |
| SC-001 | comando aceito debita 20, marca 150, inicia recarga 450 e avança ids; bônus 10 → 12 | `define.md` · Success Metrics 1-2 |
| SC-002 | reaplicação reinicia 150 ticks, sem acumulação nem extensão entre Intelectuais | `define.md` · Success Metrics 3 |
| SC-003 | recarga `> 0`, Fôlego `< 20`, sem PNJ ativo ou `> 12 m`: recusa sem dano, débito ou avanço de ids | `define.md` · Success Metrics 4 |
| SC-004 | um por atacante/tick; decrementos determinísticos sem `Instant`; campos em `hash_estado` | `define.md` · Success Metrics 5 |
| SC-005 | `E` agenda no PNJ ativo mais próximo, sem comando sem alvo; HUD `Análise: N` preserva linhas/teclas e `W` | `define.md` · Success Metrics 6 |
| SC-006 | 5 testes novos e 131 preservados; estilo Rust; nada em contratos/protocolo/Cargo/stack/rede/W/R | `define.md` · Success Metrics 7 |
| T001 | constantes rotuladas (20, 12 m, 150, 450, 25) documentadas | `shape.md` · Opção A |
| T002 | campo `Pnj.marca_ticks` (0 = sem marca) | `shape.md` · Opção A |
| T003 | campo `Jogador.analise_recarga_ticks` | `shape.md` · Opção A |
| T004 | ambos os campos serializados em `hash_estado`, ordem estável | `shape.md` · Assumptions |
| T005 | `"analise_materialista"` em `ACOES_PERMITIDAS` e nível 3 em `nivel_minimo_para_acao` | `shape.md` · Opção A |
| T006 | validação: PNJ ativo no `destino`, `<= 12 m`, Fôlego `>= 20`, recarga 0, recusas sem efeito | `shape.md` · Opção A |
| T007 | aplicação atômica: débito 20, marca 150, recarga 450, ids só em aceito, um por atacante/tick | `shape.md` · Opção A |
| T008 | não-acumulação: reaplicação reinicia para 150 ticks | `shape.md` · Assumptions |
| T009 | bônus no `aplicar_dano_a_pnj`: 10 → 12, piso zero e dano não positivo preservados | `shape.md` · Opção A |
| T010 | decrementos por tick determinísticos, sem `Instant` | `shape.md` · Assumptions |
| T011 | cliente: tecla `E`, PNJ ativo mais próximo dentro de 12 m, sem comando sem alvo, HUD `Análise: N` | `shape.md` · Opção A |
| T012 | 5 testes novos + 131 preservados; rastros e governança: R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**; não-execução | contrato; `.specify/memory/constitution.md`; `intake.md` |

Os nomes definitivos de T001..T012 serão os de `specs/054-analise-materialista/tasks.md` (ainda não criado no momento da leitura); a coluna "Eco" apenas rastreia a cobertura deste assessment, com validação prevista em `.specify/validations/analise-materialista/` e release em `.specify/releases/analise-materialista/`. **R6 é N/A** por constituição v1.2.2 e não recebe tarefa. Todo número de habilidade citado é `[HIPÓTESE]`/`escolha de protótipo`; nada aqui é especificação, plano ou implementação.

Veredito: go
