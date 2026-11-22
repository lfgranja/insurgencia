# Feature Specification: Verifica NV externa

**Feature Branch**: `004-verifica-nv-externa`

**Created**: 2026-11-19

**Status**: Draft

**Input**: Pedido do mantenedor em 2026-11-19, com veredito `go` da decisão R1 (opção A — substituição cirúrgica com ressalvas e notas de superação) registrada em `decision.md`, evidência em `research.md` e nos três arquivos de `research-evidence/`, e contrato de governança em `docs/governance/nv-resolution-2026-11-19.md`.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Godot: marca e licença com ressalva de escopo (Priority: P1)

O mantenedor que lê as avaliações precisa que as afirmações sobre o nome "Godot" e sobre a licença MIT deixem de ser rótulos `[NV]` e passem a dizer exatamente o que a política de marcas e a licença estabelecem: o nome exige autorização escrita **fora** das quatro hipóteses descritivas automáticas, e motor/fork comercial exige licença customizada; o motor/editor é MIT, sem EULA nos canais oficiais, com a única obrigação de incluir aviso de copyright e texto da licença. Hoje `licencas_lockin.md:24` e `godot.md` já trazem parte do fato, mas sem a ressalva das quatro hipóteses; `alternativas.md:9,24`, `02-analise-por-opcao.md:11`, `03-recomendacao.md:38,91` e `AVALIACAO-COMPARATIVA-ENGINES.md:241` repetem a regra sem o escopo completo.

**Why this priority**: é o fato mais citado nos documentos integrados e o de maior risco jurídico se a ressalva for perdida — "autorização escrita" lida como regra geral contradiz a política vigente de 21/07/2025.

**Independent Test**: basta ler os destinos de NV-A01/NV-A02 e confirmar, em cada um, a presença da ressalva das quatro hipóteses e da regra do fork comercial, com citação da política de marcas (21/07/2025) e da governança/licença oficiais, mais duas fontes independentes — sem depender das histórias 2, 3 ou 4.

**Acceptance Scenarios**:

1. **Given** um destino de NV-A01 com a frase "nome exige autorização escrita" sem escopo, **When** a substituição é aplicada, **Then** o texto distingue as quatro hipóteses automáticas ("made with", produto compatível, evento/publicação, brinde não vendido) da regra de autorização escrita e da exigência de licença customizada para motor/fork comercial, citando a política v. 21/07/2025, a página de governança e o artigo de 02/12/2024 com URL e data de acesso 19/11/2026.
2. **Given** um destino de NV-A02, **When** a substituição é aplicada, **Then** o texto afirma MIT/Expat sem EULA nos canais oficiais e explicita que a única condição é incluir aviso de copyright e o texto da licença ao redistribuir, citando `LICENSE.txt`, `/license` e "Complying with licenses" (≥2 fontes com URL e data).
3. **Given** um snapshot histórico (`docs/avaliacao-engines/pesquisa/**`) que traz o fato de Godot, **When** a evidência é aplicada, **Then** o corpo datado permanece byte-for-byte e uma nota de superação append-only acrescenta a ressalva e as fontes, sem apagar a afirmação original.

---

### User Story 2 — Stride, Fyrox, Urho3D/Dviglo e Defold (Priority: P1)

O auditor precisa que os marcadores sobre a issue Stride #1666, a manutenção de Stride e Fyrox, as licenças de Urho3D/Dviglo e a conta Defold sejam substituídos por fatos com fonte e ressalva: a issue permanece aberta após 4.4.3 sem fix registrado (**documental, não empiricamente testado**); Stride e Fyrox estão ativos em 19/11/2026; Urho3D é MIT mas arquivado desde 25/01/2023; Dviglo é MIT embora a API reporte `license: null`; nenhuma conta é exigida para download/instalação/uso do Defold.

**Why this priority**: concentra o maior número de destinos e a ressalva mais frágil — "sem correção" pode ser lido como teste empírico, o que a constituição proíbe afirmar sem comando executado.

**Independent Test**: basta verificar os destinos de NV-A03–NV-A08 com as ressalvas e duas fontes cada, sem depender de Godot ou Lightyear; a divergência Fyrox ("1.0.1/2.0.0-rc.1" no corpus versus 1.0.0/RC1/RC2 na evidência) deve constar corrigida em `02-analise-por-opcao.md:116`.

