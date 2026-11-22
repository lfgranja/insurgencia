# Implementation Plan — Ilha Inferior: ponto de coleta com estoque e recarga validados no servidor

**Data:** 2026-11-20T09:38Z · **Fonte:** GDD §16.3 (`docs/GDD - Insurgência.md:1504`) e §16.4 (`:1529-1531`); R1 `go` em `.specify/assessments/ilha-inferior/decision.md`.

## Arquivos (exatamente os do plano)
Criação: `nucleo/desafios/ilha_inferior.gd`, `tests/test_ilha_inferior.gd`. Modificação: `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `modulos/exemplo/mapa.json`. Leitura (sem alteração): `nucleo/desafios/ilha_topo.gd`, `nucleo/desafios/ilha_sincronia.gd`, `tests/test_ilha_topo.gd`, `Makefile`, `scripts/ci/*.py`. Nenhum outro path é tocado; nenhum artefato R4/R7 é escrito nesta etapa.

## Steps
1. Escrever `nucleo/desafios/ilha_inferior.gd` com `CICLO_SEGUNDOS=3.0`, `MATERIAIS_POR_CICLO=5`, `CICLOS_ESTOQUE=3`, `RECARGA_SEGUNDOS=20.0`, `EVENTO_COLETA` e os campos `proximo_ciclo_em`, `ciclos_restantes`, `recarregando_ate`, `materiais_creditados` e `eventos` (FR-001).
2. Implementar `coletar(peer_id, agora)` com os motivos literais, o crédito de 5 Materiais por ciclo de 3 s e o evento com tempo do tick (FR-002, SC-001).
3. Implementar esgotamento no 3º ciclo, `recarregando_ate = agora + 20.0`, recusa durante a recarga, `_recarregar()` e `reiniciar()` com limpeza completa (FR-003, SC-002).
4. Integrar em `nucleo/mundo.gd`: preservar os preloads de `IlhaSincronia`/`IlhaTopo`, acrescentar o de `IlhaInferior`, criar `var ilha_inferior`, adicionar o ramo `coletar` em `_fase_skills()` com `float(tick) * PASSO_SEGUNDOS` e expor `materiais`/recarga no snapshot sem copiar valores do comando (FR-004).
5. Validar em `nucleo/sim/validacao.gd`: incluir `coletar` em `ACOES_PERMITIDAS`, recusar `materiais`/`quantidade`/`credito` com `CREDITO_FORJADO` e `alvo` inválido com `PONTO_INVALIDO`, antes de qualquer mutação (FR-005, SC-003).
6. Declarar `ilha_inferior` em `modulos/exemplo/mapa.json` com os parâmetros literais, preservando `ilha_meio` e `ilha_topo` (FR-006).
7. Escrever `tests/test_ilha_inferior.gd` (`extends SceneTree`, tempo injetado, `: Mundo` explícito) com crédito por ciclo, esgotamento/recarga, recusa de material forjado e `reiniciar()`, imprimindo `PASS ...` e `OK test_ilha_inferior` (FR-006, SC-001..SC-004).
8. Verificar a suíte com o binário fixado: `PASS`, `OK test_ilha_inferior`, exit 0 e zero `FAIL`; registrar saídas com data/hora na portaria de validação na fase própria, sem escrever R4/R7 aqui (FR-007, SC-004).
9. Verificar a porta de regressão `make check GODOT=$G` (exit 0) e os 5 gates determinísticos PASS com o conjunto completo de artefatos da spec (FR-007, SC-005).
10. Convergência READ-ONLY: revisar spec/plano/tasks contra GDD 1504 e 1529–1531, corrigir divergências na fonte e reexecutar os 5 gates (SC-005).

## Comandos de verificação
```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
"$G" --headless --path . --script res://tests/test_ilha_inferior.gd
make check GODOT=$G
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```
Nenhum desses comandos foi executado nesta etapa de escrita; toda saída permanece **[NV]** e só entra com data/hora na portaria de validação (R4). Versão, changelog com FR/T e rollback pertencem a R7. Nenhum artefato das portarias é escrito aqui.

## Riscos e mitigação
- Tempo de cliente mascarando ciclos/recarga → usar exclusivamente `float(tick) * PASSO_SEGUNDOS`; o teste injeta `agora`, sem `OS.get_ticks_msec()` ou sono.
- Crédito arbitrário → o cliente envia apenas `coletar`; `nucleo/sim/validacao.gd` recusa `materiais`/`quantidade`/`credito` com `CREDITO_FORJADO`, e o saldo só nasce do retorno de `coletar()`.
- Preloads removidos por acidente → `IlhaSincronia` e `IlhaTopo` permanecem em `nucleo/mundo.gd`; a integração é estritamente aditiva.
- Erros conhecidos de tipagem/hash → nenhuma instância de `Mundo.new()`/`_mundo_com_topo()` com `:=` (usar `: Mundo` explícito); nenhum `sha256_string` (manter `HashingContext` em `hash_estado()`).
- Números divergentes entre mapa, código, spec e GDD → 3/5/3/20 idênticos em `docs/GDD - Insurgência.md:1504`, `:1530`, spec, mapa e teste.
- Regressão das suítes existentes → `make check GODOT=$G` precisa terminar com exit 0; nenhuma alteração de `Makefile`, gates ou baselines.
- Árvore suja com trabalho alheio → staging explícito dos cinco paths na fase de commit, nunca `git add -A`; nenhum branch, commit ou push nesta etapa.
- Convergência que mascara divergência → correção sempre na fonte (spec/plano/tasks), nunca no artefato derivado.
