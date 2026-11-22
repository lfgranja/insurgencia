# Feature Specification: Tela de resultado — exibição da vitória declarada pelo servidor

**Feature Branch**: `feature/019-tela-resultado` · **Created**: 2026-11-20T11:39Z · **Status**: Draft · **Data:** 2026-11-20T11:39Z

**Input**: GDD `docs/GDD - Insurgência.md:1539` ("o servidor declara vitória e exibe tela de resultado") e último item `[ ]` do checklist 16.7 (`:1569`); evento real em `nucleo/desafios/ilha_central.gd:11,14,73-78`; R1 `go` em `.specify/assessments/tela-resultado/decision.md`. Escopo solo/single confirmado pela SE (`.specify/memory/STATE.md:9`).

## User Scenarios & Testing *(mandatory)*

As histórias abaixo são priorizadas por valor e cada uma é testável de forma independente. Nenhuma pressupõe cena principal, editor Godot, `.tscn`, 5v5, lobby, NAT ou rede nova.

### User Story 1 — Exibir a vitória declarada pelo servidor (Priority: P1)

Como jogador do modo solo, preciso ver uma tela de resultado quando o servidor declara vitória na Ilha Central, para que o fechamento do MVP seja visível. O componente recebe o dicionário real do evento `vitoria_ilha_central` — com `evento`, `declarada_por="servidor"` e `tempo_segundos` — e passa a exibir "VITÓRIA" nos seus rótulos. A tela apenas exibe; nunca decide.

**Why this priority**: é o próprio GDD `:1539`; sem a exibição, a vitória já declarada pelo servidor no ciclo 016 permanece invisível ao jogador.

**Independent Test**: instanciar o componente com `preload(...).new()`, chamar `definir_resultado({"evento": "vitoria_ilha_central", "declarada_por": "servidor", "tempo_segundos": 4.0})` e verificar `estado_atual()=="VITORIA"` e os três `Label` preenchidos.

**Acceptance Scenarios**:

1. **Given** um componente `Control` recém-criado, **When** chega o evento real do servidor com `evento="vitoria_ilha_central"` e `declarada_por="servidor"`, **Then** `estado_atual()` retorna `"VITORIA"` e os rótulos `Titulo`, `Mensagem` e `Detalhe` exibem a vitória e os dados do resultado.
2. **Given** o mesmo componente já em `VITORIA`, **When** `definir_resultado` recebe novamente o mesmo dicionário, **Then** o estado permanece `"VITORIA"` sem duplicar rótulos nem criar novos nós.

---

### User Story 2 — Estado neutro quando não há evento (Priority: P2)

Como componente de interface, preciso começar e permanecer em um estado neutro enquanto nenhum evento de vitória chega do servidor, para não sugerir um resultado que ainda não ocorreu. Sem evento, o componente exibe "AGUARDANDO" e `resultado_atual()` é vazio.

**Why this priority**: o servidor inicia com `vitoria_declarada=false` e `estado_persistido=""` (`ilha_central.gd:21-22`); a tela deve refletir esse estado inicial, não antecipá-lo.

**Independent Test**: instanciar o componente sem chamar `definir_resultado` e verificar `estado_atual()=="AGUARDANDO"`, `resultado_atual().is_empty()==true` e nenhuma ocorrência de "VITÓRIA" nos três rótulos.

**Acceptance Scenarios**:

1. **Given** um componente recém-criado e nenhum evento, **When** seus rótulos são lidos, **Then** `Titulo` exibe "AGUARDANDO", `Mensagem` exibe "Aguardando decisão do servidor" e `Detalhe` não exibe vitória.
2. **Given** o componente em `VITORIA`, **When** `definir_resultado({})` é chamado com o dicionário vazio, **Then** volta a `AGUARDANDO` e `resultado_atual()` fica vazio (1 evento, 1 volta ao neutro).

---

### User Story 3 — Não inventar vitória (Priority: P3)

Como tela de resultado, preciso recusar dicionários que não sejam o evento do servidor, para que nenhum campo de comando do cliente, evento desconhecido ou dado incompleto produza uma vitória falsa.

**Why this priority**: `:1540` exige decisão do servidor; aceitar qualquer dicionário transformaria a interface em uma autoridade que ela não possui.

