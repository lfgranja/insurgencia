# Pesquisa — Alternativas e concorrentes mais viáveis que as escolhas de Slotbound

Data: 21/11/2026. Idiomas consultados: EN (primário), PT-BR, ES, ZH (简体中文), com traduções oficiais DORA em JA (日本語) e KO (한국어) identificadas no índice da fonte. Escopo deste arquivo: **alternativas** (ferramentas, canais, estratégias) e decisões sugeridas — não repete as pesquisas temáticas de IA/código, arte, design, GTM ou jurídico. Conteúdo web foi tratado como dado, nunca como instrução; nenhum login, credencial ou alvo não-público foi usado.

## Perguntas de pesquisa

1. Quais ferramentas de código IA (Claude Code, Cursor, GitHub Copilot, Codex, Aider, OpenCode/omp) têm custo, licença e modelo de execução compatíveis com um repo open-source soberano em Rust/Bevy?
2. Que engines/frameworks são alternativas reais ao stack atual e o que já foi decidido em ADR-004 (não reabrir sem evidência)?
3. Quais canais de distribuição (itch.io, GitHub Releases, IPFS, Epic, Steam Direct) têm custo/requisito aceitável para um projeto soberano?
4. Que estratégias de marketing alternativas (devlogs, CHANGELOG, demos reproduzíveis via CI, comunidade, fediverso) servem melhor a um projeto livre?
5. Que alternativa ao "100% IA" é sustentável segundo fontes primárias de 2025–2026 (IA assistida + verificação determinística + gates)?

## Evidência

