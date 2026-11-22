# Contract: Emenda pontual do GDD a partir dos ADRs 001/002

**Versão do contrato**: `1.0.0` | **Created**: 2026-11-19 | **Fase**: F1 (Phase 1 — Contracts)

**Escopo**: define a forma das sete passagens emendadas de `docs/GDD - Insurgência.md` — mapa linha → FR → `T###` → autoridade, regras de redação e rastreabilidade, versionamento SemVer, changelog e rollback documental. Não define código, build, teste, gate, pipeline, migração, serviço externo, efeito remoto nem emenda da norma externa. A origem autoritativa do conteúdo é `spec.md`; as decisões de desenho estão em `research.md` e as entidades em `data-model.md`. O GDD é o único artefato normativo de origem alterado pela feature; os artefatos de planejamento são apenas conferidos por leitura, sem alteração. As verificações documentais executadas neste ciclo retornam resultado observável; os gates determinísticos locais pertencem à portaria R4 (validate), não a tarefas desta feature.

## 1. Identificadores, caminho canônico e âncoras

| ID | Tipo | Significado | Localização |
|---|---|---|---|
| `GDD` | Documento | Único artefato normativo de origem alterado pela feature; os artefatos de planejamento são apenas conferidos por leitura, sem alteração. | `docs/GDD - Insurgência.md` |
| `P-1389` … `P-1673` | Passagem | Sete trechos emendados, um por FR. | Linhas `1389,1451,1459,1594,1659,1668,1673` do GDD |
| `ADR-001` | Autoridade | Independência de CI e não reabertura Godot-first. | `docs/adr/001-excecao-linguagem-gdscript.md:14,32,40` |
| `ADR-002` | Autoridade | Licença MIT única, sem alternativa e sem AGPL para o código do ciclo. | `docs/adr/002-licenca-mit-nucleo.md:14,28,37-41` |
| `DEC-R1` | Decisão | Veredito `go`, opção A, de 2026-11-19. | `.specify/assessments/emenda-gdd-adrs/decision.md:5,21-25` |
| `A-1265`, `A-1678` | Âncora | Premissa Godot-first, intocável; usada só como referência de comparação. | `docs/GDD - Insurgência.md:1265,1678` |

**Regra de numeração**: a rastreabilidade usa a numeração materializada dos ADRs. A numeração provisória da avaliação de stack (`ADR-002` = CI, `ADR-006` = licenciamento) é divergência registrada em `research.md RQ-2` e não é citada em nenhuma passagem emendada.

**Regra de âncora**: nenhuma passagem do mapa §2 pode alterar, revê ou reverter `A-1265` ou `A-1678`. A comparação textual das duas âncoras contra o baseline deve registrar zero alterações (SC-002).

## 2. Mapa das sete passagens (linha → FR → T###)

| Passagem | Linha / seção | FR | `T###` de origem | Autoridade no GDD | Conteúdo divergente verificado | Estado-alvo exigido |
|---|---|---|---|---|---|---|
| `P-1389` | `1389` / 15.5 | FR-001 | `T008` | ADR-002 | "Servidor pode ser AGPL"; soberania depende de portabilidade, não da licença do servidor. | Ressalva restrita a servidores de terceiros; explícito que não impõe AGPL ao código do projeto (MIT única do ADR-002); portabilidade e direito de saída preservados; nenhuma decisão sobre licenças de terceiros. |
| `P-1451` | `1451` / 15.9 | FR-002 | `T009` | ADR-002 | "Licença MIT ... para o **Núcleo**; AGPL-3.0 para o servidor"; `LICENSE` e `COPYING`. | MIT única para Núcleo e servidor; sem AGPL-3.0 e sem `COPYING`; declaração/licença MIT mantida. |
| `P-1459` | `1459` / 15.9 | FR-003 | `T010` | ADR-002 | "Licença MIT/AGPL e contrato de contribuição protegem o código e a autoria". | Par "MIT/AGPL" substituído por MIT única; proteção de código e autoria preservada; ressalva sobre instância injusta preservada. |
| `P-1594` | `1594` / 18 | FR-004 | `T015` | ADR-001 | "integração contínua desde o primeiro dia". | Promessa substituída pelo registro do DEFER: CI e gate remoto diferidos neste ciclo, sem efeito remoto; nenhum gatilho, pipeline ou build. |
| `P-1659` | `1659` / 20.3 | FR-005 | `T011` | ADR-002 | "N3 — Licença do Núcleo: MIT ou GPL? A fonte menciona ambas; a Godot é MIT." | N3 respondida por MIT única; sem alternativa "ou GPL" e sem reabertura da licença do motor; ressalva de que o motor não é o único motor MIT preservada. |
| `P-1668` | `1668` / 20.4 | FR-006 | `T012` | ADR-002 | N3 bloqueia "Publicação do repositório e do catálogo de módulos"; prazo "Primeira publicação". | N3 não consta como bloqueante; registrada como decidida pelo ADR-002; emenda não condicionada a publicação externa. |
| `P-1673` | `1673` / 20.4 | FR-007 | `T013` | ADR-002 | "Enquanto N3, N4, N6 e N8 não são respondidos ...". | N3 removida da lista; N4, N6 e N8 preservados; Fase 0 segue com N3 respondido; âncoras intocadas. |

