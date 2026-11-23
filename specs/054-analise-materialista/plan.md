# Implementation Plan — 054-analise-materialista

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; `converge` é **append-only**; R1 `go`; R4 (validate) **condicionado ao roteiro de verificação** e antes de R7 (release `1.0.0-local`). **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação é registrada, não executada. Portaria confirmada em `.specify/memory/constitution.md:22-26,48-50` e decisão 80/20 em `.specify/memory/STATE.md` (2026-11-23T16:55Z).

**Fronteira explícita**: este agente escreve **somente** `specs/054-analise-materialista/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/habilidades-054/` (branch `054-habilidades-wer`, base `5d2919`). Não toca `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, `AGENTS.md`, `.specify/assessments/analise-materialista/`, `.specify/validations/analise-materialista/`, `.specify/releases/analise-materialista/`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, GDD, manifesto, CI ou documentos derivados. O código de implementação (`crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs`, `crates/cliente/src/main.rs`) pertence a agentes de implementação na etapa 12. Não executa gates, build, testes ou formatadores; não faz commit/push/PR. Tudo o que depende de execução externa fica **[NV]** até o orquestrador. Nenhuma janela normal é aberta: cliente **somente com `--minimizado`**.

**Rotulagem obrigatória**: a âncora é GDD §10.2:554-559 (E do Intelectual: nível 3, 20 Fôlego, 12 m, −25% por 5 s, não-acumulação). **Todos os números do corte são `[HIPÓTESE]`/protótipo**: 3, 20, 12 m, 25%, 150 ticks (5 s × 30 Hz) e 450 ticks (15 s × 30 Hz), com `PASSO_SEGUNDOS = 1/30`. A execução de 0,3 s, a visibilidade de estruturas por 10 s, o bônus da Sabotagem Estrutural e a aplicação a jogadores/estruturas ficam fora do corte. A cadência por `tick`, a atribuição (nunca soma) da marca, a recarga por jogador, a escolha de `E` como tecla livre e a variante de recusa `AnaliseEmRecarga` são simplificações determinísticas rotuladas nos 4 artefatos. Scorecard **4×2×2×3 = 48/625** contra demais W/E/R **24/625** e rede **10/625** (bloqueada por ADR-004). Nenhuma habilidade W/R, magia, RNG, canalização, visibilidade, mapa, protocolo, Cargo, stack, rede, Lightyear ou sockets é introduzida.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`; `tasks` antes de `analyze`; `converge` append-only; R4 antes de R7 `1.0.0-local`.
2. **01 Intake**: sessão `20261123T165500Z-analise-materialista`, título `insurgencia - análise materialista`; 80/20 = **4×2×2×3 = 48/625** (Análise Materialista) contra demais W/E/R **4×2×1×3 = 24/625** e rede **5×2×1×1 = 10/625** (bloqueada por ADR-004 até decisão da SE); decisão em `.specify/memory/STATE.md` (2026-11-23T16:55Z).
3. **02 Research**: GDD §10.2:554-559 (regra e critérios de aceitação de E do Intelectual), §8.1:351 (convenção `[HIPÓTESE]`) e §10.1 (Fôlego/convenções); `.specify/memory/STATE.md` (decisão 16:55Z); `crates/sim/src/lib.rs` (`PASSO_SEGUNDOS = 1/30`, `Pnj` com `ataque_ticks`, `Jogador.folego`, `adicionar_pnj`, `adicionar_jogador`, `aplicar_dano_a_pnj`, `ACOES_PERMITIDAS`, `nivel_minimo_para_acao`, `ValidacaoComandos::validar`, `CodigoRecusa`, `fase_prepare`, `fase_skills`, `avancar_tick`, `serializar_estruturas`, `serializar_jogadores`, `hash_estado`, `DANO_ATAQUE = 10`); `crates/cliente/src/main.rs` (`texto_hud`, `posicao_pnj_mais_proximo`, `solicitar_atacar_pnj`, handlers de tecla, `PEER_ID`); `crates/sim/tests/simulacao.rs` (87 testes locais; 131 no total do repo); constituição v1.2.2. `specs/054-analise-materialista/analyze.md` e `converge.md` são lidos como insumo, não editados nesta fronteira.
4. **03 Define (problema)**: a sim já tem PNJs, Fôlego, `atacar_pnj` e dano com piso zero, mas não tem `marca_ticks`, `analise_recarga_ticks`, constantes de análise, ação `analise_materialista`, bônus de 25%, decaimento por tick, segmento de hash da marca, linha `Análise: N` nem tecla `E`; logo os critérios de GDD §10.2:557-559 não são observáveis no estado determinístico. Critérios observáveis por FR/SC, com todos os números rotulados como protótipo.
5. **04 Shape (concept)**: 3 opções avaliadas no scorecard (Análise Materialista, demais W/E/R, rede); recomendação = `analise-materialista`, reaproveitando `BTreeMap`, `ValidacaoComandos`, `DANO_ATAQUE`, `aplicar_dano_a_pnj`, `fase_skills` e `hash_estado`, sem stack, mapa, contrato ou janela nova.
6. **05 Decide**: scorecard **4×2×2×3 = 48/625**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A; critério impacto × confiança × facilidade ÷ risco, denominador 625.
7. **06 Specify**: substituir o stub por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006; regra canônica distinguida de todos os números `[HIPÓTESE]` e das derivações (150/450 ticks), com scorecard e fronteira explícita.
8. **07 Clarify**: nível 3 é o único novo limiar; 20/12 m/25%/5 s/15 s são rotulados, 150 e 450 derivam de 30 Hz; a marca é atribuída, não somada; o decaimento corre antes de `fase_skills` (`fase_prepare` → `avancar_ticks_analise` → `fase_skills`) e não usa relógio externo; a recarga é por jogador e a marca por PNJ; `AnaliseEmRecarga` é a recusa prototípica para recarga > 0; só PNJs ativos podem ser marcados; o bônus inteiro 10→12 incide em `aplicar_dano_a_pnj` com piso zero; `E` é livre e `W` continua movimento; cliente apenas `--minimizado`.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = **revisão de requisitos**, não implementação, execução ou verificação ao vivo.
11. **Revisão do checklist**: 6 FR/6 SC verificáveis por inspeção; rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança e aos achados A1..A10 de `analyze.md`; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG linear; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: A1..A10, convergência e veredito, **após `tasks`** conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa. Os insumos já presentes no diretório são lidos, não reescritos por esta fronteira.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 em `crates/sim/src/lib.rs` (`Pnj.marca_ticks`/`Jogador.analise_recarga_ticks` documentados e zerados na criação; cinco constantes `[HIPÓTESE]`; `"analise_materialista"` no nível 3 com validação na ordem par/replay/ação/nível/destino/fôlego/recarga, `AnaliseEmRecarga`, cota por atacante/tick e monotônicos só em aceito; `aplicar_analise_materialista` com débito 20, marca 150 por atribuição e recarga 450; `avancar_ticks_analise` antes de `fase_skills`, em ordem de `id`/`peer_id`; `aplicar_dano_a_pnj` com `dano + dano*25/100` quando marcado e piso zero; hash cobrindo marca e recarga), em `crates/cliente/src/main.rs` (`Análise: N`, tecla `E` com alvo a no máximo 12 m, linhas/teclas existentes preservadas, `--minimizado`) e em `crates/sim/tests/simulacao.rs` (5 testes novos nomeados, 131 existentes preservados).
15. **13 Converge (append-only)**: entrada datada em `specs/054-analise-materialista/converge.md`, nunca reescrita; **[NV]** para execução e capturas. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando **documentalmente e condicionado ao roteiro de 10 passos**; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade e rollback documentado (nunca executado), somente após R4.

