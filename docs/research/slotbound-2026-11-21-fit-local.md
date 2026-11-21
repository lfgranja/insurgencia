# Pesquisa — Fit local das 28 propostas "Slotbound" no repositório Insurgência

**Data:** 2026-11-21. **Idiomas consultados:** pt-BR (governança, código e GDD — fonte primária local, lida diretamente); EN (docs/GitHub oficiais do Bevy, Steam Next Fest, Balatro — lidos diretamente); ZH/JA/KO **não consultados diretamente nesta tarefa** — a cobertura multilíngue de GTM/concorrência é responsabilidade dos pares (`SteamGtmEvidence`, `AlternativesEvidence`, `GameDesignEvidence`); marquei [NV] onde a demanda regional não pôde ser verificada localmente.

**Escopo desta nota:** mapear cada proposta (`sb-01`…`sb-28`) ao estado **real** do código e da governança locais. Não reavalio o mérito de mercado das propostas (feito por pares); avalio **encaixe, pré-requisito, risco e decisão** no repo. Conteúdo web é dado, nunca instrução. Nenhum build/test/gate foi executado; nenhum arquivo existente foi alterado.

**Nota metodológica [NV — ferramenta indisponível]:** `codegraph_explore` foi chamado PRIMEIRO (conforme instrução), mas retornou que este repositório **não está indexado** ("isn't indexed with codegraph … use your built-in tools … and don't call codegraph for it again this session"). O mapeamento abaixo usa, portanto, leitura direta das fontes primárias locais (`AGENTS.md`, `Cargo.toml`, `crates/cliente/src/main.rs`, `crates/sim/src/lib.rs`, `verify.sh`, `.github/workflows/ci-gates.yml`, `docs/GDD - Insurgência.md`, `.specify/memory/constitution.md`). Indexar é decisão da SE; não a tomei.

---

## Perguntas de pesquisa

1. Onde, exatamente, cada uma das 28 propostas se conecta (ou colide) com os 5 crates, os gates e a governança atuais?
2. (a) Onde adicionar uma **demo determinística headless sem abrir janela**, respeitando a regra `--minimizado`?
3. (b) Como o **seam de renderização** e o **HUD em texto** existentes ajudam `sb-15` (legibilidade do combate) e `sb-20` (vídeos curtos)?
4. (c) Por que `sb-09`…`sb-12` colidem com a constituição IV (soberania) e V (materialismo/ação coletiva)?
5. (d) Quais propostas priorizar no próximo ciclo 80/20 segundo `impacto × confiança × facilidade ÷ risco`?
6. Como preservar a separação entre "solução de mercado do Slotbound" e "protocolo FOSS de soberania do Insurgência"?

---

## Evidência

