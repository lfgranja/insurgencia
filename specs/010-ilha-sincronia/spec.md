# Feature Specification: Desafio da Ilha — sincronia reduzida de 2 jogadores em 5 s

**Feature Branch**: `010-ilha-sincronia` · **Created**: 2026-11-20 · **Status**: Draft · **Data:** 2026-11-20T03:10Z

**Input**: Task 4 de `docs/plans/2026-11-19-ciclo-80-20-omoba.md` (linhas 723–1025), literal.

## User Scenarios & Testing

### User Story 1 — Dois jogadores capturam a Ilha Meio em até 5 s (P1)
Dois peers sinalizam placas distintas (1 e 2) em uma janela iniciada pela primeira sinalização; o servidor mede o intervalo com `tick * PASSO_SEGUNDOS` e marca `ilha_concluida` se o segundo sinal chegar em até 5,0 s. Um único jogador nas duas placas não captura; sinais depois de resolvido são descartados.

**Independent Test**: `tests/test_ilha_sincronia.gd` — captura em 4,8 s e 4,9 s, recusa em 5,3 s, mesmo jogador nas duas placas e `estado_persistido` único.

### User Story 2 — Sincronia industrial e evento entre clientes (P1)
O desafio roda no tick do `Mundo`; a placa é validada no ponto único de aplicação; o gás puné amostras de 1,0 s e expulsa após 3,0 s; o evento `ilha_concluida` é idêntico entre clientes.

**Independent Test**: `tests/test_loopback_ilha.gd` — linha `DELTA_EVENTO_MS`, três `PASS` e `OK test_loopback_ilha`.

## Requirements

- **FR-001**: `nucleo/desafios/ilha_sincronia.gd` (`extends RefCounted`, `class_name IlhaSincronia`) com `JANELA_SEGUNDOS: float = 5.0`, `JOGADORES_NECESSARIOS: int = 2`, `DANO_POR_AMOSTRA: int = 8`, `INTERVALO_AMOSTRA_SEGUNDOS: float = 1.0`, `EXPOSICAO_EXPULSAO_SEGUNDOS: float = 3.0` e os campos `janela_iniciada_em`, `placas`, `capturada`, `exposicao`, `amostras_aplicadas`, `estado_persistido` (`"ilha_concluida"` ao capturar; `""` após `reiniciar()`).
- **FR-002**: `sinalizar(peer_id: int, placa: int, agora: float) -> Dictionary` inicia a janela na primeira chamada usando `agora` do servidor e responde com `DESAFIO_JA_RESOLVIDO`, `PLACA_INVALIDA`, `PLACA_OCUPADA`, `MESMO_JOGADOR_NAS_DUAS_PLACAS`, `JANELA_CUMPRIDA`, `JANELA_EXPIRADA` ou `AGUARDANDO_SEGUNDO_JOGADOR`; `aplicar_gas(dt: float) -> Dictionary` devolve `{"dano": Dictionary, "expulsos": Array}` com 8 pontos por amostra vencida de 1,0 s e expulsão aos 3,0 s; `reiniciar() -> void` limpa placas, exposição, amostras e captura para reconexão. O único estado persistido é `ilha_concluida`.
- **FR-003**: `modulos/exemplo/mapa.json` declara `mapa_id="arquipelago.exemplo"` e a ilha `ilha_meio` com `tipo="sincronia"`, `placas=[1, 2]`, `janela_segundos=5.0` e `jogadores_necessarios=2`; o Núcleo lê `placas` e `janela_segundos` do arquivo.
- **FR-004**: `modulos/base/regras.cfg` guarda a regra de produção do GDD §16.3 — `sync_window_seconds=5.0`, `required_players=3`, `gas_damage_per_second=8`, `gas_expulsion_seconds=3.0` — fora do Módulo de exemplo; a janela de 5 s é idêntica às duas regras, que diferem apenas em `required_players` (3 na base, 2 no exemplo).
- **FR-005**: `nucleo/mundo.gd` mantém `const IlhaSincronia = preload(...)`, `ilha_sincronia` e `estado_persistido`; o ramo `sinalizar_sincronia` em `_fase_skills()` chama `sinalizar` com `float(tick) * PASSO_SEGUNDOS`; `_fase_finish()` aplica o gás, debita a vida dos jogadores presentes, remove os expulsos e copia `estado_persistido` quando capturada.
- **FR-006**: `nucleo/sim/validacao.gd` valida `sinalizar_sincronia` no ponto único de aplicação: `alvo` fora de 1..2 é recusado com `PLACA_INVALIDA`; demais comandos seguem o contrato já mesclado.
- **FR-007**: `tests/test_ilha_sincronia.gd` e `tests/test_loopback_ilha.gd` usam `extends SceneTree`, imprimem `PASS ...`, `OK <suíte>` e terminam com `quit(0)`/`quit(1)`, sem linhas `FAIL`; o teste de loopback imprime a linha `DELTA_EVENTO_MS`.
- **FR-008**: Registrar as saídas executadas (com data/hora e marcação **[NV]** para o não observado) na portaria de validação R4; versão, changelog com FR/T e rollback em R7. Nenhum artefato R4/R7 é escrito nesta especificação.

## Success Criteria

- **SC-001**: `<G> --headless --path . --script res://tests/test_ilha_sincronia.gd` → catorze linhas `PASS ...`, `OK test_ilha_sincronia` e código de saída `0`.
- **SC-002**: `<G> --headless --path . --script res://tests/test_loopback_ilha.gd` → linha `DELTA_EVENTO_MS`, três linhas `PASS ...`, `OK test_loopback_ilha` e código de saída `0`.
- **SC-003**: Dano de 8 pontos por amostra de 1,0 s e 24 pontos por jogador em 3 s (vida 100 → 92 → 84 → 76), com expulsão após 3,0 s e nenhum gás depois da captura.
- **SC-004**: 5 gates determinísticos do alvo PASS ao final (`check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate`).

## Out of scope
`manifesto.json`, `Makefile`, `evidence.csv`, partidas 5v5, condição completa de vitória da Ilha Central, elenco Artesão/Guerreiro, deploy, remote, CI remoto, factory, alteração de baselines e artefatos R4/R7. Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos) é feita por este ciclo; o que não foi executado permanece **[NV]**.
