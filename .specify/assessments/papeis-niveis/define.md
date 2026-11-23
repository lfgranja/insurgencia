# Define — Papéis, Níveis e Desbloqueio Q/W/E/R

- **Slug**: `papeis-niveis`
- **Created**: 2026-11-23T15:08Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (GDD §10:504-524 e §10.2–§10.5, STATE.md ciclo 052, constituição v1.2.2)

## Problem Statement

O GDD §10.1:515-524 descreve papéis materiais e uma progressão curta — nível 1–12 com desbloqueio Q no 1, W no 2, E no 3 e R no 6 — mas o Núcleo Rust não possui `funcao` nem `nivel` em `Jogador`, e `ACOES_PERMITIDAS` aceita `atacar_forte`, `construir_barreira`, `construir_ponte`, `propaganda`, `propaganda_no` e `atacar_no` desde o primeiro tick. Jogador, servidor e cliente não compartilham uma verdade observável sobre qual papel material está em jogo, qual é o nível corrente ou quais ações deveriam estar disponíveis; a regra de progressão existe apenas como linha de tabela **`[HIPÓTESE]`**, sem estado determinístico nem recusa de comando. Sem isso, a progressão pensada para partidas curtas (§10.1:523) não pode ser verificada e o cliente não pode exibir a identidade material do jogador.

## Affected Users & Stakeholders

- **Users**: jogadores da Comuna — precisam que nível e papel material sejam previsíveis e visíveis (`Função: <nome>`, `Nível: N/12`) e que o desbloqueio siga a sequência escrita no GDD; jogadores que testam limites — precisam que uma ação de nível superior seja recusada de forma explícita (`AcaoBloqueada`), não silenciosamente ignorada.
- **Stakeholders**: mantenedores do Núcleo Rust — precisam de progressão por tick com fronteira exata (899/900), teto 12 e `hash_estado` refletivo; SWE/orquestrador — precisa de uma fatia testável sem mudança de contrato, protocolo, Cargo ou stack; designer — precisa que teto, desbloqueio e ritmo de 30 s continuem rotulados `[HIPÓTESE]`, e que as habilidades completas do §10.2–§10.5 não sejam incorporadas "de graça".

## Goals

- Introduzir `enum Funcao` com os cinco papéis materiais do GDD §10.1 (Artesão, Guerreiro, Intelectual, Camponês, Agitador) e os campos `Jogador.funcao` e `Jogador.nivel`, com nível inicial **1**.
- Fixar `TICKS_POR_NIVEL = 900` (**30 s a 30 Hz**, `[HIPÓTESE]`/protótipo) e `NIVEL_MAXIMO = 12`, com ganho de **+1 a cada 900 ticks** e paragem no teto — sem ganho antes de 900 e sem ultrapassar 12.
- Mapear o desbloqueio das **ações já existentes**, todas `[HIPÓTESE]` conforme GDD §10.1:522: nível 1 → `atacar_forte` (Q); nível 2 → `construir_barreira` e `construir_ponte` (W); nível 3 → `propaganda` e `propaganda_no` (E); nível 6 → `atacar_no` (R).
- Fazer `ValidacaoComandos::validar` rejeitar comando de ação bloqueada com o novo `CodigoRecusa::AcaoBloqueada`, mantendo a ordem e os demais códigos de recusa existentes.
- Refletir `funcao` e `nivel` em `hash_estado`, em posição estável, para que a progressão seja auditável por determinismo.
- Exibir no cliente `Função: <nome>` e `Nível: N/12` **sem novas teclas**, reaproveitando o fluxo de comandos já existente.
- Preservar a compatibilidade: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, dependências, `async`, sockets ou Lightyear; estilo `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`.

## Non-Goals

- Novas habilidades W/E/R do GDD §10.2–§10.5 — Oficina de Campo, Análise Materialista, Plano Mestre, Fuga pelas Sombras, Espalhar Rumores, Sabotagem Estrutural etc. W/E/R aqui significam **apenas** o desbloqueio das ações existentes mapeadas no contrato.
- Valores de Fôlego (100, 10/s fora de combate, 5/s em combate), alcances (2/6/12/25 m), recargas e dimensões do §10.1–§10.5 — todos `[HIPÓTESE]` e não necessários à progressão mínima.
- Escolha/justificativa do valor padrão de `Funcao` como regra canônica: o default é `[HIPÓTESE]` a confirmar na spec (candidato `Artesao`), não uma decisão de design fechada neste assessment.
- Rebalanceamento de dano, custo, PV, Fôlego, alcance ou duração das ações já existentes; esses números vieram dos ciclos 015/045/046/047/050 e não são reabertos.
- PVE/PNJs, IA de defensores, mapa, ilhas, spawn, tarefas de ilha, condições de vitória/derrota — todas as frentes dos ciclos 049–051, aqui fora do recorte.
- Netcode, lobby, matchmaking, predição/reconciliação, `async`, sockets, Lightyear (ADR-004) e qualquer alteração de protocolo ou contrato.
- Adotar R6, simular emissão de issues ou reabrir o empate 48/625 com PVE/PNJs.

