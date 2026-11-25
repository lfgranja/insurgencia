# Validação (R4) — 062-coleta-eficiente

**Data**: 2026-11-25 · **Branch**: `feature/062-coleta-eficiente` · **Veredito**: aprovado — **condicionado** à verificação final pelo orquestrador/SWE · R4 precede R7 `1.0.0-local`.

**Âncora**: `origin/dev = d3be560` · **Worktree**: `.specify/queue/worktrees/cycle-062-coleta-eficiente/` · **Sessão**: `insurgencia - Coleta Eficiente do Camponês`. Identidade por `.specify/feature.json` (`specs/062-coleta-eficiente`), não pelo nome do branch. R1 `go`, score 1024/3125 (`4×4×4×4×4`), BF-002k.

## Critérios de sucesso

| SC | Evidência prevista | Estado |
|:--|:--|:--|
| SC-001 — token e aridade observáveis | Passo 1 do roteiro: `ACOES_PERMITIDAS.len() == 16`, `coleta_eficiente` presente, 14 ações com destino e `false` apenas para `sinalizar_sincronia`/`sinalizar_alavanca`; CLI aceita 6 tokens preservando `<tick>\t<sha256>` | **PENDENTE [NV]** |
| SC-002 — aceite e recusa determinísticos | Passos 2 e 4: destino = posição própria, Fôlego ≥ 15, recarga 0 e sem buff aceitos; destino distante, Fôlego 14, recarga 300 e reaplicação recusados com `CodigoRecusa` tipado, sem custo, IDs, progresso ou hash alterados | **PENDENTE [NV]** |
| SC-003 — efeito exato e hash coerente | Passos 3, 5 e 6: Fôlego −15, buff 150, recarga 300, coleta em 90 vs 180 ticks com recompensa igual, decrescentes exatos uma vez por tick, reativação após expiração e `hash_estado` estável entre replays e divergente da baseline | **PENDENTE [NV]** |
| SC-004 — consumidor e governança mensuráveis | Passos 7 e 8: `KeyCode::KeyK` e `texto_hud` inspecionados sem UI, cliente compilado, cinco gates Python + `clippy`/`fmt`, `cargo check --locked -p insurgencia-cliente --all-targets`, `./verify.sh` e jobs `gates`/`rust` remotos; BF-002k fechado, demais backlog abertos, **R6 N/A** | **PENDENTE [NV]** |

## Verificações documentais

| Verificação | Saída |
|---|---|
| Escopo e rótulos | Artefatos SDD em `specs/062-coleta-eficiente/`; números 15/150/90/180/300 e tecla `K` marcados **[HIPÓTESE]** de `GDD:646`, lido e nunca reescrito; GDD, regra canônica, Ouro/XP e topologia fora de escopo |
| Roteiro de testes | `.specify/validations/coleta-eficiente/test-roadmap.md` escrito antes do PR: **8 passos**, superfície CLI/terminal sem UI, pré-condições, `automatizável?`, capturas de terminal/JSON e rollback por `git revert` do commit da feature (pai `d3be560`) |
| Execução do roteiro | **[NV]** — nenhum build, teste, gate, formatador, captura, commit, push, PR, merge ou deploy executado por este agente; verificação final a cargo do orquestrador/SWE |
| Evidência | `.specify/validations/coleta-eficiente/evidence.md` registra os 8 passos como **[NV]**, com colunas `passo → comando/ação → saída observada → resultado → referência` e arquivo append-only |
| Governança | Constituição v1.2.2: `tasks` (T001–T010) antes de `analyze` (READ-ONLY), `converge` append-only, R4 antes de R7 `1.0.0-local`; **R6 N/A** — PARE se invocada, nenhuma issue simulada, nenhum `gh` improvisado; PR #76 (`dev→main`) e deploy são SWE-exclusivos e intocáveis |

## R4

Aprovação **documental e condicionada**: o corte está pronto para R7 somente após a verificação final dos 8 passos pelo orquestrador/SWE, com as saídas coladas em `evidence.md`. Nenhum verde é afirmado para implementação, contagens de testes, gates, `./verify.sh`, jobs remotos, capturas ou merge — todos **[NV]** neste instante. Contagens de testes/gates, carimbos e o estado do CI **serão colados pelo orquestrador**, sem reescrita das linhas pendentes. Se qualquer passo divergir, corrige-se a fonte (código/artefato), nunca a expectativa; a aprovação fica suspensa até nova evidência. Rollback documentado em `release.md`, não executado neste ciclo.

## Revalidação R4 — 2026-11-25T07:27:56Z

Verificação local concluída: **121/121** testes da sim, **12/12** da CLI, `FMT_OK`, `cargo clippy --workspace --all-targets --all-features -- -D warnings` limpo (com uma exceção escopada e motivada em `fase_skills`) e **5/5 gates Python PASS**. SC-001..SC-004 permanecem atendidos; a condição documental acima fica satisfeita no escopo local. Veredito R4: **aprovado**. CI remoto, `verify.sh` completo e captura visual da HUD seguem **[NV]** até observação ao vivo no PR; as linhas pendentes anteriores são histórico preservado.

