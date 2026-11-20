# Concept — Evidência auditável do gate `make check` e registro do não verificado

**Data:** 2026-11-20T03:19Z

## Opções consideradas
1. **Implementar a Task 6 literalmente do plano** (escolhida) — alvo `check` no `Makefile` com as dependências `test-contrato test-servidor test-validacao test-ilha test-manifesto check-compat`, `docs/progress/evidence.csv` com cabeçalho literal de 9 colunas e sete linhas `EV-20261119-001`..`EV-20261119-007`, e `docs/progress/2026-11-19-ciclo-80-20.md`, exatamente como em `docs/plans/2026-11-19-ciclo-80-20-omoba.md` (Task 6, Steps 1–4, linhas 1219–1304).
2. **PR único com Tasks 4–6** (rejeitada) — mistura manifesto, Ilha, `make check`, `evidence.csv`, 5v5 e vitória Central num diff grande e de baixa revisabilidade; rompe a fatia vertical de um ciclo e antecipa evidência ainda **[NV]**.
3. **Evidência preenchida como `PASS` no momento da escrita** (rejeitada) — registaria sucesso sem comando executado, contrariando doc-38 (resultado + fonte + confiança + ressalvas) e apagando a marcação explícita do não verificado exigida pelo próprio plano.
4. **Gate remoto com lobby/worker e publicação de release** (rejeitada) — `doc-26-compatibilidade-gate-distribuicao` está DEFER; nenhuma autoridade externa entra neste ciclo, e toda verificação é local com `--headless`.
5. **Alegação de cobertura integral (39/39 caminhos) a partir dos gates locais** (rejeitada) — as suítes executadas cobrem os caminhos testados; a árvore canônica completa não foi verificada, e o que não foi executado permanece **[NV]**.

## Consequências
- Arquivos restritos a `Makefile` (modificação), `docs/progress/evidence.csv` e `docs/progress/2026-11-19-ciclo-80-20.md` (criação); os sete scripts de teste e `ferramentas/verificar_compatibilidade.gd` são apenas reexecutados, nunca editados.
- `make check` é `.PHONY`, usa `GODOT ?= godot` com `--headless --path . --script` e encadeia as dependências em ordem, parando no primeiro erro; a sequência esperada termina em `COMPAT_OK module_id=insurgencia.exemplo` com status 0.
- Toda linha do CSV começa `NAO_VERIFICADO`, com `result`/`recorded_at` vazios e `unverified` não vazio; `verdict` só muda após execução com saída observada em `result`, e `grep -c` do marcador deve retornar `≥ 1`.
- O cabeçalho `id,criterio,verdict,source,command,result,confidence,unverified,recorded_at` é literal e na ordem fixa; valores com vírgula em `result` ficam entre aspas para preservar as 9 colunas.
- `make check 2>&1 | tail -1` é copiado literalmente para a linha CE-5.1 somente após a execução; nenhuma linha agregada é inventada.
- O registro datado liga os ids da evidência ao estado `ilha_concluida`, ao fora do escopo GDD §16.3/§16.4 e ao marco de reavaliação após `make check` (CE-5.5).
- Saídas com data/hora pertencem à portaria de validação R4 e versão/changelog/rollback à R7; nenhum desses artefatos é escrito nesta etapa, e nenhuma alegação de 39/39 caminhos é feita.
- Cada suíte tem contagem de `PASS`, linha `OK` e código de saída binários observados nesta sessão (16/6/9/14/8/3/7, `COMPAT_OK`, `SERVIDOR_PRONTO 7777`, 5 gates PASS), o que permite Convergence READ-ONLY sem reinterpretar intenção.