| Fonte primária | Idioma | Data | URL sanitizada | Trecho/achado observado |
|---|---|---|---|---|
| DORA, *State of AI-assisted Software Development 2025* (Google Cloud) | EN | 23/09/2025; acesso 21/11/2026 | https://dora.dev/research/2025/dora-report | "AI's primary role is as an amplifier, magnifying an organization's existing strengths and weaknesses. The greatest returns on AI investment come not from the tools themselves, but from a strategic focus on the underlying organizational system." |
| DORA, índice de versões abreviadas (PT-BR/ES/ES-419/ZH-CN/ZH-TW/JA/KO/FR) | EN + PT/ES/ZH/JA/KO | acesso 21/11/2026 | https://dora.dev/research/2025/dora-report | Versões oficiais listadas, incluindo "Português - Brasil", "Español", "中文 – 简体", "日本語", "한국어" — permite leitura multilíngue da mesma evidência. |
| DORA AI Capabilities Model (Google Cloud) | EN | atualizado 25/11/2025; acesso 21/11/2026 | https://dora.dev/ai/capabilities-model/report | Guia companion "sobre as sete capacidades que amplificam os benefícios da IA" — o ganho depende do sistema de engenharia, não da ferramenta. |
| DORA 2025 (PDF), citando pesquisa DORA 2024 | EN | 2025 | https://services.google.com/fh/files/misc/2025_state_of_ai_assisted_software_development.pdf | Trecho localizado via resultado de busca: "estimated 1.5% reduction in software delivery throughput and an estimated 7.2% increase in software delivery instability for every 25% increase in AI adoption". **[NV — trecho via índice de busca, PDF não lido integralmente]** |
| METR, mudança de desenho do experimento de produtividade | EN | 24/02/2026 | https://metr.org/blog/2026-02-24-uplift-update | Estudo inicial (02–06/2025): uso de IA causou slowdown de ~19–20% em desenvolvedores OSS experientes (IC +2% a +39%); reestimativa tardia sugere speedup, mas com "severity of these selection effects" e desenho redesenhado. 57 devs, 143 repos, 800+ tarefas. |
| Aider — repositório oficial | EN | acesso 21/11/2026 | https://github.com/aider-ai/aider | Licença **Apache-2.0**; 50.206 estrelas; Python; "can connect to almost any LLM, including local models"; commits automáticos no Git; lint/test integrados ("Automatically lint and test your code every time aider makes changes"). |
| OpenCode — repositório oficial | EN (READMEs em PT-BR, ES, ZH, JA, KO) | acesso 21/11/2026 | https://github.com/anomalyco/opencode | "The open source AI coding agent"; licença **MIT**; 228.342 estrelas; agentes `build` (acesso total) e `plan` (somente leitura, pede permissão para bash); documentação em 20+ idiomas. |
| GitHub Copilot — página oficial de planos | EN | acesso 21/11/2026 | https://github.com/features/copilot | Plano Free $0 (2.000 compleções/mês trecho via índice); Pro $10/mo; Pro+ $39; Max $100. |
| AIPickr, comparação de assinaturas (secundária; cita páginas oficiais, "last checked 2026-11") | EN | 12/11/2026, atualizado 14/11/2026 | https://aipickrhq.com/best-ai-coding-assistants | Claude Code: Pro $20 ($17/ano), Max $100, Ultra $200, sem plano Free. Cursor: Free, Plus $8, Premium $30, Super $100, Ultra $200; "Data Sharing was on by default" no teste Free. Codex incluído no ChatGPT Plus $20. Copilot mede agent work em GitHub AI Credits ($15/mo no Pro). |
| devtk.ai, comparação de planos (secundária; cita páginas oficiais, 核验 2026-11-20) | ZH 简体中文 | 20/11/2026 | https://devtk.ai/zh/coding-tools/ | Tabela detalhada: Claude Pro $20/Max $100/Ultra $200; Copilot Free/Pro $10/Pro+ $39/Max $100, "1 credit = $0.01"; Cursor Plus $8 (sem modelos de terceiros)/Premium $30; Codex inclusno no Plus $20; Antigravity gratuito/AI Pro $19,99. Observação: "厂商没有公布的信息直接标注'未公布',不做推测". |
| 6yuwei, comparação de assinaturas IA (secundária, ZH) | ZH 简体中文 | 14/09/2026; verificação declarada 03/11/2026 | https://blog.6yuwei.com/zh-cn/2026/09/14/ai-coding-tools-subscription-comparison | "# AI 编程订阅对比 2026：Claude Code、Codex、Cursor、Copilot 价格与额度" — confirma convergência de preços no mercado ZH. |
| Webreactiva, "Mejor IA para programar" (secundária, ES) | ES | 04/01/2026, atualizado 14/11/2026 | https://www.webreactiva.com/blog/mejor-ia-para-programar | Comparativo em espanhol de Claude Code/Codex/Cursor; útil como contraponto de percepção hispanófona. **[NV — corpo integral não lido]** |
| DataPath, "Claude Code vs Cursor vs GitHub Copilot 2026: cuál elegir" (secundária, ES) | ES | accessed 21/11/2026 (resultado de busca) | https://www.datapath.ai/blog/claude-code-vs-cursor-vs-copilot-2026 | Título e resumo disponíveis; corpo **[NV — acesso integral falhou]** |
| ADR-004 — migração para Bevy 0.20.1 + Lightyear 0.31 (fonte local primária) | PT-BR | 20/11/2026 | `docs/adr/004-migracao-bevy-lightyear.md` | Status **Accepted**; "nenhuma troca adicional de engine é permitida sem novo ADR". Fyrox rejeitada (C2=4*, sem crate de rede); Bevy sem Lightyear rejeitada. |
| Pesquisa de alternativas de engine (fonte local primária) | PT-BR | 19/11/2026 | `docs/avaliacao-engines/pesquisa/alternativas.md` | macroquad **excluída como engine-base** (C3=3, C4=3): "biblioteca de renderização com primitivos 2D/3D, não engine". Three.js e Redot também excluídos como base. |
| Steamworks — Steam Direct Fee | EN | acesso 21/11/2026 | https://partner.steamgames.com/doc/gettingstarted/appfee | "pay a $100 USD (or equivalent) fee for each new app"; não reembolsável, mas recoupable após "$1,000.00 Adjusted Gross Revenue"; VAT/GST aplicáveis. |
| Steamworks — Distributing Open Source Applications on Steam | EN | acesso 21/11/2026 | https://partner.steamgames.com/doc/sdk/uploading/distributing_opensource | "permissive licenses [...] work fine. Common permissive and acceptable licenses includes MIT License, BSD 3-clause and 4-clause, Apache 2.0 and WTFPL." Copyleft (GPL) é "problematic" com o SDK. **O núcleo MIT da Insurgência é compatível.** |
| itch.io — Open Revenue Sharing e pagamentos | EN | acesso 21/11/2026 | https://itch.io/docs/creators/payments | "You (the seller) can decide what percentage of your sales will go towards itch.io, from 0% to 100%"; padrão 10%; taxa do processador "around 30 cents and a percentage amount around 2.9%"; arquivos hospedados pelo itch.io. |
| itch.io — documentação para criadores / devlogs | EN | acesso 21/11/2026 | https://itch.io/docs/creators/faq · https://itch.io/devlogs | Hosting de arquivos e seção oficial "Developer Logs" — canal de devlog sem custo de publicação. |
| GitHub Docs — About releases | EN | acesso 21/11/2026 | https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases | Releases baseados em tags; "Up to 1000 release assets"; "< 2 GiB" por arquivo; "no limit on the total size of a release, nor bandwidth usage"; notas de release e Releases API para contagem de downloads. |
| GitHub Docs — Artifact attestations / SLSA | EN | acesso 21/11/2026 | https://docs.github.com/en/actions/concepts/security/artifact-attestations · https://slsa.dev/how-to/get-started | "unfalsifiable provenance and integrity guarantees"; guia para SLSA Build Level 3 com reusable workflows. |
| IPFS Docs — DNSLink e IPNS | EN | data da página **[NV]**; acesso 21/11/2026 | https://docs.ipfs.tech/concepts/dnslink/ · https://docs.ipfs.tech/concepts/ipns/ | DNSLink mapeia nome DNS para endereço IPFS/IPNS via TXT; IPNS = hash de chave pública apontando para `/ipfs/CID` com expiração. Trechos via índice de busca; páginas não lidas integralmente **[NV parcial]**. |
| Epic Games Store — distribuição | EN | 03/06/2025 (anúncio); acesso direto falhou (403) | https://store.epicgames.com/distribution?lang=en-US | Trecho do resultado oficial: "Keep 100% revenue share on your first $1M in net revenue per product, per year. Afterwards, return to our standard 88%/12% model." **[NV — acesso direto falhou]** |
| Bugnet, "How to Use a Devlog to Build an Audience" (secundária) | EN | 30/05/2026 | https://bugnet.io/blog/how-to-use-a-devlog-to-build-an-audience | "A devlog builds an audience by sharing the honest process—problems, decisions, and small wins" — coerente com verificação honesta (V/VI da constituição). |

