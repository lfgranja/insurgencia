# Quickstart: Verificação por leitura da feature 002

**Feature**: `002-emenda-gdd-adrs` | **Created**: 2026-11-19 | **Fase**: F1 (Phase 1 — Validation guide)

**Tipo**: procedimento de leitura e comparação textual. Este guia não contém código, compilação, teste, gate, pipeline, migração, comando de instalação ou alteração de arquivos. As etapas 1–6 descrevem o estado do planejamento; as etapas 7–12 aplicam-se após a materialização da emenda das sete passagens. Nenhum resultado é afirmado como executado neste documento.

## Pré-requisitos

- Checkout local do repositório do alvo com `specs/002-emenda-gdd-adrs/` disponível.
- Leitura dos artefatos Markdown da feature e dos documentos locais citados em `research.md`.
- Contexto da spec (`FR-001`–`FR-007`, `SC-001`–`SC-007`, mapa de 7 passagens e `## Clarifications`), de `plan.md` e de `contracts/gdd-amendment-contract.md`.
- Nenhuma engine instalada, nenhuma dependência, nenhum serviço em execução, nenhuma credencial e nenhum acesso de rede.

## Comandos de setup — não aplicável

**Não aplicável**: não há ambiente, runtime, biblioteca, engine, banco ou migração a instalar neste ciclo (spec SC-007 e Assumptions). Preparar o checkout é condição de leitura, não etapa de configuração de execução. Nenhum comando de instalação é fornecido para evitar sugerir dependência inexistente.

## Procedimento de verificação (somente leitura)

| Etapa | Ação de leitura | Resultado esperado | Referência |
|---|---|---|---|
| 1 | Abrir `spec.md` e conferir `## Clarifications` e marcadores pendentes. | Três respostas em `### Session 2026-11-19`; zero marcadores pendentes e nenhuma redação pendente. | spec `## Clarifications`; `checklists/requirements.md` (18/18) |
| 2 | Abrir `plan.md` e ler Technical Context e Constitution Check. | Todos os campos preenchidos; campos sem aplicação trazem "**Não aplicável**" com motivo; princípios I–VI reavaliados, sem violações. | plan §Technical Context, §Constitution Check |
| 3 | Abrir `research.md`. | Decisões RQ-1–RQ-10 com decisão, justificativa, alternativas e fontes exclusivamente locais; evidência contrária tratada; nenhuma URL aberta. | research §Decisões de pesquisa, §Fontes consultadas |
| 4 | Abrir `data-model.md`. | Cinco entidades (GDD, Passagem de emenda, Âncora Godot-first, Rastreabilidade ADR, Registro de emenda), com campos, validações, transições e campos não aplicáveis declarados. | data-model §Visão geral e seguintes |
| 5 | Abrir `contracts/gdd-amendment-contract.md`. | Sete passagens com linha, FR, `T###`, autoridade e estado-alvo; regras R-1…R-6; SemVer, changelog e rollback documental presentes. | contrato §1–§7 |
| 6 | Abrir `checklists/requirements.md` e `checklists/gdd-amendment.md`. | O checklist de requisitos permanece com 18 itens aprovados na clarify; o checklist de domínio contém itens `- [ ] CHK###` e nenhum `[x]`. | checklists/; contrato de fase item 9 |
| 7 | Abrir `docs/GDD - Insurgência.md` nas linhas `1389,1451,1459,1659,1668,1673` e ler as seis passagens de licenciamento. | MIT única para todo o código do ciclo; ausência de "MIT ou GPL", "MIT/AGPL", AGPL para o código do projeto e `COPYING`; a ressalva de `1389` versa sobre servidores de terceiros; portabilidade e direito de saída preservados; presença do ADR-002 em cada passagem. | FR-001–FR-003, FR-005–FR-007, SC-004, SC-005; contrato §2, R-1, R-2 |
| 8 | Ler a linha `1594` do GDD. | A promessa "integração contínua desde o primeiro dia" não aparece; o texto registra CI e gate remoto como diferidos neste ciclo, sem efeito remoto, com o ADR-001 citado na passagem; nenhum gatilho, pipeline, build ou serviço externo. | FR-004, SC-006; contrato §2, R-3 |
| 9 | Ler as linhas `1659,1668,1673`. | N3 consta como respondida pela MIT única, não bloqueia a "Primeira publicação" e não aparece na lista de perguntas não respondidas; N4, N6 e N8 permanecem como estavam; a Fase 0 segue com N3 respondido; ADR-002 citado nas três passagens. | FR-005–FR-007, SC-005; contrato §2, R-4 |
| 10 | Comparar textualmente as linhas `1265` e `1678` do GDD com o baseline registrado em `data-model.md`. | Zero alterações nas duas âncoras Godot-first; nenhuma seção vizinha às sete passagens foi reescrita. | SC-002, SC-003; contrato §1, R-5 |
| 11 | Contar as referências e o conteúdo proibido nas sete passagens. | 7 de 7 passagens com ADR citado; 0 ocorrências de N3 aberta/bloqueante, de promessa de CI não reconciliada, de código, compilação, teste, gate de build, pipeline, serviço externo ou emenda da norma externa. | SC-001, SC-004, SC-005, SC-006, SC-007; contrato §7 |
| 12 | Conferir por leitura o registro documental da emenda em `data-model.md` §Entidade Registro de emenda (morada única de `EM-002-1.0.0` e de `contagens`). | `EM-002-1.0.0` com versão, data, changelog por IDs FR/`T###`/`BUG-###` quando houver, contagens 7/7, 0/2, 7/7 e rollback documental; aprovação remota, CI, merge, proteção de branch e publicação rotulados `[NV]`; resultado observável da verificação documental executada neste ciclo. | SC-003; data-model §Entidade Registro de emenda; contrato §5 (somente formato), R-6 |

