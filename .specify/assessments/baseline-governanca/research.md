# Idea Research: Baselines locais de governança

- **Slug**: baseline-governanca
- **Created**: 2026-11-19
- **Evidence confidence (overall)**: high

Todas as asserções abaixo vêm de fontes locais do alvo: registros datados e código dos próprios gates. Nenhuma fonte remota foi consultada; o que depende de remoto está marcado `[NV]`.

## Users & Demand

- Mantenedores e auditores do aparato SDD local precisam distinguir "gate PASS" de "gate que detecta regressão": os checkers de governança rodam com `baselined=0` e nenhum arquivo de baseline existe em `docs/governance/` — [source: `.specify/memory/STATE.md:36-46`; listagem local de `docs/governance/`; confidence: high]
- A dívida 1 está explicitamente aberta no estado vivo: `docs/governance/sdd-completeness-baseline.json` não materializado, com a ressalva de evitar chaves de specs da factory — [source: `.specify/memory/STATE.md:50`; confidence: high]
- A auditoria pós-ciclo-001 classifica "verdes com `baselined=0/edges=0/files=0`" entre as coisas que **não** funcionam, e `D4` registra `converge.md` sem entradas (`files=0`) — [source: `docs/PROGRESSO.md:96`, `docs/PROGRESSO.md:68`; confidence: high]
- A fila 80/20 ranqueou "Materializar baseline local de governança" como candidato 3 (impacto 3, confiança 5, facilidade 4, risco 2; score 30,0), sem dependência de engine, download ou autorização remota — [source: `docs/governance/80-20-ranking-2026-11-19.md:33`; confidence: high]

## Prior Art

- Os ciclos 001 e 002 materializaram o aparato SDD (scripts, comandos, specs, ADRs, R4/R7), mas os checkers continuaram PASS com zero baselines: ciclo 001 `1/0/0`, ciclo 002 `2/0/0`, converge `files=0` em ambos — [source: `docs/PROGRESSO.md:87`, `docs/PROGRESSO.md:106`, `.specify/memory/STATE.md:40-43`; confidence: high]
- Suporte nativo já existe nos três scripts locais: todos leem um JSON de baseline por `--baseline` com um caminho padrão em `docs/governance/`, e `check_converge_append_only.py` expõe `--bootstrap` para imprimir o baseline a partir do estado atual — [source: `scripts/ci/check_sdd_completeness.py:24,97`; `scripts/ci/check_converge_append_only.py:18,51,54,59-62`; `scripts/ci/check_task_id_uniqueness.py:16,58`; confidence: high]
- A factory é **fonte, não autoridade** (decisão do Bloco 0.a), e o aparato foi transplantado de HEAD limpo `b1deab1` sem transplantar a árvore suja — portanto baselines da factory não podem ser adotados como baselines do alvo — [source: `docs/PROGRESSO.md:15-17`, `.specify/memory/STATE.md:34`, `docs/governance/80-20-ranking-2026-11-19.md:33`; confidence: high]
- O contrato de fase exige evidência de artefatos, não apenas comandos: R4 cola saída executada com data/hora antes de R7 — [source: `docs/governance/sdd-phase-contracts.md:24-27`; confidence: high]

## Market & Context

- Alternativa em uso hoje: rodar os gates sem baseline e aceitar PASS vazio como suficiente. Custo: nenhum ratchet de integridade e auditividade que afirma cobertura sem a ter — [source: `docs/PROGRESSO.md:39-41`, `docs/PROGRESSO.md:96`; confidence: high]
- Alternativa ingênua: copiar os baselines da factory. Custo: chaves de specs alheias gerariam entradas `stale` auto-infligidas nos três gates, que tratam baseline órfão como falha — [source: lógica de `stale` em `scripts/ci/check_sdd_completeness.py:121-129`, `scripts/ci/check_task_id_uniqueness.py:74-82`, `.specify/memory/STATE.md:50`; confidence: high]
- O alvo é predominantemente documental, sem código, engine, testes, build ou CI rodando; a tese 80/20 de fatia vertical ainda não foi executada — logo o volume de artefatos a proteger é pequeno, mas tende a crescer — [source: `docs/PROGRESSO.md:18-19`, `docs/PROGRESSO.md:67`; confidence: high]

## Data & Constraints