## Achados

1. **Nenhuma ferramenta IA proprietária é necessária no caminho crítico.** Aider (Apache-2.0) e OpenCode (MIT) são open-source, suportam modelos locais/BYOK (Aider explicitamente; OpenCode por configuração, ver [NV parcial]) e não criam dependência de serviço externo — compatível com a soberania (art. IV). Claude Code, Cursor, Copilot e Codex exigem assinatura (US$ 8–20/mo na entrada; US$ 100–200 no uso intensivo) e impõem limites de janela/ créditos.
2. **O diferencial não é a ferramenta, é o sistema.** DORA 2025 e o AI Capabilities Model (7 capacidades) dizem explicitamente que IA é amplificador; o retorno vem do sistema organizacional. Isso é evidence-based contra a tese "100% IA" como estratégia.
3. **Velocidade de código IA não é fato estável.** METR mediu slowdown de ~19–20% em 2025 e, em 2026, sinal de speedup, mas com viés de seleção tão grave que o desenho foi redesenhado. Qualquer métrica "100K em 3 dias" deve ser tratada como viés de sobrevivência, não como referência.
4. **Verificação determinística é um diferencial competitivo de soberania.** Aider já integra lint/test a cada mudança; no projeto, `verify.sh` (check → clippy → nextest → fmt) e as regras da ADR-004 (`unsafe_code = forbid`, clippy pedantic deny, sem unwrap/expect) são o gate que transforma saída de LLM em evidência. GitHub artifact attestations/SLSA estendem isso à distribuição.
5. **Engine fechada: não reabrir.** ADR-004 é Accepted desde 20/11/2026 e impede nova troca sem ADR; Fyrox e macroquad já foram avaliadas e excluídas com critérios documentados. A alternativa viável não é trocar de engine, é melhorar o determinismo dentro de Bevy+Lightyear.
6. **Distribuição soberana tem ordem natural: GitHub Releases → itch.io → Steam → Epic/IPFS.** GitHub Releases é gratuito, sem limite de bandwidth e baseado em tags (rastreável); itch.io permite 0% de comissão (restam taxas de pagamento) e tem devlogs nativos; Steam custa US$100 por app e usa SDK proprietário, embora aceite MIT; Epic exige plataforma e EULA; IPFS depende de gateway/IPNS para nomes estáveis.
7. **Publicar o código é uma estratégia anti-clone mais barata que litígio.** MIT + releases versionados + artefatos atestados criam proveniência verificável sem depender de aplicação de IP em WeChat/mobile (fora de alcance e custo, [NV]).
8. **Devlogs e CHANGELOG substituem "vídeo curto bem editado" com vantagem de honestidade.** Um devlog baseado em capturas reproduzíveis sob `--minimizado`, diffs e saída de `verify.sh` é falsificável e auditável; um vídeo editado não é.