**Regras do mapa**:

1. Exatamente sete linhas, uma por FR, sem repetição de `FR`, de `T###` ou de linha.
2. Toda passagem alterada contém, no próprio GDD, a referência textual ao ADR da coluna "Autoridade no GDD" (SC-004: 7/7).
3. Nenhuma passagem contém numeração provisória de ADR, código, compilação, teste, gate de build, pipeline, serviço externo ou emenda da norma externa (SC-007).
4. O texto ao redor de cada linha é preservado; qualquer oitava alteração é fora de escopo e exige rollback documental (§5).

## 3. Identificadores FR e T

### 3.1 Rastreabilidade FR → passagem → tarefa

| FR | Passagem | `T###` de origem | Evidência de conformidade (verificação por leitura) |
|---|---|---|---|
| FR-001 | `P-1389` | `T008` | Ressalva de terceiro esclarecida, sem AGPL imposta ao projeto, com ADR-002 no GDD. |
| FR-002 | `P-1451` | `T009` | MIT única para Núcleo e servidor, sem `COPYING`, com ADR-002 no GDD. |
| FR-003 | `P-1459` | `T010` | Par "MIT/AGPL" ausente, ressalva preservada, com ADR-002 no GDD. |
| FR-004 | `P-1594` | `T015` | Promessa de CI dia 1 ausente, DEFER registrado, sem efeito remoto, com ADR-001 no GDD. |
| FR-005 | `P-1659` | `T011` | N3 respondida por MIT único, sem alternativa, com ADR-002 no GDD. |
| FR-006 | `P-1668` | `T012` | N3 não bloqueia a "Primeira publicação", sem dependência externa, com ADR-002 no GDD. |
| FR-007 | `P-1673` | `T013` | N3 removida da lista, N4/N6/N8 intactos, com ADR-002 no GDD. |

Cada FR da tabela é coberto por pelo menos uma tarefa `T###` na fase tasks (`tasks.md`), no formato `- [ ] T### [P?] [USn?] descrição com caminho`. As tarefas de verificação por leitura (`T014`, `T016`, `T017`–`T022`) complementam a materialização sem introduzir comportamento vetado pela spec.

### 3.2 Identificadores `BUG-###` (achados documentais)

Achados de qualidade detectados na verificação por leitura usam ID `BUG-###` — prefixo `BUG-` seguido de três dígitos (ex.: `BUG-001`). Uma entrada de changelog (§5) ou de rollback (§5) pode combinar IDs FR, IDs `T###` e IDs `BUG-###`. Um `BUG-###` não autoriza código, gate, CI, efeito remoto, emenda da norma externa, alteração de âncora ou reabertura de Godot-first.

## 4. Regras de redação e verificação por leitura

| Regra | Conteúdo | Verificação por leitura | Origem |
|---|---|---|---|
| R-1 | Toda passagem emendada cita o ADR que a decide, na própria passagem. | Contar 7 ocorrências de `ADR-001`/`ADR-002` nas sete passagens. | FR-001–FR-007, SC-004 |
| R-2 | Nenhuma passagem de licenciamento mantém "MIT ou GPL", "MIT/AGPL" ou AGPL imposta ao código do projeto. | Ler `1389,1451,1459,1659,1668,1673` e confirmar ausência dessas formulações. | FR-001–FR-003, FR-005–FR-007, SC-005 |
| R-3 | A passagem de CI não promete integração contínua no dia 1 e não introduz gatilho, pipeline, build ou serviço externo. | Ler `1594` e confirmar o registro do DEFER e a ausência de efeito remoto. | FR-004, SC-006 |
| R-4 | N3 não aparece como pergunta aberta nem bloqueante; N4, N6 e N8 permanecem como estavam. | Ler `1659,1668,1673` e comparar com as perguntas preservadas. | FR-005–FR-007, SC-005 |
| R-5 | As âncoras `1265,1678` são idênticas ao baseline; nenhuma seção vizinha é reescrita. | Comparação textual das duas âncoras e do texto ao redor das sete passagens. | SC-002, SC-003 |
| R-6 | Nenhum resultado afirma aprovação em plataforma, CI executada, merge, proteção de branch ou publicação externa obtida. | Conferir a rotulação `[NV]` em toda evidência documental do ciclo. | SC-003, constituição VI |

**Critérios de aceite da redação**: uma passagem só é considerada emendada quando o estado-alvo do mapa §2 é verificável por leitura e a regra aplicável R-1…R-6 é satisfeita sem alterar o texto ao redor. Nenhuma redação é afirmada como executada neste contrato. As verificações documentais executadas neste ciclo retornam resultado observável; os gates determinísticos locais pertencem à portaria R4 (validate), não a tarefas desta feature.

## 5. Versionamento, changelog e rollback documental

### 5.1 Versionamento SemVer