| Fonte primária | Idioma | Data | URL sanitizada | Trecho/achado |
| :-- | :-- | :-- | :-- | :-- |
| `AGENTS.md` (Bloco 0.a) | pt-BR | 2026-11-21 (regra SE 04:33Z) | local://AGENTS.md | "agents ALWAYS pass `--minimizado` … NEVER open a normal window while the SE works"; 5 crates, 34 testes; "Network" em `metricas` é dado sintético, não medição. |
| `.specify/memory/constitution.md` | pt-BR | ratificado 2026-11-19 | local://.specify/memory/constitution.md | IV: "Nenhuma dependência proprietária no caminho crítico; nenhum serviço externo como ponto único de falha." V: "Sem magia, sem 'campeão' solitário … vitória exige coordenação." VI: "Nada é afirmado como verde sem comando executado." |
| `Cargo.toml` | — (código) | 2026-11-21 | local://Cargo.toml | `[workspace] members = ["crates/*"]`, `resolver = "2"`. |
| `crates/cliente/src/main.rs` | pt-BR/código | 2026-11-21 | local://crates/cliente/src/main.rs | `main()` lê `--minimizado` (:65), `ModoJanela` (:54), `configurar` chama `janela.set_minimized(true)` (:423-427). `Update` encadeia `mover_jogador → sincronizar_avatar → sincronizar_sim → atualizar_texto` (:70-78). `atualizar_texto` (:547-561) monta HUD 100% em `format!` de posição/ilha/limites — sem assets externos. `Mundo` é apenas espelho: **não chama `avancar_tick`**. |
| `crates/sim/src/lib.rs` | pt-BR/código | 2026-11-21 | local://crates/sim/src/lib.rs | "Simulação autoritativa e determinística … sem depender da engine" (:4-7). `Mundo::avancar_tick` (:119-124) → prepare→skills→finish; `hash_estado()` (:153-176) SHA-256 canônico; `BTreeMap`/`VecDeque`, `PASSO_SEGUNDOS = 1/30`. |
| `verify.sh` | — (código) | 2026-11-21 | local://verify.sh | Gate = check (`--all-targets`) → clippy (`--all-targets`, `-D warnings`) → nextest/test → fmt. **`--all-targets` inclui `examples/`**: um exemplo headless em `crates/sim/examples/` é automaticamente gated. |
| `.github/workflows/ci-gates.yml` | — (config) | 2026-11-21 | local://.github/workflows/ci-gates.yml | Jobs `gates` (5 scripts Python) + `rust` (`./verify.sh`); toolchain 1.99.1; libs X11/Wayland instaladas para Bevy. Verde observado ao vivo autoriza merge autônomo (constituição, norma de CI/CD). |
| `docs/GDD - Insurgência.md` | pt-BR | 1ª redação 2026-11-19 | local://docs/GDD%20-%20Insurg%C3%AAncia.md | §2.6 usa `alea` (azar/caixas de saque) como **contraste** ao `agon` dos Desafios de Ilha; §4.4 Pilar IV proíbe "vitória por soma de desempenhos independentes" e "carry"; §4.4 nota precedência I→III→II→IV; §5.1 "Não é um campeão, não é um herói escolhido"; §3.2 público = engenheiros sem estúdio. |
| Bevy `examples/app/headless.rs` (oficial) | EN | ramo `main`, acessado 2026-11-21 | https://github.com/bevyengine/bevy/blob/main/examples/app/headless.rs | "configure the `ScheduleRunnerPlugin` to run your application without windowing … `bevy = { version = "*", default-features = false }`"; `run_once()` / `run_loop(…)` sem janela. |
| Bevy `MinimalPlugins` (dev-docs oficial) | EN | acessado 2026-11-21 | https://dev-docs.bevy.org/bevy/prelude/struct.MinimalPlugins.html | "minimal plugins for a Bevy application: TaskPoolPlugin, …" — sem renderer/windowing. |
| Steam Next Fest (página oficial) | EN | publicado 2026-11-09, modificado 2026-11-13 | https://store.steampowered.com/sale/nextfest | "Steam Next Fest: February 2027 Edition – Coming Soon … featuring demos, livestreams and more". [NV — escopo GTM é do par `SteamGtmEvidence`] |
| Balatro — site oficial / FAQ | EN; FAQ lista 日本語/한국어/简体中文/简体 etc. | acessado 2026-11-21 | https://www.playbalatro.com/faq | "The poker roguelike … deckbuilder"; seletor de idioma da FAQ inclui JA/KO/ZH. [NV — apenas metadados de idioma observados; conteúdo traduzido não lido] |

**Regra de ouro aplicada:** o que é affirmação do vídeo/resumo é tratado como **afirmação a verificar**; onde não há observação local ou fonte primária lida, uso `[NV]`.

---

## Achados

1. **O repo já é "headless por dentro".** `insurgencia-sim` não depende da engine (Arquitetura: `contratos ← sim ← metricas`; `exportacao` independente; `cliente` é binário Bevy + `lib.rs` puro). Isso é o ativo mais subutilizado para `sb-16`/`sb-17`/`sb-20`: a demo não precisa de `DefaultPlugins`.