**Acceptance Scenarios**:

1. **Given** `docs/avaliacao-engines/pesquisa/defold_flax_stride.md:52` com "sem correção na 4.4 `[NV]`", **When** a substituição é aplicada, **Then** o texto diz que a issue #1666 permanece `open` após 4.4.3 e que nenhum fix foi registrado na issue ou nas notas de release, com a ressalva explícita de que não houve build/import empírico, citando a API da issue, o timeline/comentários e as release notes 4.4 (≥2 fontes com URL e data).
2. **Given** `docs/avaliacao-engines/pesquisa/licencas_lockin.md:30` com "Fork Dviglo: `[NV]` nesta rodada", **When** a substituição é aplicada, **Then** o texto afirma MIT com base no README e em `licenses/dviglo/LICENSE`, explica que `license: null` na API decorre da ausência de arquivo de licença na raiz, e registra a atividade (último push 09/03/2024), citando três fontes com URL e data.
3. **Given** `docs/avaliacao-engines/pesquisa/defold_flax_stride.md:11` com "conta Defold `[NV]`", **When** a substituição é aplicada, **Then** o texto afirma que download, instalação e uso do editor/motor não exigem conta, com a ressalva de que contas periféricas (fórum, Asset Portal, build servers) não foram auditadas, citando download, manual de instalação e FAQ (≥2 fontes com URL e data).
4. **Given** `docs/avaliacao-engines/02-analise-por-opcao.md:116` com "1.0.1 estável (MSRV 1.87), 2.0.0-rc.1 prévia", **When** a substituição de NV-A05 é aplicada, **Then** o texto registra v1.0.0 (29/03/2026) com RC1 (21/10/2025) e RC2 (09/02/2026), MIT confirmado e repositório ativo em 19/11/2026, citando a API do repositório, `LICENSE.md` e a API de releases — sem alterar nota, vetor ou índice da ficha.

---

### User Story 3 — Lightyear ↔ Bevy: matriz, book e cadência (Priority: P1)

O leitor que escolhe entre Bevy puro e Bevy+Lightyear precisa que a matriz de compatibilidade, o estado do book e a cadência de versões deixem de ser `[NV]` sem virar regra geral: 0.28–0.30 → Bevy 0.19 e 0.31 → Bevy 0.20; o book é WIP ("collection of notes"); "4 versões por ciclo do Bevy" é observação do ciclo 0.19, não regularidade; a dupla licença MIT/Apache-2.0 está confirmada.

**Why this priority**: a cadência mal generalizada já alimenta a nota C8 de várias opções; uma leitura "sempre 4 versões" enviesaria a avaliação de manutenção futura.

**Independent Test**: basta verificar os destinos de NV-A09–NV-A12 e confirmar a ressalva do ciclo 0.19 e a tabela upstream citada em `bevy.md:11`, `bevy.md:23`, `bevy.md:26`, `licencas_lockin.md:36`, `01-matriz.md:80`, `02-analise-por-opcao.md:45`, `03-recomendacao.md:88` e `AVALIACAO-COMPARATIVA-ENGINES.md:155`, sem depender das histórias 1, 2 ou 4.

**Acceptance Scenarios**:

1. **Given** `docs/avaliacao-engines/pesquisa/bevy.md:11` com a matriz marcada `[NV]`, **When** a substituição é aplicada, **Then** o texto reproduz a tabela oficial (0.28–0.30 → 0.19; 0.31 → 0.20) e o requisito Rust 1.97.1, citando README, API de dependências do crates.io e `Cargo.toml` (≥2 fontes com URL e data).
2. **Given** `docs/avaliacao-engines/pesquisa/bevy.md:23` com "book WIP", **When** a substituição é aplicada, **Then** o texto cita a descrição "collection of notes" do `introduction.md` e a seção Examples incompleta do `SUMMARY.md`, além do README, com URL e data de acesso.
3. **Given** `docs/avaliacao-engines/pesquisa/bevy.md:26` com "4 versões do Lightyear por ciclo do Bevy", **When** a substituição é aplicada, **Then** o texto restringe a contagem ao ciclo 0.19 (0.28.0, 0.29.0, 0.30.0, 0.30.1), registra que o total varia de 1 a 13 por ciclo e que a UI de Releases marca 0.30.0 como "Latest" enquanto o crates.io já tem 0.31.0, citando crates.io, Releases e README — sem generalizar.
4. **Given** `docs/avaliacao-engines/pesquisa/licencas_lockin.md:36` com a dupla licença, **When** a substituição é aplicada, **Then** o texto afirma `MIT OR Apache-2.0` com base em `Cargo.toml`, `LICENSE-MIT`, `LICENSE-APACHE` e crates.io, com a ressalva de que a UI do GitHub exibe apenas "Apache-2.0" por detecção parcial.

