# Concept: Baselines locais de governança

- **Slug**: baseline-governanca
- **Created**: 2026-11-19
- **Recommended option**: Option A — Bootstrap local integral dos três baselines

## Options

### Option A — Bootstrap local integral dos três baselines

- **Sketch**: partir do estado concreto do alvo no ato da execução e gravar os três JSON de baseline nos caminhos que os próprios scripts leem, usando os formatos e schemas que eles já consomem (inclusive o `--bootstrap` disponível para o gate de converge). Cada arquivo registra explicitamente que a origem é o alvo, não a factory. A saída executada dos três gates é colada como evidência antes de R4. Quem roda os gates passa a ver, na própria linha de saída, o contador de itens baselinados e a distinção entre regressão e baseline obsoleto.
- **Appetite**: small (dias) — três arquivos JSON documentais e uma rodada de gates; nenhuma dependência externa, nenhuma alteração de código.
- **Trade-offs**: fecha a dívida 1 inteira, ativa as verificações de `stale` e `unbaselined` e preserva a regra factory-como-fonte. Se o estado atual estiver limpo, os mapas podem ficar vazios e o ganho imediatamente detectável é limitado: o valor está na proveniência e no ratchet, não em uma contagem maior. Exige disciplina de re-baselinar quando o estado mudar, sob pena de `stale` virar ruído.
- **Rabbit holes**: inflar a contagem registrando gaps reais como aceitos só para ver `baselined>0`; copiar da factory por conveniência; tratar baseline vazio como se fosse cobertura implementada; reescrever os scripts para "adaptar" o formato.

### Option B — Somente o baseline de converge-append-only

- **Sketch**: materializar apenas `converge-append-only-baseline.json` a partir do estado atual — o único gate com gerador de bootstrap nativo — e adiar os outros dois. A mudança observável é restrita à integridade de `converge.md`.
- **Appetite**: small (dias), menor que A.
- **Trade-offs**: resolve a parte mais mecanizada e de menor ambiguidade do problema, com risco mínimo. Deixa dois terços da dívida 1 em aberto e mantém dois gates com `baselined=0`; a narrativa de "verde vazio" continua aplicável.
- **Rabbit holes**: declarar a dívida fechada com um terço entregue; confundir presença do gerador `--bootstrap` com necessidade de só baselinar aquele gate.

### Option C — Manter os gates sem baseline e aceitar a dívida

- **Sketch**: não gravar baseline algum; registrar a dívida 1 como aceita e seguir com os gates emitindo PASS vazio. Nenhuma mudança observável para quem roda os checks.
- **Appetite**: nenhum (dia zero).
- **Trade-offs**: custo zero e risco zero de herdar a factory, mas falha diretamente os goals de auditabilidade e detecção de regressão; contradiz a classificação de "não funciona" em `docs/PROGRESSO.md:96`.
- **Rabbit holes**: racionalizar "PASS com `baselined=0`" como verde legítimo; usar a ausência de código/engine no alvo como desculpa para adiar governança documental.

## Recommendation

Option A. É a única que cobre os três gates apontados na dívida 1 e no problema, com appetite small e evidência high de que os formatos já existem nos scripts locais (`docs/governance/` como caminho padrão, schemas `047.2.0`/`087.1.0`, `--bootstrap` para converge). Alinha-se ao candidato 3 do 80/20 (score 30,0, confiança 5, facilidade 4, risco 2) e ao goal de não herdar a factory. A ressalva do `Evidence Against` — estado limpo pode gerar mapas vazios — é tratada como restrição de honestidade, não como motivo para inventar entradas: se não há gaps reais, o baseline é vazio por definição e o critério de sucesso passa a ser proveniência + ratchet ativo, registrado em R4.

## Out of Scope (for the recommended option)

- Alterar lógica, limiares, schemas ou caminhos padrão dos scripts de gate.
- Copiar baseline, specs ou configurações da factory; usar a árvore suja dela como fonte.
- Declarar CI, merge, publicação, deploy ou proteção de branch como verdes — permanecem `[NV]`.
- Materializar engine, código, telemetria, banco ou CI.
- Reabrir as dívidas 2–6 do STATE ou reclassificar dívidas documentais como resolvidas.
- Escrever a especificação técnica, a arquitetura ou as tarefas deste ciclo.

## Assumptions to Validate

- O estado autoritativo no ato da materialização é o produzido pelos próprios scripts sobre o alvo, não as contagens históricas divergentes de STATE/PROGRESSO (`spec_dirs=0/1/2`).
- `docs/governance/` continua sendo o diretório autoritativo de baselines (valor padrão de `--baseline` nos três scripts) e os schemas atuais permanecem estáveis até a implementação.
- Baseline com mapas vazios, desde que proveniência local e saída dos gates sejam registradas, satisfaz a dívida 1 — a ser confirmado na clarificação da spec.
- [NEEDS CLARIFICATION: há dívida atual que deva ser corrigida antes de virar entrada aceita em `allow_missing`/`allow_duplicates`?]
