# Idea Research: Tela de resultado — exibição da vitória declarada pelo servidor

- **Slug**: tela-resultado
- **Created**: 2026-11-20T11:39Z
- **Evidence confidence (overall)**: high — as afirmações centrais repousam em leitura local e datada de fontes primárias do próprio alvo (GDD, `nucleo/desafios/ilha_central.gd`, `docs/progress/evidence.csv`, checklist 16.7 e registros de validação do ciclo 016). O único estado citado sem observação direta é o resultado dos gates; tudo que depende de comando, execução de Godot ou ambiente remoto permanece **[NV]**.

## Fontes primárias locais

- `docs/GDD - Insurgência.md:1537-1541` — §16.4, condição de vitória na escala do MVP: sem três ilhas a canalização não inicia; com Artesão e Guerreiro vivos e presentes e 4 s contínuos o **servidor** declara vitória; morte ou saída impede o avanço; interrupção zera o progresso. [source: leitura local, 2026-11-20] (confidence: high)
- `docs/GDD - Insurgência.md:1539` — cláusula literal pendente: "o servidor declara vitória e **exibe tela de resultado**". [source: leitura local, 2026-11-20] (confidence: high)
- `docs/GDD - Insurgência.md:1569` e `:1596` — checklist 16.7: a declaração pelo servidor está marcada como concluída ("servidor declara; 18 `PASS`"), restando como último item `[ ]` voltado ao MVP a exibição da tela de resultado; itens de Fase 5/deploy permanecem `[ ]` e são alheios a este ciclo. [source: leitura local, 2026-11-20] (confidence: high)
- `nucleo/desafios/ilha_central.gd:11,14,70-82` — evento real do servidor: `EVENTO_VITORIA="vitoria_ilha_central"` (`:11`), sinal `vitoria_ilha_central(resultado: Dictionary)` (`:14`), `estado_persistido = EVENTO_VITORIA` (`:73`), dicionário com `evento`, `declarada_por="servidor"` e `tempo_segundos` (`:74-76`), emissão em `:78` e retorno com `estado="VITORIA"`, `vitoria=true` e `resultado` (`:79-82`). [source: leitura local, 2026-11-20] (confidence: high)
- `docs/progress/evidence.csv:8-10` — 7 registros `PASS` da fatia 80/20 com `make check` (EV-20261119-007), E2E UDP real em loopback (EV-20261120-008) e um `NAO_VERIFICADO` explícito de NAT/clientes externos (EV-20261120-009). Nenhuma linha registra apresentação de resultado. [source: leitura local, 2026-11-20] (confidence: high)
- `.specify/validations/vitoria-central/validate.md:27` — ressalva literal: "A 'tela de resultado' é representada pelo estado/evento persistido do servidor; nenhuma cena de UI Godot foi implementada". `.specify/releases/vitoria-central/release.md:23` lista "Cena de UI de resultado" como fora de escopo. [source: leitura local, 2026-11-20] (confidence: high)
- `.specify/memory/STATE.md:9,17,25` — "UI de resultado"/"cena de UI de resultado" aparece entre os remanescentes **[NV]** em todos os registros vivos. [source: leitura local, 2026-11-20] (confidence: high)

## Fato central: o evento existe; a apresentação, não

| Lado | Evidência local | Consequência |
|---|---|---|
| Evento/decisão (existe) | `ilha_central.gd:11,14,70-82` produz `vitoria_ilha_central`, `declarada_por="servidor"`, `tempo_segundos`, `estado="VITORIA"` e estado persistido | A tela não precisa inventar, decidir ou revalidar a vitória; consome o dicionário do servidor. |
| Apresentação (falta) | Diretório `nucleo/apresentacao/` ausente; nenhum `.tscn` de resultado no alvo (os únicos são `src/core/IlhaBase.tscn` e `src/core/PersonagemBase.tscn`); `validate.md:27` diz explicitamente que nenhuma cena de UI foi implementada | GDD `:1539` não é satisfeito na íntegra: falta o "exibe tela de resultado". |
| Escopo (já fechado) | `.specify/memory/STATE.md:9` fixa solo/single; o evento e as chaves foram fechados no ciclo 016 | Nenhuma rede nova, 5v5 ou decisão de desenho adicional entra neste ciclo. |

