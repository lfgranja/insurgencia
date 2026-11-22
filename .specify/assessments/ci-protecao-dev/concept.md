# Concept: CI de gates determinísticos e proteção de dev

- **Slug**: ci-protecao-dev
- **Created**: 2026-11-20
- **Recommended option**: Opção A — workflow único com job de gates e proteção pós-observação

## Options
Três opções são consideradas, da menor intervenção à verificação granular; a recomendação é nomeada ao final com base nos objetivos de `problem.md`.

### Option A — Workflow único com job de gates e proteção pós-observação
- **Sketch**: um único workflow responde a PR e push para `dev`/`main` e executa, em sequência, os cinco gates determinísticos do alvo como uma verificação única. Depois de uma execução real, o mantenedor lê o nome do status check reportado, registra-o com evidência e só então configura a proteção da branch `dev` para exigir aquele nome. O resultado é um único sinal de verificação por PR, auditável e fácil de nomear.
- **Appetite**: medium — dias de configuração e documentação, mais a espera por uma execução ao vivo e por uma decisão humana de proteção; sem desenvolvimento de produto.
- **Trade-offs**: ganha simplicidade, um único nome de check para observar e proteção binária (todos os gates passaram ou não); sacrifica granularidade — um único check não diz qual dos cinco gates falhou sem ler o log. Riscos: a lacuna de diretórios derivados da spec 005 pode fazer o gate de completude falhar no início; a proteção depende de autorização separada.

### Option B — Matriz de cinco jobs, um por gate
- **Sketch**: cinco checks separados, nomeados um por gate, permitindo proteger `dev` com uma lista de checks e ver na UI qual gate falhou.
- **Appetite**: medium — mais YAML, cinco nomes a observar e mais superfície de configuração.
- **Trade-offs**: ganha diagnósticos por gate e proteção seletiva; sacrifica a simplicidade de "um sinal = verificação completa" e multiplica o trabalho de observação e de manutenção de nomes. Risco: proteger com subconjunto dos cinco checks e dar falsa sensação de cobertura.
- **Rabbit holes**: parametrizar matrizes de gates com nomes dinâmicos, duplicar passos de setup e criar divergência entre a lista de gates do workflow e a lista de `AGENTS.md`.

### Option C — Manter verificação apenas local
- **Sketch**: nenhum workflow; o mantenedor continua executando os cinco gates localmente e registrando as saídas em documentos.
- **Appetite**: small — nenhuma configuração remota.
- **Trade-offs**: ganha zero exposição remota; não resolve o problema, pois não gera check em PR nem protege `dev`, deixando `statusCheckRollup=[]` e `[NV]` inalterados.
- **Rabbit holes**: confundir "gates locais PASS" com "CI verde" e afirmar proteção sem API.

## Recommendation

Opção A. Ela ataca diretamente os dois objetivos de maior valor: todos os cinco gates passam a condicionar o merge em `dev`/`main` e o nome exigido pela proteção vem de observação ao vivo, não de suposição. É a opção com menor superfície de erro — um check a observar, um conjunto de gates a manter — e preserva a hierarquia de autoridade: a criação do workflow é automatizável, a mutação de proteção é um ato humano separado. A opção B é uma evolução razoável se, no futuro, for necessário diagnosticar gates individualmente na UI, mas hoje aumenta o trabalho sem aumentar a garantia; a opção C não resolve o problema. Nenhuma opção inclui publicação/deploy nem importa o gate Forgejo da factory.

## Out of Scope (for the recommended option)

- Publicação, release, deploy, publicação de pacotes ou imagens e qualquer segredo de credencial.
- Importação de `.forgejo/**`, uso de `fj` e adoção de gates não materializados no alvo.
- Mudança no código dos gates, nas baselines de ratchet, em ADRs ou em documentos de governança para produzir verde artificial.
- Nomes de check presumidos; proteção aplicada antes da execução observada; supressão de falha.
- Stage, commit, merge, reescrita de histórico ou escrita fora de `specs/005-ci-protecao-dev/` nesta etapa de planejamento.

## Assumptions to Validate

- O repositório permanece público (`visibility=PUBLIC`, evidência 2026-11-20T00:26:47Z; leitura via `gh api` em 2026-11-20T00:29Z).
- A conta de automação tem permissão para criar workflows e, em ato separado, para consultar e alterar a proteção de `dev` via API.
- Os cinco gates terminam com `PASS` no runner após a materialização dos diretórios derivados da spec 005 pelas fases de validação e release; antes disso, a falha é honesta e registrada.
- O nome do check pode ser lido em uma execução real (`gh pr checks` ou API de check-runs) e permanece estável enquanto o workflow não for renomeado.
- A autorização do mantenedor cobre a criação do workflow; a proteção de `dev` depende de autorização explícita adicional e de verificação por API.