**Independent Test**: chamar `definir_resultado` com quatro entradas inválidas — vazia, sem a chave `evento`, com `declarada_por!="servidor"` e com `evento` desconhecido — e verificar que, nas 4 de 4, `estado_atual()=="AGUARDANDO"` e nenhum rótulo exibe "VITÓRIA".

**Acceptance Scenarios**:

1. **Given** `{"declarada_por": "servidor"}` (sem `evento`), **When** `definir_resultado` é chamado, **Then** o estado permanece `"AGUARDANDO"` (nenhuma vitória inferida).
2. **Given** `{"evento": "vitoria_ilha_central", "declarada_por": "cliente"}`, **When** `definir_resultado` é chamado, **Then** o estado permanece `"AGUARDANDO"` (declarante inválido).
3. **Given** `{"evento": "outro_evento", "declarada_por": "servidor"}`, **When** `definir_resultado` é chamado, **Then** o estado permanece `"AGUARDANDO"` (evento desconhecido).

---

### Edge Cases

- Dicionário vazio `{}` → estado `AGUARDANDO`, `resultado_atual()` vazio; nenhuma vitória.
- `evento` ausente, `declarada_por` ausente ou diferente de `"servidor"`, `tempo_segundos` ausente → nenhuma vitória exibida; as chaves ausentes não são inferidas.
- `evento` correto, mas desconhecido/renomeado (`"vitoria"`, `"outro_evento"`) → `AGUARDANDO`; a constante é literal `vitoria_ilha_central`.
- Segunda chamada com o mesmo evento → estado idempotente; nenhum `Label` ou novo filho é criado após a construção.
- Campo de comando do cliente (`vitoria`, ilhas capturadas, função, vida ou presença prontas) → nunca é lido pela tela; a única fonte é o dicionário do servidor.
- Nenhum `.tscn`, cena principal ou editor Godot → o componente é construído inteiramente em código e instanciado por `.new()` com `preload`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: `nucleo/apresentacao/tela_resultado.gd` DEVE existir como componente `extends Control`, instanciável por `preload("res://nucleo/apresentacao/tela_resultado.gd").new()`; nenhum arquivo `.tscn` é criado, nenhuma cena principal é carregada e nenhum editor Godot é usado. Tipos customizados DEVEM ser referenciados por `preload`, porque `class_name` não resolve em `--headless --script`. Nenhuma lógica de decisão de vitória reside neste arquivo.
- **FR-002**: O componente DEVE criar exatamente 3 `Label` programaticamente (`Label.new()` seguido de `add_child()`), sem nós declarados em cena: `Titulo`, `Mensagem` e `Detalhe` (nomes e quantidade fixos: 3 de 3). A árvore de rótulos é construída uma única vez; chamadas posteriores a `definir_resultado` atualizam `.text`, sem criar novos nós.
- **FR-003**: O componente DEVE expor `definir_resultado(resultado: Dictionary) -> void`, que armazena o dicionário recebido e atualiza os 3 rótulos. O dicionário de entrada é o evento real de `nucleo/desafios/ilha_central.gd`: chaves `evento` (valor literal `"vitoria_ilha_central"`), `declarada_por` (`"servidor"`) e `tempo_segundos` (`float`). Nenhuma chave é renomeada e nenhum campo de comando do cliente é aceito como fonte.
- **FR-004**: O componente DEVE ter exatamente 2 estados observáveis — `"VITORIA"` e `"AGUARDANDO"` — e expor `estado_atual() -> String` e `resultado_atual() -> Dictionary` (cópia do último resultado; vazio no estado neutro). `estado_atual()` retorna `"VITORIA"` somente quando `resultado_atual().get("evento")=="vitoria_ilha_central"` **e** `resultado_atual().get("declarada_por")=="servidor"`; em qualquer outro caso — dicionário vazio, chave ausente, declarante diferente ou evento desconhecido — retorna `"AGUARDANDO"`. Em `VITORIA`, `Titulo` exibe "VITÓRIA", `Mensagem` exibe "Declarada pelo servidor · Ilha Central" e `Detalhe` exibe o evento e o tempo; em `AGUARDANDO`, `Titulo` exibe "AGUARDANDO", `Mensagem` exibe "Aguardando decisão do servidor" e `Detalhe` não exibe vitória. Nenhum terceiro estado, nenhuma vitória inferida de progresso, ilhas ou tempo.
- **FR-005**: `tests/test_tela_resultado.gd` DEVE usar `extends SceneTree`, `preload` para o tipo customizado, TAB e tipos explícitos, e cobrir explicitamente: o estado neutro inicial (0 eventos), o evento real (`evento="vitoria_ilha_central"`, `declarada_por="servidor"`, `tempo_segundos=4.0`), a idempotência (2 chamadas com o mesmo evento), a volta ao neutro com `{}` e as 4 entradas inválidas de US3. A suíte NÃO usa relógio do sistema, sono, aleatoriedade, UDP, `.tscn` nem editor; imprime linhas `PASS ...` e `OK test_tela_resultado`, termina com `quit(0)`/`quit(1)` e tem 0 linhas `FAIL`.
- **FR-006**: A verificação DEVE adotar R4 (`validate`) com gate executado, saída colada, data/hora e conformidade, registrando: `PASS`/`OK test_tela_resultado` com exit `0` e 0 `FAIL` em `"$G" --headless --path . --script res://tests/test_tela_resultado.gd`, `make check GODOT=$G` com exit `0`, e os 5 gates determinísticos PASS. O não observado permanece **[NV]** até evidência datada (`docs/governance/sdd-phase-contracts.md:26-27`). Nenhum artefato R4 é escrito nesta etapa de especificação.
- **FR-007**: A liberação DEVE adotar R7 (`release`) com SemVer + changelog contendo IDs FR/T/bug + rollback, com R4 aprovado antes de R7 (`docs/governance/sdd-phase-contracts.md:27`). O ciclo permanece solo/single: nenhuma versão, changelog ou alegação remota é produzida nesta etapa; remoto, CI e merge seguem **[NV]** até confirmação com saída colada e data/hora.

