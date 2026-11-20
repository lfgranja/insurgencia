# Implementation Plan — Tela de Resultado: exibição do evento declarado pelo servidor

**Data:** 2026-11-20T11:39Z · **Fonte:** GDD §16.4 linha 1539 ("o servidor declara vitória e exibe tela de resultado") e linha 1541 ("a vitória é declarada pelo servidor, não pelo cliente"). O evento consumido é o gerado por `nucleo/desafios/ilha_central.gd`: `resultado = {"evento": "vitoria_ilha_central", "declarada_por": "servidor", "tempo_segundos": agora}`, emitido pelo sinal `vitoria_ilha_central(resultado: Dictionary)`. A tela é um `Control` programático, sem cena própria, sem registro como cena principal e sem qualquer meio de rede. Nenhuma regra de vitória é reimplementada na apresentação.

## Arquivos (exatamente os do plano)

Implementação: `nucleo/apresentacao/tela_resultado.gd` (FR-001..FR-005), `tests/test_tela_resultado.gd` (FR-006, FR-007). Leitura sem alteração: `nucleo/desafios/ilha_central.gd`, `docs/GDD - Insurgência.md`, `Makefile`, `scripts/ci/*.py`, `AGENTS.md`, `project.godot`. Nenhum outro path é tocado nesta etapa de escrita: nenhum `.tscn`, nenhuma alteração de `project.godot`/main scene, nenhum gate, baseline, `evidence.csv`, artefato R4/R7, socket, UDP/ENet, lobby ou partida 5v5 (FR-008, SC-004).

## Steps

1. Criar `nucleo/apresentacao/tela_resultado.gd` como `extends Control`, sem `class_name` dependente de resolução em headless e sem `@tool`. A tela não carrega cena, não é registrada como main scene e não altera `project.godot`; os rótulos são criados por código (FR-001, SC-004).
2. Criar os rótulos em `_init()` ou sob demanda, por instância: um para o título e um para o detalhe/estado. Nenhum `.tscn`, nenhum `load`/`instantiate` de cena e nenhum nó externo; em modo `--headless` a instância é construída apenas com `TelaResultadoScript.new()` tipado explicitamente via `preload` (FR-002, SC-004).
3. Definir `definir_resultado(evento: Dictionary) -> void` como único ponto de atualização da tela. O método lê somente as chaves reais do resultado de `ilha_central.gd` — `evento`, `declarada_por` e `tempo_segundos` — e não recebe vitória pronta do cliente, não reconstrói a regra das três ilhas/4 s e não consulta relógio, aleatoriedade ou rede (FR-003, FR-005).
4. Fixar o critério binário do título: `VITORIA` somente quando `evento.get("evento") == "vitoria_ilha_central"` **e** `evento.get("declarada_por") == "servidor"`; em qualquer outro caso — dicionário vazio, chave ausente, outro evento, `declarada_por` diferente ou grafia diversa — o título é `AGUARDANDO`. Comparação literal de strings, sem dedução, sem "quase vitória" e sem aceitar declaração de cliente (FR-004, SC-001).
5. Preencher o rótulo de detalhe somente com dados do dicionário recebido (por exemplo `tempo_segundos`), sem `Time`, `OS`, sono, UDP ou estado externo; ausência de dado não inventa texto de vitória. A tela apenas apresenta o que o servidor declarou (FR-003, FR-005, SC-001).
6. Garantir reaplicação determinística: chamar `definir_resultado` com um evento válido e depois com um inválido troca `VITORIA` para `AGUARDANDO`, e o caminho inverso troca `AGUARDANDO` para `VITORIA`; duas instâncias com o mesmo evento produzem o mesmo par título/detalhe. Nenhum estado oculto, cache ou singleton (FR-004, FR-005, SC-002).
7. Escrever `tests/test_tela_resultado.gd` com `extends SceneTree` e `const TelaResultadoScript = preload("res://nucleo/apresentacao/tela_resultado.gd")`, instâncias tipadas explicitamente (`: Control`), TAB, tipos explícitos, `print("PASS ...")`, `printerr` para `FAIL`, marco final `OK test_tela_resultado` e `quit(0)`/`quit(1)`. A suíte tem **no mínimo 12 verificações** cobrindo: instância é `Control`; rótulos existem sem cena; estado inicial `AGUARDANDO`; evento real do servidor gera `VITORIA`; `declarada_por="cliente"` gera `AGUARDANDO`; outro valor de `evento` gera `AGUARDANDO`; dicionário vazio gera `AGUARDANDO`; chave `evento` ausente gera `AGUARDANDO`; chave `declarada_por` ausente gera `AGUARDANDO`; comparação sensível a grafia/caixa gera `AGUARDANDO`; reaplicação válida→inválida e inválida→válida; detalhe com `tempo_segundos` sem relógio; determinismo entre duas instâncias (FR-006, FR-007, SC-002).
8. Fechar o escopo negado: nenhum `main_scene` novo, nenhum `.tscn`, nenhum `load` de cena, nenhum UDP/ENet/socket, nenhum lobby/matchmaking/NAT/5v5, nenhuma regra de produção (seis ilhas/Plano Mestre/20 s), nenhuma vitória inventada e nenhum artefato R4/R7 escrito nesta etapa (FR-008, SC-004).
9. Verificar com o binário fixado: executar o teste em headless (todas as verificações `PASS`, `OK test_tela_resultado`, exit `0`, zero `FAIL`), `make check GODOT=$G` com exit `0` e os 5 gates determinísticos PASS. Saída efetivamente observada entra com data/hora apenas na portaria R4; versão, changelog com FR/T e rollback pertencem a R7. Nada disso é afirmado como executado nesta etapa de escrita (FR-008, SC-002, SC-003).
10. Convergência READ-ONLY: revisar plano/tasks/spec contra GDD §16.4 (1539 e 1541) e contra as chaves literais emitidas por `nucleo/desafios/ilha_central.gd`, corrigindo divergências sempre na fonte e reexecutando os 5 gates. Não observado permanece **[NV]**; R4 precede R7 e nenhum artefato das portarias é escrito aqui (FR-008, SC-005).

