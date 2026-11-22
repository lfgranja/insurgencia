# Índice de pesquisa e avaliação — Insurgência

> Escopo: artefatos de pesquisa/avaliação presentes em `docs/` e `.specify/assessments/` no
> alvo. Contagens verificadas em 2026-11-19 (comandos e saídas em `docs/PROGRESSO.md`).
> Afirmações remotas, de CI, merge ou publicação são **[NV]** até confirmação ao vivo.
> Este índice é descritivo; não substitui os artefatos e não registra conclusões novas.

## Contagens verificadas

| Conjunto | Contagem | Verificação |
|:--|--:|:--|
| Arquivos `docs/**/*.md` | 28 | `find docs -name '*.md' -type f \| wc -l` |
| Documentos na raiz de `docs/` | 7 | listagem |
| Arquivos em `docs/avaliacao-engines/` | 14 | 6 de estrutura + 8 em `pesquisa/` |
| Arquivos em `docs/avaliacao-stack/` | 5 | listagem |
| Spec em `docs/specs/` · plano em `docs/plans/` | 1 · 1 | listagem |
| Diretórios em `.specify/assessments/` | 175 | 171 numerados + 4 `ev-*` |
| `intake.md` · `decision.md` · `research.md` | 171 · 171 · 4 | contagem por nome de arquivo |
| Prefixos `ast` / `cor` / `doc` / `srv` / `ev-*` | 40 / 28 / 45 / 58 / 4 | contagem de diretórios |

## Avaliações na raiz de `docs/`

| Artefato | Descrição (uma linha) |
|:--|:--|
| `GDD - Insurgência.md` | Documento de projeto canônico (v1.0) do framework livre para MOBAs; define pilares, arquitetura Núcleo/Módulo, regras, licenças e as seções §§4, 14, 16 usadas como contexto pela spec e pelo plano. |
| `ASSESS-omoba-bevy.md` | Matriz auditável das 171 decisões sobre o inventário de `o-moba/omoba-bevy` (`speckit.assess`: intake → research → decide), com origem e escopo de aplicação à Insurgência. |
| `AVALIACAO-COMPARATIVA-ENGINES.md` | Avaliação comparativa de engines open source com protocolo adaptado da factory; critérios, pontuação e recomendação para o Insurgência. |
| `AVALIACAO-engines-open-source.md` | Avaliação direta de Godot, Bevy e alternativas FOSS aptas ao projeto, com campo "Objeto" e data de 2026-11-19. |
| `AVALIACAO-STACK-MDMV.md` | Verificação de conformidade com a norma `STACK_TECNOLOGICA_MDMV.md` (Ativo), a fonte da verdade para escolhas tecnológicas do projeto. |
| `Esporte vs. E-sport_ Controle das Regras.md` | Ensaios/dado de entrada sobre controle das regras: esporte como prática cultural de domínio público vs. e-sport como software proprietário; fonte externa registrada no próprio arquivo. |
| `Jogos MOBA Open Source e Frameworks.md` | Levantamento (a partir de prompt externo registrado) de jogos MOBA open source e frameworks, usado como insumo de paisagem tecnológica. |

## Conjuntos estruturados em `docs/`

| Diretório | Arquivos | Descrição |
|:--|--:|:--|
| `docs/avaliacao-engines/` | 14 | Conjunto em capítulos: cabeçalho, critérios, contrato de escrita, matriz, análise por opção e recomendação, mais `pesquisa/` com 8 notas (alternativas, bevy, godot, defold_flax_stride, libgdx_urho_panda, monogame_love_haxe, licencas_lockin, netcode_modularidade). |
| `docs/avaliacao-stack/` | 5 | Conjunto em capítulos: cabeçalho, contrato, matriz, análise da "Aroeira" e análise da Insurgência (aderência à stack padrão). |

## Assessments em `.specify/assessments/`

Cada diretório numerado contém `intake.md` (captação da ideia, origem e incógnitas) e
`decision.md` (scorecard, racional, pilar atendido e classe 80/20). Contagens por prefixo:

| Prefixo | Diretórios | Artefatos | Tema |
|:--|--:|:--|:--|
| `ast-*` | 40 | 40 intakes + 40 decisões | Aparato e entrega: CLI/Make, quality gate, CI multi-job/noturno, crosscheck Android/iOS, releases, checkout byte-exact, integridade de assets, Blender/procedural, sprites/IA, captura e publicação. |
| `cor-*` | 28 | 28 intakes + 28 decisões | Núcleo e rede: workspace Rust/Bevy, contratos e wire versionado, anti-rollback, snapshots e poda, tick em três fases, pathfinding, anti-trapaca, reconciliação ECS, fog-of-war, bots nativos e Ekza/Solana. |
| `doc-*` | 45 | 45 intakes + 45 decisões | Documentação e governança: missão e eixos, licenças e proveniência, portais de contribuição, portões de qualidade, runbooks, release SemVer, QA multiplayer, métricas, histórico datado (doc-31), plano de produto e skills. |
| `srv-*` | 58 | 58 intakes + 58 decisões | Servidor e produto: API Axum/SQLx, OpenAPI, migrações e privilégio mínimo, pairing/recuperação, outbox e matchmaking, presença, importador de avatares, Apple/StoreKit/Solana, bots headless, empacotamento mobile e releases. |
| `ev-*` | 4 | 4 `research.md` | Pesquisas de fundamentação (motor, plataforma, processo, licenças) — detalhadas abaixo. |
| **Total** | **175** | **171 intakes + 171 decisões + 4 research** | inventário completo verificado em 2026-11-19 |

### Pesquisas `ev-*`