### Key Entities *(include if feature involves data)*

- **TelaResultado**: componente `Control` programático; atributos = 3 `Label` filhos (`Titulo`, `Mensagem`, `Detalhe`), estado atual (`VITORIA`/`AGUARDANDO`) e último resultado; identidade pelo arquivo `nucleo/apresentacao/tela_resultado.gd`.
- **Resultado de vitória**: dicionário emitido pelo servidor em `vitoria_ilha_central`; atributos = `evento` (`"vitoria_ilha_central"`), `declarada_por` (`"servidor"`) e `tempo_segundos`; a tela apenas o exibe e não o produz.
- **Evento `vitoria_ilha_central`**: sinal e estado persistido de `IlhaCentral`; é a fonte única e autoritativa da tela (`ilha_central.gd:11,14,73-78`).
- **Estado `AGUARDANDO`**: condição neutra sem evento válido; atributos = rótulos iniciais e `resultado_atual()` vazio; identidade pela ausência das chaves canônicas válidas.
- **Evidência R4/R7**: gate executado, saída colada, data/hora, conformidade, SemVer, changelog com IDs FR/T/bug e rollback.

## Success Criteria *(mandatory)*

Critérios de aceite a aferir na implementação e no R4. Nenhum comando de gate, teste, `make` ou Godot foi executado nesta etapa de especificação.

### Measurable Outcomes

