# Intake — Coleta jogável no cliente

**Data**: 2026-11-23T03:54Z · **Slug**: `coleta-jogavel` · **Sessão**: `session_id=20261123T035400Z-coleta-jogavel`, `session_title=insurgencia - coleta jogavel no cliente`.

## Ideia

Capturar, sem tocar em código, a fatia 80/20 do ciclo 041: tornar **jogável no cliente** a coleta determinística de materiais cuja regra já vive no Núcleo. A regra canônica do GDD §7.1 permanece intacta — **6 s = 180 ticks a 1/30 s em ponto neutro → +3 Suprimentos e +2 Materiais**; sair antes **zera o acúmulo parcial**. O Núcleo já contém `TICKS_COLETA=180`, os campos `materiais`/`suprimentos`/`coleta_ticks`/`coleta_ponto`, a validação de destino, o guarda `BTreeSet<u64>` contra dupla contagem no mesmo tick e a cobertura de `hash_estado`; o ciclo acrescenta o **auto-avanço determinístico**: após um comando válido `coletar_materiais`, a coleta avança **1 tick por jogador por tick** enquanto o jogador permanece no mesmo ponto, `mover` para posição diferente cancela, e um comando no mesmo tick não conta duas vezes (padrão `BTreeSet<u64>` do ciclo 040, espelhando `avancar_construcoes_automaticas`). No cliente, a tecla `C` em *just pressed* agenda um comando `coletar_materiais` com `destino` = posição atual e `seq`/`request_id` monotônicos; o HUD passa a exibir `Suprimentos` e o progresso de `Coleta`, além de Materiais/Barreiras/Construindo. O doc comment de `Comando.destino` é corrigido para citar `mover`, `coletar_materiais` e `construir_barreira`.

**Motivação**: fechar o laço da Fase 1 do pulso de autogestão (GDD §7.1) como mecânica observável de ponta a ponta — regra no Núcleo, avanço determinístico por tick, entrada jogável e feedback no HUD — sem alterar `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`.

## Contexto do ciclo (observado no contrato da sessão; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/coleta-cliente-041/`, branch `041-coleta-jogavel`, base `bf05cfa`.
- Alvo de especificação `specs/041-coleta-jogavel`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012.
- Constituição v1.2.2: R1 `go`, R4 antes de R7 `1.0.0-local`; **R6 N/A** — não pertence à sequência executável enquanto não existir o adaptador de issues: dívida de automação, nunca issue simulada, nunca `gh` improvisado.
- Estado verificado no checkout: `crates/sim/src/lib.rs` já expõe `TICKS_COLETA`, a ação `coletar_materiais`, os campos de coleta/estoque, `aplicar_coleta` com guarda `BTreeSet` e `avancar_construcoes_automaticas`; não há ainda `avancar_coletas_automaticas`, entrada `C` nem HUD de Coleta/Suprimentos.

## Scorecard único do ciclo

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Coleta no cliente (escolhida) | 5×5×5×4 = 500/625 | **500/625** |
| Só corrigir o doc comment de `destino` | 2×5×5×5 = 250/625 | 250/625 |
| Ponte provisória | 3×3×3×4 = 108/625 | 108/625 |
| Servidor + Lightyear (netcode) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade × risco-invertido, escala 1–5, denominador 5⁴ = 625 (o quarto fator é a segurança: 5 = risco mínimo, 1 = risco máximo). Este é o **único** scorecard do assessment; `research.md`, `shape.md` e `decision.md` repetem os mesmos valores, sem pontuação paralela.

Escolhe-se a coleta no cliente: é o maior valor do scorecard (500/625) e a única opção que transforma uma regra canônica já implementada no Núcleo em fatia jogável e verificável por `hash_estado`. "Só o doc comment" (250/625) é correção necessária, mas irrelevante como entrega isolada; a ponte provisória (108/625) é `[SUPOSIÇÃO]` no GDD §10.3:574, sem valores canônicos próprios; netcode (10/625) sincronizaria uma fatia que ainda não é jogável. Contraste rastreável: o ciclo 039 pontuou a coleta no Núcleo em 180/625 (`.specify/assessments/coleta-materiais-deterministica/decision.md`); com o estado já no sim e o contrato fechado, confiança e facilidade sobem para 5 e o ciclo 041 chega a 500/625.

## Evidências de entrada (coladas; não reexecutadas)

- GDD §7.1:303 — Fase 1 "Coletar", Camponês em ponto neutro, "3 Suprimentos + 2 Materiais", "6 s".
- GDD §7.1:310 — "Um Camponês que permanece 6 s em ponto neutro recebe exatamente 3 Suprimentos e 2 Materiais no pulso; sair antes zera o acúmulo parcial."
- `crates/sim/src/lib.rs` (lido): `PASSO_SEGUNDOS = 1.0/30.0`, `TICKS_COLETA = 180`, `ACOES_PERMITIDAS` com `coletar_materiais`, `Comando.destino: Option<(f64,f64)>` documentado hoje como "usado por `mover`", `Jogador` com `materiais`/`suprimentos`/`coleta_ticks`/`coleta_ponto`, `aplicar_coleta` com `BTreeSet<u64>`, `avancar_construcoes_automaticas`, `hash_estado` com os campos de coleta.
- Contrato da sessão (imutável): auto-avanço 1 tick/jogador/tick no mesmo ponto, cancelamento por `mover`, ids monotônicos no cliente, HUD com `Suprimentos` e `Coleta`, correção do doc comment; nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/coleta-jogavel/`. Não executa gates, build, testes ou formatadores; não faz commit/push/PR; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` nem código-fonte. Tudo o que depende de execução permanece **[NV]**. R6 não é invocada — N/A por constituição v1.2.2.