## Implicações para Insurgência

Mapa explícito ID → impacto das alternativas deste arquivo (IDs fora do escopo recebem apenas uma nota de fronteira; não são repesquisados aqui):

| ID | Alternativa mapeada | Impacto concreto |
|---|---|---|
| `sb-01` | ALT-1 Aider, ALT-2 OpenCode, ALT-3/4/5/6 assinaturas | Substituir "100% IA" por "IA assistida por agente open-source + gate". Aderência direta ao art. IV. |
| `sb-02`, `sb-03`, `sb-04` | fora do escopo | Ver pesquisa de arte/UI; sem alternativa de fornecedor avaliada aqui. |
| `sb-05`, `sb-06` | fora do escopo | Ver pesquisa de GTM/promo. |
| `sb-07` | ALT-2 (modo `plan` de OpenCode), ALT-7 (manter Bevy) | Prototipagem com agente somente-leitura para explorar código; sem troca de engine. |
| `sb-08` | ALT-1 + `verify.sh` + ALT-13/14 | **Maior ganho**: comprehension debt combatido por diff revisável, lint/test automático e gate determinístico. DORA/METR sustentam. |
| `sb-09`–`sb-15` | fora do escopo | Ver pesquisa de design; alternativa de engine (ALT-7) não muda o gênero. |
| `sb-16` | ALT-8 GitHub Releases, ALT-9 itch.io, ALT-10 IPFS | Demo grátis publicada primeiro como release versionada/atestada; itch.io como espelho de descoberta. |
| `sb-17` | ALT-11 Steam Direct, ALT-12 Epic | Steam como canal tardio (US$100 + SDK), não pré-requisito; MIT compatível segundo doc oficial. |
| `sb-18` | ALT-8/9/11 | Runway otimizado: zero custo de distribuição até validação; não perseguir métrica de wishlist como prova de valor. |
| `sb-19` | ALT-15 devlogs + ALT-16 comunidade/fediverso | Substituir "streamers + algoritmo" por audiência própria e incrementos auditáveis. Alcance no fediverso **[NV]**. |
| `sb-20` | ALT-13 devlogs | Captura reproduzível sob `--minimizado` em vez de vídeo editado — mesma função provada, menor risco de factualidade. |
| `sb-21` | ALT-17 proveniência aberta | Em vez de transformar clones em marketing, tornar o original verificável (MIT +Release + atestação). |
| `sb-22` | ALT-14 CHANGELOG + releases + atestação | Hotfix vira tag + notas + artefato atestado; rastreável e reversível. |
| `sb-23` | READMEs traduzidos (OpenCode PT/ES/ZH/JA/KO como padrão), itch.io, DNSLink | Localização documental dos repos e da página, sem infraestrutura central. |
| `sb-24`, `sb-25`, `sb-26` | fora do escopo (jurídico) | ALT-11 toca só na compatibilidade de licença com Steam; disclosure/IP ver pesquisa jurídica. |
| `sb-27` | ALT-1/2 (gratuitos, locais) | Dev solo iniciante reduz burn: US$0 em ferramenta obrigatória + gates já existentes. |
| `sb-28` | DORA/METR + ALT-14/15 | Substituir 100K/3 dias por métricas verificáveis do próprio repo (testes, gates, downloads de releases via API). |

