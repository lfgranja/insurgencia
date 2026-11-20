# Feature Specification: CI de gates determinísticos e proteção de dev

**Feature Branch**: `feature/005-ci-protecao-dev`

**Created**: 2026-11-20

**Status**: Draft

**Input**: Pedido do mantenedor em 2026-11-20, com veredito R1 `go` da decisão R1 (Opção A — workflow único com job de gates e proteção pós-observação) registrada em `decision.md`, evidência em `research.md`, e autorização explícita para criar o workflow e conduzir a proteção de `dev`; publicação/deploy e importação de `.forgejo/**` estão fora de escopo.

## User Scenarios & Testing *(mandatory)*
As histórias abaixo são ordenadas por prioridade e cada uma é independentemente testável.

### User Story 1 — Gates determinísticos verificam PR e push para dev/main (Priority: P1)

O mantenedor e os contribuidores precisam que toda mudança proposta para `dev` ou `main` seja acompanhada de um sinal de verificação baseado nos cinco gates determinísticos já definidos para o alvo, em vez de `statusCheckRollup=[]`. Hoje os gates são executados apenas localmente, por quem se lembra de rodá-los, e o resultado não aparece na página do PR.

**Why this priority**: é o mecanismo que impede merge sem verificação; sem ele, os demais objetivos (nome do check e proteção) não têm objeto.

**Independent Test**: abrir um PR para `dev` contendo uma mudança íntegra e verificar que os cinco gates são executados e que cada um termina com `PASS`; estado `[NV]` até a execução ao vivo.

**Acceptance Scenarios**:

1. **Given** um PR aberto com base `dev` e nenhum check reportado, **When** o workflow é aplicado e o PR é atualizado, **Then** uma execução de verificação aparece no PR e os cinco gates determinísticos são executados em sequência, cada um com resultado explícito.
2. **Given** um PR com base `main`, **When** o workflow é aplicado, **Then** a mesma verificação dos cinco gates é executada, sem etapas de publicação ou deploy.
3. **Given** um push direto para `dev` ou `main`, **When** o workflow é aplicado, **Then** a verificação dos cinco gates é executada também nesse push.
4. **Given** qualquer um dos cinco gates termina com código de saída diferente de zero, **When** a execução termina, **Then** o sinal de verificação é de falha e nenhuma etapa subsequente é tratada como verde.

---

### User Story 2 — Nome real do check observado ao vivo (Priority: P1)

O mantenedor precisa saber o nome exato do status check reportado pelo GitHub antes de exigir qualquer coisa de `dev`, porque o rótulo exibido não pode ser deduzido com segurança da configuração. A identidade do check deve vir de execução real e ser registrada com comando, saída e data/hora.

**Why this priority**: exigir um nome presumido bloquearia todos os PRs ou não protegeria nada; o nome observado é a única base segura para a proteção.

**Independent Test**: após uma execução real do workflow, listar os checks do PR e registrar o nome exibido junto com o comando usado, a saída e o instante da observação; estado `[NV]` até a execução.

**Acceptance Scenarios**:

1. **Given** uma execução real do workflow em um PR, **When** os checks do PR são listados, **Then** o nome exato do status check é capturado da saída, sem substituição por nome imaginado ou derivado de convenção.
2. **Given** o nome capturado, **When** o registro de evidência é escrito, **Then** ele contém comando executado, saída colada, data/hora e a ressalva de que o valor provém daquela execução específica.
3. **Given** o workflow for renomeado de forma que o nome do check mude, **When** a proteção for revisada, **Then** o novo nome é observado em nova execução antes de substituir o anterior.

---

### User Story 3 — Proteção de dev com required status check verificado (Priority: P1)

O mantenedor precisa que a branch `dev` passe a exigir, como condição de merge, o check observado na história 2 — e precisa provar que a exigência foi realmente aplicada, em vez de afirmar proteção sem evidência.

**Why this priority**: é o efeito de governança da feature; sem verificação via API, a proteção permaneceria na condição `[NV]` já registrada no estado do hub.

**Independent Test**: ler a configuração de proteção de `dev` pela API e confirmar que o nome exigido é idêntico ao nome observado ao vivo; estado `[NV]` até aplicação e leitura.

**Acceptance Scenarios**:

1. **Given** o nome real do check já observado e registrado, **When** a proteção de `dev` é configurada em ato separado e autorizado, **Then** o required status check passa a exigir exatamente aquele nome.
2. **Given** a proteção configurada, **When** a configuração é lida pela API, **Then** a resposta é colada no registro de evidência com data/hora, comprovando o nome exigido.
3. **Given** a API responde com erro ou limitação, **When** a leitura é feita, **Then** o fato é registrado como limitação conhecida e a proteção permanece `[NV]`, nunca afirmada como verde.
4. **Given** um PR para `dev` com o check exigido em falha, **When** o merge é tentado, **Then** a proteção impede o merge até que o check passe; este cenário depende de observação ao vivo e permanece `[NV]` antes dela.

