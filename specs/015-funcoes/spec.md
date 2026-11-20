# Feature Specification: Funções — Barricada do Artesão e Ataque Forte do Guerreiro validados no servidor

**Feature Branch**: `015-funcoes` · **Created**: 2026-11-20 · **Status**: Draft · **Data:** 2026-11-20T09:41Z

**Input**: GDD §16.3 (`docs/GDD - Insurgência.md:1505-1506`), §16.4 (`:1534-1535`), Task 3 (`:1563`), pendência (`:1593`) e hipóteses H4/H5 (`:1678-1679`). R1 `go` em `.specify/assessments/funcoes/decision.md`.

## User Scenarios & Testing

### User Story 1 — Barricada do Artesão: 20 Materiais, 1,5 s, 200 PV, 12 s (P1)
O Artesão solicita `construir_barreira`. O servidor lê o saldo do jogador, debita 20 Materiais somente se a canalização for aceita e, após exatamente 1,5 s, cria uma barreira com 200 pontos de vida que bloqueia passagem por 12 s. Qualquer dano durante a canalização a interrompe e devolve os 20 Materiais; dano na barreira reduz os PV, e 200 PV a destroem.

**Why this priority**: é o critério central de GDD 1534 e o único uso de Materiais do protótipo; sem custo e duração decididos no servidor, a economia da Ilha Inferior perde sentido e a tese "intenção do cliente, decisão do servidor" não se sustenta.

**Independent Test**: `tests/test_funcoes.gd` com tempo injetado — com 19 Materiais a construção é recusada (`MATERIAIS_INSUFICIENTES`); com 20, `conclui_em=0.0+1.5`; `avancar(1.5)` cria barreira com `pv=200` e `expira_em=13.5`; `receber_dano(2, 200, 2.0)` a destrói.

### User Story 2 — Ataque Forte do Guerreiro: 1,2 s, 2× dano, 6 s de recarga (P1)
O Guerreiro solicita `atacar`. O servidor inicia uma canalização de 1,2 s; ao concluir, aplica `DANO_ATAQUE_FORTE=200` (`2 × DANO_BASE=100`) e registra `recarga_ate=agora+6.0`. Enquanto `agora < recarga_ate`, novos ataques são recusados com `EM_RECARGA` — inclusive na validação, antes de qualquer mutação.

**Why this priority**: é o critério de GDD 1535, que nomeia explicitamente a recusa do servidor na recarga; sem ela o dano dobrado poderia ser repetido a cada tick.

**Independent Test**: `atacar(2, 0.0)` inicia canalização e termina em 1,2 s com `dano=200`; `recarga_ate=7.2`; `atacar(2, 7.0)` é recusado com `EM_RECARGA`; `atacar(2, 7.2)` é aceito.

### User Story 3 — Nenhum valor forjado pelo cliente (P1)
Um cliente modificado que envia `materiais`, `credito`, `dano`, `pv` ou `recarga` no comando, ou que tenta construir sem saldo, recebe recusa no ponto único de validação; nenhum saldo é debitado, nenhuma barreira é criada e nenhum ataque é aplicado. O cliente informa apenas a intenção; o Núcleo calcula custo, dano e tempo.

**Why this priority**: é a continuação direta de GDD `:1515`/`:1531`, exigida também para as funções; sem isso um cliente poderia forjar recursos e dano e burlar os custos canônicos.

**Independent Test**: `ValidacaoComandos.validar` recusa `acao="construir_barreira"` com `materiais=999` e `acao="atacar"` com `dano=9999`, com `valido=false` e `codigo="MATERIAIS_FORJADOS"`/`DANO_FORJADO`; o saldo do jogador permanece inalterado.

### Edge Cases
- `construir_barreira` com saldo inferior a 20: recusado com `MATERIAIS_INSUFICIENTES`, sem iniciar canalização e sem débito.
- Segunda tentativa de `construir_barreira` ou `atacar` com canalização/barreira ativa: recusada com `CANALIZACAO_OU_BARREIRA_ATIVA` / `CANALIZACAO_EM_ANDAMENTO`.
- Dano durante a canalização da barreira: interrompe (`interrompida=true`, `tipo_interrompido="barreira"`, `reembolso=20`) e não cria barreira; dano durante a canalização do ataque também a interrompe, sem dano e sem recarga.
- Dano maior que os PV restantes da barreira: absorve até 200, remove a barreira (`barreira_destruida=true`) e devolve `dano_restante` ao Núcleo para aplicar na vida do jogador.
- Ataque em `agora == recarga_ate - TOLERANCIA`: recusado; em `agora >= recarga_ate`: aceito; a tolerância é `0.000001`, igual à da regra pura.
- `reiniciar()` no meio de canalização, com barreira ativa ou em recarga: limpa `canalizacao`, `barreiras` e `recarga_ate` para reconexão.