2. **[(a)] Ponto exato para a demo determinística headless:** criar `crates/sim/examples/demo_deterministica.rs` (novo arquivo, sem tocar nos existentes), usando **apenas** `insurgencia_sim`: `Mundo::new()` → `adicionar_jogador(1)` → `agendar_comando(…)` → laço `avancar_tick()` com `PASSO_SEGUNDOS` → imprimir `hash_estado()` a cada tick. Vantagens verificáveis: (i) `verify.sh` já roda `cargo check/clippy --all-targets`, então o exemplo entra no gate sem novo job; (ii) nenhuma janela é aberta — nem minimizada — porque não há Bevy `DefaultPlugins`; (iii) produz saída reproduzível para captura/diff. Alternativa pesada: novo crate `crates/demo-headless/` (mais isolamento, mais manutenção). Se for desejável HUD visual sem janela, usar `MinimalPlugins`/`ScheduleRunnerPlugin` (evidência Bevy acima) — **nunca** `DefaultPlugins` com janela normal.

3. **[(b)] O "hook de renderização" é o seam `Mundo → apresentação`.** Em `cliente/src/main.rs`, `sincronizar_sim` copia posição para `Mundo.jogadores[PEER_ID=1]`, e `sincronizar_avatar`/`atualizar_texto` leem `JogadorLocal` para Transform/Text. É um adaptador de saída puro, compatível com a constituição II. O HUD é `format!` de estado (posição, ilha, limites) — texto/ASCII, sem texturas, sem serviço externo. Para `sb-15`: iterar legibilidade mexendo **só** no adaptador de apresentação (marcadores, ordem dos campos, destaque de ilha), sem tocar na sim. Para `sb-20`: capturar quadros reproduzíveis sob `--minimizado` (janela minizada, sem perturbar a SE) ou, melhor, direto do exemplo headless do achado 2. **Pré-requisito crítico:** hoje o cliente **não** chama `avancar_tick`; sem isso não há "combate" a tornar legível — `sb-15` depende de ligar a sim (demo headless ou driver no cliente).

4. **[(c)] `sb-09`…`sb-12` colidem com IV e V por estrutura, não por estética.** Auto-battler (`sb-09`), caça-níquel (`sb-10`), fusão (`sb-11`) e gambling-roguelike (`sb-12`) tipicamente centram vitória em **soma de desempenhos independentes**, RNG/opacidade de tabelas e progressão por heróis/campeões — exatamente o que a constituição V e o GDD §4.4/§5.1 proíbem ("carry", "campeão solitário", "vitória por soma de desempenhos independentes"). A constituição IV agrava: ballet/loot serviceado por backend proprietário cria dependência no caminho crítico e ponto único de falha. O GDD §2.6 já posiciona `alea` como **contraste** (não meta) ao `agon` dos Desafios de Ilha. A nota de precedência do GDD (I→III→II→IV) impede que um ganho de "juice" atropelasse essas proibições.

5. **Constituição IV também derruba `sb-26`.** Para um protocolo FOSS cuja tese é "fork é feature" (GDD §1, §3.3, §4.1), perseguir clones por IP é contraditório: a própria tese exige que qualquer pessoa redistribua e modifique sem autorização. "Converter clones em marketing" (`sb-21`) é sintoma de alcance, não estratégia de soberania.

6. **Os gates já são a resposta local ao "comprehension debt" (`sb-08`).** `verify.sh` (4 estágios) + 5 gates Python + 34 testes + `hub_docs_gate` são a autoridade determinística; saída de LLM é informativa (constituição I/VI). Isso é exatamente o antídoto para código 100% IA sem compreensão — e já está implementado.

7. **GTM de plataforma é pré-requisito externo, não fit de código.** `sb-17`/`sb-18`/`sb-19`/`sb-23`/`sb-24`/`sb-25` dependem de Steam/Next Fest/streamers/regulação; nenhum deles pode ser "adotado" no repo hoje sem o demo headless (achado 2) e sem evidência de plataforma (coverada por pares). `sb-22` (hotfixes) é a exceção: o canal `CI verde → merge autônomo via gh` já existe.