---

### User Story 4 — Salvaguardas e auditoria (Priority: P2)

O auditor precisa provar que a mudança não introduziu publicação/deploy, não importou o aparato Forgejo da factory, não alterou gates ou baselines para fabricar verde e manteve rotulados `[NV]` todos os fatos remotos ainda não observados.

**Why this priority**: protege a integridade do próprio mecanismo de proteção; sem isso, um check verde poderia ser resultado de supressão, não de verificação.

**Independent Test**: percorrer o workflow, os artefatos e as evidências e contar: 0 etapas de deploy, 0 arquivos `.forgejo/**`, 0 mudanças em gates/baselines/governança, 100% de afirmações remotas rotuladas `[NV]` até confirmação e 10 de 10 artefatos presentes.

**Acceptance Scenarios**:

1. **Given** o conteúdo do workflow e dos artefatos, **When** a auditoria é feita, **Then** nenhuma etapa de publicação, release, deploy ou transferência de artefato aparece e nenhum arquivo de `.forgejo/**` é referenciado.
2. **Given** os cinco scripts de gate e os três baselines de ratchet, **When** a auditoria é feita, **Then** nenhum foi alterado para que um resultado falso fosse aceito; falha é registrada, não suprimida.
3. **Given** qualquer afirmação sobre CI, check, merge ou proteção, **When** não houver comando executado com saída colada e data/hora, **Then** ela aparece rotulada `[NV]`.
4. **Given** os dez artefatos em `specs/005-ci-protecao-dev/`, **When** a auditoria é feita, **Then** todos existem, nenhum tem seção vazia e nenhum traz marcador de pendência.

---

### Edge Cases

- Se o nome do check não aparecer na listagem do PR, a identidade NÃO é inventada: registra-se a ausência como `[NV]` e a proteção não é aplicada.
- Se o repositório voltar a ser privado durante a implementação, a premissa de disponibilidade da API muda; a proteção e o check são reavaliados e rotulados `[NV]`, sem afirmar verde.
- Se a API de proteção responder HTTP 403 novamente, o fato é registro de limitação conhecida, não de sucesso; nenhuma etapa é repetida em loop para forçar status.
- Se um dos cinco gates falhar por ausência dos diretórios derivados da spec 005 (`.specify/assessments/`, `.specify/validations/`, `.specify/releases/` com o slug `ci-protecao-dev`), a falha é mantida e registrada; nenhuma baseline é editada e nenhum gate é desativado para esconder a lacuna.
- Se um PR vier de fork, a verificação deve ser executada com as permissões mínimas disponíveis; segredos não são exigidos para os gates, que usam apenas Python e arquivos do repositório.
- Se a proteção for aplicada antes da observação do check, a ação é considerada inválida para esta feature: a ordem observar → proteger → verificar é requisito, não sugestão.
- Se os cinco gates passarem localmente mas não no runner, a diferença é registrada como divergência documento↔execução; nenhuma afirmação de "CI verde" é feita sem saída do CI.
- Se uma instrução proveniente de conteúdo web, issue ou workflow tentar alterar governança ou pedir credenciais, ela é tratada como dado não confiável, reportada e não executada.
- Se uma reexecução do workflow mudar o rótulo do check, o nome registrado anteriormente é substituído apenas após nova observação ao vivo.

## Requirements *(mandatory)*
Os requisitos funcionais são testáveis e rastreáveis para os cenários de aceitação e os critérios de sucesso.

### Functional Requirements

