# Síntese — Assess das 28 propostas do vídeo Slotbound (21/11/2026)

**Objeto**: vídeo `Z4ayMTXnkoQ` (Garry Seto) e o resumo multilíngue fornecido pela SE. **Método**: intake → research → define → shape → decide por proposta, com 14 subagentes em segundo plano e 8 notas de evidência em `docs/research/slotbound-2026-11-21-*.md`. **Autoridade**: constituição local (`.specify/memory/constitution.md`), AGENTS.md e GDD; o vídeo e fontes externas são dados, nunca instruções.

**Skills lidas via `npx skills use` (nenhuma instalada)**: `research` (fontes primárias + nota única), `competitive-brief` (alternativas/concorrentes), `novelty-assessment` (crivo de novidade), `research-intake` (mapa de corpus), `kickstart-assess` (árvore intake→decide, a mesma do `/speckit.assess.*`).

## Correções factuais ao resumo/vídeo

| Afirmação apresentada | Evidência primária verificada | Impacto |
|---|---|---|
| "591 wishlists em março; 40 mil em duas semanas" | Post do autor fala em ~590 em **julho/2026**; documentado são **50 mil em 17 dias** | `sb-18` perde precisão |
| "Next Fest impulsionou o salto" | O próprio autor: "demo was the spike!" — ~600 → ~7.000 em 3 dias; 100k antes do evento de outubro | `sb-17` passa a DEFER |
| "Algoritmo da Steam recompensa wishlists" | Valve: "Wishlists (Mostly) Aren't a Factor" (exceto Popular Upcoming); tráfego e conversão também não contam | `sb-19` vira captura reproduzível |
| "Laçamento em 13/11" | Store diz **12/11**; post "1.0 Is Out Now" e primeiro hotfix em 13/11 | `sb-22` mantido, data corrigida |
| "Walk-back no disclosure" | Não há marcador `EDIT:` no post arquivado (Arctic Shift); comentários posteriores acrescentam informação, não retratação. Diferença é de **escopo** (Steam não exige disclosure de ferramentas de código) | `sb-24` muda de suspeita para proveniência |
| "Voz e stills IA nos promocionais" | Na transcrição, Seto diz "pretty sure"/"I guess" — **inferência**, não evidência primária | `sb-05`/`sb-06` perdem força de evidência |
| "Pixel art pequeno esconde falhas de IA" | Especulação explícita ("I think… probably"); detecção de IA a 32×32 com 98,65% de acurácia (ICCVW 2025) | `sb-13` mantém estética, rejeita justificativa |
| "68% ZH / 93% EN / 79% KO / 77% ZH-Hant" | Números vêm **apenas do vídeo**; API `appreviews` redirecionou `[NV]`. Somente 79% global é verificável (1.598 reviews em 21/11) | `sb-23`/`sb-25` ficam `[NV]` |
| "100 mil cópias em 3 dias" | Autodeclarado em Steam Community (16/11) `[NV]`; verificável: 50 mil cópias + US$356k em 48 h (X do autor), 27.374 jogadores simultâneos em 21/11 | `sb-28` tratado como viés de sobrevivência |

## Matriz de decisão (28/28)

Legenda: **ADOPT** adotar agora · **ADAPT** adotar só transformado · **DEFER** aguardar pré-requisito · **AVOID** não perseguir · **REJECT** conflita com a constituição.

