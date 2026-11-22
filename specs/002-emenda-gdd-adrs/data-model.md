# Data Model: Emenda pontual do GDD a partir dos ADRs 001/002

**Feature**: `002-emenda-gdd-adrs` | **Created**: 2026-11-19 | **Fase**: F1 (Phase 1 — Design)

**Origem**: spec `## Key Entities` (GDD, ADR-001, ADR-002, Mapa de 7 passagens, Âncoras Godot-first), `## Requirements` (FR-001–FR-007), `## Success Criteria` (SC-001–SC-007) e decisões D1–D9 de `plan.md`.

**Premissa**: as "entidades" abaixo são trechos de um arquivo Markdown versionado (`docs/GDD - Insurgência.md`) e registros documentais de apoio, não tabelas, objetos de runtime ou contratos de API. Não há persistência, migração, chave estrangeira ou ciclo de vida executável neste ciclo. Todo campo listado é obrigatório na materialização da emenda, salvo indicação em contrário.

## Visão geral

| Entidade | Identidade | Finalidade | Origem |
|---|---|---|---|
| GDD | `GDD` | Único artefato alterado; documento normativo e fonte de leitura para mantenedores, forks e governança. | spec Key Entities, FR-001–FR-007 |
| Passagem de emenda | `P-1389`, `P-1451`, `P-1459`, `P-1594`, `P-1659`, `P-1668`, `P-1673` | Uma entrada por passagem divergente, com conteúdo verificado, autoridade e estado-alvo. | spec Mapa de 7 passagens; contrato §2 |
| Âncora Godot-first | `A-1265`, `A-1678` | Premissa intocável usada apenas como referência de comparação na auditoria. | spec Key Entities, SC-002 |
| Rastreabilidade ADR | `ADR-001`, `ADR-002`, `DEC-R1` | Identificar a autoridade invocada por cada passagem, sem fonte externa. | spec Key Entities, SC-004 |
| Registro de emenda | `EM-002-1.0.0` | Versão, changelog, contagens de verificação e rollback documental da emenda. | data-model §Entidade Registro de emenda (morada única de `EM-002-1.0.0` e `contagens`); regra de formato em contrato §5; sdd-phase-contracts.md:27 |

```text
DEC-R1 (decisão R1, go/opção A)
   ├── ADR-001 ──────► P-1594 (CI e gate remoto diferidos)
   └── ADR-002 ──────► P-1389, P-1451, P-1459, P-1659, P-1668, P-1673

GDD ── contém ──► 7 Passagens de emenda ──► 2 Âncoras intactas (comparação)
GDD ── registra ─► EM-002-1.0.0 (changelog + rollback documental)

P-1389: ressalva_terceiros + nao_impoe_agpl + portabilidade_saida + ref_adr
P-1451: licenca_unica + sem_copying + ref_adr
P-1459: substituicao_par_mit_agpl + protecao_codigo + ref_adr
P-1594: sem_promessa_ci + defer_gate_remoto + sem_efeito_remoto + ref_adr
P-1659: n3_respondida + sem_alternativa + ressalva_motor + ref_adr
P-1668: n3_nao_bloqueante + sem_dependencia_publicacao + ref_adr
P-1673: n3_removida_lista + n4_n6_n8_preservados + fase0_segue + ref_adr
```

## Entidade GDD

**Arquivo canônico**: `docs/GDD - Insurgência.md` (único artefato normativo de origem alterado pela feature; os demais artefatos de planejamento são apenas conferidos por leitura, sem alteração).

| Campo | Tipo | Obrigatório | Origem | Regra de validação |
|---|---|---|---|---|
| `id` | Literal `GDD` | Sim | spec Key Entities | Imutável; um único documento normativo no escopo. |
| `caminho` | Caminho | Sim | spec Key Entities | `docs/GDD - Insurgência.md`; nenhum outro documento é emendado. |
| `escopo_emenda` | Lista de 7 linhas | Sim | spec Mapa de 7 passagens | Exatamente `1389,1451,1459,1594,1659,1668,1673`; nenhuma linha adicional é reescrita. |
| `texto_ao_redor` | Texto | Sim | spec Assumptions | Preservado integralmente em cada passagem; a emenda é pontual e revertível por comparação textual. |
| `estado` | Enum | Sim | contrato §4 | `Divergente` → `Emendado` → `Verificado`; `Emendado` → `Revertido` somente por rollback documental. |

**Transições de estado**: `Divergente` → `Emendado` (sete passagens reescritas conforme o contrato); `Emendado` → `Verificado` (leitura cruzada das passagens e das âncoras); `Verificado` → `Revertido` (novo commit de rollback, sem apagar histórico). Nenhuma transição altera as âncoras nem reabre Godot-first.

## Entidade Passagem de emenda

Há exatamente sete instâncias, uma por requisito funcional. Os campos abaixo aplicam-se a todas; os campos específicos são listados em seguida.

