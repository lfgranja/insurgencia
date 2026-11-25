# Intake — Coleta Eficiente (BF-002k)

**Data**: 2026-11-25 · **Origem**: BF-002k (`Coleta Eficiente`) · **Slug**: `coleta-eficiente` · **Feature**: `specs/062-coleta-eficiente` · **Âncora**: `origin/dev = d3be560` (PR #80 mesclado em `dev`) · **R1**: `go` (registrado em `.specify/memory/STATE.md`, nota `2026-11-25T06:40:58Z`).

## Ideia
Dar **efeito observável** ao ativo `Q — Coleta Eficiente` do Camponês no núcleo determinístico, na automação via CLI e no cliente Bevy. Hoje `crates/sim/src/lib.rs` já possui a canalização de coleta (`coleta_ticks`, `TICKS_COLETA = 180`, `aplicar_coleta`, `avancar_coletas_automaticas`), mas a whitelist `ACOES_PERMITIDAS` tem 15 tokens e **não contém** `coleta_eficiente`: o ativo existe apenas no GDD (`docs/GDD - Insurgência.md:646`) e não pode ser solicitado, validado ou observado.

O recorte do ciclo 062 é a **parte ativa**: comando `coleta_eficiente` (novo token, whitelist com 16 tokens, com destino, nível 1), custo 15 Fôlego, buff de 150 ticks (5 s × 30 Hz) que dobra a taxa de coleta (conclusão em 90 ticks em vez de 180) e recarga de 300 ticks (10 s), **sem acumulação**. Destino válido é a própria posição do jogador; a recusa preserva Fôlego, IDs monotonicamente crescentes e hash. Novos campos entram em `serializar_jogadores`/`hash_estado` e decrescem uma vez por tick. Consumidores: jogador da simulação, automação via `insurgencia-servidor` e tecla protótipo `K` no cliente com HUD derivada do espelho.

Os números vêm de `GDD:646` e permanecem **`[HIPÓTESE]`** — não são cânone. A passiva de Ouro/XP fica **fora de escopo** por não haver primitiva de Ouro/XP no núcleo.

## 80/20 (decisão de escopo)
Critério `impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹`, denominador 3125; filtro eliminatório exige comportamento observável e consumidor fora dos artefatos do ciclo:

| Opção | Cálculo | Resultado |
|---|---|---|
| BF-002k `Coleta Eficiente` (recorte ativo, escolhida) | 4 × 4 × 4 × 4 × 4 | **1024/3125** |
| BF-002n | — | 216/3125 |
| BF-015b | — | 144/3125 — inelegível (ordem/topologia canônica indefinida em `GDD:376,381,1502`) |
| BF-003b | — | 100/3125 — inelegível (topologia) |
| BF-014 | — | 250/3125 — diagnóstico |

BF-002d..j/l..t dependem de primitivas ausentes (invisibilidade, torres, canalização genérica, Ouro, topologia) ou de decisão canônica; BF-001/006/007/009/010 são autorização/capacidade SWE; BF-004/005 são execução externa. Fonte: `.specify/memory/STATE.md`, nota `2026-11-25T06:40:58Z`.

## Evidências de entrada (lidas nesta sessão; nada reexecutado)
- [source: `docs/GDD - Insurgência.md:646`, lido] `Q — Coleta Eficiente`: ativa 15 Fôlego, execução 0,2 s, alcance 2 m, recarga 10 s, "ativa dobra a taxa de coleta por 5 s", critério "não se acumula com outra ativação"; passiva +25% Ouro/+25% XP em pontos neutros; todo o bloco marcado `[HIPÓTESE]`.
- [source: `crates/sim/src/lib.rs`, lido] `TICKS_COLETA: i64 = 180` (linha 18), `coleta_ticks` em `Jogador` (linha 354), `ACOES_PERMITIDAS` com 15 tokens (linhas 233–249) sem `coleta_eficiente`, `aplicar_coleta` (1559) e `avancar_coletas_automaticas` (1585), `concluir_coleta` (1611), `serializar_jogadores` (2141) e `hash_estado` (2348).
- [source: `.specify/memory/STATE.md:625-635`, lido] nota `2026-11-25T06:40:58Z`: escolha BF-002k, score 1024/3125, DoD (0.9) e governança/R6.
- [source: git/worktree] worktree isolado `.specify/queue/worktrees/cycle-062-coleta-eficiente`, branch `feature/062-coleta-eficiente`, HEAD/âncora `d3be560`; `.specify/feature.json → specs/062-coleta-eficiente`. Único PR aberto #76 (`dev→main`), SWE-exclusivo e intocável.

## Restrição de execução
Este agente escreve **somente** os cinco artefatos SDD desta pasta (`intake.md`, `research.md`, `problem.md`, `concept.md`, `decision.md`). Não edita código, GDD, ADR, CI, stack, protocolo, `.specify/memory/STATE.md`, `converge.md` de outros ciclos nem `.specify/feature.json`; não executa gates, build, formatador, testes, commit, push, PR, merge ou deploy. R6 não é executada e nenhuma issue é simulada. Tudo o que depende de execução permanece **[NV]**.