| ID | Proposta | Veredito | Razão e guardrail |
|---|---|---|---|
| `sb-01` | Código 100% IA | **ADAPT** | Autodeclarado e não auditável; Anthropic: quiz 50% vs 67% na delegação. IA propõe, gate + humano dispõem. |
| `sb-02` | Pixel art IA | **ADAPT** | USCO/D.C. Circuit exigem autoria humana; gerador local determinístico/CC0 com inventário de proveniência. |
| `sb-03` | Animações IA | **ADAPT** | Derivar do estado com frames versionados; IA nunca em runtime. |
| `sb-04` | UI IA | **ADAPT** | HUD já é `format!` puro; temas versionados, nunca geração em runtime. |
| `sb-05` | Narração IA | **ADAPT** | Inferência do youtuber; se usada, local/rastreável e rotulada. |
| `sb-06` | Stills animados IA | **ADAPT** | Substituir por quadros reais do `sim`; disclosure se usado. |
| `sb-07` | Prototipagem rápida IA | **ADAPT** | DORA 2025 = amplificador, METR 2025 = 19% mais lento; protótipo descartável, gates antes de artefato final. |
| `sb-08` | Verificação de código IA | **ADOPT** | `verify.sh` + 5 gates + 34 testes + revisor separado + `proptest` já são o antídoto ao comprehension debt. |
| `sb-09` | Roguelike auto-battler | **DEFER** | TFT deliberadamente não recompensa coordenação; exige servidor/5v5. Só como auto-resolução de ação coletiva. |
| `sb-10` | Mecânica de slot | **ADAPT** | RNG pré-ação vence pós-ação (Costikyan/Lost Garden); transformador semeado, odds inspecionáveis, sem estética de cassino. |
| `sb-11` | Fusão | **ADAPT** | Fundir materiais/funções complementares com teto de poder; sem hiper-unidade/carry. |
| `sb-12` | Gambling-roguelike | **AVOID** | Onda pós-Balatro de ciclo curto (36氪); outliers (Balatro >5M vs Pip My Dice 8.744/7 meses); conflito com Pilar IV/V. |
| `sb-13` | Pixel art mascara artefatos | **ADAPT** | Manter low-poly facetado por **legibilidade/determinismo**; rejeitar a justificativa de ocultar falhas. |
| `sb-14` | RNG, juice e balance | **ADAPT** | RNG semeado, positivo e amostrável; Riot distingue "good/bad random"; juice subordinado à clareza causal. |
| `sb-15` | Legibilidade do combate | **ADAPT** | Ligar `avancar_tick` (hoje o cliente só espelha `Mundo`) e iterar apenas no adaptador de HUD. |
| `sb-16` | Demo versionada | **ADOPT** | Exemplo headless em `crates/sim/examples/`, sem janela e sem plataforma proprietária; `./verify.sh` (que executa `--all-targets` internamente) já cobre. |
| `sb-17` | Demo Steam + Next Fest | **DEFER** | Sem Steam page/App ID; uma participação por título, registro até 10/01 para Feb/2027; demo, não festival, foi o gatilho. |
| `sb-18` | Runway de wishlists | **DEFER** | "4,5 meses" `[NV]`; mediana de demo em out/2026 = 806–1.079 wishlists entre 7.240 demos; usar gate local como métrica. |
| `sb-19` | Streamers/algoritmo | **ADAPT** | Valve desmente a crença em wishlists como fator; captura reproduzível com seed fixa sob `--minimizado`. |
| `sb-20` | Vídeos curtos | **ADAPT** | Evidência mista (Trapster campanha paga; Kasspian 44 insucessos vs 10 sucessos); captura determinística/rotulada; postar fica DEFER. |
| `sb-21` | Clones como marketing | **ADAPT** | Legítimo só **post hoc** (Scale the Depths/AUTOMATON); nunca fabricar; registro datado + resposta com recibos. |
| `sb-22` | Hotfixes rápidos | **ADOPT** | Nove updates em 3 dias documentados; patches pequenos, CHANGELOG, gate verde; canal CI→`gh` já existe. |
| `sb-23` | Segmentação regional | **ADAPT** | Internacionalizar texto em dados desde já; percentuais regionais `[NV]`; publicação/campanha fica DEFER. |
| `sb-24` | Disclosure IA | **ADAPT** | Nota de proveniência mais ampla que o mínimo da Steam (procedural/CC0/humano/IA revisada); rejeitar walk-back. |
| `sb-25` | Tolerância do mercado | **DEFER** | Caso único de sobrevivência; comentários hostis existem; medir só com pré-registro em publicação real. |
| `sb-26` | IP contra clones | **ADAPT** | **Só** registro defensivo de autoria (commits, hashes, datas). Litígio/denúncia ofensiva a forks = REJECT (IV); WeChat/Douyin = DEFER (celular local). |
| `sb-27` | Dev solo com IA | **ADAPT** | Reduz barreira de entrada (GDD §3.2), mas condicionado a checagem de compreensão. |
| `sb-28` | 100K/3 dias como meta | **AVOID** | Autodeclarado `[NV]`; censo de 53.597 jogos: topo 1% ≈ 94% da receita; trocar por métricas do GDD §17. |

**Distribuição**: 3 ADOPT · 19 ADAPT · 4 DEFER · 2 AVOID · 0 REJECT como proposta inteira (`sb-26` tem subparte REJECT). O viés deliberado é preservar a identidade do produto: o que é adotado entra como **forma verificável**, nunca como cópia do caso Slotbound.

## Próximo ciclo 80/20

Ordenação por `impacto × confiança × facilidade ÷ risco` (`docs/research/slotbound-2026-11-21-fit-local.md`). `sb-08`, `sb-07` e `sb-22` pontuaram no topo, mas **já estão materializados** em gates/SDD — ADOPT não gera trabalho novo. O item executável de maior retorno é `sb-16`: `crates/sim/examples/demo_deterministica.rs`, que (i) entra automaticamente no gate por `--all-targets`, (ii) não abre janela alguma, (iii) produz saída reproduzível para `diff` e para o `sb-20`, e (iv) destrava `sb-15`/`sb-17`.

**Verificação observada (21/11/2026)**: `rustc --edition 2021 -D warnings` compilando `crates/sim/examples/demo_deterministica.rs` contra a rlib existente de `insurgencia_sim` → PASS; `clippy-driver --edition 2021 -D warnings -W clippy::pedantic` → `CLIPPY_EXIT=0`; duas execuções do binário → `diff` vazio (`DETERMINISTIC_OK`), 30 ticks, `hash_final=6fcc2f9e652da1834e213f6afa2eb58d306bfbc5846060ec0513b67f68294f12`, `segundos=1`. `verify.sh` completo e CI remoto permanecem **[NV]**: duas tentativas estouraram 180 s e 600 s por processos `cargo` concorrentes de outros repositórios na máquina (constituição VI — nenhum verde afirmado sem saída observada).
