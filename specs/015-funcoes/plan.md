# Implementation Plan — Funções: Barricada do Artesão e Ataque Forte validados no servidor

**Data:** 2026-11-20T09:41Z · **Fonte:** GDD §16.3 (`docs/GDD - Insurgência.md:1505-1506`) e §16.4 (`:1534-1535`); R1 `go` em `.specify/assessments/funcoes/decision.md`.

## Arquivos (exatamente os do plano)
Criação/implementação: `nucleo/sim/funcoes.gd`, `tests/test_funcoes.gd`. Modificação: `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`. Leitura (sem alteração): `nucleo/desafios/ponto_coleta.gd`, `tests/test_ilha_inferior.gd`, `Makefile`, `scripts/ci/*.py`. Nenhum outro path é tocado; nenhum artefato R4/R7 é escrito nesta etapa.

## Steps
1. Manter em `nucleo/sim/funcoes.gd` (`extends RefCounted`, `class_name Funcoes`) as constantes literais `CUSTO_BARREIRA_MATERIAIS=20`, `CANALIZACAO_BARREIRA_SEGUNDOS=1.5`, `PV_BARREIRA=200`, `DURACAO_BARREIRA_SEGUNDOS=12.0`, `CANALIZACAO_ATAQUE_SEGUNDOS=1.2`, `DANO_BASE=100`, `DANO_ATAQUE_FORTE=200`, `RECARGA_ATAQUE_SEGUNDOS=6.0`, `TOLERANCIA=0.000001` e os dicionários `canalizacao`, `barreiras`, `recarga_ate` (FR-001).
2. Garantir `construir_barreira(peer_id, agora, materiais)` com recusa `MATERIAIS_INSUFICIENTES`/`CANALIZACAO_OU_BARREIRA_ATIVA`, canalização de 1,5 s, `pv=200` e criação/expiração em `avancar()` com duração de 12 s; o saldo vem do servidor, nunca do comando (FR-002, SC-001).
3. Garantir `receber_dano(peer_id, dano, agora)` com absorção até 200 PV, `barreira_destruida`, interrupção da canalização com `reembolso=20` e `dano_restante` devolvido ao Núcleo (FR-003, SC-002).
4. Garantir `atacar(peer_id, agora)` com canalização de 1,2 s, `dano=200` (2×100), `recarga_ate=agora+6.0`, recusa `EM_RECARGA` e limpeza da recarga expirada em `avancar()` (FR-004, SC-003).
5. Integrar em `nucleo/mundo.gd`: preservar os preloads de `IlhaSincronia`/`IlhaTopo`/`PontoColeta`, acrescentar o de `Funcoes`, criar `var funcoes`, adicionar os ramos `construir_barreira` e `atacar` em `_fase_skills()` com `float(tick) * PASSO_SEGUNDOS`, debitar 20 Materiais só no aceite e chamar `funcoes.avancar()` em `_fase_finish()`, aplicando `dano_restante` na vida (FR-005).
6. Validar em `nucleo/sim/validacao.gd`: incluir `construir_barreira` e `atacar` em `ACOES_PERMITIDAS`, recusar campos forjados (`materiais`/`credito`/`saldo`/`quantidade`/`valor`/`materiais_creditados`/`dano`) com `MATERIAIS_FORJADOS`/`DANO_FORJADO` e a recarga no servidor, antes de qualquer mutação (FR-006, SC-003).
7. Escrever `tests/test_funcoes.gd` (`extends SceneTree`, tempo injetado, `: Mundo` explícito) com os cenários de 1534–1535, interrupção por dano, expiração em 12 s, recarga em 6 s e `reiniciar()`, imprimindo `PASS ...` e `OK test_funcoes` (FR-007, SC-001..SC-004).
8. Verificar a suíte com o binário fixado: `PASS`, `OK test_funcoes`, exit 0 e zero `FAIL`; registrar saídas com data/hora na portaria de validação na fase própria, sem escrever R4/R7 aqui (FR-007, SC-004).
9. Verificar a porta de regressão `make check GODOT=$G` (exit 0) e os 5 gates determinísticos PASS com o conjunto completo de artefatos da spec (FR-007, SC-005).
10. Convergência READ-ONLY: revisar spec/plano/tasks contra GDD 1505–1506 e 1534–1535, corrigir divergências na fonte e reexecutar os 5 gates (SC-005).

## Comandos de verificação
```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
"$G" --headless --path . --script res://tests/test_funcoes.gd
make check GODOT=$G
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```
Nenhum desses comandos foi executado nesta etapa de escrita; toda saída permanece **[NV]** e só entra com data/hora na portaria de validação (R4). Versão, changelog com FR/T e rollback pertencem a R7. Nenhum artefato das portarias é escrito aqui.

## Riscos e mitigação
- Tempo de cliente mascarando canalização/recarga → usar exclusivamente `float(tick) * PASSO_SEGUNDOS`; o teste injeta `agora`, sem `OS.get_ticks_msec()` ou sono.
- Custo ou dano arbitrário → o cliente envia apenas `construir_barreira`/`atacar`; o saldo vem de `jogador["materiais"]` no servidor, `DANO_ATAQUE_FORTE=200` é constante e `nucleo/sim/validacao.gd` recusa campos forjados antes de qualquer mutação.
- Preloads removidos por acidente → `IlhaSincronia`, `IlhaTopo` e `PontoColeta` permanecem em `nucleo/mundo.gd`; a integração é estritamente aditiva.
- Erros conhecidos de tipagem/hash → nenhuma instância de `Mundo.new()` com `:=` (usar `: Mundo` explícito); nenhum `sha256_string` (manter `HashingContext` em `hash_estado()`).
- Números divergentes entre código, spec e GDD → 20/1,5/200/12 e 1,2/2×/6 idênticos em `docs/GDD - Insurgência.md:1505-1506`, `:1534-1535`, spec, `nucleo/sim/funcoes.gd` e teste.
- Regressão das suítes existentes → `make check GODOT=$G` precisa terminar com exit 0; nenhuma alteração de `Makefile`, gates ou baselines.
- Árvore suja com trabalho alheio → staging explícito dos quatro paths na fase de commit, nunca `git add -A`; nenhum branch, commit ou push nesta etapa.
- Convergência que mascara divergência → correção sempre na fonte (spec/plano/tasks), nunca no artefato derivado.
