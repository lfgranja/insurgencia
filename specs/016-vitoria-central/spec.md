# Feature Specification: Vitória da Ilha Central — três ilhas de rota e 4 s de canalização conjunta

**Feature Branch**: `016-vitoria-central` · **Created**: 2026-11-20T09:48Z · **Status**: Draft · **Data:** 2026-11-20T09:48Z

**Input**: GDD §16.3 (`docs/GDD - Insurgência.md:1507`), §16.4 (`:1537-1541`), Fase 4 (`:1563`) e pendência (`:1593`). R1 `go` em `.specify/assessments/vitoria-central/decision.md`.

## User Scenarios & Testing

### User Story 1 — Pré-requisito: três ilhas controladas pela mesma equipe (P1)
Um cliente solicita o início da canalização na Ilha Central. O servidor conta as três ilhas de rota sob controle da equipe (Ilha Meio, Ilha Topo e ponto de coleta da Ilha Inferior); com menos de três, recusa com `ILHAS_INSUFICIENTES` e a canalização não começa. O cliente não informa ilhas capturadas nem vitória: o estado vem do servidor.

**Why this priority**: é o primeiro critério de GDD `:1538`; sem ele qualquer equipe poderia reivindicar a vitória sem controlar as rotas do protótipo.

**Independent Test**: `IlhaCentral.iniciar_canalizacao(2, dupla, 0.0)` retorna `aceito=false`, `motivo="ILHAS_INSUFICIENTES"` e `canalizando=false`; com `3` e a dupla válida, retorna `aceito=true` e `conclui_em=4.0`.

### User Story 2 — Apenas Artesão e Guerreiro vivos e presentes, por 4 s contínuos (P1)
Com as três ilhas, o servidor exige Artesão e Guerreiro vivos e presentes na Ilha Central, identificados por `peer_id`. Outros jogadores (Caudillo, Camponês ou qualquer membro) não substituem nenhum dos dois. A canalização avança somente enquanto os dois permanecem; em `3,99 s` não há vitória e em `4,0 s` o servidor a declara, emitindo o evento `vitoria_ilha_central` e o resultado persistido que a tela apenas exibe.

**Why this priority**: são os critérios de GDD `:1539`–`:1540` e a redução de escala de `:1507`; sem identidade das funções e sem os 4 s contínuos, o resultado deixaria de ser uma decisão do servidor baseada em Ação Coletiva.

**Independent Test**: dupla com função faltante, morta ou ausente é recusada com `FUNCOES_AUSENTES` e `faltantes`; dupla válida em `0.0` completa em `4.0` com `vitoria=true`, `declarada_por="servidor"` e `estado_persistido="vitoria_ilha_central"`.

### User Story 3 — Interrupção zera o progresso (P1)
Se uma das funções morre ou deixa a Ilha Central durante a canalização, o servidor interrompe e zera o progresso. Um novo período exige 4,0 s completos: 2,5 s seguidos de interrupção e mais 3,0 s não vencem; após reinício em 3,0 s, só os 4,0 s seguintes (em 7,0 s) declaram vitória. Nenhum progresso é acumulado entre tentativas.

**Why this priority**: é o critério de GDD `:1541`; sem zeração, a vitória poderia ser obtida por soma de fragmentos descontínuos.

**Independent Test**: após `avancar(2.5)` e `interromper()`, `progresso=0.0` e `avancar(3.0)` retorna `INATIVA`; reiniciada em 3,0 s, `avancar(6.0)` dá `progresso=3.0` sem vitória e `avancar(7.0)` dá vitória.

### Edge Cases
- Menos de três ilhas controladas: `ILHAS_INSUFICIENTES`, sem iniciar e sem evento de vitória.
- Artesão ausente, Guerreiro ausente, função com `vida <= 0` ou `presente=false`: `FUNCOES_AUSENTES` com `faltantes`; se já havia canalização, o servidor interrompe.
- Outro jogador ocupando o lugar de uma função: `verificar_funcoes` recusa por `peer_id`; troca de `peer_id` após o início também interrompe.
- `avancar(3.99)`: `estado="CANALIZANDO"`, `progresso=3.99`, `vitoria=false`; `avancar(4.0)`: `estado="VITORIA"`, `progresso=4.0`, evento único.
- Vitória já declarada: chamadas posteriores de `avancar` retornam `VITORIA` sem duplicar `eventos`; `iniciar_canalizacao` retorna `VITORIA_JA_DECLARADA`.
- `reiniciar()` limpa `vitoria_declarada`, `estado_persistido`, `eventos`, `canalizando` e `progresso` para reconexão.

## Requirements