## Comandos de verificação

```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
"$G" --headless --path . --script res://tests/test_tela_resultado.gd
make check GODOT=$G
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```

Nenhum desses comandos foi executado nesta etapa de escrita; toda saída permanece **[NV]** e só entra com data/hora na portaria de validação (R4). A verificação do teste usa a invocação direta `--headless --path . --script`, sem inventar alvo novo no `Makefile`. Versão, changelog com FR/T e rollback pertencem a R7; R4 precede R7 e nenhum artefato dessas portarias é escrito aqui.

## Riscos e mitigação

- Tela registrada como cena principal → `Control` programático; `project.godot` e a main scene permanecem intocados (FR-001, SC-004).
- `.tscn` introduzido por hábito → rótulos criados em `_init()`/sob demanda; nenhum `load`, `instantiate` ou arquivo de cena (FR-002).
- Vitória inventada pelo cliente → título `VITORIA` exige simultaneamente `evento="vitoria_ilha_central"` e `declarada_por="servidor"`; tudo o mais é `AGUARDANDO` (FR-004, SC-001).
- Regra de vitória duplicada na UI → a tela somente lê o dicionário emitido por `ilha_central.gd`; nenhuma contagem de ilhas, tempo de canalização ou validação de função é refeita (FR-003, FR-005).
- Estado preso após uma vitória → `definir_resultado` é reaplicável nos dois sentidos, sem cache ou singleton (FR-004, SC-002).
- `class_name` não resolvido em headless → tipo obtido por `preload` e instância tipada explicitamente (FR-006, FR-007).
- Suíte frágil → no mínimo 12 verificações binárias, marcos `PASS`/`OK`/`FAIL` e `quit(0)`/`quit(1)` (SC-002).
- Rede e escala indevidas → nenhum socket, UDP/ENet, lobby, NAT, matchmaking ou 5v5; regra de produção inalterada (SC-004).
- Regressão dos gates → `make check GODOT=$G` precisa terminar com exit `0` e 5 gates PASS; nenhum gate ou baseline é alterado (SC-003).
- Árvore suja com trabalho alheio → staging explícito dos dois paths na fase de commit; nenhum branch, commit ou push nesta etapa.
- Convergência que mascara divergência → correção na fonte (spec/plano/tasks), nunca no artefato derivado; não observado continua **[NV]**.