## Riscos, contra-evidência e [NV]

- **Contra-evidência ao "local-first":** Aider e OpenCode suportam modelos locais, mas a qualidade em Rust/Bevy com modelos menores não foi medida aqui **[NV — sem benchmark local]**. A recomendação é torná-los opcionais, não obrigatórios.
- **Custo real de IA assistida:** planos de US$8–20/mo parecem baixos, mas o modelo de créditos (Copilot AI Credits, limites de 5h no Claude, tokens de terceiros no Cursor) torna o teto imprevisível; as tabelas são compilações secundárias que citam páginas oficiais, não extração direta de todas as páginas **[NV parcial]**.
- **Aider em "maintenance mode":** afirmado por comparador secundário (1devtool.com); não confirmado no repositório oficial lido — **[NV]**. Não usar como argumento isolado de decisão.
- **DORA 1,5%/7,2%:** número citado no PDF via índice de busca, não lido na íntegra **[NV]**. A afirmação qualitativa ("amplificador") foi lida diretamente na página oficial e é a que sustenta a recomendação.
- **IPFS:** DNSLink/IPNS resolvem nomes estáveis, mas gateways públicos reintroduzem ponto único de falha se não houver gateway próprio — inferência de arquitetura **[INFERENCE]**; custo de operação não medido **[NV]**.
- **Epic:** 100%/US$1M é trecho de página oficial obtido por resultado de busca; acesso direto bloqueado por Cloudflare (403) **[NV — acesso falhou]**.
- **itch.io/Devlogs:** taxa de processamento ~US$0,30 + 2,9% é da documentação oficial; comissão 0% é permitida, mas o efeito de descoberta (tráfego) com 0% não foi medido **[NV]**.
- **Fediverso:** nenhuma fonte primária de alcance/Growth foi verificada neste arquivo **[NV]** — tratar como canal de relação, não de métrica.
- **Engine:** nenhuma evidência nova que justifique reabrir ADR-004; o custo de reescrita (31 arquivos `.gd` já migrados, churn de API Bevy 0.x) permanece registrado na ADR.

## Alternativas/concorrentes mais viáveis

### Matriz alternativa → ID → custo/benefício