## Success Metrics

- `Jogador.nivel` começa em **1** para todo jogador e nunca excede **12**, inclusive após 900 ticks adicionais no teto (baseline: comportamento inexistente — **[NV]** até verificação).
- **899 ticks** não aumentam o nível; o **900.º tick** aumenta exatamente **+1**; o ganho ocorre uma única vez por janela de 900 ticks, sem acúmulo duplo (baseline: **[NV]**).
- No nível 1, `atacar_forte` (Q) é aceito; W, E e R são recusados com `CodigoRecusa::AcaoBloqueada` (baseline: **[NV]**).
- Ao atingir o nível 2, `construir_barreira` e `construir_ponte` (W) passam a ser aceitos; antes disso são recusados. Ao atingir o nível 3, `propaganda` e `propaganda_no` (E) passam a ser aceitos; antes disso são recusados (baseline: **[NV]**).
- `atacar_no` (R) só é aceito a partir do nível 6; níveis 2 a 5 continuam recusando com `AcaoBloqueada`, sem antecipação (baseline: **[NV]**).
- `funcao` e `nivel` aparecem em `hash_estado` com ordem estável; o HUD exibe `Função: <nome>` e `Nível: N/12` e nenhuma tecla nova é introduzida (baseline: **[NV]**).

## Cost of Inaction

Sem o recorte, a linha do GDD §10.1 (Q1/W2/E3/R6, teto 12) continua sem recipiente observável: o Núcleo não distingue papéis materiais, não registra progressão e não recusa ações fora de nível; o HUD não comunica identidade nem progresso. Qualquer ciclo futuro que queira "jogador de nível N" ou regras por função precisaria reconstruir estado, validação e hash desde zero, e a tese do §10.1 de que funções são papéis materiais (P2) seguiria sem verificação no Núcleo. O custo é a perda do único mecanismo de progressão escrito no GDD — não a ausência das habilidades completas de §10.2–§10.5.

## Open Questions

- R3 = 0 clarificações de design pendentes: o contrato fixou constantes, mapeamento e HUD; o que segue é validação de especificação, não renegociação.
- **[NV]** Confirmar no `/speckit.specify` o valor padrão de `Jogador.funcao` (`Artesao` como `[HIPÓTESE]`) e se o ganho de nível é global por jogador, independentemente da função.
- **[NV]** Confirmar a posição de `CodigoRecusa::AcaoBloqueada` na ordem de recusas de `ValidacaoComandos::validar` e a ordem dos campos no `hash_estado`.
- **[NV]** Conformidade de R4/R7 (`1.0.0-local`) depende de gates não executados neste assessment.
- **[NV]** Comportamento de progressão, validação e HUD precisa de verificação em tempo de execução pelo orquestrador.
- Fora de escopo por decisão, não lacuna: habilidades completas W/E/R, Fôlego/alcances/recargas e PVE/PNJs — todos `[HIPÓTESE]` ou de outras frentes.

## Rastreabilidade (escopo do problema)

- FR-001 → `enum Funcao` (cinco papéis materiais) e `Jogador.funcao`; FR-002 → `Jogador.nivel` (inicial 1, teto 12); FR-003 → `TICKS_POR_NIVEL = 900`, `NIVEL_MAXIMO = 12` e +1/900 ticks; FR-004 → mapeamento Q1/W2/E3/R6 das ações existentes; FR-005 → `CodigoRecusa::AcaoBloqueada` em `ValidacaoComandos::validar` e cobertura de `funcao`/`nivel` em `hash_estado`; FR-006 → HUD `Função: <nome>` / `Nível: N/12`, sem novas teclas.
- SC-001..SC-006 → `Success Metrics` acima; T001..T012 → entregáveis do handoff em `decision.md`, com definição canônica prevista em `specs/052-papeis-niveis/` (hoje em template).
- Todos os números do GDD §10.1 e os 900 ticks são `[HIPÓTESE]`/protótipo. R6 é **N/A** por constituição v1.2.2 e não entra neste rastro.