---

### User Story 4 — Salvaguardas e verificação documental (Priority: P2)

O auditor precisa provar que a substituição não apagou marcadores metodológicos, não introduziu alegação de execução e não reescreveu histórico; precisa também que a auditoria seja feita por contagens simples, sem gates e sem efeito remoto.

**Why this priority**: é o que torna a feature auditável e impede que a correção de 12 fatos destrua as 249 salvaguardas registradas em governança; vem depois das substituições porque só faz sentido sobre o resultado delas.

**Independent Test**: basta percorrer os destinos listados em FR-005/FR-006/FR-007 e contar: 12 fatos resolvidos, 0 fatos sem duas fontes, 0 alegações de benchmark, 100% de marcadores B/C preservados, 100% de snapshots com nota de superação e zero linhas reescritas — sem rodar nenhum script de gate.

**Acceptance Scenarios**:

1. **Given** a lista de marcadores de classe C em FR-005, **When** a auditoria é feita, **Then** nenhum deles foi removido de `godot.md:17,24,29,35-39`, `bevy.md:3,24,32`, `netcode_modularidade.md:21-28`, `03-recomendacao.md:20,83-92`, `02-analise-por-opcao.md:19,29,37,63,65,75,86,97`, `monogame_love_haxe.md:23` (ausência de fundação neutra em LÖVE/Haxe/OpenFL) e `AVALIACAO-STACK-MDMV.md:334-346`.
2. **Given** qualquer destino de substituição, **When** a auditoria é feita, **Then** 0 frases afirmam benchmark, build, protótipo, custo por tick ou falha observada de engine; Stride #1666 aparece sempre como "sem correção registrada, não testado empiricamente".
3. **Given** os snapshots históricos em `docs/avaliacao-engines/pesquisa/**`, **When** a auditoria é feita, **Then** cada arquivo tocado contém nota de superação append-only e nenhuma linha anterior foi reescrita, inclusive a linha `Data: 19/11/2026`.
4. **Given** os 10 artefatos em `specs/004-verifica-nv-externa/`, **When** a auditoria é feita, **Then** todos existem, nenhum tem seção vazia e nenhum contém `TBD` ou `[NEEDS CLARIFICATION]` pendente.

---

### Edge Cases