| # | Alternativa | IDs | Custo | Benefício | Fit soberano | Sugestão |
|---|---|---|---|---|---|---|
| ALT-1 | Aider (Apache-2.0) como agente git-native | sb-01, sb-07, sb-08, sb-27 | US$0 de licença; token/modelo à parte | Commits atômicos, repomap, lint/test automático, modelos locais | Alto | **ADAPT** — opcional offline; exigir `verify.sh` após cada mudança. |
| ALT-2 | OpenCode (MIT) com agente `plan` (somente leitura) | sb-01, sb-07, sb-08, sb-27 | US$0 de licença; BYOK | Agente read-only para exploração, 20+ idiomas de docs | Alto | **ADAPT** — uso permitido em exploração; saída nunca autoritativa. |
| ALT-3 | GitHub Copilot Free/Pro | sb-01, sb-07 | $0 / $10 por mês; créditos | Integração em IDEs, BYOK no Free/Pro | Médio (provedor no caminho) | **DEFER** — só se ganho medido em tarefa Rust/Bevy. |
| ALT-4 | Claude Code Pro | sb-01, sb-07, sb-08 | $20/mo ($17/ano), limites 5h/semana | Agente terminal forte para refactors multi-arquivo | Baixo (sem modelo local, assinatura) | **DEFER** — ferramenta pessoal, nunca dependência do repo. |
| ALT-5 | Cursor | sb-01, sb-07 | $0–$200/mo; editor próprio | Editor integrado | Baixo (lock-in de editor, Data Sharing default on) | **AVOID** — conflita com soberania e com regra da janela (editor próprio não agrega ao gate). |
| ALT-6 | Codex (embutido no ChatGPT Plus) | sb-01, sb-07 | $20/mo (já pago, se assinante) | Nenhuma fatura adicional para quem já paga | Baixo/médio | **DEFER** — valor acessório, não critério de adoção. |
| ALT-7 | Manter Bevy 0.20.1 + Lightyear 0.31 (ADR-004) | sb-07 + todos | Zero (já decidido) | Rede autoritativa, ECS determinístico | Alto | **ADOPT** — qualquer troca por Godot/Fyrox/macroquad: **REJECT** sem novo ADR com evidência. |
| ALT-8 | GitHub Releases como distribuição primária | sb-16, sb-18, sb-22, sb-23 | US$0; 2 GiB/asset, sem limite de bandwidth | Tags, notas, API de downloads, proveniência | Altíssimo | **ADOPT** — publicar binários + checksums em cada tag. |
| ALT-9 | itch.io (0–100% de comissão) como espelho | sb-16, sb-20, sb-23 | 0% plataforma + ~$0,30/2,9% processador | Devlogs nativos, hosting de arquivos, descoberta | Alto (hosting externo, mas opcional) | **ADAPT** — espelho de descoberta; nunca fonte única. |
| ALT-10 | IPFS + DNSLink/IPNS | sb-16, sb-23 | US$0 de protocolo; custo de gateway próprio **[NV]** | Resiliência a remoção/censura | Médio (gateway) | **DEFER** — adotar só com gateway próprio e risco de nome IPNS aceito. |
| ALT-11 | Steam Direct | sb-17, sb-18, sb-24 | US$100 por app + SDK proprietário | Maior audiência; MIT compatível com SDK | Médio/baixo | **DEFER** — após release estável e resources de store; verificar licenças de terceiros antes. |
| ALT-12 | Epic Games Store | sb-17, sb-18 | 0% até US$1M/produto/ano, depois 12% **[NV parcial]** | Termos financeiros agressivos | Baixo (plataforma + EULA) | **DEFER** — depende de acesso à plataforma e avaliação de EULA. |
| ALT-13 | Devlogs honestos c/ captura reproduzível | sb-19, sb-20, sb-28 | US$0; tempo de escrita | Audiência própria; falsificável; casa com art. VI | Alto | **ADAPT** — captura sempre sob `--minimizado`, com seed/comando anexado. |
| ALT-14 | CHANGELOG + notas de release + artifact attestations | sb-08, sb-20, sb-22 | US$0 (GitHub Actions) | Proveniência "unfalsifiable", SLSA L3 alcançável | Alto | **ADOPT** — integrar ao fluxo de release já existente. |
| ALT-15 | Demo reproduzível via CI (gate primeiro) | sb-16, sb-17, sb-28 | US$0 em repo público; tempo de CI | Substitui demo "bonitinha" por evidência executável | Alto | **ADAPT** — depende do cenário headless determinístico; **DEFER** até esse pré-requisito. |
| ALT-16 | Comunidade/fediverso (Mastodon etc.) | sb-19, sb-23 | US$0; moderação | Relação direta sem algoritmo de loja | Alto, alcance **[NV]** | **ADAPT** — canal de apoio, não KPI. |
| ALT-17 | Proveniência aberta anti-clone (MIT + releases + atestação) | sb-21, sb-26 | US$0 | Autoria verificável sem litígio caro | Alto | **ADAPT** — preferir a litígio transfronteiriço (custo/jurisdição **[NV]**). |

### Sugestões de decisão (focadas nas alternativas)

