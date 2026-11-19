# Data Model: Exceção de linguagem e licença MIT-only do Núcleo

**Feature**: `001-excecao-linguagem-licenca-nucleo` | **Created**: 2026-11-19 | **Fase**: F1 (Phase 1 — Design)

**Origem**: spec `## Key Entities` e `## Requirements` (FR-001–FR-013), `## Success Criteria` (SC-001–SC-008) e decisões D1–D8 de `plan.md`.

**Premissa**: as "entidades" abaixo são registros documentais — seções de arquivos Markdown versionados no Git — e não tabelas, objetos de runtime ou contratos de API. Não há persistência, migração, chave estrangeira ou ciclo de vida executável neste ciclo. Todo campo listado é obrigatório na materialização do ADR, salvo indicação em contrário.

## Visão geral

| Entidade | Identidade | Finalidade | Origem |
|---|---|---|---|
| ADR-001 — exceção de linguagem | `ADR-001` | Autorizar GDScript como linguagem de engine e script de jogo sem reabrir Godot-first. | spec FR-002–FR-004, Key Entities |
| ADR-002 — licença MIT-only do Núcleo | `ADR-002` | Fixar MIT como posição única para todo o código do ciclo. | spec FR-005–FR-008, FR-013, Key Entities |
| Registro de rastreabilidade | 4 entradas imutáveis por referência | Associar três desvios e a decisão R1 aos ADRs que os resolvem. | spec FR-012, SC-002, SC-008, Key Entities |

```text
DEC-R1 (decisão R1, go/opção A)
   ├── DEV-LING-01 ──────► ADR-001
   ├── DEV-EXC-01 ───────► ADR-001
   └── DEV-LIC-01 ───────► ADR-002

ADR-001 ─────────────────► DEC-R1
ADR-002 ─────────────────► DEC-R1

ADR-001: linguagem_adotada + clausula_nao_reabertura + criterios_reeavaliacao
ADR-002: posicao_licenca + exclusao_alternativas + regra_copyleft + ressalva_motor
```

## Entidade ADR-001 — exceção de linguagem

**Arquivo canônico**: `docs/adr/001-excecao-linguagem-gdscript.md` (criado na fase 12, não nesta fase).

| Campo | Tipo | Obrigatório | Origem | Regra de validação |
|---|---|---|---|---|
| `id` | Literal `ADR-001` | Sim | FR-001 | Imutável; exatamente um registro com este ID no ciclo. |
| `titulo` | Texto | Sim | FR-009 | Identifica a exceção de linguagem para GDScript. |
| `data` | Data ISO 8601 (`YYYY-MM-DD`) | Sim | FR-009 | Data de registro do ADR; coerente com o histórico versionado. |
| `contexto` | Texto | Sim | FR-009 | Descreve o conflito entre a presunção normativa e a escolha Godot/GDScript, sem alterar `GDD:1265,1678`. |
| `linguagem_adotada` | Texto | Sim | FR-002 | Declara GDScript como linguagem de engine e script de jogo adotada no ciclo. |
| `decisao` | Texto | Sim | FR-002, FR-009 | Autoriza a exceção local; não introduz segunda linguagem nem troca de engine. |
| `clausula_nao_reabertura` | Texto | Sim | FR-003, SC-004 | Afirma explicitamente que Godot-first permanece vinculante e não é reaberta, revista ou revertida. |
| `alternativas_consideradas` | Texto | Sim | FR-009 | Registra emenda normativa e servidor Rust como alternativas não adotadas no ciclo. |
| `consequencias` | Texto | Sim | FR-009 | Inclui a contingência de emenda normativa futura, sem executá-la e sem reabrir o pilar. |
| `criterios_reeavaliacao` | Texto | Sim | FR-004 | Exige evidência prévia e aprovação formal para qualquer mudança de linguagem. |
| `autoridade_aprovacao` | Texto | Sim | FR-009, FR-011 | Nomeia o maintainer como autoridade; aprovação humana é necessária; registro efetivo permanece `[NV]`. |
| `rastreabilidade` | Lista de IDs | Sim | FR-012 | Contém `DEV-LING-01`, `DEV-EXC-01` e `DEC-R1`; nenhum desses IDs pode faltar. |
| `versao` | SemVer | Sim | contrato §4 | `1.0.0` na materialização; evolução conforme regras do contrato. |
| `estado` | Enum | Sim | FR-011 | `Rascunho` → `Pendente de aprovação` → `Aprovado`; `Emendado`/`Revogado` somente por rollback documental. |

**Transições de estado**: `Rascunho` → `Pendente de aprovação` (registro completo); `Pendente de aprovação` → `Aprovado` (somente com registro humano; status remoto continua `[NV]`); `Aprovado` → `Emendado` ou `Revogado` (novo commit versionado, nunca apagamento do histórico). Não existe transição automática nem transição que reabra Godot-first.

## Entidade ADR-002 — licença MIT-only do Núcleo

**Arquivo canônico**: `docs/adr/002-licenca-mit-nucleo.md` (criado na fase 12, não nesta fase).

