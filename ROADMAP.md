# ROADMAP.md — Insurgência

> **Nota datada (2026-11-19).** Baseline em `2026-11-19T21:04Z`: HEAD `6e25758`, `specs_dirs=0`,
> `feature.json` AUSENTE, 4 checkers SDD PASS com 0 specs, `assessments=175` (171 intakes + 171
> decisões), `bugs/validations/releases=0`, `docs/**/*.md=28`. Detalhes: `docs/PROGRESSO.md`.
> **"80/20" ordena a fila**, nunca é percentual de conclusão. Toda afirmação remota, de CI, merge
> ou publicação é **[NV]** até confirmação ao vivo.

## 1. Princípios de sequenciamento

1. Gate determinístico local é verdade; saída de agente é informativa. Transição exige comando
   executado + saída colada + data/hora; PASS em baseline vazia não é cobertura.
2. Spec e plano do ciclo **já existem**; este roadmap só os ordena. `[x]` = revisão de requisitos,
   não implementação — não move fase. Cada fase fecha com evidência (fonte, comando, resultado,
   confiança, não verificado), nunca com "passou" sem comando no build atual.
3. Factory (`b1deab1`) é fonte, não autoridade; gate do alvo = `scripts/ci/`. Sem merge,
   auto-merge, force-push ou publicação — **[NV]**.

## 2. Fases do ciclo 80/20 (spec CE-1..CE-5 / plano Tasks 1–6)

| Fase | Entrega | Âncora | Marco verificável |
|---|---|---|---|
| **F0 — Abertura** (ranking #1) | ADR de exceção de linguagem (GDScript) + resolução N3 (MIT/AGPL); decidir no SDD | `docs/governance/80-20-ranking-2026-11-19.md` #1; `AVALIACAO-STACK-MDMV.md` desvios #1/#3/#10 | `decision.md` com `go` (R1), 7 desvios endereçados. Entregável **documental**, não código. `feature.json`/spec dir (D1) ausentes: abrir antes de reivindicar cobertura |
| **F1 — Núcleo e contrato** | Estrutura canônica (`nucleo/`, `modulos/`, `manifesto.json`), `module.json` (11 campos, 10 obrigatórios), Módulo de exemplo | CE-1 / Task 1 | `OK_CAMINHOS`; 11/11 campos; 3/3 recusas de tipo; `git diff --stat -- nucleo/` vazio na troca de Módulo |
| **F2 — Servidor e validação** | Servidor *headless*, *handshake* com valores padrão, validação autoritativa (sequência, `seq`, janela) | CE-2 / Tasks 2–3 | 2/2 `join_accept` em `127.0.0.1:7777` ≤ 2 s; 3/3 estados prontos recusados; 20/20 *snapshots* com 5 campos — registrar comando e saída; hoje **não verificado** |
| **F3 — Ilha Meio** | Sincronia de 2 placas em 5 s; único estado persistido `ilha_concluida`; gás 8/amostra (24 em 3 s) | CE-3 / Task 4 | 2/2 capturas em 4,9 s; 2/2 falhas em 5,3 s; 0 capturas por 1 jogador; 1 sinal por ilha; regra de produção 3/5 s permanece no Módulo base |
| **F4 — Manifesto e gate local** | `manifesto.json` (6 campos), SHA-256 de 32 bytes, 4 códigos de recusa | CE-4 / Task 5 | `make check-compat`: 4/4 códigos exatos em 12 execuções; 0 aceitações indevidas; sem lobby/worker e sem release (doc-26 DEFER) |
| **F5 — Evidência e reavaliação** | `docs/progress/evidence.csv` (9 colunas), arquivo datado, `make check` | CE-5 / Task 6 | 5/5 critérios com ≥ 1 linha; 0 `PASS` com `unverified` não vazio; **reavaliar** Ilha Central/elenco/5v5 com base em capacidade e retorno dos mantenedores |

**Portas SDD:** constituição → intake → research → define → shape → decide → specify → clarify →
plan → checklist → analyze → tasks → implement → converge → validate (R4) → release (R7). R3 ≤ 3
esclarecimentos, um por vez; R4 antes de R7; R7 exige SemVer + changelog com IDs FR/T/bug + rollback;
`converge.md` append-only.

## 3. Fila após o ciclo (candidatos 3–6 — só gatilhos)

| # | Candidato | Score | Gatilho | Status |
|---:|---|---:|---|---|
| 3 | Godot 4.7.2 + suíte headless | 10,0 | artefato confirmado; F1–F2 escritos | aberto; comandos "a validar"; **[NV]** |
| 4 | Gate CI remoto (GitHub Actions) | 16,0 | autorização remota + F5 evidenciado | aberto; CI **[NV]** |
| 5 | Adaptador PostgreSQL + `sqlx` | 8,0 | runtime + medição de capacidade | aberto; sem runtime |
| 6 | OTLP/Collector/VM+Grafana | 4,5 | runtime e telemetria justificados | aberto; prematuro |

Nenhum item 3–6 é "em andamento" ou "quase pronto"; telemetria, CI, deploy e publicação sem
verificação ao vivo — **[NV]**.

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

## 5. [NV] — não verificado

- `origin` GitHub: commits, PRs, proteção, tags; CI, merge, release, publicação, deploy — **[NV]**.
- Gates de produção da factory no alvo: não materializados; ausência não é RED — **[NV]**.
- Comandos "a validar" da spec (Godot 4.7.2, `make check`, suíte headless) — **[NV]**.