8. **Universo de 175 diretórios de assessment untracked pertence à SE.** Não foram lidos, movidos, renomeados ou includos em staging; nenhum comando `git add`/`commit` foi executado. Esta nota é o único arquivo escrito.

---

## Implicações para Insurgência

Mapa explícito **ID → impacto local** (decisão detalhada na Matriz de encaixe):

| ID | Impacto no repo / fluxo SDD |
| :-- | :-- |
| `sb-01` | Reforça constituição I: IA é ferramenta, gate é autoridade. |
| `sb-02` | Tensão com arte procedural low-poly; exige licença rastreável (IV). |
| `sb-03` | Renderização de avatar é estática; precisa de sistema de animação novo. |
| `sb-04` | HUD já é procedural (`atualizar_texto`) — inspiração de IA só cosmética. |
| `sb-05`,`sb-06` | Fora do código; impactam `docs/evidencias/` e disclosure, não o gate. |
| `sb-07` | Já é prática corrente no fluxo SDD (specs/, LLM informativo). |
| `sb-08` | **Alvo estrutural**: formalizar checklist de comprehension debt sobre `verify.sh`. |
| `sb-09`,`sb-10`,`sb-11`,`sb-12` | **Conflito direto** com IV/V e GDD §2.6/§4.4/§5.1. |
| `sb-13` | Já resolvido localmente por low-poly procedural (facets), sem IA. |
| `sb-14` | RNG determinístico (`fator`/`misturar`/SHA-256) já existe; "juice" é incremental. |
| `sb-15` | Depende de ligar `avancar_tick`; depois, seam de HUD é o ponto de iteração. |
| `sb-16` | **`crates/sim/examples/demo_deterministica.rs`** (achado 2) — maior ganho local. |
| `sb-17` | Mesmo demo, empacotado para Steam; depende de plataforma. |
| `sb-18` | Métricas em GDD §17 são de soberania, não wishlists. |
| `sb-19`,`sb-21`,`sb-23` | Externos; sem fit de código hoje. |
| `sb-20` | Captura reproduzível via `--minimizado` + HUD em texto (achado 3). |
| `sb-22` | Canal `CI → gh merge` já existe; basta disciplina. |
| `sb-24`,`sb-25` | Documental (ADRs/disclosure); não altera gates. |
| `sb-26` | Contradiz tese FOSS/fork (IV). |
| `sb-27` | Público já definido em GDD §3.2; IA só reduz barreira de entrada. |
| `sb-28` | Anti-métrica: GDD §17 mede verificação da tese, não 100K/3 dias. |

---

## Matriz de encaixe

Legenda: **Impacto/Confiança/Facilidade/Risco** = A/M/B (alto/médio/baixo) — heurística local, **não dado empírico**. Decisões: ADOPT / ADAPT / DEFER / AVOID / REJECT (conforme framework do orquestrador).