| Campo | Tipo | Obrigatório | Origem | Regra de validação |
|---|---|---|---|---|
| `id` | Literal `ADR-002` | Sim | FR-001 | Imutável; exatamente um registro com este ID no ciclo; não é o ADR-002 de CI do documento externo (ver research.md RQ-2). |
| `titulo` | Texto | Sim | FR-009 | Identifica a licença MIT-only do Núcleo. |
| `data` | Data ISO 8601 (`YYYY-MM-DD`) | Sim | FR-009 | Data de registro do ADR. |
| `contexto` | Texto | Sim | FR-009 | Registra MIT vigente, N3 aberta e a divergência documental pendente de aprovação. |
| `posicao_licenca` | Texto | Sim | FR-005, SC-005 | Estabelece MIT como posição única para Núcleo e servidor headless do ciclo. |
| `exclusao_alternativas` | Texto | Sim | FR-006 | Exclui expressamente cláusula "MIT ou GPL" e licenciamento AGPL para o código do ciclo. |
| `regra_copyleft` | Texto | Sim | FR-007 | Nenhum código copyleft é copiado; contribuições de terceiros são identificadas e atribuídas. |
| `ressalva_motor` | Texto | Sim | FR-008 | Não altera a licença do motor, não a declara exclusiva e preserva a ressalva de que o motor não é o único MIT. |
| `fora_escopo` | Texto | Sim | FR-013 | Registra a decisão sobre o arquivo de licenças de terceiros como fora do ciclo, sem resolução implícita. |
| `clausula_nao_reabertura` | Texto | Sim | FR-003, SC-004 | Mesma regra do ADR-001: Godot-first não é reaberta, revista ou revertida. |
| `alternativas_consideradas` | Texto | Sim | FR-009 | Registra AGPL no servidor, "MIT ou GPL" e emenda documental como alternativas não adotadas. |
| `consequencias` | Texto | Sim | FR-009 | Registra a pendência de mudança documental como dependente de aprovação explícita. |
| `autoridade_aprovacao` | Texto | Sim | FR-009, FR-011 | Maintainer como autoridade; aprovação humana necessária; registro efetivo `[NV]`. |
| `rastreabilidade` | Lista de IDs | Sim | FR-012 | Contém `DEV-LIC-01` e `DEC-R1`; nenhum desses IDs pode faltar. |
| `versao` | SemVer | Sim | contrato §4 | `1.0.0` na materialização. |
| `estado` | Enum | Sim | FR-011 | Mesma máquina de estados do ADR-001. |

**Transições de estado**: idênticas às do ADR-001, com a restrição adicional de que nenhuma transição pode introduzir cláusula alternativa, AGPL para o código do ciclo ou mudança na licença do motor.

## Entidade Registro de rastreabilidade

Realizado como uma entrada explícita em cada ADR (`rastreabilidade`) e como matriz consolidada no contrato (`contracts/adr-contract.md §2`). Nenhuma entrada é deduzida de prosa corrida.

| ID da referência | Tipo | Descrição | ADR responsável | Origem local |
|---|---|---|---|---|
| `DEV-LING-01` | Desvio | Rust padrão vs Godot/GDScript vinculantes (desvio #1 da avaliação). | ADR-001 | `docs/AVALIACAO-STACK-MDMV.md:387` |
| `DEV-EXC-01` | Desvio | Exceção de linguagem sem categoria e sem ADR (desvio #3). | ADR-001 | `docs/AVALIACAO-STACK-MDMV.md:389` |
| `DEV-LIC-01` | Desvio | MIT vigente vs AGPL/N3 aberta (desvio #10). | ADR-002 | `docs/AVALIACAO-STACK-MDMV.md:396` |
| `DEC-R1` | Decisão | Veredito `go`, opção A, registrado em 2026-11-19. | ADR-001 e ADR-002 | `.specify/assessments/excecao-linguagem-licenca-nucleo/decision.md:5,29-31` |

**Regras de validação**:

1. Cobertura: 3 de 3 desvios com ADR correspondente (SC-002) e 4 de 4 referências presentes (SC-008).
2. Integridade: nenhum desvio sem ADR, nenhum ADR sem `DEC-R1`, nenhuma referência apontando para ID inexistente.
3. Exclusão: o desvio de CI (#4 da avaliação) não entra nesta matriz; sua inclusão alteraria FR-001 e SC-002.
4. Imutabilidade: IDs são estáveis; a correção de uma associação é feita por emenda versionada, não por renomeação silenciosa.

## Campos não aplicáveis a entidades documentais (com motivo)

| Campo usual de modelo de dados | Declaração | Motivo |
|---|---|---|
| Chave primária autoincremental, FK, índice | Não aplicável | A identidade é o ID literal do ADR e a associação é por ID de referência em Markdown; não há banco de dados. |
| Migração de esquema, DDL, seed | Não aplicável | Nenhuma estrutura persistente é alterada; o único suporte é o Git (spec FR-010, SC-007). |
| DTO, payload de API, contrato HTTP/gRPC | Não aplicável | Não há interface de rede nem serviço; o contrato existente descreve campos documentais (`contracts/adr-contract.md`). |
| Estado de runtime, cache, TTL, concorrência | Não aplicável | Os registros são lidos por pessoas; não há thread, request ou processo. |
| Métricas de latência, vazão, SLA | Não aplicável | Os critérios de sucesso são contagens e comparação textual (SC-001–SC-008). |
| Campos de segredo, token ou credencial | Não aplicável | Norma de trabalho proíbe segredos em arquivos; nenhuma integração remota existe neste ciclo. |