## Requirements

### Functional Requirements
- **FR-001**: `nucleo/sim/funcoes.gd` (`extends RefCounted`, `class_name Funcoes`) mantém as constantes literais `CUSTO_BARREIRA_MATERIAIS=20`, `CANALIZACAO_BARREIRA_SEGUNDOS=1.5`, `PV_BARREIRA=200`, `DURACAO_BARREIRA_SEGUNDOS=12.0`, `CANALIZACAO_ATAQUE_SEGUNDOS=1.2`, `RECARGA_ATAQUE_SEGUNDOS=6.0`, `DANO_BASE=100`, `DANO_ATAQUE_FORTE=2*100=200` e `TOLERANCIA=0.000001`, além dos dicionários `canalizacao`, `barreiras` e `recarga_ate`. Nenhum desses números é configurável pelo cliente.
- **FR-002**: `construir_barreira(peer_id, agora, materiais) -> Dictionary` recusa `MATERIAIS_INSUFICIENTES` com `custo=20` quando `materiais < 20`, e `CANALIZACAO_OU_BARREIRA_ATIVA` quando já há canalização ou barreira do mesmo jogador. A aceitação registra `canalizacao` com `conclui_em=agora+1.5` e responde `CANALIZACAO_BARREIRA_INICIADA` com `pv=200`; `avancar(agora)` cria a barreira ao completar 1,5 s, com `expira_em=agora+12.0`, e a remove ao atingir 12 s. O argumento `materiais` é sempre lido do jogador no servidor, nunca do comando.
- **FR-003**: `receber_dano(peer_id, dano, agora) -> Dictionary` absorve dano da barreira até `PV_BARREIRA=200`, remove-a com `barreira_destruida=true` ao zerar os PV e devolve `dano_restante`. Qualquer dano interrompe canalização ativa (`interrompida=true`), com `reembolso=20` quando o tipo interrompido é `barreira`, sem criar barreira nem aplicar ataque.
- **FR-004**: `atacar(peer_id, agora) -> Dictionary` recusa `EM_RECARGA` com `recarga_ate` enquanto `agora < recarga_ate - TOLERANCIA`, e `CANALIZACAO_EM_ANDAMENTO` se outra canalização estiver ativa. A aceitação registra `canalizacao` com `conclui_em=agora+1.2` e responde `CANALIZACAO_ATAQUE_INICIADA` com `dano=200`; `avancar(agora)` conclui o ataque, fixa `recarga_ate=agora+6.0` e limpa a recarga expirada. O servidor recusa o uso durante os 6 s de recarga.
- **FR-005**: `nucleo/mundo.gd` mantém integralmente os preloads existentes de `IlhaSincronia`, `IlhaTopo` e `PontoColeta`, acrescenta `const Funcoes = preload("res://nucleo/sim/funcoes.gd")` e `var funcoes = Funcoes.new()`, e adiciona os ramos `construir_barreira` e `atacar` em `_fase_skills()` com `float(tick) * PASSO_SEGUNDOS`. O débito de 20 Materiais ocorre somente quando a resposta é `aceito=true`, usando `jogador["materiais"]` do servidor; `_fase_finish()` chama `funcoes.avancar(float(tick) * PASSO_SEGUNDOS)` e aplica `dano_restante` na vida. Nenhum valor do comando é copiado para saldo, barreira, dano ou recarga.
- **FR-006**: `nucleo/sim/validacao.gd` inclui `construir_barreira` e `atacar` em `ACOES_PERMITIDAS` e recusa, antes de qualquer mutação, comandos que tragam `materiais`, `credito`, `saldo`, `quantidade`, `valor`, `materiais_creditados` ou `dano` (códigos `MATERIAIS_FORJADOS`/`DANO_FORJADO`), além de manter as recusas existentes de `seq`/`request_id` repetidos e de ações desconhecidas. O uso de `atacar` em recarga também é recusado no servidor, espelhando `atacar()`; a validação apenas autoriza a intenção e o Núcleo calcula o valor.
- **FR-007**: `tests/test_funcoes.gd` (`extends SceneTree`, tempo injetado, sem relógio do sistema, sono ou aleatoriedade, instâncias de `Mundo` tipadas explicitamente como `: Mundo`) cobre os três cenários de 1534–1535, as interrupções, a expiração, a recarga e `reiniciar()`, imprime `PASS ...` e `OK test_funcoes` e termina com `quit(0)`/`quit(1)`, com zero linhas `FAIL`. `make check GODOT=$G` termina com exit 0 e os 5 gates determinísticos ficam PASS. Registrar saídas efetivamente executadas (com data/hora e marcação **[NV]** para o não observado) na portaria de validação R4; versão, changelog com FR/T e rollback em R7. Nenhum artefato R4/R7 é escrito nesta especificação, e nenhuma alegação de vitória da Ilha Central, partidas 5v5 ou conexão UDP real é feita aqui.

