# Implementation Plan — 053-pnj-defensores

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; R1 `go`; R4 (validate) antes de R7 (release `1.0.0-local`); `converge` append-only. **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação é registrada, não executada. Portaria confirmada em `AGENTS.md:70`.

**Fronteira explícita**: este agente escreve **somente** `specs/053-pnj-defensores/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/pnj-053/` (branch `053-pnj-defensores`, base `4975fd4`). Não toca `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, `crates/sim`, `crates/cliente`, documentos derivados, artefatos de validação nem código-fonte (salvo agentes de implementação na etapa 12, que recebem `crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs` e `crates/cliente/src/main.rs`). Não executa gates, build, testes ou formatadores; não faz commit/push/PR. Tudo o que depende de execução externa fica **[NV]** até o orquestrador.

**Rotulagem obrigatória**: o efetivo de **4 PNJs** e o dano de **8 dmg/s por 3 s** são `[HIPÓTESE]` em GDD §8.1:351,361 (convenção de números) e §8.4/§8.5 descrevem guardas/patrulhas; **24 PV** é a soma hipotética 8 × 3 s, e os **100 PV** do combatente são `[HIPÓTESE]` da justificativa de §8.1:361. **90 ticks por ataque** é derivação de 3 s × 30 Hz (`PASSO_SEGUNDOS = 1 / 30`), e **2 m** reaproveita `ALCANCE_CONTATO_M`; o **dano 10 do jogador** reutiliza `DANO_ATAQUE` da spec 047. A cadência pelo `tick` global com a guarda `tick > 0`, a injeção de quatro posições fixas sem mapa, a manutenção de PNJs inativos no `BTreeMap` e o mapeamento de `T` para a ação `atacar_pnj` são simplificações determinísticas, rotuladas como não-canônicas nos 4 artefatos. Novas habilidades W/E/R (GDD §10.2–10.6), mapa, rede e seleção de papel permanecem fora de escopo.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`.
2. **01 Intake**: sessão `20261123T155300Z-pnj-defensores`, título `insurgencia - pnjs defensores`; 80/20 = **4×2×2×3 = 48/625** (PNJs defensores) contra novas habilidades W/E/R **48/625** e rede **10/625** (bloqueada por ADR-004 até decisão da SE); decisão em `.specify/memory/STATE.md` (2026-11-23T15:53Z). O desempate com W/E/R cabe à âncora direta em GDD §8.4/§8.5 e ao reaproveitamento do estado determinístico existente.
3. **02 Research**: GDD §8.1:351 (convenção `[HIPÓTESE]`), §8.1:361 (4 forças de segurança, 8 dmg/s por 3 s = 24 pontos, 100 PV de combatente), §8.4:399-413 (Rota Inferior e PNJs neutros) e §8.5:416-436 (patrulha policial e negociação com PNJs); `.specify/memory/STATE.md` (decisão 15:53Z); `crates/sim/src/lib.rs` (`PASSO_SEGUNDOS = 1/30`, `Mundo`, `adicionar_jogador`, `fase_skills`, `fase_defesa`, `avancar_tick`, `ACOES_PERMITIDAS`, `nivel_minimo_para_acao`, `ValidacaoComandos::validar`, `CodigoRecusa`, `aplicar_dano_a_sudito`/`a_barreira`/`a_ponte`, `DANO_ATAQUE = 10`, `ALCANCE_CONTATO_M = 2.0`, `serializar_estruturas`, `hash_estado`); `crates/cliente/src/main.rs` (`texto_hud`, `posicao_no_mais_proximo`, handlers de tecla, `PEER_ID`); `crates/sim/tests/simulacao.rs` (suíte determinística existente).
4. **03 Define (problema)**: a sim já tem jogadores, Fôlego, ilhas, Súditos, Nós, barricadas e desfechos, mas não tem entidade PNJ, dano a PNJ, contra-ataque de guarda nem ação `atacar_pnj`; logo a oposição PVE de GDD §8.1/§8.4/§8.5 não é observável no estado determinístico. Faltam `Pnj`, `Mundo.pnjs`, `adicionar_pnj`, `aplicar_dano_a_pnj`, a ação `atacar_pnj`, as constantes de dano/cadência/alcance, a resolução em `fase_defesa`, o segmento `pnjs` no hash, a linha `PNJs: N` e o handler de `T`. Critérios observáveis por FR/SC, com todos os números rotulados como protótipo.
5. **04 Shape (concept)**: 3 opções avaliadas no scorecard (PNJs defensores, novas habilidades W/E/R, rede); recomendação = `pnj-defensores`, reaproveitando `BTreeMap`, `ValidacaoComandos`, `DANO_ATAQUE`, `ALCANCE_CONTATO_M` e `fase_defesa`, sem mudança de stack, sem mapa e sem protocolo.
6. **05 Decide**: scorecard **4×2×2×3 = 48/625**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A; critério impacto × confiança × facilidade ÷ risco, denominador 625.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006; regras canônicas distinguidas de todos os números `[HIPÓTESE]` e das derivações (24 PV, 90 ticks, 2 m), scorecard e fronteira explícita.
8. **07 Clarify**: 4 PNJs, 8 dmg/s por 3 s e 100 PV são hipóteses rotuladas, não cânone operacional; 24 PV = 8 × 3 s e 90 ticks derivam de 3 s a 30 Hz; o ataque usa `tick > 0 && tick % 90 == 0`, sem contador em `Pnj`; um PNJ morto no tick de ataque não contra-ataca; `Pnj` inativo permanece no mapa e o HUD conta só ativos; `atacar_pnj` é ação de nível 1 com `destino` no ponto do PNJ, não habilidade W/E/R; `DestinoInvalido` vem depois de par/replay/ação desconhecida/bloqueio de nível; `texto_hud` lê o espelho autoritativo e `hash_estado` cobre `pnjs`; `T` é a única tecla nova; R6 N/A ≠ dívida executada.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = **revisão de requisitos**, não implementação.
11. **Revisão do checklist**: FR/SC verificáveis por inspeção; 6 FR/6 SC; rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG linear; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: fora da fronteira deste agente, executado depois de `tasks` conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 em `crates/sim/src/lib.rs` (`Pnj` com exatamente `id`/`posicao`/`pv`/`ativo` documentados e `Clone + Copy + Debug`; `Mundo.pnjs: BTreeMap<u64, Pnj>` e `adicionar_pnj` com `PV_PNJ = 24`/`ativo = true` e `TOTAL_PNJS_PADRAO = 4`, rotulados; `aplicar_dano_a_pnj` com `saturating_sub`, `ativo = pv > 0`, retorno `true` na transição e entrada mantida; `ACOES_PERMITIDAS` com `atacar_pnj`, validação com `DestinoInvalido` para destino ausente/sem PNJ ativo, cota de um ataque por atacante/tick e `DANO_ATAQUE = 10`; `DANO_ATAQUE_PNJ = 8`, `TICKS_ATAQUE_PNJ = 90`, `ALCANCE_ATAQUE_PNJ_M = 2.0` e contra-ataque em `fase_defesa` com `tick > 0 && tick % 90 == 0`, ordem de `id`/`peer_id`, vida `saturating_sub(8).max(0)`; segmento `pnjs` em `serializar_estruturas`/`hash_estado`), em `crates/cliente/src/main.rs` (`PNJs: N` em `texto_hud`, handler `KeyT` com `posicao_pnj_mais_proximo`, dica de tecla e conservação das demais linhas/teclas) e em `crates/sim/tests/simulacao.rs` (cobertura determinística de 24→14→0, dano 0/negativo/id inexistente, cota, recusa, ticks 0/89/90/180, desempate e replay).
15. **13 Converge (append-only)**: primeira entrada datada em `specs/053-pnj-defensores/converge.md`; nunca editar converge de outros ciclos. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando documentalmente; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade, rollback (nunca executado), após R4.