- `1.0.0`: versão inicial do registro documental da emenda materializada.
- **MAJOR** (`2.0.0`, ...): alteração de escopo (passagem acrescentada ou removida), de autoridade invocada ou de estado-alvo de uma passagem.
- **MINOR** (`1.1.0`, ...): acréscimo de esclarecimento ou de referência sem mudar o estado-alvo das sete passagens.
- **PATCH** (`1.0.1`, ...): correção textual, de pontuação ou de linha citada, sem mudança de sentido.

### 5.2 Changelog

Cada entrada registra: versão, data `YYYY-MM-DD`, descrição, IDs FR afetados e, quando houver, IDs `BUG-###`; quando a tarefa correspondente existir, também o ID `T###`. Forma documental: `### [1.0.0] — 2026-11-19` seguida de itens com os IDs. Nenhuma versão é afirmada como publicada em tag remota, mesclada ou aprovada por CI; todo efeito remoto permanece `[NV]`.

### 5.3 Rollback documental

**Gatilhos**: passagem emendada que contradiga a autoridade invocada; alteração involuntária das âncoras `1265,1678` ou de seção vizinha; rastreabilidade incompleta (menos de 7/7 ADRs citados); reintrodução de "MIT ou GPL", AGPL para o código do projeto, "DEFER com gatilho", pipeline ou efeito remoto; aprovação remota afirmada sem confirmação ao vivo.

**Procedimento**:

1. Registrar o achado com os IDs FR afetados e, quando existirem, os IDs `BUG-###` e os IDs `T###` do mapa §3.
2. Criar um novo commit que restaure a redação anterior **apenas da passagem afetada**, com nova versão SemVer e data; nunca apagar ou reescrever o registro anterior.
3. Reconfirmar que as duas âncoras permanecem idênticas ao baseline e que as demais seis passagens não foram tocadas.
4. Atualizar o mapa §2 e a rastreabilidade §3 para preservar 7/7 passagens, 7/7 FR e 7/7 ADRs citados.
5. Registrar a entrada no changelog com versão, data e IDs.
6. Manter `A-1265`, `A-1678` e a ressalva de portabilidade/direito de saída intactos em qualquer reversão.
7. Marcar como `[NV]` qualquer efeito remoto até confirmação ao vivo.

**Proibido**: apagar histórico, `force-push`, auto-merge, alterar âncoras, emendar a norma externa neste ciclo, decidir licenças de terceiros ou reabrir Godot-first como mecanismo de correção.

**Não aplicável**: rollback de runtime, banco de dados, migração, binário, configuração de CI ou serviço externo — motivo: a feature produz somente texto versionado (spec SC-007); nenhum desses artefatos existe para ser revertido.

## 6. Campos e operações não aplicáveis ao contrato (com motivo)

| Campo/operação | Declaração | Motivo |
|---|---|---|
| Endpoint, esquema HTTP/gRPC, autenticação de API | **Não aplicável** | Nenhuma interface de rede existe; o contrato é descritivo e documental. |
| Schema executável, validador automático, lint | **Não aplicável** | SC-007 veta gates e código no resultado; a conformidade é conferida por leitura (`quickstart.md`); os gates determinísticos locais pertencem à portaria R4 (validate), não a tarefas desta feature. |
| Redação final das sete passagens | **Não aplicável neste artefato** | O contrato fixa regras e estados-alvo; a redação emendada é materializada na fase de implementação. |
| Estado remoto, tag publicada, aprovação obtida | **Não aplicável (rotulado `[NV]`)** | Constituição VI e SC-003 exigem confirmação ao vivo antes de qualquer afirmação. |
| Resultado de release | **Não aplicável nesta fase** | R4 precede R7; ambas são portarias do ciclo completo, posteriores a esta fase de planejamento; o contrato apenas define o formato exigido. |
| Decisão sobre licenças de terceiros | **Não aplicável** | Fora da autoridade do ADR-002; a passagem `1389` só esclarece o escopo da ressalva. |

## 7. Critérios de conformidade (verificação por leitura)

1. As sete passagens do mapa §2 estão emendadas, uma por FR, com `T###` único e sem oitava alteração.
2. Toda passagem alterada identifica o ADR que a decide no próprio GDD: 7/7 (SC-004).
3. Não restam "MIT ou GPL", "MIT/AGPL", AGPL para o código do projeto, `COPYING` ou N3 como pergunta aberta/bloqueante (SC-001, SC-005).
4. A passagem `1594` registra CI e gate remoto como diferidos, sem promessa de dia 1, gatilho, pipeline, build ou serviço externo (SC-006).
5. As âncoras `1265,1678` são idênticas ao baseline: zero alterações (SC-002).
6. N4, N6 e N8 permanecem como estavam; a Fase 0 segue com N3 respondido (FR-007).
7. Nenhuma passagem contém código, compilação, teste, gate de build, pipeline, serviço externo ou emenda da norma externa (SC-007).
8. Nenhum resultado afirma aprovação remota, CI executada, merge, proteção de branch ou publicação obtida; tudo permanece `[NV]` (SC-003).
9. Versão, changelog e eventual rollback seguem §5, com IDs FR e, quando houver, IDs `T###` e `BUG-###`.
