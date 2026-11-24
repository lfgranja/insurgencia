# ROADMAP.md — Insurgência

> **Nota datada (2026-11-23T22:00Z) — vigente.** Ciclo 055, worktree
> `feature/055-escudo-milicia`. Fatos **[EV]** do baseline: ALVO `origin/dev=3f3b01c`; FONTE
> `b1deab1` (fonte, não autoridade); 0 PRs abertos; 53 specs no head; 136 testes; 5 gates locais
> PASS; decisão 055 **960/3125**; opção **BF-002a** (Escudo de Milícia) escolhida; **R6** fora da
> sequência executável e promoção `dev→main`/deploy **exclusivos do SWE**. **[NV]**: CI/deploy
> remoto. **[INF]**: as fases 80/20 e o horizonte GDD §16.6 abaixo seguem como plano; a nota de
> 2026-11-20T01:48Z e as seções seguintes são **histórico**. "80/20" ordena a fila, nunca percentual
> de conclusão.
>
> **Nota datada (2026-11-20T01:48Z) — histórica** (superada pela nota vigente de
> 2026-11-23T22:00Z). Baseline: HEAD local `2af7537`; `origin` público, 12 PRs
> `MERGED`, 10 execuções CI `success`, proteção de `dev` com `gates`; 7 specs, 222 tarefas `[X]`,
> 5/5 gates PASS. **Nenhum código do ciclo 80/20 existe.** "80/20" ordena a fila; nunca percentual
> de conclusão. Evidências: `docs/PROGRESSO.md`, `.specify/memory/STATE.md`, `docs/research/`.

## 1. Princípios de sequenciamento

1. Gate determinístico local é verdade; saída de agente é informativa. Transição exige comando
   executado + saída colada + data/hora; PASS em baseline vazia não é cobertura.
2. Spec e plano do ciclo **já existem**; este roadmap só os ordena. `[x]` = revisão de requisitos,
   não implementação — não move fase. Cada fase fecha com evidência (fonte, comando, resultado,
   confiança, não verificado), nunca com "passou" sem comando no build atual.
3. Factory (`b1deab1`) é fonte, não autoridade; gate do alvo = `scripts/ci/`. Sem merge,
   auto-merge, force-push ou publicação — **[NV]**.

## 2. Fases do ciclo 80/20 (spec CE-1..CE-5 / plano Tasks 1–6)