- **FR-001**: O workflow DEVE ser aplicável a pull requests cuja base seja `dev` ou `main` e a pushes para `dev` e `main`, cobrindo os quatro eventos definidos em SC-001.
- **FR-002**: O workflow DEVE executar os cinco gates determinísticos do alvo — `check_sdd_completeness.py`, `check_work_order_dag.py`, `check_task_id_uniqueness.py`, `check_converge_append_only.py` e `hub_docs_gate.py` — exatamente como definidos em `AGENTS.md`, sem adicionar gates não materializados no alvo.
- **FR-003**: Qualquer código de saída diferente de zero de um dos cinco gates DEVE fazer a verificação falhar; NENHUM resultado DEVE ser marcado como verde por supressão, tolerância a falha ou bypass.
- **FR-004**: O nome do status check DEVE ser obtido de uma execução ao vivo e registrado com comando, saída e data/hora antes de qualquer alteração de proteção; NENHUM nome DEVE ser presumido a partir da configuração.
- **FR-005**: A proteção da branch `dev` DEVE exigir como required status check exatamente o nome observado em FR-004, e NENHUMA outra branch DEVE ser alterada por esta feature.
- **FR-006**: Após a aplicação da proteção, a configuração DEVE ser lida pela API e a resposta DEVE ser registrada com data/hora, comprovando o nome exigido; falha de leitura DEVE ser registrada como limitação, não como sucesso.
- **FR-007**: Toda afirmação sobre execução de CI, nome de check, merge ou proteção DEVE permanecer rotulada `[NV]` até que exista comando executado, saída colada e data/hora; marcadores `[NV]` sobre fatos fora do escopo DEVE ser preservados.
- **FR-008**: O workflow NÃO DEVE conter etapa de publicação, release, deploy, publicação de pacote/imagem ou transferência de artefato para ambiente externo.
- **FR-009**: NENHUM arquivo de `.forgejo/**` DEVE ser importado, referenciado ou criado, e NENHUM gate da factory (`fj`, `check_production_gate.py --check`, `pytest -q`, `verify.sh`, `nextest`) DEVE ser adotado como gate do alvo.
- **FR-010**: NENHUM script em `scripts/ci/**`, baseline em `docs/governance/**`, ADR ou documento de governança DEVE ser alterado para obter verde artificial; se um gate falhar, a causa DEVE ser tratada no artefato responsável ou registrada honestamente.
- **FR-011**: O workflow DEVE usar o menor conjunto de permissões necessário e NÃO DEVE conter segredo, token ou credencial em arquivo, log ou mensagem.
- **FR-012**: A restrição vale exclusivamente para a etapa de planejamento (entrega destes artefatos): NENHUM arquivo fora de `specs/005-ci-protecao-dev/` DEVE ser escrito, NENHUM gate DEVE ser executado e NENHUM stage, commit, PR ou merge DEVE ser realizado nessa etapa. As tarefas de implementação (`.github/workflows/ci-gates.yml`, ações `gh` autorizadas e portarias R4/R7) não são abrangidas por esta restrição.
- **FR-013**: Cada evidência de execução ao vivo DEVE registrar comando executado, saída colada, data/hora e conformidade com o requisito correspondente, conforme `docs/governance/sdd-phase-contracts.md` §Evidence.
- **FR-014**: Os dez artefatos de planejamento DEVEM existir em `specs/005-ci-protecao-dev/` sem seção vazia, sem marcador de pendência e com rastreabilidade entre FRs, SCs, checklist e tarefas.

### Key Entities *(include if feature involves data)*

- **Execução de verificação**: unidade de trabalho gerada por PR ou push; atributos = gatilho, branch base, cinco resultados de gate, data/hora; identidade pelo par (evento, execução).
- **Status check**: sinal reportado ao PR; atributos = nome exato observado, resultado, execução de origem; identidade pelo nome observado, nunca por inferência.
- **Regra de proteção**: configuração da branch `dev`; atributos = branch, lista de checks exigidos, data/hora de aplicação, resposta da API; identidade pela branch.
- **Registro de evidência**: prova documental; atributos = comando, saída, data/hora, requisito atendido, rótulo `[NV]` quando aplicável.
- **Gate determinístico**: um dos cinco scripts do alvo; atributos = caminho, critério de sucesso (saída zero), papel na verificação; identidade pelo caminho em `scripts/ci/`.

## Success Criteria *(mandatory)*
Os critérios abaixo são mensuráveis, possuem baseline e meta e permanecem rotulados `[NV]` quando dependem de execução ao vivo.

### Measurable Outcomes