## Contexto de gates (citado, não reexecutado)

- Resultado **5/5 PASS em 2026-11-20T11:38:05Z** citado como contexto do orquestrador para este ciclo. Nenhum gate foi reexecutado nesta etapa; reexecução, `make check`, execução de Godot e saídas de suítes permanecem **[NV]** até registro datado com saída colada na portaria própria. [source: contexto do orquestrador, 2026-11-20T11:39Z] (confidence: high quanto ao fato de ter sido citado; **[NV]** quanto à reexecução)
- A citação de gates é contexto de partida, não evidência a favor da implementação da tela: nenhum dos cinco gates verifica presença de `Control`/`Label` ou exibição de resultado. [source: natureza documental desta etapa] (confidence: high)
- **Remoto: [NV]** — nenhuma consulta a GitHub, CI, PR, deploy, Postgres/Coolify, federação ou provedor IAM foi feita ou é necessária; todo estado remoto permanece **[NV]**. [source: atribuição do mantenedor; `.specify/memory/STATE.md:9`] (confidence: high)

## Evidência contrária e riscos

- O evento é um `Dictionary` de runtime; sem uma tela, "exibir" poderia ser confundido com `print` em log. Mitigação: a meta exige componente `Control` com `Label` filhos e texto observável, não saída de console. [source: GDD `:1539`; `ilha_central.gd:74-78`] (confidence: high)
- `class_name` não resolve em verificação `--headless --script`, e a escrita manual de `.tscn` introduz risco de formato sem editor disponível. Mitigação: construir a árvore por código e usar `preload` no teste. [source: padrão verificado das suítes do alvo; `concept.md` deste ciclo] (confidence: high)
- Uma tela poderia inferir vitória de progresso, ilhas ou tempo e usurpar a decisão do servidor. Mitigação: só exibir quando `evento=="vitoria_ilha_central"` e `declarada_por=="servidor"`; caso contrário, estado neutro `AGUARDANDO`. [source: GDD `:1541`; `ilha_central.gd:75`] (confidence: high)
- A ausência de execução observada nesta etapa impede afirmar que a tela foi exibida de fato. Mitigação: rotular toda saída pendente como **[NV]** e deixar a verificação para a portaria própria. [source: atribuição do mantenedor] (confidence: high)

## Gaps & Open Questions

- Qual é a fonte única da tela? Respondido: o dicionário do evento do servidor, com as chaves literais de `ilha_central.gd`, nunca um parâmetro de comando do cliente.
- O que a tela deve mostrar? Respondido: a vitória declarada pelo servidor, o tick (`tempo_segundos`) e o estado (`VITORIA`/`AGUARDANDO`), sem terceiro estado.
- Como verificar sem editor e sem janela gráfica? Respondido: instância programática em `SceneTree`, com `preload`, TAB, tipos explícitos, `PASS`/`OK` e `quit(0/1)`; saídas **[NV]** até execução datada.

## Sources

- Arquivos locais do alvo (política: `local-only`): `docs/GDD - Insurgência.md:1537-1541,1569,1596`, `nucleo/desafios/ilha_central.gd:11,14,70-82`, `docs/progress/evidence.csv:8-10`, `.specify/validations/vitoria-central/validate.md:27`, `.specify/releases/vitoria-central/release.md:23`, `.specify/memory/STATE.md:9,17,25`, `.specify/feature.json`.
- Leitura de diretório em 2026-11-20: ausência de `nucleo/apresentacao/`; `.tscn` existentes apenas em `src/core/`.
- Contexto do orquestrador (política: dado do pedido, não buscado na web): gates 5/5 PASS em 2026-11-20T11:38:05Z, citados e não reexecutados.
- Remoto (política: fora de escopo): GitHub, CI, deploy e serviços — **[NV]**.
