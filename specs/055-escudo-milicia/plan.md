# Implementation Plan — 055-escudo-milicia

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` precede `analyze` (READ-ONLY)**; `converge` é **append-only**; R1 `go`; R4 (validate) **condicionado ao roteiro de verificação** e antes de R7 (release `1.0.0-local`). **R6 é N/A por constituição v1.2.2**: fora da sequência executável enquanto `resolve-tasks.sh` e o adaptador de issues não existem; se invocada, **PARE e escale ao SWE** — nunca simular issue, nunca `gh` improvisado, nunca alegar execução. A dívida de automação permanece registrada como BF-007. **Promoção `dev→main` e deploy são SWE-exclusivos**; este ciclo termina no escalonamento, não em merge em `main`.

**Fronteira explícita**: este agente escreve **somente** `specs/055-escudo-milicia/{spec.md, plan.md, checklists/requirements.md, tasks.md}` no worktree `.specify/queue/worktrees/escudo-055/` (branch `feature/055-escudo-milicia`, base `3f3b01c`). Não toca `.specify/queue/concurrency.jsonl`, `.specify/memory/STATE.md`, `AGENTS.md`, `.specify/assessments/escudo-milicia/`, `.specify/validations/escudo-milicia/`, `.specify/releases/escudo-milicia/`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `Cargo.lock`, GDD, manifesto, CI ou documentos derivados. O código de implementação (`crates/sim/src/lib.rs`, `crates/sim/tests/simulacao.rs`, `crates/cliente/src/main.rs`) pertence a agentes de implementação na etapa 12. Não executa gates, build, testes, formatadores, commits, pushes ou PRs; execução e capturas ficam **[NV]** até o orquestrador, com cliente somente `--minimizado`.

**Rotulagem obrigatória**: a âncora é GDD §10.6:687-692 (W do Guerreiro: 25 Fôlego, 0,3 s, 3 m, 250 de absorção por 5 s, recarga 14 s, +15% contra dano contínuo, alvo único). **Todos os números do corte são `[HIPÓTESE]`/protótipo**: 2, 25, 3 m, 250, 150 ticks (5 s × 30 Hz), 420 ticks (14 s × 30 Hz) e 15%, com `PASSO_SEGUNDOS = 1/30`. O nível 2 vem da convenção Q1/W2/E3/R6 de §10.1. O bônus de 15% contra dano contínuo é **reservado e documentado**: o Núcleo não tem gás lacrimogêneo nem dano contínuo, logo a constante `RESISTENCIA_CONTINUA_ESCUDO_PERCENT` não é aplicada e esse critério do GDD fica fora da DoD. A cadência por tick, a atribuição (nunca soma) da absorção, a rota única `aplicar_dano_a_jogador`, a recarga por conjurador, a escolha de `V` como tecla livre e a variante de recusa `EscudoEmRecarga` são simplificações determinísticas rotuladas nos 4 artefatos. Scorecard **4×4×5×3×4 = 960/3125** contra mapa/rotas **216/3125**, rede **50/3125** (bloqueada por ADR-004) e rastreabilidade **0** (já fechada). Nenhum número é promovido a cânone e o GDD é lido, nunca reescrito.

## Passos (00 → 26)

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência executável acima; R6 **N/A** (não executada, não simulada); R1 `go`; `tasks` antes de `analyze`; `converge` append-only; R4 antes de R7 `1.0.0-local`; promoção `dev→main` e deploy SWE-exclusivos.
2. **01 Intake**: sessão `20261123T215500Z-escudo-milicia`, título `insurgencia - escudo de milicia`; 80/20 = **960/3125** (`escudo-milicia`, BF-002a) contra mapa/rotas **216/3125**, rede **50/3125** (inelegível por ADR-004) e rastreabilidade 053/054 **0** (inelegível, já fechada); critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 5⁵ = 3125; decisão em `.specify/memory/STATE.md` (2026-11-23T22:00Z).
3. **02 Research**: GDD §10.6:687-692 (regra e critérios de aceitação de W do Guerreiro), §10.1 (Q1/W2/E3/R6) e §8.1:351 (convenção `[HIPÓTESE]`); `.specify/memory/STATE.md` (decisão 22:00Z e DoD); `.specify/memory/constitution.md` v1.2.2; `crates/sim/src/lib.rs` (`PASSO_SEGUNDOS = 1/30`, `Jogador`, `adicionar_jogador`, `vida`/`folego`/`combate_ticks`/`analise_recarga_ticks`, `ACOES_PERMITIDAS`, `nivel_minimo_para_acao`, `ValidacaoComandos::validar`, `CodigoRecusa`, `aplicar_ataque`, `aplicar_ataque_pnj`, `fase_prepare`/`fase_skills`/`fase_defesa`/`fase_ataque_pnjs`/`avancar_tick`, `avancar_analises_materialistas`, `serializar_jogadores`/`hash_estado`, `DANO_ATAQUE = 10`, `DANO_ATAQUE_PNJ = 8`); `crates/cliente/src/main.rs` (`texto_hud`, `posicao_analise_materialista`, `solicitar_analise_materialista`, handlers de tecla, `PEER_ID`); `crates/sim/tests/simulacao.rs` (**136 testes** existentes).
4. **03 Define (problema)**: a sim já tem jogadores, Fôlego, ataque PvP e ataque de PNJ com piso zero, mas não tem `escudo_pv`/`escudo_ticks`/`escudo_recarga_ticks`/`escudo_alvo`, constantes de escudo, ação `escudo`, rota única de dano ao jogador, absorção, decaimento por tick, segmento de hash do escudo, linha `Escudo: N (ticks)` nem tecla `V`; logo os critérios de GDD §10.6:689-690 não são observáveis no estado determinístico. Critérios observáveis por FR/SC, com todos os números rotulados como protótipo.
5. **04 Shape (concept)**: 4 opções avaliadas no scorecard; recomendação = `escudo-milicia`, reaproveitando `BTreeMap`, `ValidacaoComandos`, `aplicar_ataque`, `fase_ataque_pnjs`, `avancar_tick`, `hash_estado` e `texto_hud`, sem stack, mapa, contrato ou janela nova. A resistência de 15% a dano contínuo fica de fora por inexistência do fenômeno no Núcleo.
6. **05 Decide**: scorecard **960/3125**, `Veredito: go` (R1), 0 clarificações em aberto (R3), R6 N/A; critério impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹, denominador 3125.
7. **06 Specify**: substituir o diretório vazio por `spec.md` com US1–US6, FR-001..FR-006 e SC-001..SC-006; regra canônica distinguida de todos os números `[HIPÓTESE]` e das derivações (150/420 ticks), com scorecard, limitação do dano contínuo e fronteira explícita.
8. **07 Clarify**: nível 2 é o único novo limiar; 25/3 m/250/5 s/14 s/15% são rotulados e 150/420 derivam de 30 Hz; a absorção é **atribuída**, não somada; `escudo_alvo` é um único `peer_id` (self = próprio `peer_id`); o dano passa pela rota única `aplicar_dano_a_jogador` antes de `vida`; o decaimento corre antes de `fase_skills` (`fase_prepare` → `avancar_escudos` → `fase_skills` → `fase_ataque_pnjs`) e não usa relógio externo; a recarga é por conjurador com recusa `EscudoEmRecarga`; o efeito incide em PvP e no ataque de PNJ com piso zero; `V` é livre e `W`/`E`/`T` permanecem; cliente apenas `--minimizado`.
9. **08 Plan**: este arquivo; passos 00→26, fronteira explícita, roteiro de 10 passos, R4 antes de R7 `1.0.0-local`, R6 N/A.
10. **09 Checklist**: um item `[x]` por FR-001..FR-006 em `checklists/requirements.md`; `[x]` = **revisão de requisitos**, não implementação, execução ou verificação ao vivo.
11. **Revisão do checklist**: 6 FR/6 SC verificáveis por inspeção; rastreabilidade FR-001..FR-006 alinhada aos artefatos de governança e aos achados A1..A10 de `analyze.md` no mapa canônico; aprovado para `tasks`.
12. **10 Tasks**: `tasks.md` com T001..T012, IDs únicos e DAG linear; `tasks` antes de `analyze`; R4 antes de R7.
13. **11 Analyze (READ-ONLY)**: A1..A10, convergência e veredito, **após `tasks`** conforme v1.2.2; nenhuma edição de spec/plano/checklist/código nesta etapa. Os insumos já presentes no diretório são lidos, não reescritos por esta fronteira.
14. **12 Implement (código Rust)**: agentes de implementação aplicam FR-001..FR-006 em `crates/sim/src/lib.rs` (quatro campos documentados e zerados em `adicionar_jogador`; seis constantes `[HIPÓTESE]`, com a de 15% reservada; `"escudo"` no nível 2 com validação na ordem par/replay/ação/nível/alvo/fôlego/recarga, `EscudoEmRecarga`, cota por conjurador/tick e monotônicos só em aceito; `aplicar_escudo` com débito 25, 250/150 no protegido e 420/`Some(alvo)` no conjurador por atribuição; `avancar_escudos` antes de `fase_skills`, em ordem de `peer_id`, com limpeza de `escudo_pv`/`escudo_alvo`; `aplicar_dano_a_jogador` substituindo o dano direto em `aplicar_ataque` e `fase_ataque_pnjs`; hash cobrindo os quatro campos), em `crates/cliente/src/main.rs` (`Escudo: {pv} ({ticks})`, tecla `V` com alvo aliado a no máximo 3 m ou self) e em `crates/sim/tests/simulacao.rs` (cinco testes nomeados).
15. **13 Converge (append-only)**: entrada datada em `specs/055-escudo-milicia/converge.md`, nunca reescrita; **[NV]** para execução e capturas. Fora da fronteira deste agente.
16. **27 Validate (R4)**: `validate.md` aprovando **documentalmente e condicionado ao roteiro de 10 passos**; gates/build/testes **[NV]** até execução pelo orquestrador; R4 antes de R7. Registro em `.specify/validations/escudo-milicia/`.
17. **26 Release (R7)**: `release.md` versão `1.0.0-local`, changelog por FR/T, rastreabilidade e rollback documentado (nunca executado), somente após R4. Registro em `.specify/releases/escudo-milicia/`.

> Os números reservados pela constituição para especificações de implementação não utilizadas neste ciclo permanecem vagos; a cadeia executável vai de 00 a 26, com `27 validate` imediatamente antes de `26 release`.

## Roteiro de verificação (10 passos)

Escrever antes do PR em `.specify/validations/escudo-milicia/test-roadmap.md` (fora da fronteira deste agente). Superfície: inspeção de `crates/sim/src/lib.rs`, `crates/cliente/src/main.rs`, `crates/sim/tests/simulacao.rs` e da fronteira de arquivos; execução da suíte (`./verify.sh`, esperado **141 testes**) pelo orquestrador e do binário **somente com `--minimizado`**. Não há surface automatizável para captura visual real do HUD neste alvo headless: toda captura visual/tela é **[NV]** até verificação humana. Rollback documental: corrigir a fonte (spec/plano/código), nunca a expectativa; limpar capturas em `$TMP`.

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | `git rev-parse HEAD`, `date -u`, listar os 4 artefatos e conferir a ordem `tasks` antes de `analyze` | base `3f3b01c`, branch `feature/055-escudo-milicia`, sessão `20261123T215500Z-escudo-milicia`, scorecard **960/3125**; `STATE.md`, `AGENTS.md`, `concurrency.jsonl` e paths alheiros intactos; R6 fora do fluxo, promoção/deploy SWE-exclusivos | sim (inspeção/terminal) |
| 2 | Conferir 6 US, 6 FR e 6 SC, o mapeamento um-para-um no checklist e a rotulação de 2/25/3 m/250/150/420/15% como `[HIPÓTESE]` nos 4 artefatos | rastreabilidade FR-001..FR-006 idêntica à dos artefatos de governança; 150 = 5 s × 30 Hz e 420 = 14 s × 30 Hz; 15% reservado sem efeito; nenhum número promovido a cânone; GDD intacto | sim (inspeção) |
| 3 | Inspecionar os quatro campos em `Jogador`, as seis constantes e a inicialização em `adicionar_jogador` | campos `i64`/`Option<u64>` documentados e zerados/`None`, inclusive na reinjeção; constantes 25/250/150/420/3.0/15 rotuladas; nenhum `HashMap`, uuid, `Instant`, RNG, crate ou estrutura paralela | sim (inspeção) |
| 4 | Validar `escudo` no nível 2 com alvo próprio/aliado a 3 m, ausente/PNJ/sem jogador/a 3,01 m, fôlego 24, recarga > 0, nível 1 e dois comandos do mesmo conjurador no tick | aceita apenas no nível 2 com fôlego 25/recarga 0; recusas `AcaoBloqueada`/`AlvoInvalido`/`FolegoInsuficiente`/`EscudoEmRecarga` na ordem documentada; cota de um por conjurador/tick; monotônicos só em aceito; mapa Q1/W2/E3/R6 preservado | sim (inspeção/cálculo) |
| 5 | Aplicar um escudo aceito e uma reaplicação por outro conjurador; desferir dano 250 e 300 contra o protegido | débito 25, 250/150 no protegido e 420/`Some(alvo)` no conjurador; reaplicação **reinicia** para 250/150, nunca 500/300; 250 totalmente absorvido com vida intacta; 300 absorve 250 e desconta 50; retorno e `combate_ticks` preservados | sim (inspeção/cálculo) |
| 6 | Percorrer os ticks `T`, `T+149`, `T+150`, `T+419` e `T+420` e aplicar dano de PNJ (8 em 90 ticks) antes e depois da expiração | duração 150→1→0 e recarga 420→1→0; `escudo_pv` zerado e `escudo_alvo = None` nos pontos exatos; ataque de PNJ absorvido em `T` e não absorvido em `T+150`; sem decremento duplo nem valor negativo | sim (inspeção/cálculo) |
| 7 | Comparar `hash_estado` entre replays idênticos e após mudar apenas um dos quatro campos; executar a suíte pelo orquestrador | hashes idênticos no replay; segmento de jogadores (ordem de `peer_id`) reflete cada campo; nenhum segmento existente removido; 5 testes novos e 136 existentes preservados (141) | parcial (inspeção); gates/testes **[NV]** |
| 8 | Inspecionar `texto_hud`, o helper de seleção e o handler de `KeyCode::KeyV`; conferir `Escudo: N (ticks)` e a conservação das linhas/teclas | linha lida de `PEER_ID` no espelho, `Escudo: 0 (0)` sem escudo; aliado mais próximo dentro de 3 m (desempate por menor `peer_id`) ou self; PNJ nunca alvo; `W`/setas, `B`/`P`/`C`/`G`/`R`/`F`/`Q`/`E`/`T` intactos, `V` única tecla nova | sim (inspeção de código); HUD visual **[NV]** |
| 9 | Conferir ausência de janela normal (somente `--minimizado`), relógio externo, RNG, I/O, rede, dano contínuo ou surface de protocolo; verificar `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]` | execução headless; estilo Rust v1.2.2 respeitado; `#[expect(...)]` como atributo de lint permitido; 15% permanece reservado; nenhuma alteração em contratos, Cargo, CI, Lightyear ou sockets | sim (inspeção); gates **[NV]** |
| 10 | Governança, R6 N/A, rollback e rótulos: conferir `tasks` antes de `analyze`, R4 condicionado e antes de R7 `1.0.0-local`, `converge` append-only e **[NV]** em toda captura | ordem v1.2.2 respeitada; R6 N/A (PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado); promoção/deploy não executados; nenhum build/gate/commit/push/PR executado por este agente; fonte corrigida se um passo falhar | parcial (revisão humana) |