> Os números reservados pela constituição para especificações de implementação não utilizadas neste ciclo permanecem vagos; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/analise-materialista/test-roadmap.md` (fora da fronteira deste agente). Superfície: inspeção de `crates/sim/src/lib.rs`, `crates/cliente/src/main.rs`, `crates/sim/tests/simulacao.rs` e da fronteira de arquivos; execução da suíte pelo orquestrador e do binário **somente com `--minimizado`**. Não há surface automatizável para captura visual real do HUD neste alvo headless: toda captura visual/tela é **[NV]** até verificação humana. Rollback documental: corrigir a fonte (spec/plano/código), nunca a expectativa; limpar capturas em `$TMP`.

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD`, `date -u`, listar os 4 artefatos e conferir a ordem `tasks` antes de `analyze` | base `5d2919`, branch `054-habilidades-wer`, sessão `20261123T165500Z-analise-materialista`, scorecard **48/625**; `STATE.md`, `AGENTS.md`, `concurrency.jsonl` e paths alheiros intactos; R6 fora do fluxo, `converge` append-only | sim (inspeção/terminal) |
| 2 | Conferir 6 US, 6 FR e 6 SC, o mapeamento um-para-um no checklist e a rotulação de 3/20/12 m/25%/150/450 como `[HIPÓTESE]` nos 4 artefatos | rastreabilidade FR-001..FR-006 idêntica à dos artefatos de governança; 150 = 5 s × 30 Hz e 450 = 15 s × 30 Hz; nenhum número promovido a cânone; GDD intacto | sim (inspeção) |
| 3 | Inspecionar `Pnj.marca_ticks`, `Jogador.analise_recarga_ticks`, as cinco constantes e a inicialização em `adicionar_pnj`/`adicionar_jogador` | campos `i64` documentados e zerados, inclusive na reinjeção; constantes 20/12.0/150/450/25 rotuladas; nenhum `HashMap`, uuid, `Instant`, RNG, crate ou estrutura paralela | sim (inspeção) |
| 4 | Validar `analise_materialista` no nível 3 com destino em PNJ ativo a 12 m, ausente/inativo/a 12,01 m, fôlego 19, recarga > 0, nível 2 e dois comandos do mesmo atacante no tick | aceita apenas no nível 3 com fôlego 20/recarga 0; recusas `AcaoBloqueada`/`DestinoInvalido`/`FolegoInsuficiente`/`AnaliseEmRecarga` na ordem documentada; cota de um por atacante/tick; monotônicos só em aceito; mapa Q1/W2/E3/R6 preservado | sim (inspeção/cálculo) |
| 5 | Aplicar uma análise aceita e uma reaplicação por outro atacante; percorrer os ticks `T`, `T+149`, `T+150`, `T+449`, `T+450` | débito 20, marca 150 e recarga 450 no tick `T`; reaplicação **reinicia** para 150, nunca 300; marca 1 em `T+149` e 0 em `T+150`; recarga 1 em `T+449` e 0 em `T+450`; sem decremento duplo nem valor negativo | sim (inspeção/cálculo) |
| 6 | Aplicar dano 10 a PNJ sem marca (24→14) e com marca (24→12), além de dano letal com marca e dano a PNJ inativo | bônus inteiro `10 + 10*25/100 = 12` uma única vez por chamada; sem marca permanece 10; piso zero e `ativo = pv > 0` preservados, entrada mantida, retorno `true` só na transição | sim (inspeção/cálculo) |
| 7 | Comparar `hash_estado` entre replays idênticos e após mudar apenas `marca_ticks`/`analise_recarga_ticks`; executar a suíte pelo orquestrador | hashes idênticos no replay; segmentos `pnjs` (ordem de `id`) e jogadores (ordem de `peer_id`) refletem a mudança; nenhum segmento existente removido; 5 testes novos e 131 existentes preservados | parcial (inspeção); gates/testes **[NV]** |
| 8 | Inspecionar `texto_hud`, o helper de seleção e o handler de `KeyCode::KeyE`; conferir `Análise: N` e a conservação das linhas/teclas | `N` conta só `ativo && marca_ticks > 0`; alvo mais próximo dentro de 12 m (desempate por menor `id`); sem alvo, nenhum comando; `W`/setas, `B`/`P`/`C`/`G`/`R`/`F`/`Q`/`T` intactos, `E` única tecla nova | sim (inspeção de código); HUD visual **[NV]** |
| 9 | Conferir ausência de janela normal (somente `--minimizado`), relógio externo, RNG, I/O, rede ou surface de protocolo; verificar `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]` | execução headless; estilo Rust v1.2.2 respeitado; `#[expect(...)]` como atributo de lint permitido; nenhuma alteração em contratos, Cargo, CI, Lightyear ou sockets | sim (inspeção); gates **[NV]** |
| 10 | Governança, R6 N/A, rollback e rótulos: conferir `tasks` antes de `analyze`, R4 condicionado e antes de R7 `1.0.0-local`, `converge` append-only e **[NV]** em toda captura | ordem v1.2.2 respeitada; R6 N/A (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado); nenhum build/gate/commit/push/PR executado por este agente; fonte corrigida se um passo falhar | parcial (revisão humana) |

