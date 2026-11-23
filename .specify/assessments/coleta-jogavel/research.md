# Research — Coleta jogável no cliente

**Data**: 2026-11-23T03:54Z · Evidências coladas do contrato da sessão, do GDD e do código verificado no checkout; sem reexecução de gates, build ou testes.

## Evidência primária

- [source: `docs/GDD - Insurgência.md` §7.1:303, lido] Pulso de autogestão a cada 30 s; Fase 1 "Coletar": Camponês em ponto neutro, decisão "ficar e arriscar emboscada ou recuar", recompensa "3 Suprimentos + 2 Materiais", tempo alvo "6 s". (confidence: high)
- [source: `docs/GDD - Insurgência.md` §7.1:310, lido] Critério canônico: "Um Camponês que permanece 6 s em ponto neutro recebe exatamente 3 Suprimentos e 2 Materiais no pulso; sair antes zera o acúmulo parcial." Pilar Ação Coletiva / Materialismo Histórico: nenhuma recompensa por abate isolado. (confidence: high)
- [source: `crates/sim/src/lib.rs:11-18`, lido] `PASSO_SEGUNDOS = 1.0/30.0` e `TICKS_COLETA = 180` — a conversão 6 s = 180 ticks é a do relógio corrente. (confidence: high)
- [source: `crates/sim/src/lib.rs:39-45`, lido] `ACOES_PERMITIDAS` já contém `"coletar_materiais"`: não há nova ação de protocolo. (confidence: high)
- [source: `crates/sim/src/lib.rs:61-72`, lido] `Jogador` já contém `materiais`, `suprimentos`, `coleta_ticks` e `coleta_ponto` — o estado da regra canônica existe. (confidence: high)
- [source: `crates/sim/src/lib.rs:104-105`, lido] `Comando.destino: Option<(f64, f64)>` tem doc comment "Destino opcional, usado por `mover`." — incompleto: o mesmo campo é usado por `coletar_materiais` e `construir_barreira`, conforme validação em `:504-513`. É o alvo da correção documental do ciclo. (confidence: high)
- [source: `crates/sim/src/lib.rs:213-268`, lido] `fase_skills` já declara `coleta_avancada: BTreeSet<u64>`, despacha `coletar_materiais` para `aplicar_coleta`, e `aplicar_coleta` guarda destino = posição, reinicia o acúmulo se o ponto mudou, incrementa `coleta_ticks`, insere o `peer_id` no conjunto e credita +3/+2 ao atingir 180. O avanço, porém, só ocorre **no tick em que há comando**. (confidence: high)
- [source: `crates/sim/src/lib.rs:302-319`, lido] `avancar_construcoes_automaticas` é o padrão do ciclo 040: varre `jogadores` em `BTreeMap`, avança 1 tick por jogador por tick enquanto o ponto de construção é o atual, registra o `peer_id` no `BTreeSet` e cancela se saldo ou posição mudam. É o espelho exato a aplicar à coleta. (confidence: high)
- [source: `crates/sim/src/lib.rs:407-408`, lido] `hash_estado` já serializa `coleta_ticks` e `coleta_ponto` (com os estoques), em ordem estável de `BTreeMap`. (confidence: high)
- [source: `crates/sim/src/lib.rs:220-229`, lido] `mover` para posição diferente já zera `coleta_ticks`, `coleta_ponto`, `construcao_ticks` e `construcao_ponto`. (confidence: high)
- [source: contrato da sessão, imutável] O ciclo acrescenta **auto-avanço determinístico**: após um comando válido `coletar_materiais`, a coleta avança 1 tick por jogador por tick enquanto o jogador permanece no mesmo ponto; `mover` para posição diferente cancela; comando no mesmo tick não conta duas vezes (`BTreeSet<u64>`). Cliente: tecla `C` em just pressed agenda comando com `destino` = posição atual e `seq`/`request_id` monotônicos; HUD mostra `Suprimentos` e progresso de `Coleta` além de Materiais/Barreiras/Construindo; doc comment de `Comando.destino` cita `mover`, `coletar_materiais`, `construir_barreira`. (confidence: high)
- [source: contrato da sessão, imutável] Worktree `.specify/queue/worktrees/coleta-cliente-041/`, branch `041-coleta-jogavel`, base `bf05cfa`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012; nenhuma mudança em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`; sem `unsafe`, `unwrap`, `expect`, `#[allow]`; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, `max_width=100`. (confidence: high)
- [source: `.specify/memory/constitution.md:23-26,48-50,78-79`, lido] Constituição v1.2.2: R1 `go`; R4 antes de R7 `1.0.0-local`; `analyze` READ-ONLY, `converge` append-only; **R6 N/A** — ratificada como intenção, mas fora da sequência executável até existir o adaptador de issues: PARE se invocada, nunca simular issue, nunca `gh` improvisado. (confidence: high)
- [source: `.specify/assessments/coleta-materiais-deterministica/decision.md`, lido] Ciclo 039 pontuou a coleta no Núcleo em 180/625 (impacto 5, confiança 4, facilidade 3, segurança 3). O estado agora existe e está auditado, o que eleva confiança e facilidade a 5 no ciclo 041. (confidence: high)
- [source: `.specify/assessments/barricada-deterministica/decision.md`, lido] Ciclo 040 pontuou a barricada em 400/625 e fixou o padrão `BTreeSet<u64>`/auto-avanço em `fase_skills`; a ponte provisória ficou em 108/625 e netcode em 10/625 — os mesmos valores do scorecard deste ciclo. (confidence: high)

