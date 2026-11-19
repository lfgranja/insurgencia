# Contract: Registros de Decisão (ADR) — exceção de linguagem e licença MIT-only

**Versão do contrato**: `1.0.0` | **Created**: 2026-11-19 | **Fase**: F1 (Phase 1 — Contracts)

**Escopo**: define a forma dos dois registros de decisão locais do ciclo 001 — campos, identificadores, rastreabilidade FR/T, versionamento SemVer, changelog e rollback. Não define código, build, teste, gate, pipeline, migração ou serviço externo. A origem autoritativa do conteúdo é `spec.md`; divergências de desenho estão em `research.md` e as entidades em `data-model.md`.

## 1. Identificadores e caminhos canônicos

| ID | Tipo | Significado | Caminho canônico (materialização na fase 12) |
|---|---|---|---|
| `ADR-001` | Registro | Exceção de linguagem — GDScript como engine/script de jogo. | `docs/adr/001-excecao-linguagem-gdscript.md` |
| `ADR-002` | Registro | Licença MIT-only do Núcleo e do servidor headless do ciclo. | `docs/adr/002-licenca-mit-nucleo.md` |
| `DEV-LING-01` | Desvio | Rust padrão vs Godot/GDScript vinculantes — avaliação #1. | Campo `rastreabilidade` do ADR-001 |
| `DEV-EXC-01` | Desvio | Exceção de linguagem sem categoria e sem ADR — avaliação #3. | Campo `rastreabilidade` do ADR-001 |
| `DEV-LIC-01` | Desvio | MIT vigente vs AGPL/N3 aberta — avaliação #10. | Campo `rastreabilidade` do ADR-002 |
| `DEC-R1` | Decisão | Veredito `go`, opção A, de 2026-11-19. | Campo `rastreabilidade` de ambos os ADRs |

**Regra de numeração**: estes são identificadores locais definidos por R1 e pela spec. O documento `docs/AVALIACAO-STACK-MDMV.md` usa `ADR-002` para integração contínua e `ADR-006` para licenciamento; essa divergência é registrada em `research.md` (RQ-2) e não autoriza renomeação, terceiro registro ou emenda do documento externo neste ciclo.

## 2. Campos obrigatórios e matriz de rastreabilidade

### 2.1 Campos comuns (aplicam-se a ADR-001 e ADR-002)

| Campo | Obrigatório | Formato / regra | Origem |
|---|---|---|---|
| `id` | Sim | Literal conforme §1; imutável e único no ciclo. | FR-001 |
| `titulo` | Sim | Texto curto que nomeie a decisão. | FR-009 |
| `data` | Sim | `YYYY-MM-DD`. | FR-009 |
| `contexto` | Sim | Texto com as fontes locais que motivam a decisão. | FR-009 |
| `decisao` | Sim | Texto afirmativo; não pode ser condicional nem ambiguamente alternativo. | FR-009 |
| `alternativas_consideradas` | Sim | Texto que registre ao menos as alternativas descritas em `data-model.md`. | FR-009 |
| `consequencias` | Sim | Texto com efeitos e contingências, sem executá-las. | FR-009 |
| `clausula_nao_reabertura` | Sim | Declaração explícita de que Godot-first permanece vinculante e não é reaberta, revista ou revertida. | FR-003, SC-004 |
| `autoridade_aprovacao` | Sim | Maintainer como autoridade; aprovação humana necessária; registro efetivo `[NV]`. | FR-009, FR-011, SC-006 |
| `rastreabilidade` | Sim | Lista de IDs conforme §2.2; nenhuma lista vazia. | FR-012, SC-008 |
| `versao` | Sim | SemVer conforme §4. | contrato §4 |
| `estado` | Sim | Enum `Rascunho` \| `Pendente de aprovação` \| `Aprovado` \| `Emendado` \| `Revogado`, conforme `data-model.md`. | FR-011 |
| `changelog` | Sim | Entradas conforme §4 e §5; inclusão de IDs FR/T. | contrato §4–§5 |

### 2.2 Campos específicos

