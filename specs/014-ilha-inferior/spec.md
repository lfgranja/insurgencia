# Feature Specification: Ilha Inferior — ponto de coleta com estoque e recarga validados no servidor

**Feature Branch**: `014-ilha-inferior` · **Created**: 2026-11-20 · **Status**: Draft · **Data:** 2026-11-20T09:38Z

**Input**: GDD §16.3 (`docs/GDD - Insurgência.md:1504`), §16.4 (`:1529-1531`), pendência em `:1592` e hipótese H3 em `:1677`. R1 `go` em `.specify/assessments/ilha-inferior/decision.md`.

## User Scenarios & Testing

### User Story 1 — Crédito de 5 Materiais a cada 3 s (P1)
O coletor solicita `coletar` junto ao ponto da Ilha Inferior. O servidor mede o tempo por `float(tick) * PASSO_SEGUNDOS`, credita exatamente 5 Materiais uma vez por ciclo de 3 s e emite o evento de coleta idêntico entre clientes. Um pedido dentro do mesmo ciclo não credita novamente.

**Why this priority**: é o critério central do GDD 1530 e o que prova que o recurso nasce no servidor, não no cliente.

**Independent Test**: `tests/test_ilha_inferior.gd` com tempo injetado — coletas em 0,0 s, 3,0 s e 6,0 s creditam 5, 10 e 15 Materiais; nova coleta em 7,0 s recusada com `CICLO_EM_ANDAMENTO`.

### User Story 2 — Esgotamento após 3 ciclos e recarga em 20 s (P1)
Após 3 ciclos (15 Materiais), o ponto esgota e entra em recarga por exatamente 20 s. Tentativas nesse intervalo são recusadas sem alterar o saldo; após 20 s o estoque volta a 3 ciclos e o ponto volta a creditar.

**Why this priority**: critério do GDD 1530; sem isso o ponto seria uma fonte infinita e a economia do protótipo perderia sentido.

**Independent Test**: com recarga iniciada em 6,0 s (esgotamento), tentativa em 25,0 s recusada com `RECARGA_EM_ANDAMENTO` e em 26,0 s aceita, restaurando `ciclos_restantes=3`.

### User Story 3 — Crédito inventado é recusado (P1)
Um cliente modificado que envia Materiais prontos no comando (`materiais`, `quantidade` ou `credito`), ou que tenta creditar sem passar pelo ponto, recebe recusa no ponto único de validação; nenhum saldo é criado e nenhum evento é emitido.

**Why this priority**: critério do GDD 1531; sem isso qualquer cliente poderia forjar recursos e burlar o custo de futuras habilidades.

**Independent Test**: `nucleo/sim/validacao.gd` recusa `acao="coletar"` com `materiais=999` usando `CREDITO_FORJADO`; a suíte verifica `valido=false`, saldo inalterado e zero eventos.

### Edge Cases
- Coleta antes do primeiro ciclo ou logo após um crédito: recusada com `CICLO_EM_ANDAMENTO`, sem avançar `proximo_ciclo_em`.
- Coleta com estoque zerado fora da janela de recarga: inicia a recarga e responde `ESTOQUE_ESGOTADO`, sem crédito.
- Comando `coletar` com `alvo` inválido (fora do id do ponto): recusado com `PONTO_INVALIDO` antes de qualquer mutação.
- Campo `materiais`/`quantidade`/`credito` presente no comando: recusado com `CREDITO_FORJADO` mesmo que o valor seja pequeno; o cliente nunca informa quantidade.
- `reiniciar()` após esgotamento ou no meio da recarga: limpa ciclos, janela, recarga, saldo e eventos para reconexão.

## Requirements

