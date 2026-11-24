# Evidência — Roteiro de Testes 055 (execução autônoma)

**Data/hora**: 2026-11-23T21:57Z → 2026-11-24T00:17Z. **Sessão**: `20261123T215500Z-escudo-milicia` · `insurgencia - escudo de milicia`. **Worktree**: `.specify/queue/worktrees/escudo-055/`, branch `feature/055-escudo-milicia`, base `3f3b01c`. **Capturas**: `/tmp/tmp-escudo-055-saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | branch correto, 14 artefatos, decisão **960/3125**, R1 `go`, DoD registrada | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | Estado e constantes | `escudo_pv`, `escudo_ticks`, `escudo_recarga_ticks`, `escudo_alvo`; constantes 25/250/150/420/3.0/15 (última reservada) | PASS | `estado-escudo.txt` |
| 3 | Ação `escudo` | token no sim, nível 2, validação, cota por atacante, monotônicos só em aceite; ACR recheck OK | PASS | `acao-escudo.txt` |
| 4 | Alvo único/não acumulação | `escudo_alvo: Option<u64>`; reaplicação atribui 250/150/420; limpeza na transição de recarga→0 (T+420) | PASS | `alvo-unico.txt` |
| 5 | Rota única de dano | `aplicar_dano_a_jogador -> i64` com absorção antes de `vida` em PvP e PNJ, piso zero | PASS | `rota-dano.txt` |
| 6 | Decrementos/expiração | duração 150 ticks, recarga 420 ticks, uma vez por tick; `escudo_absorve_dano_de_pnj`, `escudo_expira_apos_150_ticks`, `escudo_recarga_420_ticks` PASS | PASS | `decrements-escudo.txt`, `roadmap.txt` |
| 7 | Hash/replay | `hash_estado_cobre_escudo`: 1 passed; quatro campos no segmento de jogadores, segmentos antigos preservados | PASS | `hash-escudo.txt`, `roadmap.txt` |
| 8 | HUD `Escudo: {pv} ({ticks})` | `texto_hud` lê `escudo_pv`/`escudo_ticks`; 98 linhas (<100); renderização visual real **[NV]** | PASS (estrutural) | `hud-escudo.txt` |
| 9 | Tecla `V` e fronteira | `just_pressed(KeyCode::KeyV)`, helper `destino_escudo` escolhe aliado ativo ≤3 m (menor `peer_id`), self como fallback, sem comando sem alvo; W/E/T preservados; sem contrato/protocolo/Cargo/rede | PASS | `tecla-v-fronteira.txt` |
| 10 | Governança/R6/rollback | R1/R4/R7 `1.0.0-local`, R6 **N/A/bloqueado** (BF-007), promoção/deploy SWE-exclusivos, rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado local**: **10/10 PASS (documental/CLI_HEADLESS)**. `evidence_status: passed` para decisão, estado e testes focados; CI remoto e merge em `dev` permanecem **[NV]** até observação ao vivo.

## Verificação Rust observada

- `cargo check -p insurgencia-sim --tests`: **EXIT 0**.
- `cargo test -p insurgencia-sim --test simulacao`: **97 passed, 0 failed** (92 anteriores + 5 novos).
- Cinco testes nomeados executados individualmente: **5/5 PASS** (`roadmap.txt`).
- `cargo fmt --all`: OK.
- `cargo check -p insurgencia-cliente` e `./verify.sh` local: **timeout por contenção de CPU** de processos `flora cargo` concorrentes; não classificados como RED. A autoridade do gate completo é o CI remoto `gates`+`rust` no head do PR.

## ACR

Primeira rodada: 3 BLOCKER (`V` sempre em si, HUD com grandeza errada, `escudo_alvo` não limpo na recarga-zero) e 1 NONBLOCKER (assinatura de `aplicar_dano_a_jogador`). Correções aplicadas na fonte por `FixAcrClient055` e `FixAcrSim055`; recheck: **RECHECK OK**. Nenhum teste foi afrouxado.

## Limitações

Escudo de Milícia, 25 Fôlego, 3 m, 250/150/420 e tecla `V` são `[HIPÓTESE]`/protótipo de GDD §10.6. A execução de 0,3 s e o bônus de +15% contra dano contínuo não têm efeito implementado: o Núcleo não possui dano contínuo nem canalização; a constante está documentada como reservada e fora da DoD. Captura visual real da janela **[NV]**; promoção `dev→main` e deploy são competência do SWE.
