# Feature Specification: Ilha Topo — três alavancas em ordem fixa com captura validada no servidor

**Feature Branch**: `013-ilha-topo` · **Created**: 2026-11-20 · **Status**: Draft · **Data:** 2026-11-20T09:28Z

**Input**: GDD §16.3 (`docs/GDD - Insurgência.md:1502`) e §16.4 (`:1518-1521`); pendência em `:1590`. R1 `go` em `.specify/assessments/ilha-topo/decision.md`.

## User Scenarios & Testing

### User Story 1 — Sequência correta captura e emite evento (P1)
O jogador ativa as três alavancas na ordem fixa 1 → 2 → 3, cada uma dentro de 8 s, concluindo em até 45 s. O servidor mede o tempo por `tick * PASSO_SEGUNDOS`, valida a intenção, grava `estado_persistido="ilha_topo_concluida"` e emite o evento de captura idêntico entre clientes.

**Why this priority**: é o critério central do GDD 1519 e o que prova que a captura não é inventada pelo cliente.

**Independent Test**: `tests/test_ilha_topo.gd` com tempo injetado — ativações em 1,0 s, 3,0 s e 6,0 s → captura, estado persistido e evento; verificação separada do limite em 45,0 s.

### User Story 2 — Alavanca fora de ordem reinicia em 2 s sem captura (P1)
Uma ativação fora da ordem esperada zera a sequência, liga o aviso visual e só aceita nova tentativa após 2 s. Nenhum estado persistido ou evento de captura é produzido nesse caminho.

**Why this priority**: critério do GDD 1520; protege a regra de que erro reinicia apenas a sequência da ilha, nunca a partida (GDD 1021).

**Independent Test**: sequência 1 → 3 no tempo 2,0 s → `ALAVANCA_FORA_DE_ORDEM`, `reinicio_ate=4,0 s`; nova tentativa em 3,0 s recusada, em 4,0 s aceita, sem captura.

### User Story 3 — Três ativações fora da janela falham no servidor (P1)
Um cliente que envia as três ativações com atraso (acima de 8 s por alavanca ou além do limite de 45 s) recebe falha no ponto único de validação; a ilha permanece neutra.

**Why this priority**: critério do GDD 1521; sem isso, qualquer cliente poderia reivindicar a captura.

**Independent Test**: três comandos em 9,0 s, 20,0 s e 50,0 s recusados com `JANELA_EXPIRADA`/`LIMITE_EXCEDIDO`; `capturada=false` e `estado_persistido=""` ao final.

### Edge Cases
- Primeira ativação após o limite de 45 s: falha, sem janela aberta.
- Alavanca com id fora de 1..3: recusada com `ALAVANCA_INVALIDA` antes de qualquer mutação.
- Tentativa durante os 2 s de reinício: recusada, sem avançar o índice.
- Sinalização após captura: descartada com `DESAFIO_JA_RESOLVIDO`.
- `reiniciar()` após captura: limpa índice, janela, aviso, captura e estado persistido para reconexão.

## Requirements