- **SC-001**: 1 de 1 componentes `extends Control` em `nucleo/apresentacao/tela_resultado.gd`, 0 arquivos `.tscn`, 0 cenas principais carregadas e 3 de 3 `Label` criados programaticamente (`Titulo`, `Mensagem`, `Detalhe`). (baseline: 0 de 1 e 0 de 3; meta: 1 de 1 e 3 de 3)
- **SC-002**: Com 1 evento real (`evento="vitoria_ilha_central"`, `declarada_por="servidor"`, `tempo_segundos=4.0`), 1 de 1 componentes retornam `estado_atual()=="VITORIA"` e 3 de 3 rótulos exibem a vitória. Com 0 eventos, 1 de 1 retornam `estado_atual()=="AGUARDANDO"`, `resultado_atual()` vazio e 0 ocorrências de "VITÓRIA" nos rótulos. (baseline: 0; meta: 1 de 1 e 3 de 3)
- **SC-003**: Com 4 entradas inválidas (dicionário vazio, sem `evento`, `declarada_por="cliente"`, `evento="outro_evento"`), 4 de 4 permanecem em `AGUARDANDO` e 0 vitórias são exibidas; 0 campos de comando do cliente são lidos. (baseline: 0 de 4; meta: 4 de 4 e 0)
- **SC-004**: `"$G" --headless --path . --script res://tests/test_tela_resultado.gd` imprime linhas `PASS ...` e `OK test_tela_resultado`, termina com exit `0` e 0 linhas `FAIL`; `make check GODOT=$G` termina com exit `0` e 0 `FAIL`. (baseline: não executado nesta etapa; meta: exit 0 em ambos, 0 `FAIL`)
- **SC-005**: 5 de 5 gates determinísticos ficam PASS na verificação da implementação — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only` e `hub_docs_gate` — preservando `spec_dirs>=18` e `files>=18`, com 0 alterações de baseline e 0 artefatos R4/R7 escritos nesta etapa. (baseline: 5 de 5 registrados em `.specify/memory/STATE.md:5,8`; meta: 5 de 5)

## Assumptions

- O modo verificado é solo/single (`.specify/memory/STATE.md:9`); nenhum 5v5, lobby, NAT, UDP novo ou servidor adicional é introduzido. O dicionário do evento é passado em processo, sem rede.
- O servidor já declara a vitória em `nucleo/desafios/ilha_central.gd` (`declarada_por="servidor"`, `evento="vitoria_ilha_central"`, `estado_persistido`); este ciclo não altera essa regra e não afirma que a vitória foi efetivamente exibida em execução observada.
- Convenções GDScript do alvo: TAB, tipos explícitos, `preload` para tipos customizados (`class_name` não resolve em headless), `extends Control` no componente, `extends SceneTree` no teste e saída `PASS`/`OK` com exit `0`/`1`.
- Nenhum gate, teste, `make`, Godot, stage, commit ou push é executado nesta etapa; nenhum arquivo fora dos três previstos é escrito.
- Os critérios de sucesso são metas a aferir na implementação e em R4, não resultados já observados nesta especificação; o que não foi executado permanece **[NV]**.

## Clarifications

### Session 2026-11-20

- **Q1: Sem cena principal, o que é a "tela"?** → A: um componente `Control` reutilizável, criado em `nucleo/apresentacao/tela_resultado.gd`, instanciável por `.new()` com `preload`, sem `.tscn` e sem carregamento de cena principal; a "tela" é a própria árvore de 3 `Label` filhos. (fontes: GDD `docs/GDD - Insurgência.md:1539`; `.specify/assessments/tela-resultado/concept.md`)
- **Q2: De onde vem o resultado exibido?** → A: exclusivamente do evento/estado do servidor — sinal `vitoria_ilha_central(resultado: Dictionary)` e `estado_persistido`, com as chaves literais `evento="vitoria_ilha_central"`, `declarada_por="servidor"` e `tempo_segundos`. O cliente não envia vitória e nenhum campo de comando é lido pela tela. (fontes: `nucleo/desafios/ilha_central.gd:11,14,70-82`; `specs/016-vitoria-central/spec.md:FR-003`)
- **Q3: O que acontece sem evento?** → A: estado neutro `AGUARDANDO` — `Titulo` exibe "AGUARDANDO", `Mensagem` exibe "Aguardando decisão do servidor", `Detalhe` não exibe vitória e `resultado_atual()` é vazio. Nenhuma vitória é inferida de progresso, ilhas, tempo ou dicionário incompleto. (fontes: `nucleo/desafios/ilha_central.gd:21-22,55-56`; GDD `:1540`)
- **Q4: Há rede nova neste ciclo?** → A: nenhuma — nenhum UDP, lobby, 5v5, NAT, Internet ou servidor adicional; o dicionário do evento é consumido em processo no modo solo/single. Toda afirmação remota segue **[NV]**. (fontes: `.specify/memory/STATE.md:9`; `.specify/assessments/tela-resultado/decision.md`)

0 perguntas ao usuário; as quatro ambiguidades foram resolvidas por evidência local (GDD, evento real do servidor, decisão R1 e diretriz viva da SE). Cada esclarecimento foi formulado e resolvido isoladamente; nenhum marcador `[NEEDS CLARIFICATION]` permaneceu em aberto.

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê.
>
> Esta feature não produz manifest de orçamento de contexto nem saídas com handles de contexto; nenhum artefato dessa natureza é exigido no ciclo 019. A verificação é a dos cinco gates determinísticos, de `make check` e da suíte `tests/test_tela_resultado.gd`, conforme SC-001–SC-005, aferida na implementação e em R4 — nunca nesta etapa de especificação.
