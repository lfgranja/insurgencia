# Implementation Plan — Vitória da Ilha Central: três ilhas de rota e 4 s de canalização conjunta

**Data:** 2026-11-20T09:48Z · **Fonte:** GDD §16.3 (`docs/GDD - Insurgência.md:1507`) e §16.4 (`:1537-1541`); R1 `go` em `.specify/assessments/vitoria-central/decision.md`.

## Arquivos (exatamente os do plano)
Criação/implementação da regra e integração: `nucleo/desafios/ilha_central.gd`, `nucleo/mundo.gd`. Prova determinística: `tests/test_vitoria_central.gd`. Leitura (sem alteração): `nucleo/desafios/ilha_topo.gd`, `nucleo/desafios/ilha_sincronia.gd`, `nucleo/desafios/ponto_coleta.gd`, `nucleo/sim/funcoes.gd`, `nucleo/sim/validacao.gd`, `tests/test_funcoes.gd`, `Makefile`, `scripts/ci/*.py`. Nenhum outro path é tocado; nenhuma cena de interface, gate, baseline, `Makefile` ou artefato R4/R7 é escrito nesta etapa.

## Steps
1. Manter em `nucleo/desafios/ilha_central.gd` (`extends RefCounted`, `class_name IlhaCentral`) as constantes `ILHAS_NECESSARIAS=3`, `CANALIZACAO_SEGUNDOS=4.0`, `EVENTO_VITORIA="vitoria_ilha_central"`, `FUNCOES_EXIGIDAS=["artesao","guerreiro"]`, o sinal `vitoria_ilha_central(resultado)` e o estado completo de canalização/vitória, sem parâmetros vindos do cliente (FR-001).
2. Garantir `iniciar_canalizacao(ilhas_controladas, funcoes_presentes, agora)` com recusa `ILHAS_INSUFICIENTES` abaixo de 3 ilhas e `FUNCOES_AUSENTES`/`faltantes` sem Artesão e Guerreiro vivos e presentes por `peer_id`; aceite com `conclui_em=agora+4.0` e `progresso=0.0`, sem substitutos (FR-002, SC-001, SC-002).
3. Garantir `avancar(agora)` com progresso igual a `agora - inicio` (tolerância `0.000001`), `CANALIZANDO` em 3,99 s e `VITORIA` somente em 4,0 s, com evento único, `declarada_por="servidor"` e estado persistido para a tela de resultado (FR-003, SC-001).
4. Garantir `verificar_funcoes` por identidade de `peer_id`, `interromper()` com zeração do progresso e `reiniciar()` limpando vitória/eventos; morte ou saída de uma função impede a continuidade e um novo período exige 4,0 s completos (FR-004, SC-003).
5. Integrar em `nucleo/mundo.gd`: preservar os preloads de `IlhaSincronia`, `IlhaTopo`, `PontoColeta` e `Funcoes`, acrescentar o de `IlhaCentral`, manter `definir_funcao`, `definir_presenca_central`, `_contar_ilhas_concluidas`, `_funcoes_no_centro`, `_atualizar_ilha_central` e `vitoria_declarada_persistida()`, e atualizar tudo em `_fase_finish()` com `float(tick) * PASSO_SEGUNDOS` (FR-005).
6. Manter `hash_estado()` com `HashingContext`, incluindo `vitoria_declarada_persistida()` e `ilha_central.progresso`, e preservar a recusa de campos forjados em `nucleo/sim/validacao.gd`; nenhuma instância de `Mundo.new()` com `:=` (usar `: Mundo` no teste) e nenhum `sha256_string` (FR-005).
7. Escrever `tests/test_vitoria_central.gd` (`extends SceneTree`, tempo injetado, `: Mundo` explícito, `HashingContext`) com constantes 3/4,0, recusas, início, 3,99 s vs 4,0 s, evento/resultado do servidor, interrupção em 2,5 s, novo período 3,0 s→7,0 s, morte durante, `reiniciar()`, integração em 130 ticks e paridade de hash; imprimir `PASS ...` e `OK test_vitoria_central` (FR-006, SC-001..SC-004).
8. Verificar a suíte com o binário fixado: `PASS`, `OK test_vitoria_central`, exit `0` e zero `FAIL`; registrar saídas com data/hora na portaria de validação na fase própria, sem escrever R4/R7 aqui (FR-007, SC-004).
9. Verificar a porta de regressão `make check GODOT=$G` (exit `0`) e os 5 gates determinísticos PASS com o conjunto completo de artefatos da spec (FR-007, SC-005).
10. Convergência READ-ONLY: revisar spec/plano/tasks contra GDD 1507 e 1537–1541, corrigir divergências na fonte e reexecutar os 5 gates (SC-005). Nenhuma alegação de 5v5, UDP real ou regra de produção (seis ilhas/20 s).

## Comandos de verificação
```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
"$G" --headless --path . --script res://tests/test_vitoria_central.gd
make check GODOT=$G
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```
Nenhum desses comandos foi executado nesta etapa de escrita; toda saída permanece **[NV]** e só entra com data/hora na portaria de validação (R4). Versão, changelog com FR/T e rollback pertencem a R7. Nenhum artefato das portarias é escrito aqui.

## Riscos e mitigação
- Vitória forjada pelo cliente → o servidor decide em `IlhaCentral.avancar()`; o cliente envia apenas intenções, e `hash_estado()` inclui o resultado persistido.
- Escala indevida → GDD `:1507` fixa três ilhas e 4 s no protótipo; seis ilhas/Plano Mestre/20 s permanecem documentados como regra de produção, fora deste ciclo.
- Substituição das funções → `verificar_funcoes` compara `peer_id` das duas funções exigidas; Caudillo/Camponês não suprem ausência, morte ou saída.
- Progresso acumulado → `interromper()` zera `progresso`; cada novo início reinicia a contagem de 4,0 s.
- Preloads removidos por acidente → `IlhaSincronia`, `IlhaTopo`, `PontoColeta` e `Funcoes` permanecem em `nucleo/mundo.gd`; a integração é estritamente aditiva.
- Erros conhecidos de tipagem/hash → nenhuma instância de `Mundo.new()` com `:=` (`: Mundo` explícito); nenhum `sha256_string` (`HashingContext` mantido); variáveis locais não se chamam `i` onde houver `for i`.
- Números divergentes entre código, spec e GDD → 3 ilhas/4,0 s/3,99 s idênticos em `docs/GDD - Insurgência.md:1507`, `:1537-1541`, spec, `nucleo/desafios/ilha_central.gd` e teste.
- Regressão das suítes existentes → `make check GODOT=$G` precisa terminar com exit 0; nenhuma alteração de `Makefile`, gates ou baselines.
- Árvore suja com trabalho alheio → staging explícito dos três paths na fase de commit, nunca `git add -A`; nenhum branch, commit ou push nesta etapa.
- Convergência que mascara divergência → correção sempre na fonte (spec/plano/tasks), nunca no artefato derivado.