### Functional Requirements
- **FR-001**: `nucleo/desafios/ilha_central.gd` (`extends RefCounted`, `class_name IlhaCentral`) mantém as constantes literais `ILHAS_NECESSARIAS=3`, `CANALIZACAO_SEGUNDOS=4.0`, `EVENTO_VITORIA="vitoria_ilha_central"` e `FUNCOES_EXIGIDAS=["artesao","guerreiro"]`, o sinal `vitoria_ilha_central(resultado: Dictionary)` e o estado `canalizando`, `canalizacao_iniciada_em`, `canalizadores`, `progresso`, `vitoria_declarada`, `estado_persistido` e `eventos`. Nenhum número ou resultado é configurável pelo cliente; a regra de produção (seis ilhas, Plano Mestre, 20 s) não é aplicada ao protótipo.
- **FR-002**: `iniciar_canalizacao(ilhas_controladas, funcoes_presentes, agora) -> Dictionary` recusa, antes de qualquer mutação de progresso, com `motivo="ILHAS_INSUFICIENTES"` quando `ilhas_controladas < 3` e com `motivo="FUNCOES_AUSENTES"` e `faltantes` quando falta Artesão ou Guerreiro vivo e presente. A aceitação fixa `canalizando=true`, `canalizacao_iniciada_em=agora`, `progresso=0.0`, `conclui_em=agora+4.0` e os `peer_id` das duas funções em `canalizadores`; outros jogadores presentes não preenchem `faltantes`.
- **FR-003**: `avancar(agora) -> Dictionary` calcula o progresso somente como `agora - canalizacao_iniciada_em` (com tolerância de `0.000001`) e retorna `estado="CANALIZANDO"` e `vitoria=false` antes de 4,0 s. Ao atingir `agora >= inicio + 4.0`, o **servidor** declara a vitória: `estado="VITORIA"`, `vitoria=true`, `progresso=4.0`, `canalizando=false`, `estado_persistido=EVENTO_VITORIA`, `resultado.declarada_por="servidor"`, `resultado.evento="vitoria_ilha_central"` e sinal `vitoria_ilha_central` emitido uma vez. Esse evento/resultado é a fonte da tela de resultado; nenhum campo de comando do cliente pode conter vitória pronta, e nenhuma cena de interface é afirmada como executada nesta etapa.
- **FR-004**: `verificar_funcoes(funcoes_presentes) -> bool` exige as duas funções canônicas vivas, presentes e com os mesmos `peer_id` registrados no início; `interromper() -> Dictionary` desfaz `canalizando`, `canalizacao_iniciada_em`, `canalizadores` e `progresso` (volta a `0.0`). Morte (`vida <= 0`) ou saída (`presente_central=false`) de qualquer função interrompe no servidor; um novo início exige outros 4,0 s contínuos. `reiniciar()` limpa também `vitoria_declarada`, `estado_persistido` e `eventos`.
- **FR-005**: `nucleo/mundo.gd` preserva integralmente os preloads existentes de `IlhaSincronia`, `IlhaTopo`, `PontoColeta` e `Funcoes`, acrescenta `const IlhaCentral = preload("res://nucleo/desafios/ilha_central.gd")` e `var ilha_central = IlhaCentral.new()`, e mantém `definir_funcao`, `definir_presenca_central`, `_contar_ilhas_concluidas`, `_funcoes_no_centro`, `_atualizar_ilha_central` e `vitoria_declarada_persistida()`. `_fase_finish()` atualiza a regra com `float(tick) * PASSO_SEGUNDOS` (30 Hz), interrompe se uma função morre/sai e copia o evento persistido para `estado_persistido`; `hash_estado()` usa `HashingContext` e inclui `vitoria_declarada_persistida()` e `ilha_central.progresso`. Nenhuma instância de `Mundo.new()` é declarada com `:=` e nenhum `sha256_string` substitui `HashingContext`.
- **FR-006**: `tests/test_vitoria_central.gd` (`extends SceneTree`, tempo injetado, sem relógio do sistema, sono, aleatoriedade ou UDP; instâncias de `Mundo` tipadas explicitamente como `: Mundo` e `HashingContext` para paridade) cobre as constantes `3`/`4.0`, as recusas com duas ilhas, função faltante, morta, ausente e substitutos, o início válido, `3.99 s` vs `4.0 s`, o evento/resultado com `declarada_por="servidor"`, a interrupção em `2.5 s` com zeração, o novo período `3.0 s`→`7.0 s`, a morte durante a canalização, `reiniciar()` e a integração no `Mundo` com 130 ticks e hash idêntico em duas instâncias. A suíte imprime `PASS ...` e `OK test_vitoria_central`, termina com `quit(0)`/`quit(1)` e tem zero linhas `FAIL`.
- **FR-007**: A verificação usa `"$G" --headless --path . --script res://tests/test_vitoria_central.gd` com `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`, exigindo `PASS`, `OK test_vitoria_central`, exit `0` e zero `FAIL`; `make check GODOT=$G` deve terminar com exit `0` e os 5 gates determinísticos (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) devem ficar PASS. Saídas efetivamente executadas, com data/hora e **[NV]** para o não observado, são registradas na portaria de validação R4; versão, changelog com FR/T e rollback pertencem a R7. Nenhum artefato R4/R7 é escrito nesta especificação, e nenhuma alegação de partidas 5v5, UDP real, escala de produção (seis ilhas/20 s), vitória efetivamente executada ou cobertura integral da árvore canônica (39/39) é feita aqui.