**SC-003 neste ciclo**: 100% das verificações documentais executadas neste ciclo retornam resultado observável (contagens e comparação textual). Os gates determinísticos locais pertencem à portaria R4 (validate), não a tarefas desta feature. Nenhum verde remoto é afirmado e nenhum resultado é antecipado neste guia; todo efeito remoto permanece `[NV]`.

## Critérios de encerramento

- As etapas 1–6 descrevem o estado deste planejamento sem exigir execução de comandos.
- As etapas 7–12 são concluídas somente quando as sete passagens estiverem emendadas; antes disso, seu resultado é "aguarda materialização na fase de implementação", não uma falha nem um verde.
- Se alguma contagem divergir (7 passagens, 2 âncoras, 7 referências de ADR), a verificação para e a divergência é registrada com o ID FR/SC correspondente e, quando aplicável, com um `BUG-###`; nenhum resultado é maquiado.
- Se aparecer código, teste, build, gate, pipeline, serviço externo, emenda normativa, reintrodução de "DEFER com gatilho" ou alteração de âncora, a passagem está fora do contrato e exige rollback documental (`contracts/gdd-amendment-contract.md §5`).
- Nenhuma etapa autoriza afirmar aprovação em plataforma, execução de CI, merge, proteção de branch ou publicação externa; todo efeito remoto permanece `[NV]` até confirmação ao vivo.

## Comandos de teste, build e gate — não aplicável

**Não aplicável**: os critérios de sucesso são contagens e comparações textuais (SC-001–SC-007) e nenhum depende de execução (spec Assumptions). SC-007 veta código, compilação, testes e gates no resultado da feature; SC-003 exige apenas que 100% das verificações documentais executadas neste ciclo retornem resultado observável. O gate determinístico `python3 scripts/ci/check_sdd_completeness.py` existe no alvo (`AGENTS.md:14-22`), mas sua execução, saída colada e data/hora pertencem à portaria R4 (validate), não a tarefas desta feature; nenhum resultado dele é aqui antecipado e nenhum verde remoto é afirmado.

## Campos não aplicáveis ao procedimento (com motivo)

| Campo | Declaração | Motivo |
|---|---|---|
| Tempo de execução, timeout, taxa de sucesso de comandos | **Não aplicável** | Nenhum comando é executado por este guia. |
| Ambiente de teste, fixtures, dados de entrada | **Não aplicável** | Não há runtime, banco ou serviço; a entrada é o próprio documento. |
| Credenciais, tokens, acesso remoto | **Não aplicável** | Norma de trabalho proíbe segredos e o ciclo não acessa remote; status remoto é `[NV]`. |
| Saída de console colada | **Não aplicável nesta fase** | Somente a portaria R4 (validate) produz evidência de execução do gate com saída e data/hora; neste ciclo, as verificações documentais retornam resultado observável por leitura (SC-003). |
| Versão de engine ou linguagem do projeto | **Não aplicável** | GDScript é objeto do ADR-001, não ferramenta deste procedimento de leitura. |
| Decisão sobre licenças de servidores de terceiros | **Não aplicável** | Fora da autoridade do ADR-002; a leitura apenas confere que a ressalva foi esclarecida quanto ao escopo. |