### Functional Requirements
- **FR-001**: `nucleo/desafios/ilha_inferior.gd` (`extends RefCounted`, `class_name IlhaInferior`) com `const CICLO_SEGUNDOS: float = 3.0`, `const MATERIAIS_POR_CICLO: int = 5`, `const CICLOS_ESTOQUE: int = 3`, `const RECARGA_SEGUNDOS: float = 20.0` e `const EVENTO_COLETA: String = "ilha_inferior_coleta"`; campos `proximo_ciclo_em`, `ciclos_restantes` (inicia em 3), `recarregando_ate`, `materiais_creditados` e `eventos`. Nenhum desses números é configurável pelo cliente.
- **FR-002**: `coletar(peer_id: int, agora: float) -> Dictionary` usa exclusivamente o tempo `agora` fornecido pelo servidor e responde com `CREDITO_CONCEDIDO`, `CICLO_EM_ANDAMENTO`, `ESTOQUE_ESGOTADO`, `RECARGA_EM_ANDAMENTO` ou `DESAFIO_JA_RESOLVIDO` quando aplicável. Acerto credita exatamente 5 Materiais, avança `proximo_ciclo_em = agora + 3.0`, decrementa `ciclos_restantes`, incrementa `materiais_creditados` e registra evento com `peer_id`, `materiais=5` e `tempo_segundos=agora`.
- **FR-003**: Ao completar o 3º ciclo, `ciclos_restantes` chega a 0 e `recarregando_ate = agora + 20.0`; enquanto `agora < recarregando_ate`, toda coleta falha com `RECARGA_EM_ANDAMENTO` e `materiais=0`. Ao atingir 20 s, `_recarregar()` restaura `ciclos_restantes=3` e limpa `recarregando_ate`, sem reajustar o saldo já creditado. `reiniciar() -> void` limpa janela, estoque, recarga, saldo e eventos.
- **FR-004**: `nucleo/mundo.gd` mantém integralmente os preloads existentes de `IlhaSincronia` e `IlhaTopo`, acrescenta `const IlhaInferior = preload("res://nucleo/desafios/ilha_inferior.gd")`, `var ilha_inferior` e o ramo `coletar` em `_fase_skills()`, que chama `coletar(peer_id, float(tick) * PASSO_SEGUNDOS)`. O campo `materiais` do jogador e o estado da recarga só recebem o valor devolvido por `coletar()`; nenhum valor do comando é copiado para o saldo.
- **FR-005**: `nucleo/sim/validacao.gd` inclui `coletar` em `ACOES_PERMITIDAS` e recusa, antes de qualquer mutação, todo comando que traga `materiais`, `quantidade` ou `credito` com o código `CREDITO_FORJADO`, bem como `alvo` fora do id do ponto com `PONTO_INVALIDO`. O crédito nunca é otimista: a validação apenas autoriza a intenção e o servidor calcula o valor.
- **FR-006**: `modulos/exemplo/mapa.json` declara a ilha `ilha_inferior` com `tipo="coleta"`, `ciclo_segundos=3.0`, `materiais_por_ciclo=5`, `ciclos_estoque=3` e `recarga_segundos=20.0`, preservando `ilha_meio` e `ilha_topo`. `tests/test_ilha_inferior.gd` (`extends SceneTree`, tempo injetado, sem relógio do sistema, sono ou aleatoriedade) cobre os três cenários de 1530–1531 e `reiniciar()`, imprime `PASS ...`, `OK test_ilha_inferior` e termina com `quit(0)`/`quit(1)`, sem linhas `FAIL`. Instâncias de `Mundo.new()` são tipadas explicitamente como `: Mundo`.
- **FR-007**: Tempo, crédito, estoque e evento são de autoridade do servidor: nenhum valor enviado pelo cliente é aceito como prova de coleta, e o evento é idêntico entre clientes. Registrar as saídas efetivamente executadas (com data/hora e marcação **[NV]** para o não observado) na portaria de validação R4; versão, changelog com FR/T e rollback em R7. Nenhum artefato R4/R7 é escrito nesta especificação, e nenhuma alegação de funções implementadas, vitória da Ilha Central ou conexão UDP real é feita aqui.

### Key Entities
- **IlhaInferior**: regra pura do ponto de coleta; guarda ciclo, estoque restante, recarga, saldo creditado e eventos.
- **Comando `coletar`**: intenção do cliente (`peer_id` e `alvo` do ponto), validada no servidor; nunca carrega quantidade de Materiais.
- **Evento `ilha_inferior_coleta`**: registro idempotente de cada ciclo creditado, com 5 Materiais e o tempo do tick.

## Success Criteria

- **SC-001**: Coletas em 0,0 s, 3,0 s e 6,0 s creditam exatamente 5 Materiais por ciclo (15 ao total) e emitem um evento por ciclo; tentativa dentro de um ciclo não credita.
- **SC-002**: Após 3 ciclos o ponto esgota e recarrega em exatamente 20 s: coleta em 25,0 s é recusada e em 26,0 s é aceita com estoque restaurado a 3 ciclos.
- **SC-003**: Um comando que inventa Materiais (`materiais`/`quantidade`/`credito`) ou que credita sem passar pelo ponto recebe `CREDITO_FORJADO` em `nucleo/sim/validacao.gd`; saldo e eventos permanecem inalterados.
- **SC-004**: `"$G" --headless --path . --script res://tests/test_ilha_inferior.gd` imprime linhas `PASS ...`, `OK test_ilha_inferior` e termina com exit `0`, com zero linhas `FAIL` e resultados repetíveis com o mesmo tempo injetado.
- **SC-005**: Ao final da implementação, os 5 gates determinísticos (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) estão PASS e `make check GODOT=$G` termina com exit `0`, sem regressão das suítes existentes.

## Assumptions
- O tempo chega ao Núcleo como `float(tick) * PASSO_SEGUNDOS` (30 Hz), mesma convenção já usada pelas Ilhas Meio e Topo; nenhum relógio de cliente é consultado.
- O estoque é de 3 ciclos por recarga, conforme GDD 1504/1530; a recarga começa no instante do 3º crédito e restaura o estoque, não o saldo acumulado.
- O padrão de validação, snapshot e testes de `013-ilha-topo` é reutilizado; nenhum gate, baseline ou `Makefile` é alterado neste ciclo; os preloads existentes em `nucleo/mundo.gd` são preservados.
- Execuções de Godot, gates e `make check` não foram observadas nesta etapa de escrita e permanecem **[NV]** até registro datado na portaria própria.

## Out of scope
Partidas 5v5, funções Artesão/Guerreiro (inclusive o custo de 20 Materiais da Barricada), condição completa de vitória da Ilha Central, conexão UDP real entre clientes, `manifesto.json`, `Makefile`, `evidence.csv`, deploy, remote, CI remoto, factory, alteração de baselines e portarias R4/R7. Nenhuma alegação de funções implementadas, de vitória efetiva, de UDP real ou de cobertura integral da árvore canônica (39/39 caminhos) é feita por este ciclo; o que não foi executado permanece **[NV]**.