| Registro | Campo | Regra obrigatória | Origem |
|---|---|---|---|
| ADR-001 | `linguagem_adotada` | GDScript como linguagem de engine e script de jogo. | FR-002 |
| ADR-001 | `criterios_reeavaliacao` | Evidência prévia e aprovação formal; nenhuma segunda linguagem no ciclo. | FR-004 |
| ADR-002 | `posicao_licenca` | MIT para todo o código do ciclo, incluindo Núcleo e servidor headless. | FR-005 |
| ADR-002 | `exclusao_alternativas` | Exclui "MIT ou GPL" e AGPL para o código do ciclo. | FR-006 |
| ADR-002 | `regra_copyleft` | Nenhuma cópia de código copyleft; contribuições de terceiros identificadas e atribuídas. | FR-007 |
| ADR-002 | `ressalva_motor` | Não altera nem atribui licença exclusiva ao motor adotado. | FR-008 |
| ADR-002 | `fora_escopo` | Arquivo de licenças de terceiros fora do ciclo, sem decisão implícita. | FR-013 |

### 2.3 Matriz de rastreabilidade de desvios

| Referência | ADR-001 | ADR-002 | Resultado exigido |
|---|---|---|---|
| `DEV-LING-01` | Presente | Não se aplica | Desvio de linguagem com decisão local. |
| `DEV-EXC-01` | Presente | Não se aplica | Exceção sem registro coberta por registro. |
| `DEV-LIC-01` | Não se aplica | Presente | Desvio de licença com decisão local. |
| `DEC-R1` | Presente | Presente | Ambos os ADRs derivados da decisão `go`. |

"Não se aplica" acima significa que a referência não deve ser incluída naquele registro; não é um campo pendente. Cobertura exigida: 3/3 desvios (SC-002) e 4/4 referências (SC-008).

### 2.4 Estrutura canônica do documento ADR

Ordem dos títulos: `# <ID> — <título>`; `## Contexto`; `## Decisão`; `## Alternativas consideradas`; `## Consequências`; `## Não reabertura de Godot-first`; `## Critérios de reavaliação` (ADR-001) ou `## Licenciamento` (ADR-002); `## Autoridade de aprovação`; `## Rastreabilidade`; `## Histórico de versões`. Os campos `data`, `versao` e `estado` aparecem no cabeçalho do documento. Nenhum título é opcional dentro do registro que o contém; seções específicas de um ADR não são exigidas no outro.

## 3. Identificadores FR e T

### 3.1 Rastreabilidade FR → elemento contratual

| FR | Elemento contratual que o materializa | Evidência de conformidade |
|---|---|---|
| FR-001 | §1 (dois IDs fixos) | Exatamente `ADR-001` e `ADR-002` no ciclo. |
| FR-002 | §2.2 `linguagem_adotada` | GDScript declarado como engine/script de jogo. |
| FR-003 | §2.1 `clausula_nao_reabertura` | Declaração presente nos dois ADRs. |
| FR-004 | §2.2 `criterios_reeavaliacao` | Evidência e aprovação formal exigidas. |
| FR-005 | §2.2 `posicao_licenca` | MIT para Núcleo e servidor headless. |
| FR-006 | §2.2 `exclusao_alternativas` | Sem "ou GPL" e sem AGPL no código do ciclo. |
| FR-007 | §2.2 `regra_copyleft` | Sem cópia copyleft; atribuição de terceiros. |
| FR-008 | §2.2 `ressalva_motor` | Licença do motor intacta e não exclusiva. |
| FR-009 | §2.1 campos comuns | Todos os campos mínimos presentes. |
| FR-010 | Escopo deste contrato e `quickstart.md` | Nenhum código, build, teste, gate, CI, serviço ou emenda normativa nos ADRs. |
| FR-011 | §2.1 `autoridade_aprovacao` e `estado` | Aprovação humana necessária; remoto `[NV]`. |
| FR-012 | §2.3 matriz | Nenhum desvio sem decisão local. |
| FR-013 | §2.2 `fora_escopo` | Arquivo de licenças de terceiros fora do ciclo. |

### 3.2 Identificadores `T###` (fase tasks)

Cada FR da tabela §3.1 deve ser coberto por ao menos uma tarefa `T###` na fase 11 (`tasks.md`), no formato `- [ ] T### [P?] [USn?] descrição com caminho`. Os números `T###` não são fixados neste contrato porque `tasks.md` ainda não existe — trata-se de não aplicabilidade deliberada deste artefato, não de um campo pendente. A associação FR→T será verificada na fase analyze. Nenhuma tarefa pode introduzir comportamento que FR-010 veta.

### 3.3 Identificadores `BUG-###` (achados documentais)

Achados de qualidade e inconsistências detectados na verificação por leitura usam ID no formato `BUG-###` — prefixo `BUG-` seguido de três dígitos (ex.: `BUG-001`). A lista de IDs afetados de uma entrada de changelog (§4) ou de rollback (§5) pode combinar IDs FR, IDs `T###` e IDs `BUG-###`. Um `BUG-###` não autoriza código, gate, CI, efeito remoto, emenda da norma externa ou reabertura de Godot-first (FR-010, §5).