| ID | Onde encaixa no repo | Pré-requisito | Risco | Decisão sugerida |
| :-- | :-- | :-- | :-- | :-- |
| `sb-01` | Todo o workspace Rust; `verify.sh` | Nenhum (já vigente) | Médio (comprehension debt) | **ADAPT** — IA informativa; gate determinístico permanece autoridade. |
| `sb-02` | Cliente procedural (`criar_terreno`, `criar_ilha`); GDD §3.3 | Licença rastreável; substituir/evitar texturas | Alto (IV + artefatos) | **ADAPT** → procedural/CC0 rastreável; não pixel art IA. |
| `sb-03` | `criar_avatar` (estático), Bevy `Update` | Sistema de animação + assets | Médio | **DEFER** — fora do caminho crítico. |
| `sb-04` | `atualizar_texto` (:547), `Text` Bevy | Nenhum | Baixo | **ADAPT** — UI procedural já existe; IA só como referência. |
| `sb-05` | `docs/evidencias/` (fora do crate) | Pipeline de vídeo + disclosure | Médio | **DEFER** — ver `AiPromoEvidence`. |
| `sb-06` | `docs/evidencias/` | Pipeline de vídeo | Médio | **DEFER** — ver `AiPromoEvidence`. |
| `sb-07` | `specs/`, `.omp/commands/`, fluxo SDD | Nenhum | Baixo | **ADOPT** — já prática;LLM nunca roteia estado (I). |
| `sb-08` | `verify.sh` + 5 gates Python + 34 testes | Checklist explícito de revisão | Baixo | **ADOPT** — formalizar comprehension-debt review no gate. |
| `sb-09` | Nenhum (sem modo solo/auto-battler) | Reescrita da condição de vitória | Alto (IV/V) | **REJECT** — "vitória por soma de desempenhos independentes". |
| `sb-10` | Nenhum; GDD §2.6 usa `alea` como contraste | — | Alto (V) | **REJECT** — azar como motor central contradiz o Pilar II/IV. |
| `sb-11` | Nenhum (sem elenco de campeões) | — | Alto (V) | **REJECT** — poder por fusão de heróis ≈ "campeão solitário". |
| `sb-12` | Nenhum; referência em GDD §2.6 | — | Alto (IV/V) | **REJECT** como direção; tema apenas em §2.6 como anti-exemplo. |
| `sb-13` | `malha`/`achatar` (facetas), `CORES_TERRENO` | Nenhum | Baixo | **ADAPT** — já temos low-poly que resolve legibilidade sem IA. |
| `sb-14` | `fator()`, `misturar()`, `hash_estado()` | Medição de legibilidade | Baixo | **ADAPT** — RNG determinístico existe; juízo de balance incremental. |
| `sb-15` | `sincronizar_avatar` + `atualizar_texto` (seam) | **Ligar `avancar_tick`** no cliente/demo | Médio | **ADAPT** — iterar no adaptador de apresentação, não na sim. |
| `sb-16` | **`crates/sim/examples/demo_deterministica.rs`** (novo) | Nenhum (sim é headless) | Médio | **ADAPT** — demo determinística, sem janela alguma. |
| `sb-17` | Mesmo exemplo + `docs/governance/` release R7 | Steam/Next Fest + build | Alto (plataforma) | **DEFER** — depende de `sb-16` + acesso à loja. |
| `sb-18` | `docs/PROGRESSO.md`, GDD §17 | Dados de runway/wishlists **[NV]** | Médio | **DEFER** — métricas locais são de soberania. |
| `sb-19` | Nenhum | Streamers, algoritmo **[NV]** | Alto | **DEFER** — externo. |
| `sb-20` | Captura do HUD sob `--minimizado` / exemplo headless | `sb-16` para quadros reproduzíveis | Baixo | **ADAPT** — vídeo = captura determinística, não promessa de edição IA. |
| `sb-21` | Nenhum (clone é externo) | Alcance em ZH **[NV]** | Médio | **AVOID** — sintoma, não estratégia; ver `sb-26`. |
| `sb-22` | `verify.sh` + CI + `gh` PR flow | Nenhum | Baixo | **ADOPT** — canal de merge autônomo já existe. |
| `sb-23` | `docs/` (hoje só pt-BR) | Tradução/capacidade **[NV]** | Médio | **DEFER** — ver `SteamGtmEvidence`. |
| `sb-24` | `docs/adr/`, disclosure documental | Política da loja **[NV]** | Médio | **DEFER** — ver `DisclosureLegalEvidence`. |
| `sb-25` | Nenhum (clima de mercado) | Evidência externa **[NV]** | Médio | **DEFER/AVOID** como premissa de planejamento. |
| `sb-26` | Contradiz GDD §1/§3.3/§4.1 (fork é feature) | — | Alto (IV) | **REJECT** — litígio contra clones nega a tese FOSS. |
| `sb-27` | GDD §3.2 (público = engenheiros) | Nenhum | Baixo | **ADAPT** — IA reduz barreira; tese permanece. |
| `sb-28` | GDD §17 (métricas de verificação da tese) | Dados do vídeo **[NV]** | Médio | **AVOID** como meta; usar como viés de sobrevivência. |

