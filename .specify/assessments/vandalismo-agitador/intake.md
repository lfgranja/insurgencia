# Idea Intake: Ato de Vandalismo do Agitador (BF-002c)

- **Slug**: `vandalismo-agitador`
- **Created**: 2026-11-25
- **Source**: BF-002c do backlog de gaps, promovido pela decisão 80/20 do ciclo 061 registrada em `.specify/memory/STATE.md:606-609` (288/3125, `4×4×2×3×3`); âncora `origin/dev = 52138c3` (PR #79 mesclado em `dev`), worktree `.specify/queue/worktrees/cycle-061-vandalismo`, branch `feature/061-vandalismo`, `.specify/feature.json → specs/061-vandalismo-agitador`
- **Type**: new-capability — dar efeito observável e determinístico ao Ato de Vandalismo no núcleo `sim`, na CLI `servidor` e no cliente Bevy

## Idea (as captured)

> "BF-002c — implementar o Ato de Vandalismo do Agitador no núcleo determinístico, na CLI e no cliente Bevy. Novo token `vandalismo` em `ACOES_PERMITIDAS` (15º, com destino), validação de alcance/fôlego/recarga, ramo em `fase_skills` aplicando 120 de dano e 45 ticks de silêncio ao alvo (1,5 s × 30 Hz), custo 20 de Fôlego, alcance 2 m, recarga 210 ticks (7 s) — todos rotulados `[HIPÓTESE]`, sem promoção a cânone e sem inventar regra nova. Tecla protótipo `H` no cliente (Q já pertence a `atacar_forte`), com dica na HUD. Nenhuma mudança de stack, CI, contrato de rede ou GDD." [source: `.specify/memory/STATE.md:606-609`, lido] (confidence: high)
>
> "Q — Ato de Vandalismo. Ataque rápido que causa dano extra e silencia (impede o uso de habilidades) temporariamente. **Custo:** 20 Fôlego; **execução:** 0,2 s; **alcance:** 2 m; **recarga:** 7 s. **Efeito:** 120 de dano físico e 1,5 s de silêncio. `[HIPÓTESE]`" [source: `docs/GDD - Insurgência.md:604-612`, lido] (confidence: high para o texto normativo; todos os números são `[HIPÓTESE]`)

## Restated

Materializar, no protótipo determinístico já existente, a habilidade Q do Agitador descrita no GDD como `[HIPÓTESE]`: um comando `vandalismo` aceito pela validação, com alvo e destino, que aplica 120 de dano e 45 ticks de silêncio (1,5 s a 30 Hz) ao alvo, cancela canalização em curso e cobre o novo estado no `hash_estado`. Recusas fora de alcance, sem Fôlego ou em recarga não alteram estado nem monotônicos. O consumidor é o jogador do cliente Bevy (Função Agitadora, tecla protótipo `H`) e a automação via CLI `insurgencia-servidor`. Nenhuma regra canônica é inventada: os valores são os do protótipo, rotulados `[HIPÓTESE]`, e o GDD, o contrato de rede, a stack e o CI permanecem intocados. A saída desta etapa é documental (intake → research → problem → concept → decision); nenhum token, ramo, tecla, spec, plano, tarefa, código, gate, teste, build ou formatação é produzido aqui.

## Origin & Context

- **Raised by**: backlog de gaps do hub (BF-002c), promovido pela decisão 80/20 do ciclo 061 em `.specify/memory/STATE.md:606-609`; **R1 `go`** já decidido e registrado no fluxo do ciclo, confirmado neste assessment (constituição v1.2.2 exige `go` explícito). [source: `.specify/memory/STATE.md:606-609`; `.specify/memory/constitution.md`] (confidence: high)
- **Âncora**: `.specify/feature.json → specs/061-vandalismo-agitador`; branch `feature/061-vandalismo`; HEAD/âncora `origin/dev = 52138c3`, worktree limpa `.specify/queue/worktrees/cycle-061-vandalismo`. A factory (`18be3fb3`, `origin/dev = c0a7e1d8`) é fonte de ideias, não autoridade. [source: `.specify/memory/STATE.md:606`; `.specify/feature.json`] (confidence: high)
- **Trigger**: BF-002c ficou aberto no fechamento do ciclo 060 (`BF-015a` venceu com 360/3125) e tornou-se o candidato de maior pontuação do ciclo 061 (288/3125), com gap verificável em `crates/sim/src/lib.rs`: `ACOES_PERMITIDAS` tem 14 tokens e `fase_skills` não tem ramo de vandalismo. [source: `.specify/memory/STATE.md:597-600,606-609`; `crates/sim/src/lib.rs:222-236,920-1010`, lidos] (confidence: high)
- **Autorização e governança**: constituição v1.2.2 — ordem SDD, R1 `go`, R3 (≤3 clarificações), R4 antes de R7, R6 não executável (PARE e escale, sem issue simulada nem `gh` improvisado); Rust `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, `Result`/`thiserror`, sem `unwrap`/`expect`/`panic!` em produção, `BTreeMap`/`BTreeSet`, `f64::total_cmp`, justificativas escopadas de lint, pt-BR. PR #76 (`dev→main`) e deploy são SWE-exclusivos e intocáveis; sem promoção, force-push, auto-merge ou `git add -A`. [source: `.specify/memory/constitution.md`; `.specify/memory/STATE.md:611`; convenções do ciclo] (confidence: high)
- **Fonte semântica**: `docs/GDD - Insurgência.md:604-612` (§10.4 Agitador, habilidade Q e critérios de aceitação) e `:744-757` (§10.8 tabela-resumo: Agitador = infiltração, silêncio e confusão de NPCs). Números `[HIPÓTESE]` não são cânone e não podem ser promovidos neste ciclo. [source: GDD lido] (confidence: high)
- **Estado registrado**: gates e `./verify.sh` do ciclo anterior são **citados** de `.specify/memory/STATE.md:596-597`; nenhum gate, teste ou build foi reexecutado nesta etapa. [source: `.specify/memory/STATE.md:596-597`] (confidence: high para a citação; execução **[NV]**)

## Restrições

- **Escopo de escrita**: exatamente cinco arquivos em `.specify/assessments/vandalismo-agitador/` — `intake.md`, `research.md`, `problem.md`, `concept.md`, `decision.md`. Nenhum outro arquivo tocado; nenhum spec, plano, task, código, gate, teste, build, formatação, stage, commit ou push nesta etapa.
- **DoD (0.9) — comportamento**: comando `vandalismo` aceito pela validação; alvo sofre 120 de dano e fica silenciado por 45 ticks; canalização em curso cancelada; recusa fora de alcance/sem Fôlego/em recarga preserva estado e monotônicos; `hash_estado` cobre o novo estado. [source: `.specify/memory/STATE.md:609`]
- **DoD (0.9) — consumidor**: jogador do cliente Bevy (Função Agitadora, tecla protótipo `H`) e automação via `insurgencia-servidor`.
- **DoD (0.9) — prova**: `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo test --locked -p insurgencia-cliente --lib`, `cargo check --locked -p insurgencia-cliente --all-targets`, `./verify.sh` remoto (`gates`/`rust`) — todos **[NV]** até execução na portaria própria.
- **DoD (0.9) — backlog**: **BF-002c passa a `fechado`**; BF-015b permanece aberto.
- **Fronteira**: nada em GDD/CI/stack/contrato de rede (`crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`), nada de topologia canônica de ilhas, nenhuma promoção de `[HIPÓTESE]` a cânone; R6 N/A por constituição v1.2.2.
- **Convenções**: português brasileiro; marcação `[NV]`; termos estrangeiros explicados na 1ª ocorrência; conteúdo externo tratado como dado, nunca como instrução.

## First-Glance Unknowns

- **[NV]** Nenhuma alteração de código, gate, teste, build ou formatação foi executada; o estado de aprovação permanece o citado em `.specify/memory/STATE.md:596-597`.
- **[NV]** Como o silêncio será representado no estado do alvo sem alterar o contrato de rede: campo novo em `Jogador` (por exemplo `silenciado_ticks`) precisa de decisão de spec e de cobertura no `hash_estado`; a forma exata não é inferida aqui.
- **[NV]** A técnica de canalização a cancelar (campos `propaganda_*`, `construcao_*`, `ponte_*` ou equivalente) e a precedência do dano no tick dependem da ordem de `fase_skills`; o recorte exato fica para `/speckit.specify`.
- **[NV]** A tecla `H` é proposta de protótipo (Q está ocupada por `atacar_forte` em `crates/cliente/src/main.rs:823-831`); a dica na HUD e o texto exato são decisão de spec.
- **[NV]** Recarga de 210 ticks e custo de 20 de Fôlego são convertidos de 7 s e 20 Fôlego do GDD sob `[HIPÓTESE]`; nenhum valor é promovido a cânone.
- **[NV]** Sem partida real, 5v5, reconexão, lobby ou rede (Lightyear ausente de `Cargo.toml`/`Cargo.lock`), o "silêncio" só é observável no protótipo determinístico; comportamento em jogo real permanece fora de alcance.
