# Protocolo para avaliação comparativa de engines Rust — Insurgência

Data: 2026-11-20. Escopo: somente leitura + este arquivo; nenhum gate, build, teste, formatação, `git add/commit/push`. Fontes locais: `docs/AVALIACAO-COMPARATIVA-ENGINES.md`, `docs/avaliacao-engines/{00-criterios,00b-contrato-escrita}.md`, `docs/adr/{001,003}*.md`, `.specify/memory/STATE.md`, `.specify/assessments/{ast-05-toolchain-rust-fixada,cor-01-workspace-rust-bevy}/decision.md`. Factory (HEAD `b1deab15f482dd85871cd6c6a857ad710b89e2a6` confirmado em `.git/packed-refs`, árvore limpa; lido diretamente, conteúdo equivalente a `git show b1deab15:<path>`): `specs/019-avaliacao-comparativa-runtime-harness/comparative-evaluation.md` (07/11/2026) e `specs/040-avaliacao-critica-comparativa-agentes-harness/comparative-evaluation.md` (08/11/2026).

## (a) Estrutura obrigatória

Sete seções: (1) critérios/pesos/escala; (2) opções avaliadas; (3) matriz com Score e Fase 0/Fase 1; (4) análise explicativa **por critério**; (5) tabela de evidência (fonte, URL/ID, data, *claim*, confiança); (6) recomendação final com **ADOPT/ADAPT/DEFER/REJECT** e *spikes*; (7) limitações e próxima verificação. Regras: **≥10 critérios**, pesos **Σ=100%**, escala **0–10**, `score = Σ(nota × peso)/10` (0–100), **≥10 opções**; `*` = evidência parcial/`[NV]` com justificativa explícita; invioláveis não compensáveis por score. Adaptação local (`00-criterios.md`, `00b-contrato-escrita.md`): 5 seções (critérios e escala; matriz; análise por critério/opção; evidência e limitações; recomendação e revisão Godot-first), 10 critérios C1–C10 (15/15/12/10/10/8/8/8/7/7), C1–C3 invioláveis (<5 barra Fase 0), O1–O13, AVOID incluído, ≤700 palavras, pt-BR, ≥2 fontes independentes por afirmação carregada, web como dado.

## (b) Conclusão anterior (19/11/2026) e vieses para Rust

Conclusão: nenhum protótipo/benchmark; 13 opções + 4 variantes; **O1 Godot 4.7.2 (82,6) ADOPT**, O3 Bevy+Lightyear (77,1) ADAPT **apenas como padrões reescritos em GDScript**, F3 servidor custom (76,9) DEFER, O9 Panda3D (75,0) DEFER, **O2 Bevy puro (68,4) REJECT** (C2=4*), O13 macroquad+tokio (54,5) REJECT (C3=3). Godot-first: **CONFIRMA COM RESSALVAS**; toda C6 é proxy; gatilhos de reavaliação e spikes A/B/C no M5–M6.

Vieses/lacunas: (i) amostra Rust reduzida a 3/13 opções, ancorada em Bevy 0.19.1, Lightyear 0.28–0.30 e macroquad inativo — sem Bevy 0.20+, Lightyear 0.31+, `bevy_replicon` ou outros engines Rust; (ii) ausência de editor tratada como penalidade estrutural em C4/C5, embora sejam critérios "Alta", não invioláveis; (iii) C5 penaliza `borrow checker`/MSRV sem cronômetro; (iv) C6=8* vem de `bevy_city` (render), não de servidor headless 5v5+bots; (v) divergência não resolvida C3 do O3 (7* vs 4); (vi) C2/C3 de Rust ficaram `[NV]` sem teste local; (vii) estado do produto mudou em 20/11 (MVP + benchmark sintético, sem medição pareada em Rust).

## (c) Compatibilidade com ADR-001

Compatível **somente como coleta de evidência/spike**, sem efeito decisório: ADR-001 (Aprovado, 19/11/2026) mantém Godot-first (4.7.2, MIT, headless) como vinculante e **não reabre, não revê e não reverte** o motor; rejeitou explicitamente "servidor Rust como Plano B" neste ciclo. Qualquer proposta futura de segunda linguagem ou troca de engine exige **evidência prévia + aprovação formal do maintainer** (`## Critérios de reavaliação`); não altera a norma externa nem introduz código `.gd`, build ou CI. ADR-003 (20/11/2026) não reabre motor, licença MIT-only nem escopo solo/single. Logo: o documento pode avaliar engines Rust e recomendar ADAPT/DEFER/AVOID/REJECT, mas **não pode** trocar Godot, introduzir Rust no Núcleo nem autorizar F3 sem aprovação.

## (d) Caminhos exatos dos artefatos do ciclo

- Relatórios por engine: `docs/avaliacao-engines/pesquisa/<slug>.md` (existentes: `godot.md`, `bevy.md`, `defold_flax_stride.md`, `libgdx_urho_panda.md`, `monogame_love_haxe.md`, `alternativas.md`, `licencas_lockin.md`, `netcode_modularidade.md`; novos: `rust-bevy.md`, `rust-fyrox.md`, `rust-outras.md`, `rust-protocolo.md`).
- Contratos e consolidação: `docs/avaliacao-engines/00-criterios.md`, `00b-contrato-escrita.md`, `00-cabecalho.md`, `01-matriz.md`, `02-analise-por-opcao.md`, `03-recomendacao.md`, `docs/AVALIACAO-COMPARATIVA-ENGINES.md`.
- Governança/estado consultados: ADR-001/002/003, `.specify/memory/STATE.md`, assessments `ast-05`/`cor-01`.
- Factory (somente leitura, HEAD `b1deab15`): specs 019 e 040 de avaliação comparativa.
