# Idea Research: Exceção de linguagem, licença e núcleo

- **Slug**: `excecao-linguagem-licenca-nucleo`
- **Created**: 2026-11-19
- **Evidence confidence (overall)**: **high** — evidência documental local verificada (arquivos do próprio alvo, com caminho:linha). Status remoto (CI, aprovação, merge, branch protection) permanece **`[NV]`** até confirmação ao vivo.
- **Cobertura idiomática real**: **somente PT-BR e EN**, presentes nos documentos locais citados. **Nenhuma fonte externa foi consultada**; nenhuma URL foi fornecida ou aberta. A allowlist de hosts não foi acionada (política: `local-only`).
- **Método de triangulação**: toda afirmação carregada repousa em ≥2 fontes locais independentes (ex.: GDD + AVALIACAO-STACK; LICENSE + spec/plano; ASSESS + AVALIACAO-STACK). A norma MDMV é citada apenas através do registro local que a reporta (`docs/AVALIACAO-STACK-MDMV.md:366`).

## Users & Demand

- Os demandantes imediatos são os mantenedores do Núcleo e os operadores de servidores/forks: a avaliação de stack afirma que os desvios "precisam ser decididos agora, antes que o código exista" (`docs/AVALIACAO-STACK-MDMV.md:28-30`) e que "não há como atingir conformidade integral sem (i) autorizar a linguagem e (ii) fechar a questão de licença" (`docs/AVALIACAO-STACK-MDMV.md:362`). A soberania do jogador exige regras inspecionáveis, modificáveis e redistribuíveis (`.specify/memory/constitution.md:13`) e o GDD registra a fragmentação por forks como risco de pilar (`docs/GDD - Insurgência.md:1595`). (confidence: high; 3 fontes locais independentes)
- O problema é documental, não de runtime: o repositório é "100% documental" (`docs/AVALIACAO-STACK-MDMV.md:354`), a spec declara "o repositório ainda não tem código-fonte" (`docs/specs/2026-11-19-ciclo-80-20-omoba.md:6`) e o plano repete que só `LICENSE`, `.gitignore`, `docs/` e `.specify/` já existem (`docs/plans/2026-11-19-ciclo-80-20-omoba.md:16`). Consequência: a demanda é por decisões escritas, não por implementação. (confidence: high)
- A decisão Godot-first já operacionaliza a linguagem na spec e no plano — "Godot 4.7.2 stable, GDScript e licença MIT no Núcleo" (`docs/specs/2026-11-19-ciclo-80-20-omoba.md:5`) e "Godot 4.7.2 ..., GDScript ..., licença MIT no Núcleo" (`docs/plans/2026-11-19-ciclo-80-20-omoba.md:6`) — confirmado pela síntese da avaliação (`docs/AVALIACAO-STACK-MDMV.md:366`). Logo, o que falta é o instrumento de governança, não a escolha técnica. (confidence: high)
- Não há sinal de demanda externa observada (tickets, entrevistas, telemetria): a própria avaliação registra que "nenhum gate, build, teste, lint ou compilação foi executado" (`docs/AVALIACAO-STACK-MDMV.md:447`) e a spec anota "nenhum foi rodado na escrita desta spec" (`docs/specs/2026-11-19-ciclo-80-20-omoba.md:55`). (confidence: high; ausência verificada em duas fontes)

## Prior Art

- Existe precedente local de decisões registradas por assunto: `.specify/assessments/ast-05-toolchain-rust-fixada/decision.md` (fixação de versão adaptada a Godot) e `.specify/assessments/doc-34-ci-cinco-jobs/decision.md` (CI deferida) são ADRs/decisões locais já escritos no próprio repositório, e a avaliação propõe formalizar mais seis (`docs/AVALIACAO-STACK-MDMV.md:399-410`). (confidence: high)
- A avaliação já mapeou as saídas possíveis para o conflito de linguagem — ADR local (saída A), emenda da norma (saída B) e servidor Rust como Plano B (saída C) — e recomenda A agora, C deferido e B como contingência (`docs/AVALIACAO-STACK-MDMV.md:368-374`), coerente com a tabela de decisões por desvio (`docs/AVALIACAO-STACK-MDMV.md:387-396`). (confidence: high; 2 fontes locais)
- A correção de N3 já tem proposta ancorada: ADR-006 fecha `GDD:1659`, emenda `GDD:1451` e troca `COPYING` por `COPYRIGHT.txt` (`docs/AVALIACAO-STACK-MDMV.md:381,410`), ecoando a orientação de escopos de licença explícitos e sem "OR" ambíguo no assess (`docs/ASSESS-omoba-bevy.md:43`) e a preservação de `LICENSE` no plano (`docs/plans/2026-11-19-ciclo-80-20-omoba.md:12-13`). (confidence: high; 4 fontes locais)
- Tentativa anterior de tratar a CI como gate imediato foi recusada: doc-34 foi DEFER "até existir código" (`docs/ASSESS-omoba-bevy.md:69`), e a avaliação propõe converter isso em DEFER com gatilho (`docs/AVALIACAO-STACK-MDMV.md:380,406`). (confidence: high)