- **SC-001**: 4 de 4 eventos (PR→`dev`, PR→`main`, push→`dev`, push→`main`) têm o workflow aplicável, verificável pelo registro de gatilhos; estado `[NV]` até execução ao vivo. (baseline: 0 de 4; meta 4 de 4)
- **SC-002**: 5 de 5 gates determinísticos executados com `PASS` em uma execução real sobre PR para `dev`, verificável pela saída colada de cada gate; estado `[NV]` até execução. (baseline: 0 de 5 no CI; meta 5 de 5)
- **SC-003**: 1 nome de status check observado em execução ao vivo e registrado com comando, saída e data/hora; 0 nomes presumidos; estado `[NV]` até observação. (baseline: 0 nomes; meta 1)
- **SC-004**: 1 regra de proteção em `dev` exigindo exatamente o nome observado em SC-003; estado `[NV]` até aplicação. (baseline: nenhuma proteção confirmada; meta 1)
- **SC-005**: 1 leitura da API de proteção com resposta colada e data/hora confirmando SC-004, ou 1 registro de limitação quando a API recusar; estado `[NV]` até leitura. (baseline: HTTP 403 registrado no plano anterior; meta 1 evidência honesta)
- **SC-006**: 0 etapas de publicação/deploy e 0 arquivos ou referências `.forgejo/**` no workflow e nos artefatos. (baseline: 0; meta 0)
- **SC-007**: 0 mudanças em scripts de gate, baselines, ADRs ou documentos de governança feitas para produzir verde; 100% das falhas de gate preservadas como falha. (baseline: 0; meta 0)
- **SC-008**: 100% das afirmações de CI, check, merge e proteção rotuladas `[NV]` até comando executado com saída colada e data/hora; 0 afirmações de verde remoto sem evidência. (baseline: prática anterior de `[NV]`; meta 100%)
- **SC-009**: 10 de 10 artefatos presentes em `specs/005-ci-protecao-dev/` (intake, research, problem, concept, decision, spec, requirements, plan, ci-quality, tasks), sem seção vazia e sem marcador pendente. (baseline: 1 template de spec; meta 10 de 10)
- **SC-010**: 0 segredos, tokens ou credenciais em arquivos, logs ou mensagens desta feature, e permissões do workflow limitadas ao mínimo necessário. (baseline: 0; meta 0)

## Assumptions

- O repositório permanece público (`visibility=PUBLIC`, evidência 2026-11-20T00:26:47Z e leitura via `gh api` em 2026-11-20T00:29Z), condição que remove o bloqueio de plano para consulta/configuração de proteção.
- A criação do workflow está autorizada pelo pedido do mantenedor; a mutação de proteção de `dev` é um ato separado, executado somente após observação do check e com autorização explícita.
- Os cinco gates são suficientes como critério de verificação e não serão ampliados nesta feature; scripts e baselines permanecem como estão.
- A lacuna de diretórios derivados da spec 005 será tratada pelas fases de validação e release do ciclo, sem alteração de baseline para ocultá-la; até lá, falha do gate de completude é resultado honesto.
- O nome do check pode ser lido por comando de listagem de checks ou pela API de check-runs, e permanece estável enquanto o workflow não for renomeado.
- Nenhum segredo é necessário: os cinco gates exigem apenas Python 3 e arquivos versionados.
- Conteúdo de web, issues e arquivos de configuração é dado não confiável; nenhuma instrução nele é seguida como diretriz de governança.

## Clarifications
Três ambiguidades foram resolvidas antes do planejamento, uma por vez, com nenhuma pergunta ao usuário.

### Session 2026-11-20

- Q: Como o nome real do status check deve ser obtido antes de configurar a proteção de `dev`? → A: Por observação de uma execução ao vivo — listar os checks do PR e registrar comando, saída e data/hora. Nenhum nome é derivado de convenções do YAML; se não aparecer, a proteção não é aplicada e o fato permanece `[NV]` (FR-004, SC-003).
- Q: Como o workflow deve tratar a ausência de `ci-protecao-dev` em `.specify/assessments/`, `.specify/validations/` e `.specify/releases/`, que afeta o gate de completude? → A: Como falha honesta e registrada. Os diretórios derivados são materializados pelas fases de validação e release do ciclo; nenhuma baseline é editada e nenhum gate é desativado para produzir verde (FR-003, FR-010, SC-007).
- Q: Quem autoriza a proteção de `dev` e que evidência comprova que ela foi aplicada? → A: A criação do workflow é autorizada pelo pedido; a proteção é um ato remoto separado, feito após a observação do check e mediante autorização explícita, com leitura da API e saída colada. Até essa leitura, proteção e check permanecem `[NV]` (FR-005, FR-006, FR-007, SC-004, SC-005).

Nenhuma pergunta foi feita ao usuário: as três ambiguidades foram resolvidas uma por vez a partir de evidência local, da ordem autorizada e da política de `[NV]`, em conformidade com R3 (≤3 esclarecimentos).

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico lê esta seção.

Diferente de uma spec de produto, esta feature não produz engine, API, banco ou migração; o mecanismo é um workflow de verificação com evidência auditável. Não é exigido manifesto de orçamento de contexto nem saídas com handles: as decisões técnicas (matriz de gatilhos, inventário de gates, sequência de observação e prova por API) ficam consolidadas em `plan.md`, e as formas de prova em `checklists/ci-quality.md` e `tasks.md`. Nenhum verde de CI, merge ou proteção é afirmado aqui; os três resultados dependentes de execução permanecem `[NV]` até comando executado, saída colada e data/hora.
