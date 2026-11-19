# Quickstart: Verificação local da feature 001

**Feature**: `001-excecao-linguagem-licenca-nucleo` | **Created**: 2026-11-19 | **Fase**: F1 (Phase 1 — Validation guide)

**Tipo**: procedimento de leitura e comparação textual. Este guia não contém código, compilação, teste, gate, pipeline, migração, comando de instalação ou alteração de arquivos. As etapas 1–5 descrevem o estado do planejamento; as etapas 6–9 aplicam-se após a materialização dos ADRs na fase 12 (implement). Nenhum resultado é afirmado como executado neste documento.

## Pré-requisitos

- Checkout local do repositório do alvo com o diretório `specs/001-excecao-linguagem-licenca-nucleo/` disponível.
- Leitura dos arquivos Markdown da feature e dos documentos locais citados em `research.md`.
- Contexto da spec `spec.md` (requisitos FR-001–FR-013, critérios SC-001–SC-008 e `## Clarifications`), de `plan.md` e de `contracts/adr-contract.md`.
- Nenhuma engine instalada, nenhuma dependência, nenhum serviço em execução, nenhuma credencial e nenhum acesso de rede.

## Comandos de setup — não aplicável

**Não aplicável**: não há ambiente, runtime, biblioteca, engine, banco ou migração a instalar neste ciclo (spec FR-010 e Assumptions). Preparar o checkout é condição de leitura, não etapa de configuração de execução. Nenhum comando de instalação é fornecido para evitar sugerir dependência inexistente.

## Procedimento de verificação (somente leitura)

| Etapa | Ação de leitura | Resultado esperado | Referência |
|---|---|---|---|
| 1 | Abrir `spec.md` e conferir `## Clarifications` e os marcadores de esclarecimento. | Quatro respostas em `### Session 2026-11-19`; zero marcadores pendentes e nenhum conteúdo pendente de redação. | spec `## Clarifications`; `checklists/requirements.md` (16/16) |
| 2 | Abrir `plan.md` e ler Technical Context e Constitution Check. | Todos os campos preenchidos; campos sem aplicação trazem "**Não aplicável**" com motivo; princípios I–VI reavaliados, sem violações. | plan §Technical Context, §Constitution Check |
| 3 | Abrir `research.md`. | Decisões RQ-1–RQ-9 com decisão, justificativa e alternativas; fontes exclusivamente locais; nenhuma URL inventada. | research §Decisões de pesquisa, §Fontes consultadas |
| 4 | Abrir `data-model.md`. | Três entidades (ADR-001, ADR-002, Registro de rastreabilidade), com campos, validações, transições e campos não aplicáveis declarados. | data-model §Visão geral e seguintes |
| 5 | Abrir `contracts/adr-contract.md`. | IDs, campos obrigatórios, matriz FR, regras de IDs `T###` e `BUG-###`, SemVer, changelog e rollback documental presentes. | contrato §1–§7 |
| 6 | Abrir `checklists/requirements.md` e `checklists/adr-quality.md`. | O checklist de requisitos permanece com 16 itens aprovados na fase clarify; o checklist de domínio contém itens `- [ ] CHK###` e nenhum `[x]`. | checklists/; contrato de fase item 9 |
| 7 | Abrir os dois ADRs materializados e contar os registros. | Exatamente dois: `ADR-001` em `docs/adr/001-excecao-linguagem-gdscript.md` e `ADR-002` em `docs/adr/002-licenca-mit-nucleo.md`, com todos os campos de `contracts/adr-contract.md §2`. | FR-001, FR-009, SC-001; contrato §1–§2 |
| 8 | Comparar o campo `rastreabilidade` dos dois ADRs com a matriz do contrato. | Quatro referências presentes (`DEV-LING-01`, `DEV-EXC-01`, `DEV-LIC-01`, `DEC-R1`); três desvios cobertos; `DEC-R1` nos dois registros. | FR-012, SC-002, SC-008; contrato §2.3 |
| 9 | Comparar textualmente as passagens vinculantes de Godot-first e as afirmações de licenciamento e status. | Zero alterações em Godot-first; declaração de não reabertura nos dois ADRs; MIT sem "ou GPL" e sem AGPL para o código do ciclo; licença do motor intacta; nenhum código, teste, gate, CI, serviço ou emenda normativa; aprovação humana como autoridade e todo status remoto `[NV]`. | FR-003, FR-005–FR-011, FR-013, SC-003–SC-007; contrato §7 |

## Critérios de encerramento

- As etapas 1–6 descrevem o estado deste planejamento sem exigir execução de comandos.
- As etapas 7–9 são concluídas somente quando os ADRs existirem; antes disso, seu resultado é "aguarda materialização na fase 12", não uma falha nem um verde.
- Se alguma contagem divergir (número de ADRs, desvios ou referências), a verificação para e a divergência é registrada com o ID FR/SC correspondente; nenhum resultado é maquiado.
- Se aparecer código, teste, build, gate, pipeline, serviço externo ou emenda normativa dentro de um ADR, o registro está fora do contrato e exige rollback documental (`contracts/adr-contract.md §5`).

## Comandos de teste, build e gate — não aplicável

**Não aplicável**: os critérios de sucesso são contagens e comparações textuais (SC-001–SC-008) e nenhum depende de execução (spec Assumptions). FR-010 e SC-007 vedam código, compilação, testes e gates no conteúdo dos ADRs. O gate determinístico `python3 scripts/ci/check_sdd_completeness.py` existe no alvo (`AGENTS.md:14-22`), mas sua execução, saída colada e data/hora pertencem à fase validate (R4), portaria do ciclo completo posterior a esta fase de planejamento, como o é a fase release (R7); nenhum resultado dele é aqui antecipado.

## Campos não aplicáveis ao procedimento (com motivo)

| Campo | Declaração | Motivo |
|---|---|---|
| Tempo de execução, timeout, taxa de sucesso de comandos | Não aplicável | Nenhum comando é executado por este guia. |
| Ambiente de teste, fixtures, dados de entrada | Não aplicável | Não há runtime, banco ou serviço; a entrada é o próprio documento. |
| Credenciais, tokens, acesso remoto | Não aplicável | Norma de trabalho proíbe segredos e o ciclo não acessa remote; status remoto é `[NV]`. |
| Saída de console colada | Não aplicável nesta fase | Somente a fase validate (R4) produz evidência de execução com saída e data/hora. |
| Versão de engine ou linguagem do projeto | Não aplicável | GDScript é objeto do ADR-001, não ferramenta deste procedimento de leitura. |