## Correspondência dos achados A1..A10 (`analyze.md`) aos 6 FR

| Achado | FR |
|:--|:--|
| A1 âncora GDD §10.2:554-559 e rotulação `[HIPÓTESE]` | FR-001 |
| A2 campos `marca_ticks` e `analise_recarga_ticks` | FR-001 |
| A3 cinco constantes 20/12.0/150/450/25 | FR-001 |
| A4 ação nível 3, validação, cota e monotônicos | FR-002 |
| A6 não-acumulação, reaplicação e decaimento por tick | FR-003 |
| A5 bônus inteiro 10→12 com piso zero | FR-004 |
| A7 hash com marca e recarga | FR-005 |
| A8 HUD `Análise: N`, tecla `E`, fronteira `--minimizado` | FR-005 |
| A9 cinco testes novos, 131 existentes e estilo Rust | FR-006 |
| A10 governança, `tasks` antes de `analyze`, R6 N/A e **[NV]** | FR-006 |

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 campos, constantes e rotulação `[HIPÓTESE]` | 12 implement | T011 |
| FR-002 ação `analise_materialista` nível 3, validação e cota | 12 implement | T011 |
| FR-003 aplicação 20/150/450, não-acumulação e decaimento | 12 implement | T011 |
| FR-004 bônus 10→12 em `aplicar_dano_a_pnj`, piso zero | 12 implement | T011 |
| FR-005 HUD `Análise: N`, tecla `E`, hash e fronteira headless | 12/27 | T011, T012 |
| FR-006 cinco testes, 131 existentes, governança e estilo | 00/12/26 | T001, T011, T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Hipótese promovida a cânone | Rotulação explícita nos 4 artefatos (FR-001/SC-001, passo 2): 3/20/12 m/25%/150/450 vêm de GDD §10.2:554-559 e de 30 Hz; GDD é lido, nunca reescrito |
| Marca acumulada (300 ticks) | `marca_ticks = TICKS_ANALISE_MATERIALISTA` por atribuição, nunca `+=`; cobertura no passo 5 e no teste de não-acumulação |
| Decremento duplo ou relógio externo | `avancar_ticks_analise()` uma vez por tick, após `fase_prepare()` e antes de `fase_skills()`; sem `Instant`/`sleep`/RNG; ticks `T+149`/`T+150`/`T+449`/`T+450` no passo 5 |
| Recarga burlada ou confundida com nível | Recarga por jogador, `AnaliseEmRecarga` separada de `AcaoBloqueada`; ordem de guardas explícita; passo 4 |
| Bônus aplicado duas vezes ou a PNJ sem marca | Cálculo único em `aplicar_dano_a_pnj`, só com `marca_ticks > 0`; aritmética inteira; piso zero preservado; passo 6 |
| Dano duplo por atacante/tick | Cota de uma análise por atacante/tick, consumida só por comando aceito; passo 4 |
| Hash cego à marca | `marca_ticks` no segmento `pnjs` (ordem de `id`) e `analise_recarga_ticks` em jogadores (ordem de `peer_id`); passo 7 |
| `E` conflitando com `W`/`T` ou gerando comando sem alvo | `KeyE` nova e livre; alvo filtrado por `ativo` e 12 m, desempate por menor `id`; sem alvo, `seq`/`request_id` locais não avançam; passo 8 |
| HUD sobrescrevendo linhas existentes | Apenas `Análise: N` acrescentada e dica de teclas estendida; demais linhas/teclas permanecem; passo 8 |
| Ação virar habilidade W/R ou efeito de visibilidade | Ação apenas no sim, sem canalização, magia, RNG, visibilidade, ícone ou campo de protocolo; passos 4 e 9 |
| Stack/protocolo/rede alterados ou janela normal aberta | Fronteira explícita: nenhuma mudança em contratos, `PROTOCOL_VERSION`, Cargo, manifesto, CI ou rede; ADR-004 mantém rede bloqueada; cliente somente `--minimizado`; passos 9 e 10 |
| R6 adotada ou issue simulada | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; dívida registrada |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) condicionado e antes de R7 (T012); passo 10 |
| Captura visual fabricada | Sem surface automatizável: HUD visual e execução ao vivo **[NV]**; apenas inspeção e terminal marcados como automatizáveis |
| Fronteira estourada | Só 4 artefatos SDD escritos nesta fronteira; `concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `assessments`, `validations`, `releases`, sim, cliente e contratos intactos |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar gates, build, testes e formatador; conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]`; verificar os 5 testes novos e os 131 existentes.
- Rodar o roteiro de 10 passos e o binário **somente com `--minimizado`**, registrando capturas de terminal/HUD quando houver surface; nenhuma janela normal. Comparar notas com o estado ao vivo, sem afirmar verde sem execução.
- Manter R6 fora do fluxo, a rede bloqueada por ADR-004, W/R e as demais E diferidas, e as regras dos ciclos 049–053 intactas; **todos os números rotulados como `[HIPÓTESE]`/protótipo**; R4 condicionado e antes de R7 `1.0.0-local`.
- Corrigir qualquer divergência na fonte (spec, plano, checklist, tasks ou código), nunca afrouxando expectativas de teste; rollback documentado, nunca executado neste ciclo.