- Se um fato já estiver corretamente citado no destino (ex.: `licencas_lockin.md:24` traz a política v. 21/07/2025), a ação NÃO é apagar a citação: é acrescentar a ressalva de escopo e confirmar a segunda fonte; a substituição é de precisão, não de limpeza cega.
- Se a ausência de EULA for afirmada, ela DEVE ser rotulada como afirmação negativa verificada nas fontes primárias oficiais — não como prova de que nenhum documento não-oficial exista em lojas de terceiros (Steam/Flathub).
- Se a issue Stride #1666 for citada, a redação DEVE ser "sem correção registrada" (estado `open` + ausência de registro de fix), nunca "falha confirmada por teste"; a mudança de build do Assimp na 4.4.2 tem escopo declarado em offsets de ossos e NÃO deve ser apresentada como correção de texturas incorporadas.
- Se `license: null` aparecer na API do GitHub para Dviglo, NÃO deve ser lido como ausência de licença: a licença está em `licenses/dviglo/LICENSE`.
- Se a cadência do Lightyear for citada fora do ciclo 0.19, NÃO deve ser generalizada; a leitura de "4 versões" conta publicações reais incluindo o patch 0.30.1, enquanto a tabela upstream conta séries menores (3) — ambas são consistentes e a ressalva DEVE explicar isso.
- Se um marcador pertencer à classe B (CI, merge, publicação, aprovação remota, proteção de branch) ou à classe C (salvaguarda metodológica), ele DEVE permanecer mesmo que pareça "pendente"; a feature não tem autoridade sobre ele.
- Se um documento de snapshot histórico contiver uma afirmação hoje imprecisa, o corpo NÃO é corrigido: a correção vai na nota de superação, preservando a trilha datada.
- Se a substituição parecer exigir recálculo de nota, vetor, índice ou recomendação, ela NÃO é feita: a mudança é factual/citativa, e a estabilidade dos scores é requisito explícito (FR-009, SC-007).
- Se uma afirmação não estiver nos três arquivos de evidência (por exemplo, termos exatos de GodotSteam, Nakama/Photon/PlayFab, reconexão que restaura sessão, hash no handshake), ela DEVE permanecer `[NV]`.
- Se o leitor comparar a contagem local de 12 fatos com a contagem canônica A=11/B=92/C=157, nenhuma das duas DEVE ser reescrita: a primeira é o inventário desta feature; a segunda é registro datado de governança.
- Se um destino estiver em mais de um arquivo, a mesma ressalva e o mesmo par de fontes DEVEM ser replicados em cada destino, sem remissão vaga do tipo "ver nota acima" que deixe o leitor sem URL.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: A feature DEVE substituir os 12 fatos resolvíveis NV-A01–NV-A12, inventariados em `research.md` e `plan.md`, nos documentos integrados e snapshots indicados, removendo o rótulo `[NV]` **apenas** nesses fatos.
- **FR-002**: Cada substituição DEVE conter, no próprio destino, título, URL e data de verificação (2026-11-19) de pelo menos duas fontes independentes, sem remeter o leitor a outro arquivo para obter a citação.
- **FR-003**: Cada substituição DEVE preservar a ressalva de escopo do respectivo fato: Godot (quatro hipóteses automáticas + licença customizada para motor/fork comercial; MIT sem EULA nos canais oficiais), Stride #1666 (sem fix registrado, não testado empiricamente), Lightyear (ciclo 0.19, não lei geral; UI de Releases defasada), Defold (escopo de download/instalação/uso, contas periféricas não auditadas), Dviglo (`license: null` é artefato de detecção), Fyrox (versões 1.0.0/RC1/RC2).
- **FR-004**: NENHUMA substituição DEVE afirmar benchmark, build, protótipo, execução de engine, custo por tick ou falha funcional observada; a ausência de correção da issue #1666 DEVE ser relatada como inferência documental a partir de estado aberto e ausência de registro de fix.
- **FR-005**: Os marcadores `[NV]` de classe C DEVEM permanecer integralmente, incluindo reconexão que restaura sessão, hash de conteúdo no handshake, snapshot versionado, custo/tick e benchmarks 5v5+bots, `allow_object_decoding=true` (RCE), lógica de coleta/sabotagem/negociação com PNJs e sincronia 2 jogadores/5 s, custo de editor de Módulos declarativo, viabilidade de replay determinístico e ausência de fundação neutra em LÖVE/Haxe/OpenFL, registrada em `docs/avaliacao-engines/pesquisa/monogame_love_haxe.md:23`.
- **FR-006**: Os marcadores `[NV]` de classe B DEVEM permanecer integralmente: execução externa, CI, merge, publicação/release/deploy, aprovação em plataforma remota e proteção de branch, além das medições locais ainda não executadas em `docs/AVALIACAO-STACK-MDMV.md` e `docs/avaliacao-stack/**`.
- **FR-007**: Os arquivos de snapshot histórico `docs/avaliacao-engines/pesquisa/**` DEVEM receber nota de superação append-only, sem reescrita do corpo datado; a nota DEVE apontar os fatos NV-A01–NV-A12 aplicáveis, as fontes e a data 2026-11-19.
- **FR-008**: Os documentos integrados — `docs/AVALIACAO-COMPARATIVA-ENGINES.md`, `docs/avaliacao-engines/` exceto `pesquisa/`, `docs/AVALIACAO-STACK-MDMV.md` e `docs/avaliacao-stack/**` — DEVEM ter o marcador substituído no corpo, mantendo o resto do texto inalterado.
- **FR-009**: NENHUMA nota, vetor de C1–C10, índice ponderado, score ou recomendação DEVE ser recalculada ou reescrita em consequência da substituição; a divergência de versão do Fyrox é correção de fato, não de nota.
- **FR-010**: NENHUM arquivo de `docs/governance/**`, ADR, `.specify/memory/STATE.md`, `docs/PROGRESSO.md` ou `converge.md` DEVE ser alterado; a contagem canônica A=11/B=92/C=157 permanece como registro datado.
- **FR-011**: Toda afirmação não coberta pelos três arquivos de evidência DEVE permanecer `[NV]`; nenhuma URL nova DEVE ser introduzida sem que o mesmo fato tenha segunda fonte já registrada em `research.md`.
- **FR-012**: A data atribuída a cada citação DEVE ser a data de acesso 2026-11-19; datas de versão/publicação das fontes (21/07/2025, 02/12/2024, 29/03/2026, 26/06/2026 etc.) DEVEM ser mantidas como datas do documento, não convertidas em data de acesso.
- **FR-013**: O inventário DEVE registrar, para cada um dos 12 fatos, o ID NV-Axx, o arquivo/linha de destino, a ressalva aplicável e as fontes, permitindo auditoria por contagem em `plan.md`.
- **FR-014**: A execução desta feature de planejamento NÃO DEVE rodar gates, fazer stage nem commit, e NÃO DEVE escrever fora de `specs/004-verifica-nv-externa/`.