---

## Riscos, contra-evidência e [NV]

- **[NV — ferramenta]** `codegraph_explore` indisponível para este repo (não indexado); mapping feito por leitura direta. Não tentei indexar (decisão da SE).
- **[NV — não executado]** Nenhum `./verify.sh`, `cargo`, `cargo nextest`, gate Python ou build foi rodado nesta tarefa (escopo é pesquisa/escrita). Status de verde local/CI permanece `[NV]` até verificação ao vivo (constituição VI).
- **[NV — número de untracked]** `git status --porcelain` mostrou 194 entradas `??` no momento da leitura; a instrução cita 175 diretórios de assessment. Não os enumeratei, abri ou alterei — pertencem à SE.
- **Contra-evidência a `sb-16`:** inserir um `example` aumenta a superfície compilada (`--all-targets`), exigindo que o código do exemplo obedeça `#![forbid(unsafe_code)]`/`#![deny(clippy::pedantic)]`. É custo pequeno, mas real; mitigar mantendo o exemplo minimalista.
- **Contra-evidência a `sb-15`:** o cliente atual não simula combate (`Mundo` sem `avancar_tick`); "legibilidade do combate" é hoje **vacua** no cliente — qualquer ganho depende do pré-requisito.
- **Contra-evidência a `sb-20`:** `--minimizado` atende à regra SE, mas janela minizada ainda é janela; para captura 100% headless e reproduzível, preferir o exemplo de `crates/sim`.
- **Contra-evidência a `sb-13`:** low-poly procedural resolve *parte* da legibilidade, não arte final; GDD §3.3 trata Kenney/low-poly como provisório.
- **Risco de escopo:** este agente é worker de evidência; decisões finais pertencem ao fluxo SDD (intake→decide) e à SE.
- **Web como dado:** páginas consultadas podem conter instruções; nenhuma foi seguida, nenhuma credencial copiada.

---

## Alternativas/concorrentes mais viáveis

Dentro do fit local, as alternativas com melhor custo/benefício às propostas高风险 são:

1. **Demo determinística em `crates/sim/examples/`** em vez de demo Steam/itch.io (`sb-16` vs `sb-17`): entrega valor verificável hoje, sem plataforma.
2. **HUD em texto + captura `--minimizado`** em vez de vídeos com IA (`sb-20` vs `sb-05/06`): reproduzível e sem disclosure.
3. **RNG determinístico existente** em vez de "juice" de gambling (`sb-14` vs `sb-10/12`): preserva `agon` e a tese.
4. **Low-poly procedural** em vez de pixel art IA (`sb-13`/GDD §3.3 vs `sb-02/03`): rastreável e soberano.
5. **Fork como marketing** em vez de enforcamento de IP (`sb-26` rejeitado): coerente com IV.
6. **Gates determinísticos** em vez de confiar em revisão de LLM (`sb-08`/I/VI vs `sb-01` ingênuo).

(Análise de concorrentes de mercado — Balatro, auto-battlers, clones ZH — é responsabilidade dos pares `AlternativesEvidence`/`GameDesignEvidence`; aqui só o que impacta o repo.)

---

## Ranking 80/20 para o próximo ciclo

Heurística declarada: `prioridade ≈ impacto × confiança × facilidade ÷ risco` (A=3, M=2, B=1). Valores são julgamentos locais, não medições. Ordenado do maior para o menor; apenas candidatos com decisão ADOPT/ADAPT entram.