## Correspondência dos achados A1..A10 (`analyze.md`) aos 6 FR

| Achado | FR |
|:--|:--|
| A1 âncora GDD §10.6:687-692, nível 2, rotulação `[HIPÓTESE]` e 15% fora da DoD | FR-001 |
| A2 quatro campos em `Jogador` (15% reservado) | FR-002 |
| A3 seis constantes `25`/`250`/`150`/`420`/`3.0`/`15` | FR-002 |
| A4 ação `escudo` nível 2, validação, cota e monotônicos | FR-003 |
| A5 aplicação 25/250/150/420, não-acumulação e alvo único | FR-003 |
| A6 rota única de dano: absorção em PvP e PNJ com piso zero | FR-004 |
| A7 decaimento, expiração em `T+150` e recarga em `T+420` | FR-004 |
| A8 hash cobrindo os quatro campos | FR-005 |
| A9 HUD `Escudo: N (ticks)`, tecla `V`, cinco testes, 136 existentes e estilo Rust | FR-006 |
| A10 governança, `tasks` antes de `analyze`, R6 N/A e **[NV]** | FR-006 |

> Observação: `analyze.md` rotula o par HUD/tecla como FR-006, enquanto `test-roadmap.md` o agrupa sob FR-005; a redação canônica de `spec.md`, deste plano, do checklist e de `tasks.md` segue o mapa acima, coerente com `define.md`, `research.md`, `shape.md`, `validate.md` e `release.md`: FR-005 = hash, FR-006 = cliente + testes + estilo + fronteira/governança. A cobertura do roteiro permanece idêntica em ambos os rótulos.