> Os números 14–25 da sequência são reservados pela constituição para especificações de implementação não utilizadas neste ciclo; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/pnj-defensores/test-roadmap.md` (fora da fronteira deste agente). Superfície: inspeção de `crates/sim/src/lib.rs`, `crates/cliente/src/main.rs`, `crates/sim/tests/simulacao.rs` e da fronteira de arquivos; execução da suíte determinística e do binário com `--minimizado` pelo orquestrador; capturas de terminal e, se houver surface, do texto do HUD **quando aplicável**. Não há surface automatizável para captura visual real do HUD neste alvo headless: toda captura visual/tela é **[NV]** até verificação humana. Rollback documental (corrigir a fonte, nunca a expectativa; limpar capturas em `$TMP`).

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD`, `date -u` e listar os artefatos do ciclo; conferir presença dos 4 artefatos desta fronteira e a ordem `tasks` antes de `analyze` | base `4975fd4`, branch `053-pnj-defensores`, sessão `20261123T155300Z-pnj-defensores`, scorecard **48/625**; `STATE.md`, `AGENTS.md`, `concurrency.jsonl` e paths alheiros intactos; R6 fora do fluxo | sim (inspeção/terminal) |
| 2 | Conferir 6 FR e 6 SC na spec, o mapeamento um-para-um no checklist e a rotulação de todos os números como `[HIPÓTESE]`/protótipo nos 4 artefatos (FR-001 4 PNJs/24 PV, FR-004 8/90/2 m, FR-006 governança) | rastreabilidade FR-001..FR-006 idêntica à dos artefatos de governança; 4, 24, 10, 8, 90 e 2 m marcados como hipótese/derivação; nenhum número promovido a regra nova | sim (inspeção) |
| 3 | Inspecionar `Pnj`, `Mundo.pnjs`, `adicionar_pnj`, `PV_PNJ`, `TOTAL_PNJS_PADRAO` e `aplicar_dano_a_pnj` | quatro campos documentados, `pv = 24`/`ativo = true` na criação, entrada mantida após morte; constantes rotuladas `[HIPÓTESE]`; nenhum `Instant`/`SystemTime`, contador por PNJ, crate ou dependência novo | sim (inspeção) |
| 4 | Aplicar dano 0, negativo, 10 e 14 a um PNJ (24→14→0), repeats com `id` inexistente e reinserção via `adicionar_pnj` | 10 reduz para 14 sem matar; 14 leva a 0/`ativo = false` e retorna `true`; no-ops não mudam hash; reinserção restaura 24/`true`; sem pânico | sim (inspeção/cálculo); execução ao vivo **[NV]** |
| 5 | Validar `atacar_pnj` no nível 1 com destino válido, ausente, fora do ponto e em PNJ inativo; conferir dois comandos do mesmo atacante no tick | aceito com PNJ ativo no destino e 10 de dano aplicado; `DestinoInvalido` nos demais casos; no máximo um ataque por atacante/tick; `ACOES_PERMITIDAS` e mapa Q1/W2/E3/R6 preservados, sem habilidade nova | sim (inspeção/cálculo) |
| 6 | Simular ticks 0, 1, 89, 90, 180, 270 com PNJs ativos/inativos e 0/1/2 jogadores no alcance | nenhum dano em 0/1/89; 8 de dano em 90/180/270; desempate por menor `peer_id`; PNJ inativo não ataca; vida satura em 0; ordem de `id` determinística | sim (inspeção/cálculo) |
| 7 | Comparar `hash_estado` entre replays idênticos e após mudar apenas `pv`/`ativo` de um PNJ; executar a suíte determinística pelo orquestrador | hashes idênticos no replay; segmento `pnjs` em ordem de `id` muda o hash de forma previsível; segmentos existentes preservados; cobertura nova e existente passam | parcial (inspeção); gates/testes **[NV]** |
| 8 | Inspecionar `texto_hud`, `posicao_pnj_mais_proximo` e o handler de `KeyT`; conferir `PNJs: N` (só ativos) e a conservação das linhas e teclas | valor lido do espelho `Mundo`; quatro ativos e um inativo → `PNJs: 4`; teclas `WASD`/`B`/`P`/`C`/`G`/`R`/`F`/`Q` intactas, `T` agendando no PNJ mais próximo (desempate por menor `id`); nenhum campo de protocolo novo | sim (inspeção de código); HUD visual **[NV]** |
| 9 | Conferir que nenhum gate depende de Aleatoriedade, I/O, relógio externo, rede ou segundo jogador; verificar `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]` | ataque e cadência derivados apenas de `tick`, posição e estado do PNJ; estilo Rust v1.2.2 respeitado; `#[expect(...)]` como atributo de lint permanece permitido | sim (inspeção); gates **[NV]** |
| 10 | Governança, R6 N/A, rollback e rótulos: conferir `tasks` antes de `analyze`, R4 antes de R7 `1.0.0-local`, `converge` append-only, **[NV]** em toda captura e aplicar o rollback documentado | ordem v1.2.2 respeitada; R6 N/A (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado); nenhuma mudança em contratos/protocolo/Cargo/rede; nenhum build/gate/commit/push/PR executado por este agente; fonte corrigida se um passo falhar | parcial (revisão humana) |

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 entidade `Pnj`, `Mundo.pnjs`, `adicionar_pnj` e rotulação `[HIPÓTESE]` | 12 implement | T011 |
| FR-002 `aplicar_dano_a_pnj` com dano 10, PV 24 e `ativo = pv > 0` | 12 implement | T011 |
| FR-003 ação `atacar_pnj` no nível 1, validação `DestinoInvalido` e cota por tick | 12 implement | T011 |
| FR-004 dano do PNJ (8/90 ticks/2 m) em `fase_defesa`, sem contador em `Pnj` | 12 implement | T011 |
| FR-005 HUD `PNJs: N`, tecla `T`, segmento `pnjs` no hash e fronteira determinística | 12/27 | T011, T012 |
| FR-006 governança R1/R4/R6/[NV] e estilo Rust | 00/12/26 | T001, T011, T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Hipótese promovida a cânone | Rotulação explícita nos 4 artefatos (FR-001/SC-001, passo 2 do roteiro): 4 PNJs, 24 PV, dano 10/8, 90 ticks e 2 m são protótipo/derivação; GDD é lido, nunca reescrito |
| Cadência de ataque errada | `tick > 0 && tick % TICKS_ATAQUE_PNJ == 0` em `fase_defesa` (após `fase_skills`); cobertura 0/89/90/180/270 no passo 6 e em T011 |
| PNJ morto ainda ataca | `ativo = pv > 0` em `aplicar_dano_a_pnj` e checagem em `fase_defesa`: o PNJ morto no tick não contra-ataca; passos 4 e 6 |
| Morte invisível no hash | Entrada mantida no `BTreeMap` com `ativo = false` e segmento `pnjs` no hash; passos 4 e 7 |
| Dano duplo ao jogador ou ao PNJ | Cota de um ataque por atacante/tick em `fase_skills`; iteração única por PNJ em `fase_defesa`; passos 5 e 6 |
| `atacar_pnj` virar habilidade W/E/R | Ação nova apenas em `ACOES_PERMITIDAS`/validação/despacho, sem custo, alcance, recarga, ícone, sinal ou efeito; W/E/R continuam diferidos; passo 5 |
| `DestinoInvalido` mascarando outras recusas | Ordem de guardas explícita: par/replay/ação desconhecida → nível → destino; passo 5 |
| Hash não determinístico | `BTreeMap` por `id` e serialização estável de `pnjs`; novos segmentos previsíveis; passo 7 cobre replay |
| HUD sobrescreve informações existentes | Apenas uma linha acrescentada em `texto_hud`; demais linhas e teclas permanecem; `T` é a única tecla nova; passo 8 |
| Stack/protocolo/rede alterados | Fronteira explícita: nenhuma mudança em contratos, `PROTOCOL_VERSION`, Cargo, manifesto, CI ou rede; ADR-004 mantém rede bloqueada |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida de automação registrada |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) antes de R7 (T012) |
| Captura visual fabricada | Sem surface automatizável: HUD visual e execução ao vivo **[NV]**; apenas inspeção e terminal são marcados como automatizáveis |
| Fronteira estourada | Só 4 artefatos SDD escritos; `concurrency.jsonl`, `STATE.md`, `AGENTS.md`, sim, cliente e contratos intactos nesta etapa |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar gates, build, testes e formatador; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]`.
- Rodar a suíte determinística e, se aplicável, o binário com `--minimizado`, conduzindo o roteiro de 10 passos e registrando capturas de terminal/HUD quando houver surface; comparar notas com o estado ao vivo.
- Manter R6 fora do fluxo, a rede bloqueada por ADR-004, novas habilidades W/E/R diferidas e as regras dos ciclos 049–052 intactas; **todos os números rotulados como `[HIPÓTESE]`/protótipo**; R4 antes de R7 `1.0.0-local`.