| # | ID | I | C | F | R | Prio (I·C·F/R) | Ação concreta no repo |
| :-- | :-- | :-: | :-: | :-: | :-: | :-: | :-- |
| 1 | `sb-08` | A | A | A | B | **27,0** | Adicionar checklist de comprehension debt ao review de código gerado por IA, ancorado em `verify.sh`. |
| 2 | `sb-07` | A | A | A | B | **27,0** | Formalizar "IA como prototipagem informativa" no SDD (já implícito). |
| 3 | `sb-22` | M | A | A | B | **18,0** | Registrar disciplina `CI verde → merge autônomo via gh` como norma operacional. |
| 4 | `sb-20` | M | A | A | B | **18,0** | Capturar HUD reproduzível sob `--minimizado` (achado 3). |
| 5 | `sb-04` | M | A | A | B | **18,0** | Refinar HUD textual (co-benefício com `sb-15`/`sb-20`). |
| 6 | `sb-13` | M | A | A | B | **18,0** | Consolidar low-poly procedural como estética provisória documentada. |
| 7 | `sb-14` | M | M | A | B | **12,0** | Ajustar `fator/misturar` e pesos de balance com teste de legibilidade. |
| 8 | `sb-15` | A | M | A | M | **9,0** | Ligar `avancar_tick` e iterar legibilidade no adaptador de apresentação. |
| 9 | `sb-01` | A | A | M | M | **9,0** | Explicitar fronteira IA-informativa vs gate-autoridade. |
| 10 | `sb-16` | A | M | M | M | **6,0** | Criar `crates/sim/examples/demo_deterministica.rs` (achado 2). |

**Top 3 recomendado ao fluxo SDD:** `sb-08` + `sb-07` (endurecer compreensão/autoridade sobre IA) e `sb-22` (disciplina de merge). `sb-16` fica em 10º não por baixo valor, mas porque "demo headless" é pré-requisito de `sb-15`/`sb-17`/`sb-20` e deve ser escalado quando esses virarem trabalho de spec — não antes.

**Fora do ranking (decisão já tomada):** `sb-09`…`sb-12`, `sb-26` (REJECT); `sb-03`,`sb-05`,`sb-06`,`sb-17`,`sb-18`,`sb-19`,`sb-21`,`sb-23`,`sb-24`,`sb-25`,`sb-28` (DEFER/AVOID).

---

## Fontes

**Locais (primárias, lidas diretamente nesta tarefa):**
1. `AGENTS.md` — governança local, regra `--minimizado`, inventário de crates/testes.
2. `.specify/memory/constitution.md` — princípios I–VI, normas de CI/CD.
3. `Cargo.toml` — workspace de 5 crates.
4. `crates/cliente/src/main.rs` — `main()`, `configurar`, `mover_jogador`, `sincronizar_avatar`, `sincronizar_sim`, `atualizar_texto`.
5. `crates/sim/src/lib.rs` — `Mundo`, `avancar_tick`, `hash_estado`, `PASSO_SEGUNDOS`.
6. `verify.sh` — gate de 4 estágios com `--all-targets`.
7. `.github/workflows/ci-gates.yml` — jobs `gates` + `rust`.
8. `docs/GDD - Insurgência.md` — §1, §2.6, §3.2/§3.3, §4.1/§4.4, §5.1, §17.
9. `docs/research/index.md` e notas de 2026-11-20 (contexto preexistente; não alteradas).

**Web (primárias, acessadas 2026-11-21):**
10. Bevy — `examples/app/headless.rs`: https://github.com/bevyengine/bevy/blob/main/examples/app/headless.rs
11. Bevy — `MinimalPlugins`: https://dev-docs.bevy.org/bevy/prelude/struct.MinimalPlugins.html
12. Steam Next Fest — https://store.steampowered.com/sale/nextfest
13. Balatro (site oficial/FAQ) — https://www.playbalatro.com/faq

**Não consultadas diretamente nesta tarefa:** fontes ZH/JA/KO de GTM/concorrência — ver pares. Nenhuma afirmação numérica do vídeo do Slotbound foi adotada como fato sem verificação de fonte primária; onde ausente, `[NV]`.
