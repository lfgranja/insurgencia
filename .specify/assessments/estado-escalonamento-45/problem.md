# Problem — Estado verificado e escalonamento 45

**Data**: 2026-11-22T22:59Z

O repositório chegou ao fechamento do ciclo 036 com `dev` em `efb4d4b`, gates verdes (41/41) e PR #45 `dev → main` OPEN, green e com ACR sem blockers, parado na fila SWE. Sem um registro documental consolidado, o estado verificado fica disperso entre notas de sessão, converge de outros ciclos e comentários de ACR; a decisão de **não mesclar `main`** e a prevalência da constituição v1.0.0 do alvo sobre o prompt v4.6 (v1.2.0 + R6) poderiam ser reinterpretadas como pendência ou dívida. O problema é de **rastreabilidade e escalonamento**, não de código.

## Critérios de sucesso observáveis
1. Os 14 artefatos SDD registram HEADs (`efb4d4b`/`c126b6e`), PR #44 mesclado às 21:44:38Z e PR #45 escalonado ao SWE, com a decisão explícita de não mesclar `main`.
2. Gates observados (41/41 + 5 Python PASS) e contagens reais são citados com método de verificação e ressalva **[NV]** onde não houve reexecução.
3. A divergência de governança é registrada como **decisão**, não dívida: constituição v1.0.0 do alvo prevalece (Bloco 0.a), R6 não é adotado, `resolve-tasks.sh`/`taskstoissues` não são usados.
4. Ordem normativa do alvo respeitada: `analyze` READ-ONLY **antes** de `tasks`, R4 antes de R7 `1.0.0`, `converge.md` novo e append-only (nenhum converge existente editado).
5. `test-roadmap.md` escrito antes do PR: 6–10 passos numerados, superfície documental/CLI, capturas de terminal, coluna `automatizável?`, rollback, passo de comparar notas vs HEAD/contagens e passo de confirmar que R6 não foi adotado sem ratificação.
6. Nenhum build/gate/formatador/teste/commit/push/PR executado; factory `b1deab1` somente fonte; fronteira exata respeitada.

## Custo de inação
Sem este registro, o PR #45 green e sem blockers poderia ser mesclado por engano (contrariando a decisão do SWE), a constituição v1.2.0 do prompt poderia ser adotada sem ratificação (quebrando o Princípio III), e o estado verificado de 22:58–23:01Z se perderia entre notas — com dano à honestidade das evidências do hub (Princípio VI).

## Fora de escopo
Promover `dev → main`/mesclar PR #45, reexecutar gates/build/capturas, transplantar da factory `b1deab1`, adotar R6 ou reordenar o fluxo, usar `taskstoissues`/`resolve-tasks.sh`, editar `converge.md` existente, mexer em `docs/GDD*`, baselines, ledger, CI, `STATE.md`/`PROGRESSO.md`/`feature.json`, janelas, deploy, assinatura GPG, commits/push/PRs, e qualquer paridade de `construir_barreira`.