## Market & Context

- Não há dados de mercado, adoção ou pesquisa de usuário no alvo: a evidência disponível é normativa e documental (`docs/AVALIACAO-STACK-MDMV.md:350`), e o repositório não contém runtime para medir (`docs/AVALIACAO-STACK-MDMV.md:354`; `docs/specs/2026-11-19-ciclo-80-20-omoba.md:6`). Qualquer afirmação de mercado seria ASSUMPTION e, portanto, não é registrada como evidência. (confidence: high para a ausência)
- A alternativa com a qual os mantenedores convivem hoje é manter os sete desvios de intenção sem instrumento normativo (`docs/AVALIACAO-STACK-MDMV.md:20,28-30`), com Godot-first já fixado (`docs/GDD - Insurgência.md:1265,1678`) e licença vigente MIT (`LICENSE:1`). (confidence: high)
- Custo de inação documentado: risco BLOQUEANTE de "governança sem ADR para GDScript: norma autoriza só Go" (`docs/AVALIACAO-STACK-MDMV.md:436`), risco ALTA de "N3 aberta: `LICENSE` MIT vs AGPL/`COPYING` no GDD" (`docs/AVALIACAO-STACK-MDMV.md:437`) e risco ALTA de "CI prometida no dia 1 entregue apenas como gate local" (`docs/AVALIACAO-STACK-MDMV.md:438`), todos repetidos na síntese (`docs/AVALIACAO-STACK-MDMV.md:362`). (confidence: high; 2 fontes locais)

## Data & Constraints

- **Linguagem**: a norma fixa Rust como padrão e admite exceção só para Go com ADR, conforme registrado localmente (`docs/AVALIACAO-STACK-MDMV.md:366`); o GDD adota "Godot-first (4.7.2, MIT, servidor *headless*)" (`docs/GDD - Insurgência.md:1265`) e torna a decisão vinculante em `docs/GDD - Insurgência.md:1678`, reforçado pela spec (`docs/specs/2026-11-19-ciclo-80-20-omoba.md:40-43`) e pelo plano (`docs/plans/2026-11-19-ciclo-80-20-omoba.md:5-6,10-11`). Severidade BLOQUEANTE em `docs/AVALIACAO-STACK-MDMV.md:176,387`. (confidence: high; 5 fontes locais)
- **Licença**: `LICENSE:1` é MIT vigente ("MIT License", copyright em `LICENSE:3`); a spec fixa "Licença do Núcleo | MIT | `LICENSE`" (`docs/specs/2026-11-19-ciclo-80-20-omoba.md:42`) e o plano fixa "Licença **MIT** no Núcleo; `LICENSE` na raiz é preservado" com terceiros em `COPYRIGHT.txt` (`docs/plans/2026-11-19-ciclo-80-20-omoba.md:12-13`); porém o GDD exige "MIT ... para o **Núcleo**; AGPL-3.0 para o servidor" com `LICENSE` e `COPYING` (`docs/GDD - Insurgência.md:1451`) e deixa N3 aberta ("MIT ou GPL?", `docs/GDD - Insurgência.md:1659`). A inconsistência e a correção proposta estão em `docs/AVALIACAO-STACK-MDMV.md:381,396,410` e `docs/ASSESS-omoba-bevy.md:42-43`. (confidence: high; 5 fontes locais)
- **CI**: `docs/GDD - Insurgência.md:1594` promete "integração contínua desde o primeiro dia"; `docs/ASSESS-omoba-bevy.md:69` (doc-34) registra DEFER até existir código; a spec reduz a "contrato local=CI do `make check`" (`docs/specs/2026-11-19-ciclo-80-20-omoba.md:349`); a contradição e a proposta de DEFER com gatilho estão em `docs/AVALIACAO-STACK-MDMV.md:380,406,438`. (confidence: high; 4 fontes locais)
- **Restrição de escopo**: os ADRs devem ser locais e declarar explicitamente que `GDD:1265,1678` permanece intocável (`docs/AVALIACAO-STACK-MDMV.md:370,374,405,443`); o próprio GDD sinaliza que mudança de pilar só com spike formal (`docs/GDD - Insurgência.md:1678`). Nenhuma emenda silenciosa (`docs/AVALIACAO-STACK-MDMV.md:437`). (confidence: high)
- **Constituição do alvo**: fluxo SDD ordenado e R1=`go` (`.specify/memory/constitution.md:11`); verificação honesta com `[NV]` para status remoto (`.specify/memory/constitution.md:17`); precedência da governança local em divergência (`.specify/memory/constitution.md:25`). (confidence: high)
- **`[NV]`**: aprovação humana dos ADRs, branch protection, execução de CI, merge e qualquer mudança remota não foram verificados — `docs/AVALIACAO-STACK-MDMV.md:436-443` e `.specify/memory/constitution.md:17`. (confidence: high para a marcação; status remoto = `[NV]`)

## Evidence Against the Idea