| Slug | Artefato | Descrição (uma linha) |
|:--|:--|:--|
| `ev-motor` | `.specify/assessments/ev-motor/research.md` | Compara Bevy 0.19.1 (Rust/ECS) e Godot 4.7.2, avalia portabilidade dos padrões do omoba, especificidades Rust/ECS e risco de importar a codebase; fontes oficiais lidas como dado. |
| `ev-plataforma` | `.specify/assessments/ev-plataforma/research.md` | Pesquisa de plataforma, identidade e monetização (confidence overall: medium), incluindo portais de entrada, doações, Ekza e publicação. |
| `ev-processo` | `.specify/assessments/ev-processo/research.md` | Pesquisa de processo, governança e garantia de qualidade (confidence overall: high), com histórico datado, portões e QA como ações leves do próximo ciclo. |
| `ev-licencas` | `.specify/assessments/ev-licencas/research.md` | Licenciamento e soberania jurídica: núcleo MIT (`LICENSE`) frente ao mix AGPL/MPL/CC-BY/CC0/ISC do inventário o-moba, com riscos de contágio. |

## Spec e plano do ciclo 80/20

| Artefato | Descrição (uma linha) |
|:--|:--|
| `docs/specs/2026-11-19-ciclo-80-20-omoba.md` | Spec do ciclo: 9 assunções (Godot 4.7.2/GDScript/MIT, sem serviços externos, Núcleo de 35 itens classificados), objetivos e critérios CE-1 a CE-5 para a fatia vertical. |
| `docs/plans/2026-11-19-ciclo-80-20-omoba.md` | Plano de implementação: Núcleo contratual/simulação 30 Hz + Módulo declarativo, servidor headless autoritativo, Desafio de Ilha (2 jogadores em 5,0 s), gate local por versão/hash e evidência auditável versionada. |

## Rastreabilidade e [NV]

- Os 171 pares intake/decisão são a base da matriz `ASSESS-omoba-bevy.md`; os `ev-*`
  funcionam como research de apoio decisório (ex.: `doc-31` cita `ev-processo`).
- Divergência conhecida: 39 ocorrências da etiqueta NÚCLEO, 35 itens efetivamente
  classificados NÚCLEO; a rastreabilidade usa os 35 slugs reais (spec, ASSUNÇÃO 9).
- **Gates:** os 4 checkers SDD passam com `spec_dirs=0` — cobertura vazia, não implementação.
- **[NV]:** origens remotas dos documentos, execução de CI, merge, releases, publicação e
  qualquer estado de repositório externo (inclusive `o-moba/omoba-bevy` e a factory).

## Atualização verificada — 2026-11-21T13:18Z

As contagens abaixo são as vigentes; as seções anteriores permanecem como histórico de 19–20/11 e não foram reescritas.

| Conjunto | Contagem | Comando |
|:--|--:|:--|
| specs numeradas | 30 | `find specs -maxdepth 1 -type d -name '[0-9][0-9][0-9]-*' \| wc -l` |
| assessments | 233 (28 `sb-*`) | `find .specify/assessments -mindepth 1 -maxdepth 1 -type d \| wc -l` |
| validations / releases | 30 / 30 | `find .specify/{validations,releases} -mindepth 1 -maxdepth 1 -type d \| wc -l` |
| bugs | 0 | `find .specify/bugs ...` / `find . -name 'BUG-*.md'` |
| research (md) | 13 (9 `slotbound-*`) | `find docs/research -name '*.md' \| wc -l` |

- O gate **não é mais** `spec_dirs=0`: os 5 gates SDD PASSaram às 13:18:27Z com `spec_dirs=30 gaps=0`, converge `13/13`, DAG `262 tarefas/0 ciclos`, task-id `0 duplicatas`, hub-docs `0 erros`.
- Nova família documental: `docs/research/slotbound-2026-11-21-*.md` (8 notas + síntese), com vereditos para 28 propostas do vídeo `Z4ayMTXnkoQ`.
- **[NV]**: `./verify.sh` completo em execução; CI/merge/proteções/confirmação visual da SE — ver `docs/PROGRESSO.md` para a verificação ao vivo de 13:19:29Z via `gh`.

## Atualização verificada — 2026-11-22T18:34Z

As contagens abaixo são as vigentes; a atualização de 13:18Z (30 specs / 233 assessments, `spec_dirs=30`) permanece como histórico de 21/11 e não foi reescrita.

| Conjunto | Contagem | Verificação |
|:--|--:|:--|
| specs numeradas | 34 | `find specs -maxdepth 1 -type d -name '[0-9][0-9][0-9]-*' \| wc -l` |
| assessments | 237 | `find .specify/assessments -mindepth 1 -maxdepth 1 -type d \| wc -l` |
| validations / releases | 34 / 34 | `find .specify/{validations,releases} -mindepth 1 -maxdepth 1 -type d \| wc -l` |
| bugs | 0 | `find .specify/bugs ...` / `find . -name 'BUG-*.md'` |
| research (md) | 13 | `find docs/research -name '*.md' \| wc -l` |

- Gates (18:32:17Z, observado pela orquestração via `gh`/`git`): `./verify.sh` ALL STAGES OK, 41/41; SDD 34/0; DAG 314/17/0; task-id 0; converge 17/17; hub-docs 0 erros. Âncora `feature.json → specs/034-gate-manifesto-cli`; HEAD `f9d87e0`.
- PR #40 `dev→main` `OPEN` na fila do SWE (`gates`/`rust` pass); **sem merge de `main`**. **[NV]**: estado remoto posterior e qualquer afirmação remota não observada nesta sessão.
