# ROADMAP.md — Insurgência

> **Nota datada (2026-11-20T01:48Z).** Baseline: HEAD local `2af7537`; `origin` público, 12 PRs
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