| Campo | Tipo | Obrigatório | Origem | Regra de validação |
|---|---|---|---|---|
| `id` | Literal | Sim | spec Mapa de 7 passagens | IDs estáveis `P-1389`…`P-1673`; um por passagem, sem duplicata. |
| `linha` | Inteiro | Sim | spec Mapa de 7 passagens | Um de `1389,1451,1459,1594,1659,1668,1673`. |
| `secao` | Texto | Sim | spec Mapa de 7 passagens | Seção do GDD onde a passagem está localizada (ex.: 15.5, 15.9, 18, 20.3, 20.4). |
| `conteudo_divergente` | Texto | Sim | spec Mapa de 7 passagens | Trecho verificado no GDD antes da emenda; registrado como baseline para comparação. |
| `autoridade` | Enum | Sim | spec Key Entities | `ADR-002` nas seis passagens de licenciamento; `ADR-001` na passagem de CI. |
| `fr` | ID | Sim | spec FR-001–FR-007 | Um FR por passagem, sem repetição. |
| `tarefa` | ID `T###` | Sim | contrato §3 | Pelo menos uma tarefa por passagem; ver números no contrato. |
| `ref_adr_no_gdd` | Texto | Sim | spec SC-004 | A referência ao ADR aparece no próprio GDD, na passagem emendada. |
| `estado_alvo` | Texto | Sim | spec Mapa de 7 passagens | Descrição do resultado esperado após a emenda, verificável por leitura. |
| `estado` | Enum | Sim | contrato §4 | `Divergente` → `Emendado` → `Verificado`; rollback leva a `Revertido`. |

### Campos específicos por passagem

| Passagem | FR | Campos específicos obrigatórios | Validação por leitura |
|---|---|---|---|
| `P-1389` | FR-001 | `ressalva_terceiros`, `nao_impoe_agpl`, `portabilidade_saida`, `ref_adr` | Ressalva restrita a instâncias de terceiros; nenhuma AGPL imposta ao código do projeto; portabilidade e direito de saída preservados; nenhuma decisão sobre licenças de terceiros. |
| `P-1451` | FR-002 | `licenca_unica`, `sem_copying`, `ref_adr` | MIT única para Núcleo e servidor; ausência de "AGPL-3.0 para o servidor" e de `COPYING`; declaração/licença MIT mantida. |
| `P-1459` | FR-003 | `substituicao_par_mit_agpl`, `protecao_codigo`, `ref_adr` | Par "MIT/AGPL" ausente; proteção de código e autoria preservada; ressalva sobre instância injusta preservada. |
| `P-1594` | FR-004 | `sem_promessa_ci`, `defer_gate_remoto`, `sem_efeito_remoto`, `ref_adr` | "integração contínua desde o primeiro dia" ausente; CI e gate remoto diferidos; nenhum gatilho, pipeline, build ou serviço externo. |
| `P-1659` | FR-005 | `n3_respondida`, `sem_alternativa`, `ressalva_motor`, `ref_adr` | N3 respondida por MIT único; sem "MIT ou GPL" e sem reabertura da licença do motor; ressalva de que o motor não é o único MIT preservada. |
| `P-1668` | FR-006 | `n3_nao_bloqueante`, `sem_dependencia_publicacao`, `ref_adr` | N3 não listada como bloqueante da "Primeira publicação"; registrada como decidida pelo ADR-002; publicação externa não é pré-requisito. |
| `P-1673` | FR-007 | `n3_removida_lista`, `n4_n6_n8_preservados`, `fase0_segue`, `ref_adr` | N3 ausente da lista de não respondidas; N4, N6 e N8 intactos; Fase 0 segue com N3 respondido. |

**Regras de validação da entidade**:

1. Cobertura: 7 de 7 passagens com `estado_alvo` efeito observável e uma tarefa `T###` correspondente (SC-001).
2. Integridade: nenhuma passagem sem `autoridade`, nenhuma `autoridade` fora de `ADR-001`/`ADR-002`, nenhuma referência a numeração provisória.
3. Exclusão: nenhuma oitava passagem e nenhuma seção vizinha reescrita; as duas âncoras não entram nesta entidade como alvo.
4. Imutabilidade: `linha`, `fr`, `tarefa` e `autoridade` são estáveis; correção é feita por emenda versionada, nunca por renomeação silenciosa.

## Entidade Âncora Godot-first

| Campo | Tipo | Obrigatório | Origem | Regra de validação |
|---|---|---|---|---|
| `id` | Literal | Sim | spec Key Entities | `A-1265` (decisão técnica final, subseção 14.8) e `A-1678` (N7, decisão vinculante, subseção 20.5). |
| `linha` | Inteiro | Sim | spec SC-002 | `1265` e `1678`. |
| `texto_baseline` | Texto | Sim | `docs/GDD - Insurgência.md:1265,1678` | Redação atual, usada somente como referência de comparação. |
| `alteracoes_permitidas` | Enum | Sim | spec SC-002 | `Nenhuma`; a âncora nunca é alvo de emenda, rollback ou reescrita. |

