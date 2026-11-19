# Concept: Fechar inconsistências documentais sem reabrir Godot-first

- **Slug**: excecao-linguagem-licenca-nucleo
- **Created**: 2026-11-19
- **Recommended option**: Option A — ADR local de exceção de linguagem + revisão N3

## Options

### Option A — ADR local de exceção de linguagem + revisão N3

- **Sketch**: decisões registradas no próprio repositório que explicitam a exceção de linguagem para Godot/GDScript e consolidam a posição de licenciamento N3, além de converter o diferimento de CI em posição inequívoca com gatilho — tudo sem tocar a decisão Godot-first, que permanece como premissa declarada em cada registro (`docs/AVALIACAO-STACK-MDMV.md:368-374,443`; `docs/GDD - Insurgência.md:1265,1678`).
- **Appetite**: small (dias) — o problema é documental (`docs/AVALIACAO-STACK-MDMV.md:354`) e já existem decisões locais prévias que servem de precedente (`.specify/assessments/ast-05-toolchain-rust-fixada/decision.md`; `.specify/assessments/doc-34-ci-cinco-jobs/decision.md`; `docs/AVALIACAO-STACK-MDMV.md:18`).
- **Trade-offs**: ganha fechamento rápido, reversibilidade e independência do Strategic HQ no curto prazo (`docs/AVALIACAO-STACK-MDMV.md:374`); sacrifica a autoridade normativa plena, pois a exceção depende de aceitação da norma e pode ser insuficiente no longo prazo (`docs/AVALIACAO-STACK-MDMV.md:371,374`). Risco: fechar N3 contra `docs/GDD - Insurgência.md:1451` pode gerar nova divergência documento↔decisão (`docs/AVALIACAO-STACK-MDMV.md:381,437`).
- **Rabbit holes**: reabertura involuntária do pilar por ADRs de processo (risco MÉDIO em `docs/AVALIACAO-STACK-MDMV.md:443`); disputa MIT × AGPL sem autoridade definida (`docs/AVALIACAO-STACK-MDMV.md:405`); pressão para transformar o ADR em pipeline imediata e reproduzir o "verde vazio" que o DEFER evitava (`docs/ASSESS-omoba-bevy.md:69`).

### Option B — Emenda da norma MDMV com categoria "engine de jogo"

- **Sketch**: propor ao Strategic HQ uma categoria normativa para engines de jogo que autorize GDScript como exceção de primeira classe, resolvendo a raiz do desvio 1 na própria norma em vez de contorná-lo localmente (`docs/AVALIACAO-STACK-MDMV.md:371`).
- **Appetite**: large (meses) — depende de autorização externa e está explicitamente fora de escopo deste ciclo (`docs/AVALIACAO-STACK-MDMV.md:371,405`).
- **Trade-offs**: ganha conformidade integral duradoura e remove a dependência de exceção ad hoc (`docs/AVALIACAO-STACK-MDMV.md:362`); sacrifica tempo e controle, pois a aprovação é `[NV]` e foge ao maintainer do projeto (`.specify/memory/constitution.md:17`).
- **Rabbit holes**: negociação normativa com o Strategic HQ sem previsão de prazo; redação da categoria e seus critérios; risco de a emenda ser conduzida de forma a tocar `docs/GDD - Insurgência.md:1678` e invalidar o roadmap.

### Option C — Plano B F3 com servidor Rust

- **Sketch**: manter Godot/GDScript no cliente e adotar Rust no servidor *headless*, alinhando o servidor à presunção da norma e à exigência AGPL-3.0 do GDD para esse componente (`docs/AVALIACAO-STACK-MDMV.md:372`; `docs/GDD - Insurgência.md:1451`).
- **Appetite**: medium/large — exige código e runtime, ambos ausentes no alvo (`docs/AVALIACAO-STACK-MDMV.md:354`; `docs/specs/2026-11-19-ciclo-80-20-omoba.md:6`).
- **Trade-offs**: ganha alinhamento normativo do servidor e compatibilidade com a cláusula AGPL do GDD; sacrifica a simplicidade de uma única linguagem e a natureza documental do ciclo, introduzindo segunda toolchain e exigindo verificação empírica hoje impedida (`docs/AVALIACAO-STACK-MDMV.md:372,445`).
- **Rabbit holes**: bifurcação de toolchain; definição de protocolo cliente-servidor; impossibilidade de validar sem runtime, deixando resultados `[NV]` (`docs/AVALIACAO-STACK-MDMV.md:445-447`).

## Recommendation

Recomenda-se a **Option A**. Ela ataca o problema no espaço onde ele vive — decisões documentais em um repositório sem código (`docs/AVALIACAO-STACK-MDMV.md:354`) — com appetite small e precedente local já verificado, e é a própria saída preferida pela avaliação, com C deferido e B como contingência (`docs/AVALIACAO-STACK-MDMV.md:368-374,387-396`). É a única que reduz as inconsistências de 3 para zero sem alterar o pilar Godot-first, atendendo diretamente às métricas de sucesso e à meta de zero mudanças em `docs/GDD - Insurgência.md:1265,1678`. A recomendação preserva ainda a contingência B caso a norma não aceite a exceção local. A recomendação pressupõe que a revisão de N3 não reabra o pilar — condição a validar na especificação.

## Out of Scope (for the recommended option)

- Reabrir, revisar ou reverter Godot-first (`docs/GDD - Insurgência.md:1265,1678`; `docs/AVALIACAO-STACK-MDMV.md:443`).
- Emendar a norma global MDMV — Option B fica apenas como contingência (`docs/AVALIACAO-STACK-MDMV.md:371`).
- Adotar Rust no servidor ou introduzir segunda linguagem/toolchain — Option C deferida (`docs/AVALIACAO-STACK-MDMV.md:372,374`).
- Escrever código, runtime, gates, build, testes, lint ou compilação (`docs/AVALIACAO-STACK-MDMV.md:354,447`).
- Pipeline de CI imediata sem gatilho mensurável, que reproduziria o "verde vazio" (`docs/ASSESS-omoba-bevy.md:69`).
- Verificar status remoto — aprovação humana, CI, merge e branch protection permanecem `[NV]` (`.specify/memory/constitution.md:17`).

## Assumptions to Validate

- A autoridade de aprovação das decisões locais é o maintainer do projeto, não o Strategic HQ (`docs/AVALIACAO-STACK-MDMV.md:405`); se for o contrário, a saída B torna-se necessária.
- Uma decisão local de exceção de linguagem basta para GDScript sem emenda normativa (`docs/AVALIACAO-STACK-MDMV.md:371,374`).
- O destino final de N3 pode ser fechado sem criar nova divergência com `docs/GDD - Insurgência.md:1451` (`docs/AVALIACAO-STACK-MDMV.md:381,437`).
- O gatilho do diferimento de CI é "primeiro commit de código em `nucleo/`" (`docs/AVALIACAO-STACK-MDMV.md:380,406`).
- Uma declaração textual de não-reabertura do pilar satisfaz a métrica qualitativa de "sem reabrir Godot-first" (`docs/AVALIACAO-STACK-MDMV.md:443`).
- A aprovação humana dos registros será obtida — hoje marcada como `[NV]` (`.specify/memory/constitution.md:17`).