### Functional Requirements
- **FR-001**: `nucleo/desafios/ilha_topo.gd` (`extends RefCounted`, `class_name IlhaTopo`) com `const ORDEM: Array[int] = [1, 2, 3]` (ordem fixa por partida), `const JANELA_ALAVANCA_SEGUNDOS: float = 8.0`, `const REINICIO_SEGUNDOS: float = 2.0`, `const LIMITE_SEGUNDOS: float = 45.0` e os campos `indice`, `iniciada_em`, `reinicio_ate`, `capturada`, `aviso_visual` e `estado_persistido` (`"ilha_topo_concluida"` ao capturar; `""` após `reiniciar()`).
- **FR-002**: `sinalizar_alavanca(alavanca: int, agora: float) -> Dictionary` compara com `ORDEM[indice]` usando o tempo `agora` fornecido pelo servidor e responde com `DESAFIO_JA_RESOLVIDO`, `ALAVANCA_INVALIDA`, `ALAVANCA_FORA_DE_ORDEM`, `REINICIO_EM_CURSO`, `JANELA_EXPIRADA`, `LIMITE_EXCEDIDO`, `SEQUENCIA_CAPTURADA` ou `AGUARDANDO_ALAVANCA`. Acerto avança o índice e reinicia a janela de 8 s; erro fora de ordem chama `reiniciar()`, define `reinicio_ate = agora + 2.0` e liga `aviso_visual`; concluir as três alavancas até 45,0 s marca `capturada=true` e grava o estado persistido. `reiniciar() -> void` limpa índice, janela, reinício, aviso, captura e estado.
- **FR-003**: `modulos/exemplo/mapa.json` declara a ilha `ilha_topo` com `tipo="topo"`, `alavancas=[1, 2, 3]`, `janela_alavanca_segundos=8.0`, `reinicio_segundos=2.0` e `limite_segundos=45.0`; o Núcleo lê ordem e janelas do arquivo sem alterar a ilha `ilha_meio` já existente.
- **FR-004**: `nucleo/mundo.gd` mantém `const IlhaTopo = preload("res://nucleo/desafios/ilha_topo.gd")`, `var ilha_topo` e `estado_persistido`; o ramo `sinalizar_alavanca` em `_fase_skills()` chama `sinalizar_alavanca` com `float(tick) * PASSO_SEGUNDOS`, propaga `aviso_visual` e o atraso de reinício no snapshot, e copia `estado_persistido` quando capturada.
- **FR-005**: `nucleo/sim/validacao.gd` valida `sinalizar_alavanca` no ponto único de aplicação: inclui a ação na lista permitida, recusa `alvo` fora de 1..3 com `ALAVANCA_INVALIDA` e falha as três ativações fora da janela com `JANELA_EXPIRADA`/`LIMITE_EXCEDIDO`, sem mutar captura ou estado persistido. Os demais comandos seguem o contrato já mesclado.
- **FR-006**: `tests/test_ilha_topo.gd` (`extends SceneTree`) é determinístico: tempo injetado, sem relógio do sistema, sono ou aleatoriedade; cobre captura em ordem, erro fora de ordem com reinício em 2 s, três ativações fora da janela recusadas no servidor e `reiniciar()`. Imprime `PASS ...`, `OK test_ilha_topo` e termina com `quit(0)`/`quit(1)`, sem linhas `FAIL`.
- **FR-007**: Tempo, captura e evento são de autoridade do servidor: nenhum valor enviado pelo cliente é aceito como prova de sequência; o evento e `estado_persistido` são idênticos entre clientes e existem somente após a validação de FR-005. Nenhuma alegação de partida 5v5 ou de condição de vitória da Ilha Central é feita por esta especificação.
- **FR-008**: Registrar as saídas efetivamente executadas (com data/hora e marcação **[NV]** para o não observado) na portaria de validação R4; versão, changelog com FR/T e rollback em R7. Nenhum artefato R4/R7 é escrito nesta especificação.

### Key Entities
- **IlhaTopo**: regra pura do desafio; guarda a ordem esperada, a janela corrente, o atraso de reinício, o aviso visual e a captura.
- **Comando `sinalizar_alavanca`**: intenção do cliente (`alavanca`) validada no servidor com o tempo do tick; nunca carrega estado de captura.
- **Estado persistido `ilha_topo_concluida`**: único registro durável do desafio, copiado para o `Mundo` e limpo por `reiniciar()`.

## Success Criteria

- **SC-001**: Ativações 1 → 2 → 3 em até 45 s (cada alavanca dentro de 8 s) capturam a ilha, gravam `estado_persistido="ilha_topo_concluida"` e emitem o evento ao servidor.
- **SC-002**: Alavanca fora de ordem reinicia a sequência em exatamente 2 s com `aviso_visual`, sem captura, sem estado persistido e sem evento; nenhuma tentativa nesse intervalo avança o índice.
- **SC-003**: Três ativações fora da janela (acima de 8 s por alavanca ou além de 45 s) recebem falha no servidor; a ilha permanece neutra e nenhum cliente consegue capturar por conta própria.
- **SC-004**: `"$G" --headless --path . --script res://tests/test_ilha_topo.gd` imprime linhas `PASS ...`, `OK test_ilha_topo` e termina com exit `0`, com zero linhas `FAIL` e resultados repetíveis com o mesmo tempo injetado.
- **SC-005**: Ao final da implementação, os 5 gates determinísticos (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) estão PASS e `make check GODOT=$G` termina com exit `0`, sem regressão das suítes existentes.

## Assumptions
- O tempo chega ao Núcleo como `tick * PASSO_SEGUNDOS` (30 Hz), mesma convenção já usada pela Ilha Meio; nenhum relógio de cliente é consultado.
- A ordem `[1, 2, 3]` é fixa na partida conforme GDD 1502; sortear a ordem exigiria nova especificação.
- O padrão de validação, snapshot e testes de `010-ilha-sincronia` é reutilizado; nenhum gate, baseline ou `Makefile` é alterado neste ciclo.
- Execuções de Godot, gates e `make check` não foram observadas nesta etapa de escrita e permanecem **[NV]** até registro datado na portaria própria.

## Out of scope
Partidas 5v5, condição completa de vitória da Ilha Central, Ilha Inferior, Artesão/Guerreiro, `manifesto.json`, `Makefile`, `evidence.csv`, deploy, remote, CI remoto, factory, alteração de baselines e portarias R4/R7. Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos) é feita por este ciclo; o que não foi executado permanece **[NV]**.