**Regra de validação**: ao final da emenda, a comparação textual das duas âncoras contra o baseline registra exatamente zero alterações (SC-002). Qualquer divergência é registrada como `BUG-###` e exige rollback documental da passagem que a causou, nunca ajuste da âncora.

## Entidade Rastreabilidade ADR

| ID da referência | Tipo | Passagens atendidas | Origem local |
|---|---|---|---|
| `ADR-001` | Decisão local | `P-1594` | `docs/adr/001-excecao-linguagem-gdscript.md:14,40` |
| `ADR-002` | Decisão local | `P-1389`, `P-1451`, `P-1459`, `P-1659`, `P-1668`, `P-1673` | `docs/adr/002-licenca-mit-nucleo.md:14,37-41` |
| `DEC-R1` | Decisão de governança | Todas (autorização da emenda) | `.specify/assessments/emenda-gdd-adrs/decision.md:5,21-25` |

**Regras de validação**:

1. Toda passagem emendada cita, no próprio GDD, a autoridade da coluna "Passagens atendidas" (SC-004: 7/7).
2. Nenhuma passagem cita a numeração provisória da avaliação de stack (`ADR-002` para CI, `ADR-006` para licenciamento).
3. `DEC-R1` autoriza o ciclo, mas não substitui a referência ao ADR específico de cada passagem.
4. A associação é imutável; mudança de autoridade exige nova versão do registro de emenda (contrato §5).

## Entidade Registro de emenda

| Campo | Tipo | Obrigatório | Origem | Regra de validação |
|---|---|---|---|---|
| `id` | Literal `EM-002-1.0.0` | Sim | data-model §Entidade Registro de emenda | Identifica a versão documental da emenda da feature 002; morada única desta entidade e do campo `contagens`. |
| `versao` | SemVer | Sim | contrato §5 | `1.0.0` na materialização; evolução por MAJOR/MINOR/PATCH. |
| `data` | Data ISO 8601 (`YYYY-MM-DD`) | Sim | contrato §5 | Data do registro, coerente com o changelog. |
| `changelog` | Lista de entradas | Sim | contrato §5 | Cada entrada contém versão, data, IDs FR e, quando houver, `T###` e `BUG-###`. |
| `contagens` | Lista de inteiros | Sim | spec SC-001–SC-007 | 7/7 passagens, 0/2 âncoras alteradas, 7/7 ADRs citados, 0 ocorrências de N3 aberta e 0 de promessa de CI não reconciliada. |
| `rollback` | Texto | Sim | contrato §5 | Procedimento documental por novo commit; histórico preservado. |
| `estado_remoto` | Literal `[NV]` | Sim | spec SC-003, constituição VI | Nenhuma aprovação, CI, merge ou proteção é afirmada como obtida. |

**Transições de estado**: `Rascunho` → `Registrado` (changelog e contagens **confirmados por leitura** após a verificação documental, sem escrita neste arquivo); `Registrado` → `Emendado` (nova versão SemVer por alteração de escopo ou autoridade, registrada no registro documental `EM-002-1.0.0` — `data-model.md` §Entidade Registro de emenda; o contrato §5 fornece apenas o formato e o GDD não recebe campo de versão); `Registrado` → `Revertido` (rollback documental de uma ou mais passagens do GDD). Nenhuma transição produz efeito remoto.

## Campos não aplicáveis a entidades documentais (com motivo)

| Campo usual de modelo de dados | Declaração | Motivo |
|---|---|---|
| Chave primária autoincremental, FK, índice | **Não aplicável** | A identidade é o ID literal da passagem/âncora e a associação é por ID em Markdown; não há banco de dados. |
| Migração de esquema, DDL, seed | **Não aplicável** | Nenhuma estrutura persistente é alterada; o único suporte é o Git e o arquivo Markdown. |
| DTO, payload de API, contrato HTTP/gRPC | **Não aplicável** | Não há interface de rede nem serviço; o contrato existente descreve campos documentais. |
| Estado de runtime, cache, TTL, concorrência | **Não aplicável** | As passagens são lidas por pessoas; não há thread, request ou processo. |
| Métricas de latência, vazão, SLA | **Não aplicável** | Os critérios de sucesso são contagens e comparação textual (SC-001–SC-007). |
| Campos de segredo, token ou credencial | **Não aplicável** | Norma de trabalho proíbe segredos em arquivos; nenhuma integração remota existe neste ciclo. |
| Arquivo de licenças de terceiros | **Não aplicável** | Fora da autoridade do ADR-002 e fora do escopo da feature; nenhuma decisão implícita é registrada. |
