# Evidência — Roteiro de Testes 054 (execução autônoma)

**Data/hora**: 2026-11-23T17:16–17:20Z. **Sessão**: `20261123T165500Z-analise-materialista` · `insurgencia - análise materialista`. **Worktree**: `.specify/queue/worktrees/habilidades-054/`, branch `054-habilidades-wer`, base `5d2919`. **Capturas**: `/tmp/tmp.qLawIBSlUm/saidas/`.

| Passo | Ação | Saída observada | Resultado | Captura |
|:--|:--|:--|:--|:--|
| 1 | Inventário/âncora | HEAD `5d2919`, branch correto, **14 artefatos** na fronteira; scorecard 48/625 | PASS | `carimbo.txt`, `inventario.txt` |
| 2 | Estado/constantes | `marca_ticks`, `analise_recarga_ticks`, `analise_materialista`, `BONUS_DANO_ANALISE_PERCENT` presentes | PASS | `estado-analise.txt` |
| 3 | Marca e bônus | `analise_marca_e_bonus_de_dano`: `exit=0`, 1 passed; 10 → 12 com marca | PASS | `analise_marca_e_bonus_de_dano.txt` |
| 4 | Não acumulação | `analise_nao_acumula`: `exit=0`, 1 passed; re-aplicação reinicia 150 ticks | PASS | `analise_nao_acumula.txt` |
| 5 | Recarga | `analise_respeita_recarga`: `exit=0`, 1 passed; 450 ticks e `AnaliseEmRecarga` | PASS | `analise_respeita_recarga.txt` |
| 6 | Alcance | `analise_respeita_alcance`: `exit=0`, 1 passed; 12 m | PASS | `analise_respeita_alcance.txt` |
| 7 | Hash | `hash_reflete_analise_materialista`: `exit=0`, 1 passed; campos novos serializados sem remover segmentos antigos | PASS | `hash_reflete_analise_materialista.txt` |
| 8 | HUD/tecla | `KeyCode::KeyE`, `Análise: N` e `ALCANCE_ANALISE_MATERIALISTA_M` presentes; captura visual real **[NV]** | PASS | `hud-analise.txt` |
| 9 | Fronteira/Rust | contratos/protocolo/Cargo `0`; `allow`/`unwrap`/`expect`/rede `0` nos dois arquivos | PASS | `fronteira.txt` |
| 10 | Governança | v1.2.2, R1/R4/R7, R6 N/A; scorecard **48/625** em **26 arquivos**; rollback documentado e não executado | PASS | `governanca.txt` |

**Resultado**: **10/10 PASS**. `evidence_status: passed`.

## Gate agregado (17:19Z)

`./verify.sh` **ALL STAGES OK** — check, clippy, **136 testes run: 136 passed, 0 skipped** (131 anteriores + 5 novos), fmt; `VERIFY_EXIT=0`. Quatro gates locais PASS; o quinto (`converge-append-only`) depende do bootstrap do baseline da `converge.md` nova e é executado após o append.

## Correções na fonte durante a execução

1. `17:16Z` — `format!`/`concat!` em `serializar_jogadores` perdeu quatro especificadores ao inserir `analise_recarga_ticks`. Restaurados `construcao_ponto`, `pontos_autogestao`, `ponte_ticks`, `ponte_ponto`, `propaganda_alvo`, `folego` e `folego_ticks`; nenhum segmento antigo removido.
2. `17:18Z` — `clippy::too_many_lines` em `ValidacaoComandos::validar`: extraídos `validar_ataque_forte`, `validar_analise_materialista` e `validar_ataque_pnj`, preservando ordem das recusas.
3. `17:19Z` — regressão detectada por `acao_bloqueada_por_nivel`: a refatoração removeu `"atacar_no" => 6` de `nivel_minimo_para_acao`; restaurado ao lado do novo arm `analise_materialista => 3`.

Nenhum teste foi afrouxado; todos os RED corrigidos na fonte e o verde final usa o head corrigido.

## Limitações

Análise Materialista, 20 Fôlego, 12 m, 150 ticks (5 s), recarga 450 ticks (15 s), bônus 25% (10 → 12) e tecla `E` são `[HIPÓTESE]`/protótipo de GDD §10.2. Escopo restrito a PNJs: sem afetar jogadores, estruturas, visibilidade, canalização, RNG, magia, mapa, rede, protocolo, Cargo, habilidades W/R ou janela normal. Captura visual real **[NV]**; CI remoto e merge **[NV]** até execução pelo orquestrador.
