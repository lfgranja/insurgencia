# Decision — Tela de resultado: exibição da vitória declarada pelo servidor

**Data:** 2026-11-20T11:39Z · **R1:** `go`

## Veredito

**go** — problema válido (impacto 3, confiança 5); opção escolhida = Opção 1 (componente `Control` programático, sem `.tscn`, com `Label` criados em código). A Opção 2 é rejeitada pelo risco de formato de cena e pela ausência de editor no fluxo `--headless --script`; a Opção 3, porque texto em console não é tela. O evento real já existe em `nucleo/desafios/ilha_central.gd`, então a implementação não exige nova decisão de desenho: basta consumir o dicionário do servidor.

## Gate de entrada

- Fonte: `docs/GDD - Insurgência.md:1539` ("o servidor declara vitória e exibe tela de resultado") e `:1569` (último item `[ ]` do checklist 16.7). Diretriz viva de escopo solo/single em `.specify/memory/STATE.md:9`; distribuição de papéis já fechada no ciclo 016 (`specs/016-vitoria-central/spec.md`).
- Gates determinísticos do alvo, citados e **não reexecutados nesta etapa**: **5/5 PASS** em 2026-11-20T11:38:05Z (`.specify/memory/STATE.md:5,8`; `spec_dirs=18`, `files=18`, `converge files=1 baselined=1`). Os cinco gates são `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only` e `hub_docs_gate`. Reexecução neste ciclo, `make check`, execução de Godot e status remoto/CI permanecem **[NV]** até registro datado com saída colada na portaria própria.
- `.specify/feature.json` → `specs/019-tela-resultado`; branch `feature/019-tela-resultado`, HEAD `c7d5882`.
- Binário fixado: `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`; `--version` e saídas das suítes são **[NV]** nesta etapa de escrita.

## Restrições de execução

- Chaves literais do evento real (`nucleo/desafios/ilha_central.gd:11,14,73-78`): `EVENTO_VITORIA="vitoria_ilha_central"`, `declarada_por="servidor"`, `tempo_segundos`; o resultado chega pelo sinal `vitoria_ilha_central(resultado: Dictionary)` e pelo estado persistido `estado_persistido`. Nenhum parâmetro do cliente pode carregar vitória pronta; nenhuma chave é renomeada ou "melhorada".
- A tela exibe vitória **somente** com `evento=="vitoria_ilha_central"` e `declarada_por=="servidor"`; sem evento, palavra-chave ausente ou evento desconhecido, permanece em `AGUARDANDO`. Nenhum terceiro estado e nenhuma vitória inferida por progresso, ilhas ou tempo.
- Convenções GDScript do alvo: TAB, tipos explícitos, `preload` para tipos customizados (`class_name` não resolve em headless), `extends Control` no componente e `extends SceneTree` no teste, saída `PASS`/`OK` com exit `0`/`1`. Nenhum `.tscn`, nenhuma cena principal e nenhum editor Godot.
- Apenas os três arquivos previstos neste ciclo: `.specify/assessments/tela-resultado/concept.md`, `.specify/assessments/tela-resultado/decision.md` e `specs/019-tela-resultado/spec.md`. Nenhum arquivo fora dessa lista é escrito.
- NENHUM gate, teste, `make`, Godot, stage, commit ou push é executado nesta etapa de escrita. A verificação e suas saídas com data/hora pertencem à fase própria (R4).
- Fora de escopo: partidas 5v5, lobby, NAT/Internet, UDP novo, servidor adicional, regra de produção (seis ilhas/Plano Mestre/20 s), custos de Fôlego/alcance/dimensões de produção, deploy, remote, factory, baselines, `converge.md`, `Makefile`, `evidence.csv` e portarias R4/R7. Nenhuma alegação de vitória efetivamente exibida, de rede ou de escala de produção é feita.
- A especificação é a fonte; convergence/analyze é READ-ONLY: divergência detectada é corrigida na fonte (spec/plano/tasks) e revalidada, nunca mascarada no artefato derivado.
