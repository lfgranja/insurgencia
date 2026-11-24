# Intake — Escudo de Milícia (W do Guerreiro)

**Data**: 2026-11-23T22:00Z · **Slug**: `escudo-milicia` · **Sessão**: `20261123T215500Z-escudo-milicia`, `session_title=insurgencia - escudo de milicia`.

## Ideia (como capturada)

> Capturar, sem executar nada neste assessment, a fatia 80/20 do ciclo 055: tornar **observável no cliente standalone e verificável no Núcleo determinístico** o **Escudo de Milícia** (W do Guerreiro), previsto em `docs/GDD - Insurgência.md` §10.6:687-692 como `[HIPÓTESE]`: 25 Fôlego, 0,3 s de execução, 3 m de alcance, si mesmo ou um aliado, 250 de absorção por 5 s, recarga 14 s e +15% de resistência a dano contínuo. A decisão 80/20 das 22:00Z recorta: ação `escudo` nível 2, custo 25 Fôlego, alcance 3 m, absorção de até 250 por 150 ticks (5 s a `PASSO_SEGUNDOS = 1/30`), recarga 420 ticks (14 s), um alvo, sem acumulação; quatro campos em `Jogador`, rota única `aplicar_dano_a_jogador` para PvP e dano de PNJ, hash cobrindo os campos e linha `Escudo: N (ticks)` na HUD.

Fonte colada (lida, não reexecutada): `.specify/memory/STATE.md`, entrada `2026-11-23T22:00Z — decisão 80/20 e DoD do ciclo 055 (antes da implementação)`. Nenhuma URL foi buscada; nenhuma fonte externa foi usada. Todo número citado é `[HIPÓTESE]`/`escolha de protótipo` do GDD e da decisão, nunca cânone.

## Restated

Propõe-se acrescentar ao protótipo determinístico uma habilidade de defesa que absorve dano por tempo limitado e protege um único alvo (o próprio Guerreiro ou um aliado), com recarga e sem acumulação. O ganho esperado é duplo: o jogador do cliente passa a ver o efeito na HUD e a sobreviver a PvP/PNJ de forma observável, e o Núcleo passa a ter uma única rota de dano ao jogador, com absorção auditável por hash.

## Contexto do ciclo (observado no contrato da sessão; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/escudo-055/`, branch `feature/055-escudo-milicia`, base `3f3b01c`; sessão `20261123T215500Z-escudo-milicia`. Alvo de especificação `specs/055-escudo-milicia/`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012. Assess `.specify/assessments/escudo-milicia/`, validação `.specify/validations/escudo-milicia/`, release `.specify/releases/escudo-milicia/`.
- Âncora GDD §10.6:687-692 (W do Guerreiro, `[HIPÓTESE]`): 25 Fôlego, 3 m, 250 de absorção por 5 s (150 ticks), recarga 14 s (420 ticks), um alvo. `RESISTENCIA_CONTINUA_ESCUDO_PERC = 15` fica **reservada/documentada**: o Núcleo não tem dano contínuo e o +15% contra gás lacrimogêneo está fora da DoD.
- Estado verificado no checkout no momento da leitura: `crates/sim/src/lib.rs` não tem `escudo` em `ACOES_PERMITIDAS`, nem constantes de escudo, nem `escudo_pv`/`escudo_ticks`/`escudo_recarga_ticks`/`escudo_alvo` em `Jogador`, nem `aplicar_dano_a_jogador`; o dano ao jogador tem duas rotas diretas (`aplicar_ataque` no PvP e `fase_ataque_pnjs`), e `serializar_jogadores`/`hash_estado` não veem campos de escudo. No cliente, `WASD`/setas movem, `T` ataca PNJ, `E` analisa, `Q` ataca; a tecla `V` está livre e a HUD não tem linha `Escudo:`.
- Constituição v1.2.2: R1 `go`; R3 = 0 clarificações; R4 antes de R7 `1.0.0-local`; `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável sem `resolve-tasks.sh`/adaptador. Promoção `dev→main` e deploy são SWE-exclusivos.
- Baseline: **136 testes** determinísticos; o corte prevê **5 novos** nomeados (`escudo_absorve_ate_250_dano`, `escudo_absorve_dano_de_pnj`, `escudo_recarga_420_ticks`, `escudo_expira_apos_150_ticks`, `hash_estado_cobre_escudo`) → **141**.

## Scorecard único do ciclo

| Candidato | I × C × Fch × E⁻¹ × R⁻¹ ÷ 3125 | Resultado | Consumidor |
|:--|:--|:--|:--|
| `escudo-milicia` (BF-002a) | 4×4×5×3×4 = 960/3125 | **960/3125** | jogador humano no cliente + sim determinístico |
| Mapa/rotas de PVE (BF-003) | 4×3×3×2×3 = 216/3125 | 216/3125 | jogador/servidor |
| Rede real (BF-001) | 5×2×5×1×1 = 50/3125 — **inelegível** (autorização ADR-004) | 50/3125 | SWE decide |
| Rastreabilidade 053/054 (BF-008) | 2×5×0×5×5 = 0 — **inelegível** (já fechado) | 0 | nenhum |

Critério normativo: `impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹`, escala 1–5, denominador **5⁵ = 3125**. Filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo. Impacto **4** (defesa jogável com regra no GDD), confiança **4** (valores e rotas lidas, efeito renderizado **[NV]**), fechamento **5** (dois consumidores nomeados), esforço⁻¹ **3** (sim + cliente + 5 testes, sem stack nova), risco⁻¹ **4** (rota única nova no Núcleo, mas sem contrato/protocolo/Cargo/rede). Este é o **único** scorecard do assessment: `research.md`, `shape.md` e `decision.md` repetem os mesmos valores, sem pontuação paralela.

## Incógnitas de primeira vista

- **Nenhuma clarificação bloqueante (R3 = 0)**: o contrato fixa constantes, derivação em ticks, alvo único, não-acumulação, tecla e linha da HUD. Não há `[NEEDS CLARIFICATION]` de produto a responder neste assessment.
- Incógnitas operacionais, não de produto, ficam **[NV]** até verificação: efeito renderizado da HUD com `--minimizado`; nomes definitivos de T001..T012 em `specs/055-escudo-milicia/tasks.md`; conformidade de R4 e R7 com gates não executados aqui.

## Restrição de execução

Este agente escreve **somente** os 5 artefatos em `.specify/assessments/escudo-milicia/`. Não executa gates, build, testes, formatadores, `./verify.sh`, comandos de CI, git remoto ou GitHub; não faz commit, push ou PR; não toca `crates/sim`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `STATE.md`, `AGENTS.md`, GDD, specs, validações, releases, `.specify/queue/concurrency.jsonl` nem qualquer código-fonte. Tudo o que depende de execução permanece **[NV]**. **R6 não é invocada** — N/A por constituição v1.2.2: PARE se invocada, nunca issue simulada, nunca `gh` improvisado. Promoção `dev→main` e deploy são SWE-exclusivos; este ciclo termina no escalonamento, não em merge em `main`.