### Key Entities
- **IlhaCentral**: regra pura do servidor para o fechamento do MVP; guarda pré-requisito de três ilhas, dupla de funções, progresso, evento e vitória persistida.
- **Canalização conjunta**: período contínuo de 4,0 s sustentado apenas por Artesão e Guerreiro vivos e presentes, com identidade fixada por `peer_id`.
- **Resultado de vitória**: dicionário com `declarada_por="servidor"`, `evento="vitoria_ilha_central"` e tempo, emitido como sinal e persistido para a tela de resultado; o cliente apenas o exibe.
- **Mundo**: autoridade que conta as ilhas, resolve vida/presença/função dos jogadores, atualiza a regra a cada tick e inclui o resultado no hash de estado.

## Success Criteria

- **SC-001**: Com `ilhas_controladas=3` e Artesão e Guerreiro vivos e presentes, `iniciar_canalizacao(..., 0.0)` é aceito com `conclui_em=4.0`; `avancar(3.99)` dá `progresso=3.99` e `vitoria=false`, e `avancar(4.0)` dá `progresso=4.0` e `vitoria=true` com `evento="vitoria_ilha_central"` e `declarada_por="servidor"`.
- **SC-002**: Com `ilhas_controladas=2`, com apenas uma das funções ou com função morta/ausente, `iniciar_canalizacao` retorna `aceito=false` (`ILHAS_INSUFICIENTES` / `FUNCOES_AUSENTES`) e `canalizando=false`; Caudillo e Camponês presentes não substituem a dupla (`verificar_funcoes=false`).
- **SC-003**: Após 2,5 s de canalização, `interromper()` produz `progresso=0.0` e `estado="INATIVA"` no avanço seguinte; reiniciada em 3,0 s, a tentativa dá `progresso=3.0` em 6,0 s sem vitória e só vence em 7,0 s. Morte ou saída de uma função durante a canalização produz `estado="INTERROMPIDA"` sem vitória.
- **SC-004**: `"$G" --headless --path . --script res://tests/test_vitoria_central.gd` imprime linhas `PASS ...` e `OK test_vitoria_central`, termina com exit `0` e zero linhas `FAIL`, com resultados repetíveis com o mesmo tempo injetado.
- **SC-005**: Ao final da implementação, os 5 gates determinísticos (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) estão PASS e `make check GODOT=$G` termina com exit `0`, sem regressão das suítes existentes.

## Assumptions
- O tempo chega ao Núcleo como `float(tick) * PASSO_SEGUNDOS` (30 Hz), mesma convenção de `IlhaSincronia`, `IlhaTopo`, `Funcoes` e `PontoColeta`; nenhum relógio de cliente é consultado. A suíte usa 130 ticks como horizonte determinístico para cobrir 4,0 s de canalização no `Mundo`.
- As três ilhas de rota do protótipo são Ilha Meio, Ilha Topo e Ilha Inferior; o controle é lido do estado do servidor, sem campo de comando que o forneça pronto.
- A "tela de resultado" do GDD é o consumidor do evento e do estado persistido produzidos pelo servidor; nenhuma cena de UI é implementada, executada ou alegada neste ciclo.
- Os padrões de integração e teste de `014-ilha-inferior` e `015-funcoes` são reutilizados; nenhum gate, baseline ou `Makefile` é alterado neste ciclo e os preloads existentes em `nucleo/mundo.gd` são preservados.
- Execuções de Godot, gates e `make check` não foram observadas nesta etapa de escrita e permanecem **[NV]** até registro datado na portaria própria (R4).

## Out of scope
Partidas 5v5, conexão UDP real entre clientes, a regra de produção com seis ilhas, Plano Mestre e 20 s de Propaganda, custos de Fôlego/alcance/dimensões das versões de produção (GDD `:574`, `:680`), implementação de cena de interface, `manifesto.json`, `Makefile`, `evidence.csv`, deploy, remote, CI remoto, factory, alteração de baselines e portarias R4/R7. Nenhuma alegação de vitória efetivamente executada, de partidas 5v5, de UDP real, de escala de produção ou de cobertura integral da árvore canônica (39/39 caminhos) é feita por este ciclo; o que não foi executado permanece **[NV]**.
