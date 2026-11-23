# Research — Defesa de barricada (autogestão)

**Data**: 2026-11-23T05:47Z · Evidências coladas do contrato da sessão, do GDD, de STATE.md, da constituição e da ADR-004, sem reexecução.

## Evidência primária

- [source: `docs/GDD - Insurgência.md` §7.1:306, lido] Tabela do pulso de 30 s, fase 4 "Defender": "Guerreiro na barricada, Artesão reforça", decisão "segurar o ponto ou perseguir a emboscada", recompensa "**+1 ponto de autogestão a cada 10 s de barricada intacta**", tempo alvo 8 s. (confidence: high)
- [source: `docs/GDD - Insurgência.md` §6.2:273, lido] "A condição de vitória é estabelecer a autogestão na Ilha Central. A ilha não é explodida; é convertida." A defesa de barricada alimenta diretamente a tese de vitória por conversão, não por demolição. (confidence: high)
- [source: `docs/GDD - Insurgência.md` §10.3:574,577-579, lido] Barricada do ciclo 040: 200 PV, 12 s de duração, bloqueia passagem e projéteis; "Materiais, canalização, pontos de vida e duração são canônicos"; Fôlego/alcance/recarga/dimensões são `[HIPÓTESE]`; ponte provisória é `[SUPOSIÇÃO]`. (confidence: high)
- [source: contrato da sessão, imutável] Conversão determinística a 30 ticks/s: 10 s = **300 ticks**; a barricada 040 dura 360 ticks. API Rust prevista: campo `Jogador.pontos_autogestao: i64`, campo `Barreira.defesa_ticks: i64`, fase determinística que incrementa 1/tick enquanto a barricada está intacta e o dono está no mesmo ponto, creditando +1 e zerando o contador ao completar 300; HUD exibe `Autogestão`. (confidence: high)
- [source: contrato da sessão, imutável] Regra de zeração: se o dono sai do ponto, a barricada é destruída (`pv == 0`) ou expira (`ticks_restantes == 0`), o acúmulo zera e **não credita** — nenhum crédito parcial em 299 ticks. (confidence: high)
- [source: `.specify/memory/STATE.md:304-315`, lido] Decisão 80/20 do ciclo 044 (2026-11-23T05:47Z): I/C/F/R = 4/4/4/4 → 256/625 para defesa de barricada; 3/3/3/4 → 108/625 ponte; 4/2/2/3 → 48/625 Súditos/Propaganda; 5/2/1/1 → 10/625 rede; escolha `defesa-barricada`, sem mudança de stack/dependência/protocolo. (confidence: high)
- [source: `.specify/memory/constitution.md:48-50,79-80,84`, lido] Constituição v1.2.2: R1 `go`, R4 antes de R7, `analyze` READ-ONLY após `tasks`; R6 removida da sequência executável pela emenda de 23/11 — **R6 N/A**. (confidence: high)
- [source: `docs/adr/004-migracao-bevy-lightyear.md:15-24,53`, lido] ADR-004 Accepted (20/11) adota Bevy 0.20.1 + Lightyear 0.31, mas "nenhuma troca adicional de engine é permitida sem novo ADR"; Lightyear consta apenas em ADR/docs, ausente de `Cargo.toml`/`Cargo.lock` (verificado 21/11, via `docs/PROGRESSO.md:326`). Rede bloqueada até decisão da SE. (confidence: high)
- [source: `.specify/assessments/barricada-deterministica/decision.md:61-91`, lido] Ciclo 040 entregou constantes, `Barreira`, `Mundo.barreiras`, fase de skills e hash determinístico; a defesa 044 reaproveita essa base, sem reabrir o conflito de rótulo §16.3/H4. (confidence: high)

## Interpretação

A defesa de barricada é o fechamento natural do ciclo 040: a barricada já existe como estado determinístico (200 PV, 360 ticks, coberta por `hash_estado`) e o GDD já define o que ela rende (§7.1:306: +1 autogestão/10 s). O trabalho 80/20 não é criar mecânica nova, mas tornar **observável e auditável** a passagem do tempo sob defesa: um contador por barricada, um crédito no jogador e a zeração sem crédito quando a condição de posse/ruptura falha. Nenhuma decisão de design fica pendente: o número é canônico, a conversão é determinística e a fronteira é interna ao Núcleo e ao HUD. Por isso a opção pontua 4 em confiança (o valor é canônico, mas a presença do defensor é simplificada no protótipo) e 4 em impacto (conecta loop de sessão à condição de vitória, sem entregar a vitória em si).

Consequência da duração: em 360 ticks de vida da barricada cabe no máximo **1 crédito** (300 ticks); um segundo exigiria 600 ticks, além da expiração. Isso é coerente com o GDD — a recompensa é por janela de defesa, e uma nova barricada reinicia o ciclo. A coluna "tempo alvo 8 s" de §7.1:306 é diretriz de pacing, não valor de crédito; prevalece a regra canônica de 10 s (300 ticks).

## Alternativas rejeitadas

1. **Rede (Lightyear/async/sockets)** — rejeitada (10/625): ADR-004 declarou a stack, mas Lightyear está ausente de `Cargo.toml`/`Cargo.lock` e "nenhuma troca adicional de engine é permitida sem novo ADR" (ADR-004:53); introduzir transporte exigiria decisão da SE. Impacto 5, mas confiança 2, facilidade 1 e risco invertido 1.
2. **Súditos/Propaganda** — rejeitada (48/625): a conversão de Nós e os 100 pontos de Resistência Ideológica são `[HIPÓTESE]` (GDD §6.2:275); depende de Propaganda canalizada de 20 s e do Intelectual, fora do recorte e sem valor canônico de recompensa por defesa.
3. **Ponte provisória** — rejeitada (108/625): GDD §10.3:574 marca a ponte como `[SUPOSIÇÃO]`; herda o custo de 15 Materiais de §9.3:475, mas não tem recompensa de autogestão própria nem rastro SC dedicado.

## Riscos registrados (não reabertos)

- **Simplificação dono-defensor**: o GDD §7.1:306 prevê Guerreiro na barricada e Artesão reforçando; o protótipo usa o dono no mesmo ponto. Decisão registrada em `STATE.md:315`; não estender papéis, funções ou hierarquia neste ciclo.
- **Risco invertido 4 (não 5)**: a confiança e a facilidade são altas, mas a disciplina de tick (1 incremento por barreira por tick) e a ordem estável do `hash_estado` exigem contrato explícito; por isso o fator risco/segurança é 4, não 5.
- **Dívida R6**: R6 é **N/A** por constituição v1.2.2 — removida da sequência executável; se um fluxo tentar invocá-la, PARE e escale ao SWE, sem `gh` improvisado e sem issue simulada.
- **Verificação**: gates, testes, R4 e R7 permanecem **[NV]** — nenhum foi executado por este agente.

## Rastreabilidade

Fatos colados como observados, sem reexecução. Nenhum comando de gate, build, git remoto, GitHub ou emissão de issue foi executado por este agente. Rastros do ciclo: FR-001..FR-006 (constantes/estado/condição/fase/zeração/HUD), SC-001..SC-006 (critérios observáveis), T001..T012 (entregáveis); definição canônica em `specs/044-defesa-barricada/`, scorecard em `.specify/memory/STATE.md:304-315`.