- **ALT-7 (manter Bevy+Lightyear) — ADOPT.** Nenhuma evidência nova reabre ADR-004; Fyrox/macroquad/Godot como base da engine: **REJECT** sem novo ADR.
- **ALT-1 e ALT-2 (Aider/OpenCode) — ADAPT.** Adotar como ferramentas opcionais de desenvolvedor, com gate `verify.sh` obrigatório e saída tratada como informativa (art. VI).
- **ALT-3 a ALT-6 (assinaturas) — DEFER/AVOID.** Custo recorrente e dependência de provedor sem ganho medido em Rust/Bevy; Cursor especificamente **AVOID** por lock-in de editor.
- **ALT-8 (GitHub Releases) — ADOPT**, imediatamente; **ALT-14 (CHANGELOG + atestação) — ADOPT** no mesmo fluxo de tag.
- **ALT-9 (itch.io) — ADAPT** como espelho; **ALT-15 (demo CI) — DEFER** até existir cenário headless determinístico.
- **ALT-10 (IPFS), ALT-11 (Steam), ALT-12 (Epic) — DEFER** por dependência de infraestrutura/plataforma e custos não validados para o estágio atual.
- **ALT-13 (devlogs) e ALT-16 (comunidade/fediverso) — ADAPT** com captura reproduzível sob `--minimizado`; **ALT-17 (proveniência aberta) — ADAPT** no lugar de perseguição legal a clones.

## Fontes

**Primárias**
1. DORA — State of AI-assisted Software Development 2025: https://dora.dev/research/2025/dora-report
2. DORA — AI Capabilities Model: https://dora.dev/ai/capabilities-model/report
3. METR — Changing Developer Productivity Experiment Design (24/02/2026): https://metr.org/blog/2026-02-24-uplift-update
4. Aider (Apache-2.0): https://github.com/aider-ai/aider
5. OpenCode (MIT): https://github.com/anomalyco/opencode
6. GitHub Copilot (planos): https://github.com/features/copilot
7. Steamworks — Steam Direct Fee: https://partner.steamgames.com/doc/gettingstarted/appfee
8. Steamworks — Distributing Open Source Applications on Steam: https://partner.steamgames.com/doc/sdk/uploading/distributing_opensource
9. itch.io — Open Revenue Sharing: https://itch.io/docs/creators/payments
10. itch.io — Developer Logs: https://itch.io/devlogs
11. GitHub Docs — About releases: https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases
12. GitHub Docs — Artifact attestations: https://docs.github.com/en/actions/concepts/security/artifact-attestations
13. SLSA — Get started: https://slsa.dev/how-to/get-started
14. IPFS Docs — DNSLink: https://docs.ipfs.tech/concepts/dnslink/ · IPNS: https://docs.ipfs.tech/concepts/ipns/
15. Epic Games Store — distribuição **[acesso 403]**: https://store.epicgames.com/distribution?lang=en-US
16. Locais: `docs/adr/004-migracao-bevy-lightyear.md`; `docs/avaliacao-engines/pesquisa/alternativas.md`

**Secundárias (com fontes oficiais citadas)**
17. AIPickr (EN, 12–14/11/2026): https://aipickrhq.com/best-ai-coding-assistants
18. devtk.ai (ZH, 20/11/2026): https://devtk.ai/zh/coding-tools/
19. 6yuwei (ZH, 14/09/2026): https://blog.6yuwei.com/zh-cn/2026/09/14/ai-coding-tools-subscription-comparison
20. Webreactiva (ES, 14/11/2026): https://www.webreactiva.com/blog/mejor-ia-para-programar
21. DataPath (ES): https://www.datapath.ai/blog/claude-code-vs-cursor-vs-copilot-2026
22. Bugnet — devlog (EN, 30/05/2026): https://bugnet.io/blog/how-to-use-a-devlog-to-build-an-audience

**Traduções oficiais DORA (multilingue)**: PT-BR https://cloud.google.com/dora?hl=pt-br&region=BR · ES https://cloud.google.com/dora?hl=es&region=ES · ZH-CN https://cloud.google.com/dora?hl=zh-cn&region=CN · JA https://cloud.google.com/dora?hl=ja&region=JP · KO https://cloud.google.com/dora?hl=ko&region=KR