## Rastreabilidade (FR → etapa/T)

| FR | Etapa | T |
|---|---|---|
| FR-001 regra, números `[HIPÓTESE]` e limitação do dano contínuo | 01/02/07 | T002, T004, T005 |
| FR-002 quatro campos e seis constantes (15% reservado) | 12 implement | T011 |
| FR-003 ação `escudo` nível 2, validação, cota, aplicação e alvo único | 12 implement | T011 |
| FR-004 rota única de dano, absorção, decaimento, expiração e recarga | 12 implement | T011 |
| FR-005 hash cobrindo os quatro campos | 12/27 | T011, T012 |
| FR-006 cliente `V`/HUD, cinco testes, 136 existentes, estilo e governança | 00/12/26 | T001, T011, T012 |

## Riscos e mitigação

| Risco | Mitigação |
|---|---|
| Hipótese promovida a cânone | Rotulação explícita nos 4 artefatos (FR-001, passo 2): 2/25/3 m/250/150/420/15% vêm de GDD §10.6:687-692, §10.1 e de 30 Hz; GDD é lido, nunca reescrito |
| Dano contínuo inventado para justificar o 15% | `RESISTENCIA_CONTINUA_ESCUDO_PERCENT` reservada e documentada, sem chamada, campo ou teste; limitação registrada nas clarificações e no passo 9 |
| Absorção acumulada (500 PV) ou dois alvos | `escudo_pv`/`escudo_ticks`/`escudo_alvo` por atribuição, nunca `+=`; recarga 420 > duração 150; cobertura no passo 5 e no teste de reaplicação |
| Decremento duplo ou relógio externo | `avancar_escudos()` uma vez por tick, após `fase_prepare()` e antes de `fase_skills()`; sem `Instant`/`sleep`/RNG; ticks `T+149`/`T+150`/`T+419`/`T+420` no passo 6 |
| Recarga burlada ou confundida com nível | Recarga por conjurador, `EscudoEmRecarga` separada de `AcaoBloqueada`; ordem de guardas explícita; passo 4 |
| Absorção duplicada ou dano direto remanescente | `aplicar_dano_a_jogador` como única rota, chamada por `aplicar_ataque` e `fase_ataque_pnjs`; piso zero preservado; passos 5 e 6 |
| `V` conflitando com `W`/`E`/`T` ou gerando comando sem alvo | `KeyV` nova e livre; alvo filtrado por jogador existente e 3 m, desempate por menor `peer_id`, self como fallback; sem alvo, `seq`/`request_id` locais não avançam; passo 8 |
| HUD sobrescrevendo linhas existentes | Apenas `Escudo: N (ticks)` acrescentada e dica de teclas estendida; demais linhas/teclas permanecem; passo 8 |
| Hash cego ao escudo | Quatro campos no segmento de jogadores, em ordem de `peer_id`; passo 7 |
| Ação virar habilidade extra ou efeito de magia | Ação apenas no sim, sem canalização, magia, RNG, visibilidade, ícone ou campo de protocolo; passos 4 e 9 |
| Stack/protocolo/rede alterados ou janela normal aberta | Fronteira explícita: nenhuma mudança em contratos, `PROTOCOL_VERSION`, Cargo, manifesto, CI ou rede; ADR-004 mantém rede bloqueada; cliente somente `--minimizado`; passos 9 e 10 |
| R6 adotada, issue simulada ou promoção indevida | R6 N/A por v1.2.2; PARE se invocada; nenhum `taskstoissues`/`gh` improvisado; `dev→main` e deploy SWE-exclusivos |
| Ordem invertida por hábito | `tasks` (T009) antes de `analyze` (T010); R4 (T011) condicionado e antes de R7 (T012); passo 10 |
| Captura visual fabricada | Sem surface automatizável: HUD visual e execução ao vivo **[NV]**; apenas inspeção e terminal marcados como automatizáveis |
| Fronteira estourada | Só 4 artefatos SDD escritos nesta fronteira; `concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `assessments`, `validations`, `releases`, sim, cliente e contratos intactos |

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)

- Executar `./verify.sh` e os cinco gates locais (completude SDD, DAG, task-id, converge, hub docs); conferir `forbid(unsafe_code)`, `deny(clippy::pedantic)` e ausência de `unwrap`/`.expect()`/`#[allow]`; verificar os 5 testes novos e os 136 existentes (total **141**).
- Rodar o roteiro de 10 passos e o binário **somente com `--minimizado`**, registrando capturas de terminal/HUD quando houver surface; nenhuma janela normal. Comparar notas com o estado ao vivo, sem afirmar verde sem execução.
- Manter R6 fora do fluxo, a rede bloqueada por ADR-004, o bônus de 15% reservado e as regras dos ciclos 049–054 intactas; **todos os números rotulados como `[HIPÓTESE]`/protótipo**; R4 condicionado e antes de R7 `1.0.0-local`.
- Corrigir qualquer divergência na fonte (spec, plano, checklist, tasks ou código), nunca afrouxando expectativas de teste; rollback documentado, nunca executado neste ciclo. Promover `dev→main` e publicar somente por decisão do SWE.
