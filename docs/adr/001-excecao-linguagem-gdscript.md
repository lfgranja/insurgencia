# ADR-001 — Exceção de linguagem para GDScript

- **ID**: ADR-001
- **Data**: 2026-11-19
- **Versão**: 1.0.1
- **Estado**: Aprovado

## Contexto

A norma externa do hub MDMV e a avaliação de stack do repositório registram uma tensão entre a presunção normativa de Rust como linguagem padrão e a escolha vinculante Godot-first, registrada em `docs/GDD - Insurgência.md:1265,1678` (subseção 16.2; N7). Essa escolha adota Godot (4.7.2, MIT, servidor *headless*) com GDScript como linguagem de engine e de script de jogo. A avaliação de stack registra dois desvios de intenção sem registro local correspondente: o desvio de linguagem (Rust padrão versus Godot/GDScript, `DEV-LING-01`) e a exceção de linguagem sem categoria nem ADR (`DEV-EXC-01`). A decisão R1 de 2026-11-19 (veredito `go`, opção A) autoriza fechar esses desvios com um registro local, sem alterar a norma externa nem a escolha de motor. O repositório é documental neste ciclo: nenhuma linguagem é executada, compilada ou instalada.

## Decisão

Fica autorizada, como exceção de linguagem local, a adoção de **GDScript como linguagem de engine e script de jogo** no ciclo 001, coerente com a decisão Godot-first já vigente. A exceção cobre unicamente o uso de GDScript no engine e nos scripts de jogo do Núcleo; não introduz segunda linguagem, não troca o engine e não altera a licença de nenhum componente. A decisão é afirmativa e não depende de nenhum efeito remoto, CI, merge ou aprovação já registrada.

## Alternativas consideradas

- **Emenda da norma externa MDMV** (opção B de `concept.md`): rejeitada neste ciclo por exigir autoridade externa e por estar fora de escopo; fica registrada apenas como contingência futura, sem execução e sem reabertura do pilar.
- **Servidor Rust como Plano B** (`docs/AVALIACAO-COMPARATIVA-ENGINES.md:538`): rejeitado neste ciclo por ser alternativa adiada e por conflitar com a escolha Godot-first vinculante; nenhum servidor Rust é introduzido.
- **Registro apenas em documento externo**: rejeitado porque a spec exige registros locais versionados e rastreabilidade consultável sem fonte externa.
- **Nenhuma decisão (manter o desvio sem registro)**: rejeitada por deixar qualquer trabalho futuro herdar um desvio de intenção classificado como bloqueante.

## Consequências

- O desvio de linguagem (`DEV-LING-01`) e a exceção sem registro (`DEV-EXC-01`) passam a ter decisão local correspondente, rastreável por leitura.
- A emenda normativa permanece como possibilidade futura: se a norma externa não aceitar a exceção local, ela poderá ser perseguida em ciclo próprio, com autoridade superior, sem execução neste ciclo e sem reabrir o pilar.
- Nenhuma segunda linguagem é introduzida neste ciclo; propostas futuras de mudança de linguagem ou troca de engine dependem das condições da seção `## Critérios de reavaliação`.
- O caráter documental do ciclo é preservado: nenhum código, compilação, teste, gate, pipeline, serviço externo ou emenda da norma externa decorre deste registro.

## Não reabertura de Godot-first

A decisão Godot-first, registrada em `docs/GDD - Insurgência.md:1265,1678` (subseção 16.2; N7), permanece vinculante e não é reaberta, revista ou revertida por este registro. A exceção documentada neste ADR não altera, não revê e não revoga a escolha do motor nem a combinação Godot-first (4.7.2, MIT, servidor *headless*), Núcleo + Módulos declarativos, servidor autoritativo e anti-cheat no servidor com *replay* e revisão humana. Revertê-la invalida o roadmap e a mitigação de débito técnico (N7). Nenhuma passagem vinculante do GDD é alterada; toda proposta futura de troca de engine é encaminhada a aprovação formal com evidência prévia, sem efeito neste ciclo.

## Critérios de reavaliação

A exceção pode ser reavaliada somente quando houver evidência prévia que justifique a mudança e aprovação formal pela autoridade competente. Qualquer proposta futura de segunda linguagem ou de troca de engine deve apresentar essa evidência e seguir aprovação formal antes de produzir efeito; nenhuma dessas mudanças é autorizada por este registro no ciclo atual. A reavaliação não reabre, não revê e não reverte a decisão Godot-first e não emenda a norma externa neste ciclo.

## Autoridade de aprovação

O maintainer do projeto é a autoridade de aprovação destes registros; a aprovação humana é necessária para que a decisão deixe a condição de pendente. Registro de aprovação: aprovada pelo maintainer lfgranja (autorização explícita + merges dos PRs #2/#3 em 2026-11-19); nenhum review APPROVED emitido na plataforma, e nenhum check exigido. Integração contínua: nenhum check de CI reportado (statusCheckRollup=[] nos PRs #1–#3, verificado ao vivo). A proteção de branch permanece `[NV]` por limitação de plano (HTTP 403 na API do GitHub), registrada como limitação conhecida. Nenhum critério de sucesso deste ciclo depende de status remoto.

## Rastreabilidade

| Referência | Tipo | Descrição | Origem local |
|---|---|---|---|
| `DEV-LING-01` | Desvio | Rust padrão versus Godot/GDScript vinculantes — desvio #1 da avaliação de stack. | `docs/AVALIACAO-STACK-MDMV.md:387` |
| `DEV-EXC-01` | Desvio | Exceção de linguagem sem categoria e sem ADR — desvio #3 da avaliação de stack. | `docs/AVALIACAO-STACK-MDMV.md:389` |
| `DEC-R1` | Decisão | Veredito `go`, opção A, de 2026-11-19. | `.specify/assessments/excecao-linguagem-licenca-nucleo/decision.md:5,29-31` |

As referências acima são as únicas aplicáveis a este registro, conforme a matriz do contrato (§2.3). Os demais desvios da avaliação de stack não se aplicam a este ADR e não são incluídos.

## Histórico de versões

### [1.0.1] — 2026-11-19

- Aprovado pelo maintainer lfgranja: autorização explícita do usuário em 2026-11-19 + `mergedBy=lfgranja` nos PRs #2/#3 (merges em 2026-11-19). Estado alterado de `Pendente de aprovação` para `Aprovado`; nenhum review `APPROVED` emitido na plataforma e nenhum check de CI exigido (`statusCheckRollup=[]` nos PRs #1–#3). Corpo da decisão inalterado.

### [1.0.0] — 2026-11-19

- Materialização inicial do registro de exceção de linguagem para GDScript.
- IDs FR atendidos: FR-001, FR-002, FR-003, FR-004, FR-009, FR-011, FR-012.
- IDs `T###` de origem da materialização: T008, T009, T010, T011, T012, T013, T014.