### Key Entities
- **Funcoes**: regra pura das duas habilidades; guarda canalizações, barreiras ativas e recargas por `peer_id`.
- **Comandos `construir_barreira` e `atacar`**: intenções do cliente (`peer_id`, `acao` e `seq`/`request_id`), validadas no servidor; nunca carregam materiais, dano, PV, duração ou recarga.
- **Barreira**: entidade criada ao fim de 1,5 s, com 200 PV e expiração em 12 s, destruível por dano.
- **Ataque Forte**: aplicação de 200 pontos de dano (2×100) ao fim de 1,2 s, seguida de 6 s de indisponibilidade.

## Success Criteria

- **SC-001**: `construir_barreira` com 19 Materiais é recusado (`MATERIAIS_INSUFICIENTES`); com 20 Materiais a canalização termina em exatamente 1,5 s, cria barreira com `pv=200` e `expira_em=13.5` e a barreira desaparece em 13,5 s (12,0 s de duração).
- **SC-002**: dano de 200 destrói a barreira (`pv=0`, `barreira_destruida=true`); dano de 1 durante a canalização interrompe a construção (`interrompida=true`, `reembolso=20`) sem criar barreira e sem debitar os 20 Materiais.
- **SC-003**: `atacar` em 0,0 s completa em 1,2 s com `dano=200` (2×100) e fixa `recarga_ate=7.2`; tentativa em 7,0 s é recusada com `EM_RECARGA` e em 7,2 s é aceita, com recusa idêntica em `nucleo/sim/validacao.gd`.
- **SC-004**: `"$G" --headless --path . --script res://tests/test_funcoes.gd` imprime linhas `PASS ...`, `OK test_funcoes` e termina com exit `0`, com zero linhas `FAIL` e resultados repetíveis com o mesmo tempo injetado.
- **SC-005**: Ao final da implementação, os 5 gates determinísticos (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`) estão PASS e `make check GODOT=$G` termina com exit `0`, sem regressão das suítes existentes.

## Assumptions
- O tempo chega ao Núcleo como `float(tick) * PASSO_SEGUNDOS` (30 Hz), mesma convenção de `IlhaSincronia`, `IlhaTopo` e `PontoColeta`; nenhum relógio de cliente é consultado.
- `DANO_BASE=100` é o valor literal já fixado em `nucleo/sim/funcoes.gd`; o GDD só exige o multiplicador 2×, e `DANO_ATAQUE_FORTE=200` é a consequência determinística dessa base.
- O saldo de Materiais do Artesão é o mesmo crédito da Ilha Inferior; este ciclo não altera a regra do ponto de coleta nem cria Fôlego, alcance ou dimensões de produção.
- Os padrões de integração, validação e teste de `014-ilha-inferior` são reutilizados; nenhum gate, baseline ou `Makefile` é alterado neste ciclo; os preloads existentes em `nucleo/mundo.gd` são preservados.
- Execuções de Godot, gates e `make check` não foram observadas nesta etapa de escrita e permanecem **[NV]** até registro datado na portaria própria.

## Out of scope
Partidas 5v5, condição completa de vitória da Ilha Central (três ilhas de rota e canalização conjunta de 4 s), conexão UDP real entre clientes, custos de Fôlego/alcance/dimensões das versões de produção (GDD `:574`, `:680`), `manifesto.json`, `Makefile`, `evidence.csv`, deploy, remote, CI remoto, factory, alteração de baselines e portarias R4/R7. Nenhuma alegação de vitória efetiva, de partidas 5v5, de UDP real ou de cobertura integral da árvore canônica (39/39 caminhos) é feita por este ciclo; o que não foi executado permanece **[NV]**.