### Key Entities *(include if feature involves data)*

- **Fato resolvível (NV-A01–NV-A12)**: unidade de trabalho; identidade pelo ID; atributos = afirmação, ressalva, arquivo de evidência, fontes (título/URL/data), destinos (arquivo/linha), classe A. É o único tipo de marcador que esta feature pode remover.
- **Citação auditável**: bloco no destino com título, URL, data de acesso e pelo menos duas fontes independentes; identidade pelo par (fato, fonte); garante FR-002 e SC-002.
- **Nota de superação**: apêndice em snapshot histórico; identidade pelo arquivo; conteúdo = fatos aplicáveis, fontes e data; não substitui o corpo datado (FR-007).
- **Marcador preservado (classe B/C)**: marcador fora de escopo; identidade pela classe e pelo local; imutável por FR-005/FR-006.
- **Inventário de auditoria**: registro em `research.md`/`plan.md` com contagens de fatos resolvidos, fontes, alegações de benchmark e marcadores preservados; base dos SCs mensuráveis.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 12 de 12 fatos NV-A01–NV-A12 resolvidos nos destino(s) previstos (baseline: 0 de 12 aplicados; meta 12 de 12), verificável pelo inventário de FR-013.
- **SC-002**: 0 fatos substituídos com menos de duas fontes independentes ou sem título, URL e data (baseline: risco implícito nos marcadores; meta 0).
- **SC-003**: 0 afirmações novas de benchmark, build, protótipo, execução de engine, custo por tick ou falha empírica observada (baseline: 0 nesta rodada; meta 0).
- **SC-004**: 100% dos marcadores de classe C listados em FR-005 — inclusive a ausência de fundação neutra em LÖVE/Haxe/OpenFL em `docs/avaliacao-engines/pesquisa/monogame_love_haxe.md:23` — preservados nos arquivos nomeados em US4/AC1 (baseline: presentes; meta nenhum removido).
- **SC-005**: 100% dos marcadores de classe B listados em FR-006 preservados, incluindo CI, merge, publicação, aprovação remota e proteção de branch (baseline: presentes; meta nenhum removido).
- **SC-006**: 100% dos snapshots históricos tocados com nota de superação append-only e 0 linhas do corpo reescrito, incluindo a linha `Data: 19/11/2026` (baseline: 0 notas; meta 100%/0).
- **SC-007**: 0 notas, vetores, índices, scores e recomendações alterados em consequência da substituição (baseline: 0; meta 0).
- **SC-008**: 100% das substituições com a ressalva aplicável de FR-003 presente no próprio destino (baseline: parcial; meta 12 de 12).
- **SC-009**: 0 arquivos de governança, ADRs, `STATE.md`, `PROGRESSO.md` ou `converge.md` alterados e 0 gates, stages ou commits executados por esta feature (baseline: 0; meta 0).
- **SC-010**: 10 de 10 artefatos de spec/checklist em `specs/004-verifica-nv-externa/` presentes (intake, research, problem, concept, decision, spec, requirements, plan, nv-externa-quality, tasks — `research-evidence/**` não entra nesta contagem), com nenhuma seção vazia e nenhum `TBD` ou `[NEEDS CLARIFICATION]` pendente (baseline: 1 artefato de template; meta 10 de 10).

