# Implementation Plan — Ilha Topo: três alavancas em ordem fixa com captura validada no servidor

**Data:** 2026-11-20T09:28Z · **Fonte:** GDD §16.3 (`docs/GDD - Insurgência.md:1502`) e §16.4 (`:1518-1521`); R1 `go` em `.specify/assessments/ilha-topo/decision.md`.

## Arquivos (exatamente os do plano)
Criação: `nucleo/desafios/ilha_topo.gd`, `tests/test_ilha_topo.gd`. Modificação: `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `modulos/exemplo/mapa.json`. Leitura (sem alteração): `nucleo/desafios/ilha_sincronia.gd`, `tests/test_ilha_sincronia.gd`, `Makefile`, `scripts/ci/*.py`. Nenhum outro path é tocado; nenhum artefato R4/R7 é escrito nesta etapa.

## Steps
1. Escrever `nucleo/desafios/ilha_topo.gd` com `ORDEM=[1,2,3]`, 8 s por alavanca, reinício em 2 s, limite de 45 s, campos de janela/índice/aviso/captura e `estado_persistido` (FR-001).
2. Implementar `sinalizar_alavanca(alavanca, agora)` com os motivos literais, a janela de 8 s por alavanca e a captura até 45 s (FR-002, SC-001).
3. Implementar `reiniciar()` com `reinicio_ate = agora + 2.0`, `aviso_visual` e limpeza de captura/estado; recusar tentativas durante o reinício (FR-002, SC-002).
4. Declarar `ilha_topo` em `modulos/exemplo/mapa.json` com ordem e janelas, preservando `ilha_meio` (FR-003).
5. Integrar `ilha_topo`, o ramo `sinalizar_alavanca` em `_fase_skills()`, o aviso no snapshot e a cópia de `estado_persistido` ao tick de `Mundo` (FR-004).
6. Validar `sinalizar_alavanca` em `nucleo/sim/validacao.gd`: alvo 1..3, recusa de fora da janela e nenhuma mutação otimista de captura (FR-005, SC-003).
7. Escrever `tests/test_ilha_topo.gd` (`extends SceneTree`, tempo injetado) com os cenários de captura, erro/reinício e falha de três ativações fora da janela, imprimindo `PASS ...` e `OK test_ilha_topo` (FR-006, SC-004).
8. Verificar a suíte com o binário fixado: `PASS`, `OK test_ilha_topo`, exit 0 e zero `FAIL`; registrar saídas com data/hora na portaria de validação na fase própria, sem escrever R4/R7 aqui (FR-008, SC-004).
9. Verificar a porta de regressão `make check GODOT=$G` (exit 0) e os 5 gates determinísticos PASS com o conjunto completo de artefatos da spec (FR-008, SC-005).
10. Convergência READ-ONLY: revisar spec/plano/tasks contra GDD 1502 e 1518–1521, corrigir divergências na fonte e reexecutar os 5 gates (SC-005).

## Comandos de verificação
```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
"$G" --headless --path . --script res://tests/test_ilha_topo.gd
make check GODOT=$G
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```
Nenhum desses comandos foi executado nesta etapa de escrita; toda saída permanece **[NV]** e só entra com data/hora na portaria de validação (R4). Versão, changelog com FR/T e rollback pertencem a R7. Nenhum artefato das portarias é escrito aqui.

## Riscos e mitigação
- Tempo de cliente mascarando a janela → usar exclusivamente `float(tick) * PASSO_SEGUNDOS`; o teste injeta `agora`, sem `OS.get_ticks_msec()` ou sono.
- Erro sem consequência → fora de ordem zera o índice, liga `aviso_visual` e bloqueia por 2 s; nenhum estado persistido nesse caminho.
- Captura inventada → `nucleo/sim/validacao.gd` é o ponto único de aplicação; recusa fora de 1..3 e fora da janela antes de qualquer mutação.
- Ordem divergente entre mapa e código → `ORDEM` e `alavancas` são literalmente `[1, 2, 3]` nos dois lugares; 8/2/45 idênticos em GDD, spec, mapa e teste.
- Regressão das suítes existentes → `make check GODOT=$G` precisa terminar com exit 0; nenhuma alteração de `Makefile`, gates ou baselines.
- Árvore suja com trabalho alheio → staging explícito dos cinco paths na fase de commit, nunca `git add -A`; nenhum branch, commit ou push nesta etapa.
- Convergência que mascara divergência → correção sempre na fonte (spec/plano/tasks), nunca no artefato derivado.
