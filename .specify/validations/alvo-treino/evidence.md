# Evidência — Roteiro de Testes 048 (execução autônoma)

**Data/hora**: 2026-11-23T08:28–08:29Z. **Sessão**: `20261123T081800Z-alvo-treino` · `insurgencia - alvo de treino jogavel`. **Worktree**: `.specify/queue/worktrees/alvo-treino-048/`, branch `048-alvo-treino`, base `fdc6c08`. **Capturas**: `/tmp/tmp.UPJAT38pUT/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `2ac6c07` (head do PR; base `fdc6c08`), branch correto, **14 artefatos** na fronteira | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | Peer 2 e marcador | `PEER_ALVO_TREINO=4 ocorrências`, `AlvoTreino=2`, `ALVO_TREINO_POSICAO=5` no cliente | PASS | `alvo-marcador.txt` |
| 3 | Posição/destino | `destino: Some(ALVO_TREINO_POSICAO)` presente exatamente 1 vez em `solicitar_ataque` | PASS | `alvo-marcador.txt` |
| 4 | HUD e dica | `Alvo:` e `Aproxime-se do alvo` presentes no HUD | PASS | `alvo-marcador.txt` |
| 5 | Q em alcance/fora | O core valida `AlvoInvalido` fora de 2 m; alvo em `(2,0)` permite dano ao aproximar-se. Cobertura determinística existente na sim (101 testes) | PASS (sim + inspeção) | `fronteira.txt` |
| 6 | Dano visível/piso | `Alvo: {vida}` lido do espelho; vida com piso 0 no core | PASS (inspeção) | `alvo-marcador.txt` |
| 7 | Execução minimizada | `cargo run -q -p insurgencia-cliente -- --minimizado` rodou 12 s sem erro; `exit=124` (timeout esperado) | PASS | `execucao-status.txt`, `execucao-tail.txt` |
| 8 | Fronteira | contratos/sim/Cargo `0`; `allow`/`unwrap`/`expect`/rede `0`; mudança restrita ao cliente e artefatos do ciclo | PASS | `fronteira.txt` |
| 9 | Rust | `verify.sh` ALL STAGES OK; `cargo fmt` aplicado; nenhum `#[allow]` | PASS | `gates.txt` |
| 10 | Governança | v1.2.2, R1/R4/R7, R6 N/A; scorecard **240/625** em 18 arquivos; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

**Proveniência**: a execução do roteiro e dos gates foi feita no head do PR `2ac6c07` (commit revisado pela ACR); `fdc6c08` aparece apenas como base (`baseRefOid`). O carimbo original registrava a base porque o worktree de implementação estava sujo; o head acima corrige a proveniência sem alterar as saídas observadas.

**Gate agregado (08:28Z)**: `./verify.sh` **ALL STAGES OK**, **101 testes run: 101 passed** (nenhum novo teste de sim; mudança apenas no cliente), `VERIFY_EXIT=0`. Um RED de `clippy::cast_precision_loss` (expectativa de lint não cumprida) foi corrigido na fonte antes do verde.

**Limitação [NV]**: a captura visual real do marcador e da queda de `Alvo: {vida}` após `Q` não foi automatizada; a verificação foi estrutural, por compilação, execução minimizada e inspeção de código.
