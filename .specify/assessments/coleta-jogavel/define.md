# Define — Coleta jogável no cliente

- **Slug**: `coleta-jogavel`
- **Created**: 2026-11-23T03:54Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §7.1; `crates/sim/src/lib.rs`; constituição v1.2.2)

## Problem Statement

A regra canônica da Fase 1 do pulso — 6 s em ponto neutro por +3 Suprimentos e +2 Materiais, com saída zerando o parcial (GDD §7.1:303,310) — já existe no Núcleo: `TICKS_COLETA=180`, os campos de jogador, a validação de destino, o crédito atômico e o `hash_estado`. Ainda assim, a mecânica não é **jogável**: o avanço ocorre apenas no tick em que chega um comando `coletar_materiais`, o que exige um comando por tick do cliente, e não há entrada `C` nem feedback de `Suprimentos`/`Coleta` no HUD. Sem auto-avanço determinístico, sem limits por jogador por tick e sem a superfície de entrada, o pilar Ação Coletiva / Materialismo Histórico permanece implementado no sim, mas invisível e inalcançável para o jogador; dois clientes também não teriam uma verdade observável e estável sobre o progresso da coleta.

## Affected Users & Stakeholders

- **Users**: jogadores no papel Camponês — precisam iniciar a coleta com uma tecla e ver o progresso até o crédito; jogadores das demais funções — precisam entender, pelo HUD, quando a renda de Materiais/Suprimentos ocorre e quando foi cancelada por movimento.
- **Stakeholders**: mantenedores do Núcleo Rust — precisam de avanço determinístico, no máximo um por jogador por tick, e `hash_estado` refletivo; mantenedores do cliente — precisam de um caminho de entrada simples (`C` just pressed) com ids monotônicos e de um HUD coerente com o já existente (Materiais, Barreiras, Construindo); SWE/orquestrador — precisa de uma fatia testável sem mudança de protocolo; designer — precisa que o número canônico não seja reaberto.

## Goals

- Manter a regra canônica **congelada**: 180 ticks a 1/30 s em ponto neutro → +3 Suprimentos e +2 Materiais; sair antes zera o acúmulo parcial.
- Acrescentar **auto-avanço determinístico** no Núcleo: após um comando válido `coletar_materiais`, a coleta avança 1 tick por jogador por tick enquanto o jogador permanece no mesmo ponto; `mover` para posição diferente cancela.
- Impedir dupla contagem: comando e auto-avanço no mesmo tick não podem somar dois ticks ao mesmo jogador (`BTreeSet<u64>`, padrão do ciclo 040).
- Preservar determinismo e rastreabilidade: `coleta_ticks`, `coleta_ponto`, `materiais` e `suprimentos` integralmente refletidos em `hash_estado`, em ordem estável.
- Tornar a ação jogável no cliente: tecla `C` em just pressed agenda `coletar_materiais` com `destino` = posição atual e `seq`/`request_id` monotônicos; HUD exibe `Suprimentos` e progresso de `Coleta` além de Materiais/Barreiras/Construindo.
- Corrigir o doc comment de `Comando.destino` para citar `mover`, `coletar_materiais` e `construir_barreira`.
- Manter a fronteira: nada em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`.

## Non-Goals

- Reabrir a regra canônica do GDD §7.1 ou substituir `ponto_igual` por tolerância de arredondamento.
- Criar entidade "ponto neutro" no contrato, nova ação de protocolo ou novo campo de rede — `Comando.destino` já serve como ID do ponto.
- Ponte provisória, barricada, combate, dano, IA, emboscada ou redução de defesa.
- Netcode, lobby, matchmaking, predição, reconciliação ou servidor Lightyear.
- R6: N/A por constituição v1.2.2 — dívida de automação, nunca issue simulada, nunca `gh` improvisado.
- Executar gates, build, testes, formatadores, commit, push ou PR neste assessment.

## Success Metrics

- Aos **180 ticks** exatos em ponto neutro, o jogador recebe **+3 Suprimentos e +2 Materiais** em único crédito atômico (baseline: comportamento existente no sim, mas sem auto-avanço — **[NV]** até verificação).
- Sair antes (mover para posição diferente) zera `coleta_ticks` e `coleta_ponto`; **nenhum** crédito parcial é concedido (baseline: **[NV]**).
- No máximo **1** tick de coleta por jogador por tick, mesmo com comando e auto-avanço no mesmo tick e mesmo com múltiplos comandos na fila (`BTreeSet<u64>`) (baseline: **[NV]**).
- Após um comando válido, o progresso avança sozinho **1 tick por tick** enquanto o jogador permanece no mesmo ponto e para quando ele sai (baseline: **[NV]**).
- `hash_estado` cobre `coleta_ticks`, `coleta_ponto`, `materiais` e `suprimentos` em ordem determinística; estados com progresso distinto produzem hashes distintos (baseline: **[NV]**).
- Cliente agenda a ação somente com `C` just pressed, `destino` = posição atual e `seq`/`request_id` monotônicos; HUD mostra `Suprimentos` e progresso de `Coleta` além de Materiais/Barreiras/Construindo; doc comment de `destino` cita as três ações (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, a coleta continua sendo uma regra correta porém inalcançável: o sim credita +3/+2 somente se o cliente enviar um comando a cada 1/30 s, o que nenhum jogador consegue, e o HUD não mostra nem Suprimentos nem progresso de Coleta. O materialismo histórico do MVP — a economia precedendo a ação militar (GDD §9.3:485) — segue apenas declarado, e não há superfície determinística para validar Núcleo + cliente antes de qualquer trabalho de rede. O custo é perder a única fatia com valor canônico já implementado pronta para verificação de ponta a ponta.

## Open Questions

- Nenhuma clarificação de design: a regra é fechada em §7.1 e o contrato fixa a API e a fronteira (R3 = 0 clarificações).
- **[NV]** Conformidade de R4 (antes de R7 `1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** O comportamento do auto-avanço, do `BTreeSet` e do `hash_estado` precisa de verificação em tempo de execução pelo orquestrador.
- **[NV]** Os nomes definitivos de T001..T012 são os de `specs/041-coleta-jogavel/tasks.md`, ainda em construção pelo `/speckit.specify`.

## Rastreabilidade (escopo do problema)

- FR-001 → regra canônica 6 s/180 ticks/+3/+2 e zeração do parcial; FR-002 → auto-avanço determinístico e guarda `BTreeSet`; FR-003 → `hash_estado` refletivo; FR-004 → validação de `destino` e ids monotônicos; FR-005 → entrada `C` no cliente; FR-006 → HUD e doc comment de `Comando.destino`.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis especificados em `specs/041-coleta-jogavel/`, fora desta fronteira.
- R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A** por constituição v1.2.2.