- Um ADR local pode ser insuficiente: a própria avaliação registra a saída B (emenda da norma com categoria "engine de jogo") como contingência que exige autorização do Strategic HQ e está "fora de escopo deste ciclo" (`docs/AVALIACAO-STACK-MDMV.md:371`), e a recomendação A é descrita como reversível por ADR justamente porque depende de aceitação da norma (`docs/AVALIACAO-STACK-MDMV.md:374`). (confidence: high; 2 trechos independentes do mesmo documento local + `docs/GDD - Insurgência.md:1678`)
- Fechar N3 como MIT-only contraria literalmente `docs/GDD - Insurgência.md:1451`, que pede AGPL-3.0 para o servidor e a presença de `COPYING`; sem emenda explícita e aprovada, o ADR geraria nova divergência documento↔decisão (`docs/AVALIACAO-STACK-MDMV.md:381,437`). (confidence: high; GDD + AVALIACAO-STACK)
- Forçar CI imediata pode reproduzir o "verde vazio" que o DEFER evitava: o assess chama a automação de cinco jobs "ceremony sem o que verificar" (`docs/ASSESS-omoba-bevy.md:69`), e a avaliação preserva o gate local como porta única (`docs/AVALIACAO-STACK-MDMV.md:380,406`). A saída mais fraca é apenas renomear o DEFER sem gatilho mensurável. (confidence: high; ASSESS + AVALIACAO-STACK)
- Risco de reabertura involuntária do pilar: a avaliação lista "Reabertura involuntária do GDD por ADRs de processo" como risco MÉDIO (`docs/AVALIACAO-STACK-MDMV.md:443`), e `docs/GDD - Insurgência.md:1678` condiciona o roadmap à decisão Godot-first — o benefício dos ADRs depende de disciplina documental, não de mudança de engine. (confidence: high; AVALIACAO-STACK + GDD)
- Ausência de runtime impede verificação empírica: "Ausência de runtime impedindo verificação empírica de N20–N57" (`docs/AVALIACAO-STACK-MDMV.md:445`) e " Limite de verificação: nenhum gate, build, teste, lint ou compilação foi executado" (`docs/AVALIACAO-STACK-MDMV.md:447`), ecoado pela spec (`docs/specs/2026-11-19-ciclo-80-20-omoba.md:55`). Os ADRs poderiam virar intenção sem evidência de execução — `[NV]`. (confidence: high; 3 fontes locais)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: autoria e patrocínio da proposta — `docs/AVALIACAO-STACK-MDMV.md:399-410` nomeia os ADRs, não o proponente.]
- [NEEDS CLARIFICATION: autoridade de aprovação — maintainer ou Strategic HQ (`docs/AVALIACAO-STACK-MDMV.md:405`)?]
- [NEEDS CLARIFICATION: ADR local basta para GDScript, ou é exigida emenda normativa (saída B em `docs/AVALIACAO-STACK-MDMV.md:371`)?]
- [NEEDS CLARIFICATION: destino final de N3 — MIT para Núcleo e servidor (`docs/plans/2026-11-19-ciclo-80-20-omoba.md:12`) ou AGPL no servidor (`docs/GDD - Insurgência.md:1451`)?]
- [NEEDS CLARIFICATION: gatilho exato da CI — "primeiro commit de código em `nucleo/`" (`docs/AVALIACAO-STACK-MDMV.md:380,406`) depende de confirmação.]
- [NEEDS CLARIFICATION: como medir "sem reabrir Godot-first" — declaração textual em cada ADR (`docs/AVALIACAO-STACK-MDMV.md:443`) ou gate de checagem documental?]
- `[NV]`: aprovação humana, execução de CI, status de merge e branch protection — não verificados nesta rodada (`.specify/memory/constitution.md:17`).

## Sources

- `docs/AVALIACAO-STACK-MDMV.md:20-30,176,185,368-381,387-410,436-447` — caminho local; policy: `local-only`; confiança high (evidência documental local verificada).
- `docs/GDD - Insurgência.md:1205,1265,1451,1594,1595,1659,1678` — caminho local; policy: `local-only`; confiança high.
- `LICENSE:1-21` — caminho local; policy: `local-only`; confiança high (texto MIT vigente).
- `docs/specs/2026-11-19-ciclo-80-20-omoba.md:5-6,40-43,55,349` — caminho local; policy: `local-only`; confiança high.
- `docs/plans/2026-11-19-ciclo-80-20-omoba.md:5-6,10-13,16` — caminho local; policy: `local-only`; confiança high.
- `docs/ASSESS-omoba-bevy.md:42-43,69,81,168,171` — caminho local; policy: `local-only`; confiança high.
- `.specify/memory/constitution.md:11,13,17,25` — caminho local; policy: `local-only`; confiança high.
- `.specify/assessments/ast-05-toolchain-rust-fixada/decision.md` e `.specify/assessments/doc-34-ci-cinco-jobs/decision.md` — caminhos locais; policy: `local-only`; confiança high (existência e conteúdo de decisões locais prévias).
- Nenhuma URL/host consultado: allowlist de URL não acionada; nenhum `fetch` executado. Idiomas das fontes: PT-BR e EN. Todo status remoto: `[NV]`.
