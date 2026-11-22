# Validação (R4) — 036-hud-acento-fonte-livre

**Data**: 2026-11-22T21:23Z · **Resultado**: **PASS (documental) — aprovado para R7** · R4 precede R7

| Verificação | Saída |
|---|---|
| Âncora | Pedido do SWE (“Agora quero o Ê sim”), `crates/cliente/src/main.rs:458,459-462,555`, captura prévia em Xvfb, ciclo 029 (paliativo ASCII) e PR #40 mesclado (`c126b6e`, 20:52:35Z) lidos; nenhum contrato de simulação/manifesto redefinido. |
| Escopo do código | Observado às `21:18Z`: `main.rs` com `Ê`/`—`/`·`, `AssetServer` + `font: fonte.into()`; `assets/fonts/{NotoSans-Regular.ttf,OFL.txt}` e `docs/governance/font-provenance.md` criados pelo agente de código. Nenhuma alteração de `Cargo.toml`/`Cargo.lock` por este agente. |
| Fonte e licença | 621572 bytes; SHA-256 `478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823`; `fc-match` para `charset:00ca`, `charset:2014`, `charset:00b7` → Noto Sans; OFL 1.1 integral em `assets/fonts/OFL.txt` (hash `4b0e3cf405082dc00cc396e53af77cd6ba6cfe02537e19786e5d113c9158d107`), URL primária e proveniência registradas. |
| Roteiro de testes | `.specify/validations/hud-acento-fonte-livre/test-roadmap.md` escrito antes do PR: 10 passos (hash/licença, inspeção, duas capturas Xvfb, verificação dos três glifos e ausência de `▯`, gates, fronteira/rollback), superfície CLI/visual isolada, plano de capturas sem janela no desktop da SE, coluna `automatizável?`, rollback. |
| Evidência local (observada) | `21:15–21:16Z`: HEAD `7c5c0f3`, 0 modificações rastreadas, 24 untracked, 0 PRs abertos; `verify.sh` 41/41 e 5 gates prévios verdes (SDD `35/0`, DAG `332/34/0`, converge `18/18`, hub-docs `0`). |
| Execução do roteiro | **[NV]** — nenhum build, teste, gate, captura com acento ou janela executada neste ciclo; a ser executado pelo orquestrador conforme o roteiro. |
| Baseline de converge | **[NV]** — `specs/036-hud-acento-fonte-livre/converge.md` é arquivo novo com a 1ª entrada; bootstrap do baseline append-only pelo orquestrador (fora da fronteira deste agente). |
| Governança | 14 artefatos na ordem normativa; `analyze.md` READ-ONLY antes de `tasks.md`; `converge.md` append-only; checklist revisado (`[x]` = revisão, não implementação); R1 `go` (6/6 strong, 240/625); rollback documentado; factory `b1deab1` somente fonte; `docs/GDD*`, baselines, `STATE.md`, `PROGRESSO.md`, `feature.json`, ledger, CI e paths untracked alheios intactos. |

## R4
Aprovado para R7. A aprovação é documental: baseia-se na âncora lida, na inspeção da fonte/licença e nas evidências observadas às `21:15–21:18Z`. Nenhum verde é afirmado para o roteiro, para um `./verify.sh` pós-mudança, para a captura com acento ou para gates reexecutados — todos **[NV]** até o orquestrador. A grafia correta depende da fonte embarcada e do `TextFont`, não de mudança de lógica; a regra da janela (`--minimizado`, capturas em Xvfb) permanece. Rollback documentado em `release.md`, nunca executado.