- **sdd-completeness** — `docs/governance/sdd-completeness-baseline.json`; formato `{"allow_missing": {"<spec-dir NNN-slug>": ["<artefato>", ...]}}`, schema `047.2.0`. Artefatos possíveis: `spec.md`, `plan.md`, `tasks.md`, `checklists`, `assessment`, `validate`, `release` e, para specs com número ≥ 88, `analyze.md`/`converge.md` (`strict_from=88`). Regressão = faltante fora do baseline; `stale` = entradas já satisfeitas ou feature removida. Contador exibido: `baselined_features = len(baseline)` — [source: `scripts/ci/check_sdd_completeness.py:24-31,50-90,112-156`; confidence: high]
- **converge-append-only** — `docs/governance/converge-append-only-baseline.json`; formato `{"schema_version":"087.1.0","enforce_unbaselined":true,"files":{"<caminho relativo>":{"sha256":"...","bytes":N}}}`. `compute_actual` varre `specs/[0-9][0-9][0-9]-*/converge.md`. PASS exige prefixo SHA256 estável nos `bytes` registrados e tamanho ≥ baseline; apêndices são permitidos; `enforce_unbaselined=true` sinaliza arquivo novo não baselined. `--bootstrap` gera o JSON do estado atual. Contador: `baselined_files = len(baseline)` — [source: `scripts/ci/check_converge_append_only.py:18,25-46,49-65,72-118`; confidence: high]
- **task-id-uniqueness** — `docs/governance/task-id-uniqueness-baseline.json`; formato `{"allow_duplicates": {"<spec-dir>": ["duplicate_task:T###", ...]}}` (aceita `allow_missing` como legado), schema `087.1.0`, regex `^\s*-\s*\[[ xX]\]\s+(T\d{3,})\b`. Contador: `baselined_specs = len(baseline)` — [source: `scripts/ci/check_task_id_uniqueness.py:16-17,20-53,86-102`; confidence: high]
- Estado documentado mais recente: `spec_dirs=2`, `gaps=0`, `files=0`, `duplicates=0`, `baselined=0` nos três gates (`docs/PROGRESSO.md:106` e `.specify/memory/STATE.md:40-43`). Registros anteriores divergem (`spec_dirs=0` e `spec_dirs=1`), portanto contagens históricas não são fonte autoritativa no ato da materialização — [source: `docs/PROGRESSO.md:33-36`, `docs/PROGRESSO.md:87`; confidence: high]
- Consequência: um baseline honesto derivado de um estado limpo pode ter mapas vazios. O valor materializado está em (a) resolver a dívida com arquivo local de schema conhecido, (b) ativar as verificações de `stale` e de `unbaselined`, e (c) tornar explícita a propriedade local — não em inventar entradas — [source: composição de `load_baseline`/`main` nos três scripts + `.specify/memory/STATE.md:50`; confidence: medium]
- Limite de autoridade: aprovação/merge/CI remotas e proteção de branch permanecem `[NV]`; nada disso é pré-requisito para baselines locais. [NV] — nenhuma consulta remota foi executada nesta pesquisa.

## Evidence Against the Idea

- Se o estado atual tem `gaps=0`, `files=0` e `duplicates=0`, um baseline derivado honestamente pode permanecer com mapas vazios (`baselined=0`) — exatamente o sintoma que se quer eliminar. Gravar o arquivo, por si só, não aumenta a cobertura detectada. [source: `.specify/memory/STATE.md:40-43`; composição dos `load_baseline`; confidence: high]
- Registrar gaps atuais em `allow_missing` transforma dívida tolerada em dívida aceita: o gate PASS passa a significar "dentro do baseline", não "sem dívida". Sem critério explícito do que é legítimo, a medida pode reduzir a sensibilidade em vez de aumentá-la. [source: semântica de `allow_missing`/`allow_duplicates` nos scripts; confidence: medium]
- O benefício é marginal enquanto não surgirem novos `converge.md` ou specs com gaps: D3/D4 mostram que código e convergências ainda não existem no alvo, e o próprio ranqueamento 80/20 deu apenas score 30,0 ao item. [source: `docs/PROGRESSO.md:67-68`, `docs/governance/80-20-ranking-2026-11-19.md:33`; confidence: high]
- A restrição "nunca copiar da factory" é um risco operacional real, não retórica: os três gates punem entradas de baseline sem contrapartida no alvo como `stale`. [source: `.specify/memory/STATE.md:50`, lógica de `stale` nos scripts; confidence: high]

## Gaps & Open Questions

- [NEEDS CLARIFICATION: estado exato do alvo no momento da materialização — as contagens históricas variam entre `spec_dirs=0`, `1` e `2`; os scripts devem ser a fonte no ato da execução, não esta pesquisa.]
- [NEEDS CLARIFICATION: política para baseline com mapas vazios — aceitar como ponto de partida ou exigir ao menos uma entrada por gate?]
- [NEEDS CLARIFICATION: critério para classificar um gap atual como dívida legítima (a baselinar) versus defeito a corrigir antes de baselinar.]

## Sources

- `.specify/memory/STATE.md` (host: local, policy: allowlisted-local) — dívida 1, saídas finais dos gates, factory como fonte.
- `docs/PROGRESSO.md` (host: local, policy: allowlisted-local) — inventário verificado, D1/D4, auditoria "não funciona".
- `scripts/ci/check_sdd_completeness.py` (host: local, policy: allowlisted-local) — formato `allow_missing`, schema `047.2.0`, `strict_from=88`.
- `scripts/ci/check_converge_append_only.py` (host: local, policy: allowlisted-local) — formato `files`/`sha256`/`bytes`, `--bootstrap`, `enforce_unbaselined`.
- `scripts/ci/check_task_id_uniqueness.py` (host: local, policy: allowlisted-local) — formato `allow_duplicates`, regex `T###`, schema `087.1.0`.
- `docs/governance/80-20-ranking-2026-11-19.md` (host: local, policy: allowlisted-local) — candidato 3, score 30,0.
- `docs/governance/sdd-phase-contracts.md` (host: local, policy: allowlisted-local) — R1 e R4/R7.
- [NV] nenhuma fonte remota, CI, merge ou baseline da factory foi consultado nesta pesquisa; nenhuma URL foi buscada.
