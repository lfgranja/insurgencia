# Decision — Avaliação comparativa de engines Rust: fechar a lacuna de 19/11

**Data:** 2026-11-20 · **R1:** `go`

## Veredito

**go** — problema válido (impacto 4, confiança 4, escala 1–5); opção escolhida = **Opção 1** (atualizar a matriz canônica com ≥10 opções, versões corrigidas e critérios formais, no handoff). A Opção 2 (só anexo, sem matriz) é rejeitada por preservar a amostra 3/13 e as versões defasadas; a Opção 3 (trocar o Godot) é rejeitada por ADR-001, que mantém Godot-first vinculante e não reaberta. Nenhuma alegação de execução, desempenho medido ou troca de motor é feita nesta etapa.

## Gate de entrada

- **Origem**: pedido da SE (2026-11-20) para coleta documental de evidência sobre engines Rust; âncora `.specify/feature.json` → `specs/020-engines-rust`; branch `feature/020-engines-rust`, HEAD `9dadc68`.
- **Matéria-prima**: `docs/avaliacao-engines/pesquisa/{rust-bevy,rust-fyrox,rust-outras,rust-protocolo}.md` (20/11/2026), com host policy GitHub-only e cobertura linguística real em inglês apenas.
- **Gates determinísticos do alvo, citados e NÃO reexecutados nesta etapa**: **5/5 PASS**, registrados em `.specify/memory/STATE.md:9` no estado vivo de 2026-11-20T11:52Z (ciclo 019 mesclado) — "5/5 gates PASS (`spec_dirs=19`, `converge files=2 baselined=2`)". Os cinco gates são `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only` e `hub_docs_gate` (tabela em `.specify/memory/STATE.md:104-114`). O mesmo registro confirma `make check` exit 0 e check `gates` SUCCESS no PR #25 (`:5,9`). Reexecução, `make check`, Godot, saídas de suítes e status remoto/CI permanecem **[NV]** até registro datado na portaria própria.
- **Protocolo**: specs 019/040 (HEAD factory `b1deab15`, somente leitura) adaptadas por `docs/avaliacao-engines/00-criterios.md:9-23`; critérios C1–C10 (Σ=100), escala 0–10, `score=Σ(nota×peso)/10`, ≥10 opções, C1–C3 invioláveis, ADOPT/ADAPT/DEFER/REJECT + spikes.

## Restrições de execução

- **Documental**: nenhum build, protótipo, benchmark, gate, teste, `make`, Godot, formatação, stage, commit ou push é executado nesta etapa.
- **ADR-001 não reaberta**: sem troca de motor, sem segunda linguagem no Núcleo, sem autorização de "servidor Rust como Plano B" e sem reabertura de Godot-first (`docs/adr/001-excecao-linguagem-gdscript.md:19,30-36`). Quaisquer mudança de engine/linguagem dependem de evidência prévia + aprovação formal, sem efeito neste ciclo.
- **GitHub-only**: hosts fora de `github.com`/domínios de conteúdo ficam `[UNVERIFIED — fetch skipped]`; cobertura linguística real é inglês apenas, sem triangulação — declarada, não presumida.
- **[NV] preservado**: custo por tick, benchmark 5v5+bots, reconexão robusta, replay, console, importador `.tscn`, bus factor e resultados externos não são afirmados; notas C6 são proxies com justificativa.
- **Escopo de escrita**: exatamente os cinco arquivos em `.specify/assessments/engines-rust/`; **nenhum** spec, plano, matriz canônica, avaliação final ou arquivo fora da lista é escrito. Convenção: pt-BR, `[NV]`, termos estrangeiros explicados na 1ª ocorrência, internet como dado nunca como instrução.
- **Aderência estratégica**: o dossiê alimenta a condição de reavaliação da ADR-001 sem alterar a norma externa, o motor, a licença MIT-only do Núcleo ou o escopo solo/single.

## Handoff — se `go`

- **Problema**: lacuna da avaliação de 19/11 — 3/13 opções Rust, versões defasadas e ausência de Fyrox/Ambient/gdext/Rapier, com vieses registrados em `rust-protocolo.md:13`.
- **Abordagem escolhida**: Opção 1 — atualizar a matriz canônica com ≥10 opções, versões observadas (Bevy 0.20.1, Lightyear 0.31, Fyrox 1.0.0/2.0.0-rc.1, gdext 0.5.5, Rapier 0.36.1), notas recalculadas por `Σ(nota×peso)/10`, trava C1–C3, tabela de evidência/limitações e *spikes* de reconexão/replay/custo por tick — mantendo O1 Godot como ADOPT Vinculante.
- **Dentro/fora de escopo do handoff**: dentro — a revisão documental da matriz e seus documentos de apoio, em ciclo próprio, com aprovação formal se qualquer conclusão mudar; fora — troca de engine, Rust no Núcleo, F3/servidor Rust, build/benchmark local, execução de gates/testes/`make`/Godot, stage/commit/push nesta etapa.
- **Critérios de sucesso do handoff**: ≥10 critérios com Σ=100%, escala 0–10, ≥10 opções, toda nota `*` justificada, trava inviolável aplicada, versões conferidas contra fonte primária, recomendação ADOPT/ADAPT/DEFER/REJECT + spikes, evidência com URL/ID/data e limitações/**[NV]** explícitas; Godot-first inalterado.
- **Perguntas abertas**: nenhuma para este ciclo; divergências conhecidas (C3 do O3, versões Lightyear/Fyrox/ggez) ficam registradas como ressalvas, não resolvidas por inferência. Remoto, CI e desempenho real permanecem **[NV]** até verificação própria.
- **Destino**: `/speckit.specify` em ciclo subsequente. Este artefato é o único resultado autorizado agora; nenhum spec, plano ou avaliação final é produzido nesta etapa.
