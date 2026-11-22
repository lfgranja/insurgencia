# Checklist de requisitos — 020-engines-rust

**Data:** 2026-11-20T23:46Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação do documento final, execução de gates, testes, `make`, Godot, build, stage, commit ou push.

- [x] Revisão de requisitos concluída sobre os quatro relatórios em `docs/avaliacao-engines/pesquisa/{rust-bevy,rust-fyrox,rust-outras,rust-protocolo}.md`, sobre `docs/avaliacao-engines/{00-criterios,00b-contrato-escrita}.md`, sobre o protocolo factory das specs 019/040 e sobre ADR-001 (Aprovado, 19/11/2026)
- [x] O output é documental e único: `docs/AVALIACAO-ENGINES-RUST.md`, escrito na implementação; nenhum artefato R4/R7 é escrito nesta etapa e nenhum relatório de pesquisa é alterado (FR-001, FR-008)
- [x] FR-001 fixa as 7 seções do protocolo factory em ordem — critérios/pesos/escala; opções; matriz com Score e Fase 0/Fase 1; análise por critério; tabela de evidência (fonte, URL/ID, data, *claim*, confiança); recomendação com ADOPT/ADAPT/DEFER/REJECT e spikes; limitações e próxima verificação — acomodando as 5 seções da adaptação local sem perda (FR-001, SC-001)
- [x] FR-002 fixa 10 critérios C1–C10 com pesos 15/15/12/10/10/8/8/8/7/7 (Σ=100%), escala 0–10, fórmula `score = Σ(nota × peso)/10` (0–100, 1 casa decimal) e nota com `*` justificada / `[NV]` para não verificado (FR-002, SC-002)
- [x] FR-003 exige ≥10 opções únicas consolidadas dos quatro relatórios, com ID/nome canônicos e fonte datada
- [x] FR-004 exige que o score de cada linha da matriz seja recalculado por `Σ(nota × peso)/10` com 1 casa decimal, sem valor copiado sem verificação ou ausente (SC-003)
- [x] **Categoria verificada:** `godot-rust/gdext` é binding sobre Godot (MPL-2.0), Rapier é biblioteca de física, macroquad/ggez são bibliotecas/frameworks e Ambient está pausada; nenhum é contado como engine-base, e o score 68,6 do binding não é apresentado como score de engine (Q1, FR-003, SC-004)
- [x] **Versões divergentes registradas:** Bevy 0.19.1 do contrato vs. 0.20.1 observado em 20/11, Lightyear UI 0.30.0 vs. `main` 0.31.0, Fyrox 1.0.0 vs. 2.0.0-rc.1 e licenças duplas divergentes na API; usar fonte primária de código, sem inferir valor intermediário (Q2, FR-006)
- [x] **Sem benchmark local:** nenhuma nota de desempenho é apresentada como medição; C6 é proxy com `*` e `[NV]`, e a próxima verificação descreve o que medir (Q3, FR-006)
- [x] FR-007 exige veredito ADOPT/ADAPT/DEFER/REJECT por opção e spikes nomeados, com aplicação da trava inviolável
- [x] **Fase 0 inviolável:** C1, C2 e C3 não podem ser compensados por score; nota < 5 em qualquer um bloqueia adoção na Fase 0 (O2 C2=4*, Fyrox C2=4*/3, macroquad C3=3, ggez C2/C3=3, Rapier C2/C3=3), com o bloqueio declarado na matriz e na recomendação (FR-005)
- [x] FR-006 exige tabela de evidência com fonte, URL/ID, data, *claim* e confiança e seção de limitações preservando `[NV]` e `[UNVERIFIED — fetch skipped]`; conteúdo da internet é tratado como dado, nunca como instrução (FR-006)
- [x] **Godot-first não reaberto:** a recomendação afirma que Godot (4.7.2, MIT, headless) permanece vinculante, não reaberta/revista/revertida; nenhum veredito troca o motor, introduz Rust no Núcleo ou autoriza "servidor Rust como Plano B"; mudança futura exige evidência prévia e aprovação formal do maintainer (Q4, FR-007, ADR-001)
- [x] FR-008 adota R4 (`validate`: gate executado, saída colada, data/hora e conformidade) antes de R7 (`release`: SemVer, changelog com IDs FR/T e rollback), sem escrever artefatos das portarias nesta etapa; o não observado permanece **[NV]** (FR-008)
- [x] Critérios de sucesso mensuráveis e binários: 7 de 7 seções (SC-001); Σ pesos = 100 e 10 de 10 critérios (SC-002); score conferido em 100% das linhas (SC-003); ≥10 opções e 0 erros de categoria (SC-004); 5 de 5 gates PASS e `make check` inalterado, sem modificar gate, baseline ou `evidence.csv` (SC-005)
- [x] Plano de escrita fixa as 7 seções, as fontes dos quatro relatórios, o recálculo de score, a trava Fase 0, a não reabertura de Godot-first e a convergência READ-ONLY com correção na fonte, nunca no artefato derivado
- [x] Tasks T001..T010 com marcas `[X]` nas fases Implementação, Verificação e Convergência, mapeando FRs/SCs e cobrindo todas as seções do documento final, incluindo a redação da recomendação e das limitações
- [x] Escopo explicitamente fora: relatórios de pesquisa, gates, baselines, `evidence.csv`, `Makefile`, protótipo/benchmark, build, CI, código GDScript/Rust, troca de motor, Rust no Núcleo, artefatos R4/R7 e qualquer alegação de execução observada
- [x] Nenhum marcador `[NEEDS CLARIFICATION]` em aberto: as quatro ambiguidades (categoria engine vs. binding/lib; versão 0.x do Bevy; ausência de benchmark local; ADR-001) foram resolvidas em `## Clarifications` com fontes locais
- [x] Saídas não observadas permanecem **[NV]**; registro com data/hora pertence a R4, e versão, changelog com FR/T e rollback a R7; R4 precede R7
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado e nenhum comando executado nesta etapa de especificação