| Fase | Entrega | Âncora | Marco verificável | Estado em 2026-11-20 |
|---|---|---|---|---|
| **F0 — Abertura** (ranking #1) | ADR de exceção de linguagem (GDScript) + resolução N3 (MIT/AGPL); decidir no SDD | `docs/governance/80-20-ranking-2026-11-19.md` #1 | `decision.md` com `go` (R1), 7 desvios endereçados; entregável documental | **concluída** — ciclos 001/002; ADRs 001/002 aprovadas |
| **F1 — Núcleo e contrato** | Estrutura canônica (`nucleo/`, `modulos/`, `manifesto.json`), `module.json` (11 campos, 10 obrigatórios), Módulo de exemplo | CE-1 / Task 1 | `OK_CAMINHOS`; 11/11 campos; 3/3 recusas de tipo; `git diff --stat -- nucleo/` vazio | **não iniciada** — 0/39 caminhos presentes |
| **F2 — Servidor e validação** | Servidor *headless*, *handshake* com valores padrão, validação autoritativa (sequência, `seq`, janela) | CE-2 / Tasks 2–3 | 2/2 `join_accept` em `127.0.0.1:7777` ≤ 2 s; 3/3 estados prontos recusados; 20/20 *snapshots* com 5 campos | **não iniciada** — 0 testes; Godot ausente do PATH nesta sessão |
| **F3 — Ilha Meio** | Sincronia de 2 placas em 5 s; único estado persistido `ilha_concluida`; gás 8/amostra (24 em 3 s) | CE-3 / Task 4 | 2/2 capturas em 4,9 s; 2/2 falhas em 5,3 s; 0 capturas por 1 jogador; 1 sinal por ilha | **não iniciada** — regra de produção 3/5 s permanece só no Módulo base |
| **F4 — Manifesto e gate local** | `manifesto.json` (6 campos), SHA-256 de 32 bytes, 4 códigos de recusa | CE-4 / Task 5 | `make check-compat`: 4/4 códigos exatos em 12 execuções; 0 aceitações indevidas | **não iniciada** — `Makefile` e `manifesto.json` ausentes |
| **F5 — Evidência e reavaliação** | `docs/progress/evidence.csv` (9 colunas), arquivo datado, `make check` | CE-5 / Task 6 | 5/5 critérios com ≥ 1 linha; 0 `PASS` com `unverified` não vazio; **reavaliar** Ilha Central/elenco/5v5 | **não iniciada** — `docs/progress/` não existe |

### 2A. Correções obrigatórias vindas da pesquisa de 2026-11-20

Fontes primárias em `docs/research/2026-11-20-godot-headless-oficial.md`. Não são novas decisões
normativas, mas ajustes de implementação a aplicar antes/durante F2–F5:

| P | Ajuste | Por quê |
|:--|:--|:--|
| P0 | Todo script executado por `--script` deve declarar `extends SceneTree` (ou `MainLoop`); se `main_server.gd` precisar ser `Node`, criar launcher | Doc CLI 4.7: "The script must inherit from `SceneTree` or `MainLoop`" |
| P0 | Distinguir `DisplayServer.get_name() == "headless"` (editor/testes) de `OS.has_feature("dedicated_server")` (PCK exportado) | A tag `dedicated_server` não aparece rodando pelo editor |
| P0 | Definir **envelope próprio** (magic + versão + tipo + tamanho + payload) e registrar o modo de compressão ENet no handshake | `SceneMultiplayer`: wire de alto nível é "implementation detail … may change without notice" |
| P1 | `Engine.physics_ticks_per_second = 30`, `physics_jitter_fix = 0`, acumulador para snapshots 30→20 Hz (padrão 2,1,2,1 ticks) | 30/20 não é divisor inteiro; jitter fix padrão 0,5 é desaconselhado em rede |
| P1 | `FileAccess.get_sha256()` sobre **bytes crus**; arquivos de Módulo em modo **Keep File** na exportação | JSON do Godot não é canonicalizador estrito; PCK pode alterar/remover arquivos |
| P1 | Usar `auth_timeout` (não `auth_timout`, typo do tutorial 4.7); incluir `--check-only` no gate local de sintaxe | Doc de classe vs tutorial |
| P2 | Fixar links de documentação em `/en/4.7/` (não `latest`/`4.x`); 4.8-beta já existe | `latest` é a branch master, instável |
| P2 | **Não abrir nova ADR**: ADR-001 permanece válida (Bevy é 0.20.1; netcode no Bevy é de terceiros) | `docs/research/2026-11-20-alternativas-netcode-oficial.md` |

**Portas SDD:** constituição → intake → research → define → shape → decide → specify → clarify →
plan → checklist → analyze → tasks → implement → converge → validate (R4) → release (R7). R3 ≤ 3
esclarecimentos, um por vez; R4 antes de R7; R7 exige SemVer + changelog com IDs FR/T/bug + rollback;
`converge.md` append-only.

## 3. Fila após o ciclo (candidatos 3–6 — só gatilhos)

| # | Candidato | Score | Gatilho | Status em 2026-11-20 |
|---:|---|---:|---|---|
| 3 | Godot 4.7.2 + suíte headless | 10,0 | artefato confirmado; F1–F2 escritos | versão **confirmada por fonte primária** (18/08/2026, commit `ed1daf0`); engine ausente do PATH nesta sessão; suíte ainda não existe |
| 4 | Gate CI remoto (GitHub Actions) | 16,0 | autorização remota + F5 evidenciado | **concluído** — 10/10 execuções `success` e check `gates` exigido em `dev` (verificado ao vivo) |
| 5 | Adaptador PostgreSQL + `sqlx` | 8,0 | runtime + medição de capacidade | validação local descartável concluída (ciclo 007); remoto/Coolify/IAMAdapter **[NV]** |
| 6 | OTLP/Collector/VM+Grafana | 4,5 | runtime e telemetria justificados | DEFER — sem runtime |

Nenhum item é "quase pronto" por si: telemetria, publicação e deploy seguem sem verificação ao vivo.

## 4. Horizonte do GDD §16.6 (fora do ciclo 80/20)

| Fase GDD | Semanas [HIPÓTESE] | Entrega | Marco |
|---|---|---|---|
| 0 — Fundação | 1–2 | Núcleo Godot dedicado; conexão direta; validação de inputs | 2 clientes conectam; nenhum estado pronto aceito |
| 1 — Ilha Topo | 3–4 | *Greyboxing*; 3 alavancas em ordem | sequência correta captura; errada não |
| 2 — Ilhas Meio e Inferior | 5–7 | janela de 5 s; ponto de coleta | 2 jogadores em 5 s capturam; coleta creditada |
| 3 — Funções | 8–9 | Artesão (Barricada) e Guerreiro (Ataque Forte) | custo/bloqueio e recarga respeitados |
| 4 — Fechamento do MVP | 10–12 | Ilha Central; vitória por Autogestão; Módulo externo | 3 ilhas + Artesão e Guerreiro vivos, 4 s de canalização → servidor declara vitória; sem Módulo não entra |
| 5 — Pós-MVP | além da 12 | governança, editor, funções restantes | fora da seção 16 (GDD item 19) |

**Escopo MVP (GDD §16.3/§16.4):** 2–4 jogadores, 3 ilhas + Ilha Central em *greyboxing*, sem
lobby/pareamento; vitória completa (6 ilhas, 20 s de Propaganda, 3 Nós), elenco e 5v5 pertencem a
este horizonte, não ao ciclo. Durações são hipóteses, não prazo.

**"100% do projeto" tem três camadas documentais, não um percentual:** (1) ciclo 80/20 = CE-1..CE-5,
28 subcritérios, 6 Tasks; (2) MVP = 18 critérios de aceitação do GDD §16.4, fechados quando todos
passarem em duas partidas consecutivas, uma com cliente modificado (GDD:1567); (3) pós-MVP = Fase 5
(governança, editor, funções restantes), fora da seção 16. Hoje: camada (1) planejada, não executada;
(2) e (3) não iniciadas. Detalhamento em `OBJETIVO.md` §§2A–2B.

## 5. [NV] — não verificado

- Publicação/release/deploy externo e qualquer estado de produção: **[NV]** — não autorizado.
- Postgres/Coolify remotos, IAMAdapter/IAMInterface e `pgmq`: **[NV]** — credenciais/instância fora do alvo.
- Comandos "a validar" da spec (Godot no PATH, suíte headless, `make check`, `make check-compat`):
  **[NV]** nesta sessão; a existência remota do Godot 4.7.2 está confirmada por fonte primária.
- Determinismo bit-a-bit do Núcleo GDScript, registro IANA da porta 7777 e federação em sentido
  forte: **[NV]** — ver `docs/research/`.
- Gates de produção da factory no alvo: não materializados; ausência não é RED.

## 6. Estado verificado — 2026-11-21T13:19Z (Blocos 0–4)

- Âncora: HEAD `3709c56` em `dev`, igual a `refs/heads/dev` remoto (`git ls-remote`, 13:19Z); `feature.json` → `specs/030-demo-deterministica-headless`. Factory fonte `b1deab1`, nada transplantado.
- Estoque real: 30 specs / 233 assessments / 30 validations / 30 releases / 0 bugs / 13 research; 5/5 gates SDD PASS às 13:18:27Z. Seção 5 acima conserva os **[NV]** históricos de 20/11.
- Remoto verificado ao vivo às 13:19:29Z: 0 PRs abertos; 5 runs `deterministic-gates` `success`; `dev` protegido com check obrigatório `gates` (sem approver, `enforce_admins=false`); `main` sem proteção. **Risco de confiança**: o job `rust` (`./verify.sh`) não é check obrigatório na proteção — endurecê-lo exige mutação remota e autorização explícita da SE, fora deste ciclo.
- **[NV]** remanescentes: `./verify.sh` completo; confirmação visual da SE; deploy e publicação; assinatura GPG.

### Atualização verificada — 2026-11-21T20:25Z

- O risco registrado na seção 6 ("job `rust` não obrigatório") foi **corrigido**: required checks de `dev` = `["gates","rust"]`, confirmado por `gh api` PUT + GET. `enforce_admins=false`, `reviews=null`, `restrictions=null` preservados.
- PR #36 mesclado em `dev` (`ce2d1f0`) com CI verde; 0 PRs abertos. Ciclo 032 registra o endurecimento como mudança de governança remota autorizada.
- `[NV]/DEFER`: confirmação visual da SE, deploy/publicação, assinatura GPG.

### Atualização verificada — 2026-11-22T18:34Z

- Âncora: HEAD `f9d87e0` em `dev`; `feature.json → specs/034-gate-manifesto-cli`; ciclo 035 em curso. A seção 6 (30 specs, `dev=3709c56`, "job `rust` não obrigatório") permanece como histórico de 21/11.
- Estoque real (auditoria 18:31–18:34Z): 34 specs / 237 assessments / 34 validations / 34 releases / 0 bugs / 13 research. Gates (18:32:17Z, observado pela orquestração via `gh`/`git`): `verify.sh` 41/41, SDD 34/0, DAG 314/17/0, task-id 0, converge 17/17, hub-docs 0 erros.
- Proteção de `dev` = `contexts=["gates","rust"]`, `strict=true`, `enforce_admins=false`: risco de confiança da seção 6 corrigido em 21/11. PR #40 `dev→main` `OPEN` na fila do SWE; **sem merge de `main`** (deploy é competência do SWE).
- **[NV]** remanescentes: estado remoto posterior, deploy/publicação, confirmação visual da SE, assinatura GPG.

### Atualização verificada — 2026-11-22T23:01Z (ciclo 037)

- Âncora: HEAD local `efb4d4b` (confirmado por `git rev-parse --short HEAD`); `feature.json → specs/037-estado-escalonamento-45`. Remoto **[NV]** nesta sessão (dados da orquestração 22:58–23:01Z): `dev=efb4d4b`, `main=c126b6e`, PR #44 mesclado em `dev`, PR #45 `dev→main` OPEN/green/CR sem blockers na fila SWE — sem merge/deploy; proteção de `dev` = `contexts=["gates","rust"]`, `strict=true`, `enforce_admins=false`; `main` desprotegido (observação anterior, **[NV]**).
- Estoque vigente (snapshot 22:58–23:01Z): 36 specs / 238 assessments / 36 validations / 36 releases / 0 bugs / 13 research; 34/237/34/34/0/13 de 18:34Z permanece como histórico. Verificação local read-only no worktree: specs versionadas 36 (37 diretórios com `037` não versionado), validations 36, releases 36, bugs 0, research 10 (13 na árvore canônica); contagem de 238 assessments não reproduzida localmente (**[NV]**).
- Gate: `./verify.sh` ALL STAGES OK, **41/41 testes** (observado pela orquestração, não reexecutado — **[NV]**). Deriva corrigida: a seção 6 e a nota de 18:34Z (34 specs, 34/34, `dev=3709c56`, "rust não obrigatório") ficam como histórico; o risco de confiança foi corrigido em 21/11.
- Ciclo 036: HUD com acento e fonte Noto OFL 1.1, blocker `BEVY_ASSET_ROOT` resolvido, evidência 11/11. Decisão de governança: divergência v4.6 vs constituição v1.0.0 registrada, sem adotar R6; nada importado da factory. **[NV]**: CI/merge posteriores, deploy, confirmação visual da SE, assinatura GPG; nenhum gate/build/formatter/commit/push/PR nesta nota.

### Correção de estoque — 2026-11-22T23:05Z

- O estoque de 23:01Z (36 specs / 238 assessments / 36 validations / 36 releases / 0 bugs / 13 research) é histórico; vigente é o verificado às 23:05:02Z no worktree a partir da árvore versionada de `origin/dev`: **37 specs / 65 assessments / 37 validations / 37 releases / 0 bugs / 10 research**.
- Comandos e saídas: `find specs -maxdepth 1 -type d -name '[0-9][0-9][0-9]-*' | wc -l` → 37; `find .specify/assessments -mindepth 1 -maxdepth 1 -type d | wc -l` → 65; `find .specify/validations -mindepth 1 -maxdepth 1 -type d | wc -l` → 37; `find .specify/releases -mindepth 1 -maxdepth 1 -type d | wc -l` → 37; `find docs/research -maxdepth 1 -type f -name '*.md' | wc -l` → 10; `find docs/research -type f -name '*.md' | wc -l` → 10.
- O checkout compartilhado apresenta 238 assessments / 13 research por incluir 173 diretórios untracked históricos (`ast-*`, `cor-*`, `doc-*`, `srv-*`, `ev-*`) e 3 notas untracked em `docs/research/2026-11-20-*`, preservados por decisão registrada — não versionados. O 36 de specs/validations/releases era o pré-037; agora são 37 cada.
- Âncora `efb4d4b`/`c126b6e`, PR #45 escalado, 41/41 testes e R6/divergência de governança inalterados. Correção append-only; nenhum gate/build/formatter/commit/push/PR executado; 0 linhas removidas.

### Atualização verificada — 2026-11-23T01:31Z (ciclo 038 — constituição v1.2.2)

- **Âncora**: `git rev-parse --short HEAD` = `e010363`, branch local `038-estado-pos-v122`; `git ls-remote --heads origin dev main` → `dev=e0103635`, `main=8b0b044a`. `gh pr view 48` → `dev→main` **OPEN/MERGEABLE**, `gates` e `rust` `SUCCESS` (green), na fila do SWE — **sem merge de `main`/deploy** (deploy é competência do SWE). Constituição v1.2.2 (`.specify/memory/constitution.md:84`: Version 1.2.2, Ratified 2026-11-19, Last Amended 2026-11-23). PRs #47 (00:58:13Z), #49 (01:07:04Z), #50 (01:14:07Z) mesclados em 23/11. Notas 18:34Z/23:01Z/23:05Z acima (`f9d87e0`, `efb4d4b`/`c126b6e`, PR #40/#45) permanecem como histórico; as fases e a fila 80/20 não mudam.
- **Estoque versionado (árvore de `origin/dev`, verificado neste worktree às 01:31Z)**: **37 specs / 65 assessments / 37 validations / 37 releases / 0 bugs / 10 research**. Comandos/saídas: `find specs -maxdepth 1 -type d -name '[0-9][0-9][0-9]-*' | wc -l` → 38 no disco (37 versionados + `038-estado-pos-v122` untracked); `git ls-files 'specs/*/spec.md'` → 37; `find .specify/assessments -mindepth 1 -maxdepth 1 -type d | wc -l` → 65; `find .specify/validations -mindepth 1 -maxdepth 1 -type d | wc -l` → 37; `find .specify/releases -mindepth 1 -maxdepth 1 -type d | wc -l` → 37; bugs 0; `find docs/research -maxdepth 1 -type f -name '*.md' | wc -l` → 10 (e `-type f` recursivo → 10). O 238/13 do checkout compartilhado (173 diretórios untracked históricos + 3 notas `2026-11-20-*`) fica como histórico; neste worktree a árvore versionada dá 65/10.
- **Checks**: `./verify.sh` ALL STAGES OK, **41/41 testes**; 5 gates Python PASS — observados pela orquestração, **não reexecutados nesta correção [NV]**; `python3 -m pytest -q` → exit 5 = N/A, não RED. A proteção de `dev` = `contexts=["gates","rust"]`, `strict=true`, `enforce_admins=false` (21/11) permanece; `main` desprotegido (observação anterior, **[NV]**).
- **Dívidas/sequenciamento**: deploy/publicação inexistente no workflow (`.github/workflows/ci-gates.yml` só `gates`+`rust`) — fora da sequência executável; **R6** ratificada em v1.2.0 mas **fora da sequência executável** até `resolve-tasks.sh` e o adaptador de issues existirem (`constitution.md:50`) — se um ciclo a invocar, **PARE e escale ao SWE**; nunca simular issue. `AGENTS.md` ainda diz "34 automated tests"/"29 numbered NNN-slug dirs" no corpo — corrigido por nota datada no fim, sem apagar histórico. Factory `b1deab1` é fonte, não autoridade; nada importado.
- **[NV] nesta sessão**: reexecução local dos gates, CI/merge posteriores, disparo de deploy, confirmação visual da SE, assinatura GPG. Nenhum gate/build/formatter/commit/push/PR executado; correção append-only, 0 linhas removidas.