## Interpretação

A coleta é a regra melhor ancorada do GDD e, desde o ciclo 039, também a melhor ancorada do sim: valor canônico explícito em §7.1, constante, campos, validação, crédito atômico e hash já presentes em `crates/sim/src/lib.rs`. Falta transformar esse estado em **jogabilidade contínua e observável**: o avanço hoje depende de um comando a cada tick, o que torna a mecânica injogável para um humano e deixa o cliente sem meios de iniciá-la ou acompanhá-la. A contribuição 80/20 é, portanto, de integração determinística, não de design: espelhar `avancar_construcoes_automaticas` para a coleta (1 tick/jogador/tick, guarda `BTreeSet`, cancelamento por `mover`), ligar a tecla `C` e o HUD no cliente e corrigir o doc comment de `destino`. Nenhuma decisão de design fica em aberto e nenhum campo de protocolo é acrescentado — daí confiança 5 e facilidade 5, com impacto 5 e risco 4 (500/625, o maior valor do scorecard).

## Alternativas rejeitadas (mesmo scorecard único)

1. **Só corrigir o doc comment de `destino`** — rejeitada como entrega isolada (250/625): é necessária e barata, mas impacto 2; sem o auto-avanço e a entrada `C`, a regra canônica continua injogável. No ciclo 041 ela entra como condição incluída, não como opção escolhida.
2. **Ponte provisória** — rejeitada (108/625): GDD §10.3:574 marca a ponte como `[SUPOSIÇÃO]` e §9.3:475 só lhe atribui 15 Materiais; não há valores canônicos nem SC próprios, e ela desvia o foco da Fase 1 do pulso.
3. **Servidor + Lightyear (netcode)** — rejeitada (10/625): sincronizar exige antes uma fatia jogável e determinística; GDD §16.3:1490 pede Núcleo isolado, não rede primeiro. Risco máximo (fator 1) e facilidade mínima (fator 1).

## Riscos registrados (não reabertos)

- **Dupla contagem no mesmo tick**: mitigada pelo `BTreeSet<u64>` local, já usado em `aplicar_coleta` e em `avancar_construcoes_automaticas`; o auto-avanço e o comando do mesmo tick não podem somar dois ticks ao mesmo jogador.
- **Divergência de ponto**: `ponto_igual` (comparação por `total_cmp`) é a forma canônica; `mover` para posição diferente zera `coleta_ticks`/`coleta_ponto`, preservando "sair antes zera o acúmulo parcial".
- **Determinismo do hash**: os campos de coleta e estoque já entram em `hash_estado`; qualquer mudança de ordem de iteração deve permanecer a do `BTreeMap` de `jogadores`.
- **Dívida R6 (automação, N/A)**: fora da sequência executável na v1.2.2; nenhuma issue simulada e nenhum `gh` improvisado. Não é dívida de produto.
- **Verificação**: gates, build, testes, R4 e R7 permanecem **[NV]** — nenhum foi executado neste assessment.

## Rastreabilidade

Fatos colados como observados, sem reexecução. Nenhum comando de gate, build, git remoto, GitHub ou emissão de issue foi executado por este agente. Tudo o que depende de execução permanece **[NV]**. Rastros do ciclo: FR-001..FR-006 (regra/auto-avanço/hash/validação/cliente/HUD+doc), SC-001..SC-006 (critérios observáveis), T001..T012 (entregáveis); a definição canônica vive em `specs/041-coleta-jogavel/`. R6 = N/A por constituição v1.2.2.
