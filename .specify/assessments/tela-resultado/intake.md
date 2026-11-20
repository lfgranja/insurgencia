# Idea Intake: Tela de resultado — exibição da vitória declarada pelo servidor

- **Slug**: tela-resultado
- **Created**: 2026-11-20T11:39Z
- **Source**: `docs/GDD - Insurgência.md:1539` + checklist 16.7 (`:1569`, último item `[ ]` pendente)
- **Type**: implementação — fatia vertical de apresentação, sem nova regra de jogo

## Idea (as captured)

> "Com as três ilhas, Artesão e Guerreiro vivos e presentes na Ilha Central e 4 s de canalização conjunta contínua, o **servidor declara vitória e exibe tela de resultado**." — `docs/GDD - Insurgência.md:1539`.

## Restated

Fechar a única cláusula do critério de aceitação da vitória que ainda não tem implementação: o servidor já emite o evento `vitoria_ilha_central` com `declarada_por="servidor"` e `tempo_segundos`, mas não existe componente de interface que mostre esse resultado ao jogador. A tela é consumidora do resultado — exibe a declaração, o tick e o estado — e nunca decide, infere ou forja a vitória.

## Origin & Context

- **Raised by**: GDD §16.4, condição de vitória na escala do MVP (`docs/GDD - Insurgência.md:1537-1541`); a regra pura e o evento real já estão em `nucleo/desafios/ilha_central.gd`.
- **Trigger**: checklist 16.7 (`docs/GDD - Insurgência.md:1569`) — a declaração pelo servidor está marcada em `:1596` ("servidor declara; 18 `PASS`"), mas a exibição da tela de resultado segue como último item `[ ]` voltado ao MVP; `UI de resultado` consta como **[NV]** em `.specify/memory/STATE.md:9,17,25`.
- **Âncora**: `.specify/feature.json` → `specs/019-tela-resultado`; branch `feature/019-tela-resultado`, HEAD `c7d5882`.
- **Escopo vivo**: solo/single confirmado pela SE (`.specify/memory/STATE.md:9`); sem 5v5, lobby, NAT/Internet ou serviços remotos.

## Restrições

- **Solo/single**: nenhuma partida 5v5, lobby, matchmaking, NAT traversal ou cliente externo; nada de rede nova, UDP adicional ou servidor suplementar.
- **Sem `main scene` novo**: nenhuma cena principal é criada nem registrada; o componente é instanciável por código em verificação `--headless --script`.
- **Sem `.tscn`**: a árvore de nós é construída programaticamente (`Control` + `Label` via `new()`/`add_child()`), sem arquivo de cena declarativa.
- **Sem alterar o servidor**: a vitória continua sendo decidida em `nucleo/desafios/ilha_central.gd`; a tela apenas consome o dicionário do evento, com as chaves literais `evento`, `declarada_por` e `tempo_segundos`.
- **Arquivos previstos**: `nucleo/apresentacao/tela_resultado.gd` e `tests/test_tela_resultado.gd`, além dos artefatos documentais do ciclo; nenhum gate, baseline, `Makefile`, `evidence.csv` ou portaria R4/R7 é tocado nesta etapa.
- **Convenções**: TAB, tipos explícitos e `preload` para tipos customizados (`class_name` não resolve em headless); saída `PASS`/`OK` com `quit(0/1)`.

## First-Glance Unknowns

- [NV] Nenhuma execução de Godot, suíte, `make check` ou gate é observada nesta etapa; a verificação com data/hora pertence à portaria própria.
- [NV] Status remoto, CI, merge e deploy não são verificados aqui e permanecem fora de escopo.
- [NV] Não há janela gráfica: a "exibição" é verificada pela árvore de controle e pelo texto dos `Label`, não por captura de tela.
