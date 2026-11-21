# Checklist de requisitos — 024-instrumentacao-metricas

**Data:** 2026-11-21T01:03Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação do crate `crates/metricas`, escrita de código Rust, execução de `cargo`/`verify.sh`/gates/testes/build/formatadores, execução de partidas, abertura de janela, stage, commit ou push.

- [x] Revisão de requisitos concluída sobre `.specify/feature.json`, `.specify/memory/STATE.md` (prioridade da SE, itens 2 e 3), `docs/GDD - Insurgência.md` (§16.6/§16.7, §17 GDD:1605–1622, §18, GDD:1178/1192/1516/1567/1638), specs 021–023, `Cargo.toml` do workspace, `verify.sh`, `clippy.toml`, `scripts/ci/`, `projetos/AGENTS.md` e skill `mdmv-rust-thin`
- [x] Saída desta etapa é exclusivamente documental: `spec.md`, `plan.md`, `checklists/requirements.md` e `tasks.md`; nenhum `.rs`, `Cargo.toml`, gate, baseline, `evidence.csv`, `Makefile`, relatório final ou artefato R4/R7 é escrito ou alterado aqui (FR-007, FR-008)
- [x] US1 (P1) fixa os três eventos no Núcleo — `Captura`, `TentativaSincronia`, `DivergenciaClienteServidor` — com `partida_id`, sequência, tempo monotônico e fonte declarada; o cliente não é a fonte do registro (FR-001, Q1)
- [x] US2 (P1) fixa 20 registros com as 5 métricas da §17 e mediana/p95 calculados dos eventos de US1, sem valor inferido, copiado ou ausente (FR-004, FR-005, SC-002, SC-003)
- [x] US3 (P1) fixa 2 partidas consecutivas (N e N+1), uma com cliente modificado, inputs forjados recusados/corrigidos e divergência registrada sem correção pendente (GDD:1567, FR-006, SC-004)
- [x] US4 (P2) fixa 1 relatório datado com 20 registros, 5 métricas, par consecutivo e referências GDD:1567/1605–1622 (FR-007, SC-005)
- [x] **Instrumentação no Núcleo (Q1):** eventos emitidos em `crates/sim`, nunca apenas no cliente; RTT reportado pelo cliente é dado recebido e gravado pelo Núcleo, não registro confiado ao cliente
- [x] **Limiares `[HIPOTESE]` (Q2):** M1 mediana ≤ 90 s/p95 ≤ 150 s; M2 60–90%; M3 RTT mediano ≤ 80 ms local, p95 ≤ 250 ms, ≥ 90% positivas; M4 ≥ 50%; M5 ≤ 1% de divergência não corrigida — todos marcados como hipótese, com recalibração única após 20 partidas
- [x] **Cliente modificado (Q3):** simula inputs forjados (posição pronta, dano declarado acima do real, captura sem condição prévia); servidor autoritativo recusa/corrige; sem leitura de memória, driver de kernel ou modificação do Núcleo
- [x] **Sem servidores comunitários (Q4):** nenhuma métrica depende de servidor comunitário, federação ou forks publicados; "Servidores comunitários ativos" e "Diversidade de módulos e forks publicados" ficam DEFER; sem OTLP/Uptime Kuma — o relatório datado é o substituto documental
- [x] FR-001 exige emissão no Núcleo de capturas, tentativas de sincronia na janela de 5 s e divergências, com sequência monotônica e deduplicação, sem aceitar estado pronto
- [x] FR-002 exige o crate `insurgencia-metricas` em `crates/metricas`, `edition = "2024"`, membro do workspace, sem dependência de rede/banco/HTTP/OTLP
- [x] FR-003 exige definição operacional e fonte de dados declaradas para toda métrica do relatório, na forma da tabela da §17
- [x] FR-004 fixa exatamente 5 métricas (M1–M5) e suas metas `[HIPOTESE]`, com mediana/p95 onde aplicável, e exclui as duas métricas DEFER da §17
- [x] FR-005 exige 20 de 20 registros com `partida_id` único, 0 duplicatas e 0 registros inferidos
- [x] FR-006 exige 2 partidas consecutivas com cliente modificado em uma delas, correção/recusa de todo input forjado e 0 divergências sem correção
- [x] FR-007 exige o relatório `docs/metricas/2026-11-21-relatorio-instrumentacao.md` com data/hora, referências normativas e ressalva `[HIPOTESE]`
- [x] FR-008 exige `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, sem `unwrap`/`expect`, `Result` + `thiserror`, e `verify.sh` com 4 estágios terminando em `verify.sh: ALL STAGES OK`, com 5 gates determinísticos e `Makefile`/baselines/`evidence.csv` inalterados
- [x] Critérios de sucesso mensuráveis e binários: 4 de 4 estágios `verify.sh` + 5 de 5 gates PASS (SC-001); 20 de 20 registros (SC-002); 5 de 5 métricas em 100% dos registros com fonte e mediana/p95 (SC-003); 2 de 2 partidas consecutivas com divergência corrigida (SC-004); 1 de 1 relatório datado (SC-005)
- [x] Nenhum marcador `[NEEDS CLARIFICATION]` em aberto: as quatro ambiguidades foram resolvidas em `## Clarifications` com fonte local (GDD §17/§18, STATE.md, docs/avaliacao-stack, GDD:1178/1192/1516/1567/1638)
- [x] Plano de escrita fixa criação do crate `crates/metricas`, tipos de evento, pontos de emissão no Núcleo, agregador determinístico, 20 registros, 2 partidas consecutivas e relatório datado, com FRs/SCs mapeados e ordem de R4 antes de R7
- [x] Tasks T001..T010 com marcas `[X]` nas fases Implementação, Verificação e Convergência, cobrindo todas as histórias, requisitos e critérios de sucesso
- [x] Edge cases cobertos: partida sem captura, janela sem tentativa (`0/0` → `sem_amostra`), RTT ausente, divergência duplicada, empate na mediana/p95, cliente modificado sem divergência e nenhuma métrica dependente de servidor comunitário
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado, nenhum comando executado, nenhuma partida rodada e nenhuma alegação de execução observada — todo resultado permanece **[NV]** até registro datado em R4