## Assumptions

- Os três arquivos em `research-evidence/` são suficientes e foram verificados ao vivo em 19/11/2026; nenhuma URL será buscada novamente nesta feature.
- `docs/avaliacao-engines/pesquisa/**` é o conjunto de snapshots históricos; os demais caminhos do pedido são documentos integrados passíveis de substituição no corpo.
- A contagem local de 12 fatos é a unidade de work desta feature; a contagem canônica A=11/B=92/C=157 é registro de governança datado e não será reescrita.
- A ressalva de cada fato pode ser acrescentada sem alterar o significado de notas, índices e recomendações já consolidados.
- O trabalho é documental: nenhum runtime, engine, build, gate, efeito remoto, stage ou commit é necessário para concluir o planejamento.
- Toda evidência vem de fontes primárias oficiais já coletadas; conteúdo web é tratado como dado, nunca como instrução.
- A veredito `go` de `decision.md` e o pedido explícito do mantenedor autorizam a especificação; nenhuma aprovação remota é pré-requisito.

## Clarifications

### Session 2026-11-19

- Q: Como deve ser redigida a afirmação "sem correção na 4.4" da issue Stride #1666 sem transformá-la em alegação de falha observada? → A: Como "issue permanece `open` após 4.4.3 e nenhum fix foi registrado na issue ou nas notas de release", com ressalva explícita de que não houve build/import empírico; a mudança de build do Assimp na 4.4.2 tem escopo declarado em offsets de ossos e não é apresentada como correção de texturas incorporadas (`research-evidence/stride-fyrox.md:7-13`).
- Q: Quais documentos recebem substituição no corpo e quais recebem nota de superação? → A: Snapshots históricos são `docs/avaliacao-engines/pesquisa/**` (corpo datado preservado, nota append-only); documentos integrados são `docs/AVALIACAO-COMPARATIVA-ENGINES.md`, `docs/avaliacao-engines/` exceto `pesquisa/`, `docs/AVALIACAO-STACK-MDMV.md` e `docs/avaliacao-stack/**` (substituição no corpo), conforme `docs/governance/nv-resolution-2026-11-19.md:20-22`.
- Q: A afirmação "4 versões do Lightyear por ciclo do Bevy" é lei geral? → A: Não. Vale para o ciclo 0.19 (0.28.0, 0.29.0, 0.30.0, 0.30.1); o total varia de 1 a 13 por ciclo e a tabela upstream conta séries menores (3) enquanto as publicações reais somam 4; a UI de Releases marca 0.30.0 como "Latest" embora o crates.io já tenha 0.31.0 (`research-evidence/lightyear-bevy.md:29-43`).

Nenhuma pergunta foi feita ao usuário: as três ambiguidades foram resolvidas uma por vez por evidência local (três arquivos de evidência e contrato de governança), em conformidade com R3 (≤3 esclarecimentos).

## Contexto e Orçamento *(padrão do hub MDMV — documental)*

> Seção aditiva e documental: nenhum gate determinístico a lê.

Esta feature é documental e não gera manifesto de orçamento de contexto nem saídas com handles. Diferentemente de specs com código, não há stack, API, banco ou migração: o "mecanismo" é a citação auditável em documentos já existentes. Por isso as decisões técnicas (matriz de fontes, templates de citação e de nota de superação, matriz arquivo→fato) ficam consolidadas em `plan.md`, sem artefatos separados de dados, contratos ou quickstart. Nenhum verde, CI, merge, publicação, aprovação remota ou proteção de branch é afirmado; tudo o que não foi verificado ao vivo permanece `[NV]`.