## 4. Versionamento SemVer e changelog

- `1.0.0`: versão inicial de cada ADR no momento da materialização.
- **MAJOR** (`2.0.0`, ...): alteração da decisão, da cláusula de não reabertura, do escopo de licenciamento ou remoção de campo obrigatório.
- **MINOR** (`1.1.0`, ...): acréscimo de alternativa, critério de reavaliação ou detalhe de autoridade sem mudar a decisão.
- **PATCH** (`1.0.1`, ...): correção textual, de data, de referência ou de atribuição, sem mudança de sentido.
- Cada entrada do `changelog` registra: versão, data `YYYY-MM-DD`, descrição, IDs FR afetados e, quando houver, IDs `BUG-###`; quando a tarefa correspondente existir, também o ID `T###` da fase 11.
- Forma documental: `### [1.0.0] — 2026-11-19` seguida de itens com os IDs. Nenhum release, tag remota, merge ou CI é afirmado como executado por este contrato; a fase release (R7) permanece posterior e todo efeito remoto é `[NV]`.

## 5. Rollback documental

**Gatilhos**: reabertura involuntária detectada em Godot-first; conflito de licenciamento (cláusula alternativa, AGPL no código do ciclo ou mudança na licença do motor); rastreabilidade incompleta; aprovação humana não registrada.

**Procedimento**:

1. Registrar o achado com os IDs FR afetados e, quando existirem, os IDs `BUG-###` e os IDs `T###`.
2. Criar um novo commit que emende ou revogue o ADR, com nova versão SemVer e data; nunca apagar ou reescrever o registro anterior.
3. Atualizar a matriz de rastreabilidade (§2.3) para preservar cobertura 3/3 e 4/4.
4. Registrar a entrada no changelog com versão, data e IDs.
5. Manter `clausula_nao_reabertura` e `ressalva_motor` intactos em qualquer reversão.
6. Marcar como `[NV]` qualquer efeito remoto até confirmação ao vivo.

**Proibido**: apagar histórico, `force-push`, auto-merge, alterar a licença do motor, emendar a norma externa neste ciclo ou reabrir Godot-first como mecanismo de correção.

**Não aplicável**: rollback de runtime, de banco de dados, de migração, de binário, de configuração de CI ou de serviço externo — motivo: o ciclo produz somente registros documentais versionados (spec FR-010 e SC-007); nenhum desses artefatos existe para ser revertido.

## 6. Campos e operações não aplicáveis ao contrato (com motivo)

| Campo/operação | Declaração | Motivo |
|---|---|---|
| Endpoint, esquema HTTP/gRPC, autenticação de API | Não aplicável | Nenhuma interface de rede existe; o contrato é descritivo e documental. |
| Schema executável, validador automático, lint | Não aplicável | FR-010 veta gates e código; a conformidade é conferida por leitura (`quickstart.md`). |
| Array de tarefas `T###` fixo neste contrato | Não aplicável | `tasks.md` é saída da fase 11; a regra de correspondência está em §3.2. |
| Estado remoto, tag publicada, aprovação obtida | Não aplicável (rotulado `[NV]`) | Constituição VI e SC-006 exigem confirmação ao vivo antes de qualquer afirmação. |
| Resultado de release (R7) | Não aplicável nesta fase | R4 precede R7; ambos são portarias do ciclo completo, posteriores à fase de planejamento e aqui não executadas; o contrato apenas define o formato exigido. |

## 7. Critérios de conformidade (verificação por leitura)

1. Existem exatamente dois registros com os IDs de §1 e os caminhos canônicos após a fase 12.
2. Todos os campos de §2.1 e os campos específicos aplicáveis de §2.2 estão presentes e não vazios.
3. A matriz §2.3 satisfaz 3/3 desvios e 4/4 referências, com `DEC-R1` nos dois ADRs.
4. A cláusula de não reabertura aparece nos dois registros e nenhuma passagem vinculante de Godot-first foi alterada.
5. MIT-only aparece sem "ou GPL" e sem AGPL para o código do ciclo; a licença do motor não é atribuída como exclusiva.
6. Nenhum registro contém código, compilação, teste, gate de build, pipeline, serviço externo ou emenda da norma externa.
7. Aprovação humana aparece como autoridade necessária e todo status remoto está `[NV]`.
8. Toda versão, changelog e eventual rollback seguem §4 e §5, com IDs FR e, quando houver, IDs `T###` e IDs `BUG-###`.
