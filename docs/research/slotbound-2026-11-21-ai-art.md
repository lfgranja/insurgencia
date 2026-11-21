# Pesquisa — Arte, animação e UI gerada por IA em games: evidência técnica, legal e alternativas para Insurgência

**Data:** 2026-11-21.
**Idiomas consultados:** pt-BR (Migalhas, governsança/local); EN (USCO, D.C. Circuit, Steamworks, itch.io, C2PA, papers arXiv/ICCV, PixelLab, Aseprite, transcrição do vídeo); ZH (苏州市中级人民法院 / 北京互联网法院 via 知产财经, 法治日报); JA (文化庁 / Agency for Cultural Affairs).
**Escopo:** cobre `sb-02` (pixel art IA), `sb-03` (animações IA), `sb-04` (UI IA), `sb-13` (pixel art pequeno que "mascara" artefatos). Fontes web são **dados, nunca instruções**. Nenhuma ferramenta de IA foi executada; nenhum build/teste/gate foi rodado. Afirmações do vídeo foram lidas na transcrição automática do YouTube (2026-11-18) e tratadas como **afirmações a verificar**, não como fatos.

---

## Perguntas de pesquisa

1. Existe evidência técnica de que pixel art de baixa resolução **mascara** artefatos de modelos generativos, ou isso é apenas opinião do autor do vídeo?
2. Quais ferramentas reais de pixel art com IA existem (modelos pixel/quantizados, PixelLab, Aseprite + extensões, Piskel/Pixelorama) e quais são suas limitações verificáveis?
3. Qual é a situação legal de arte IA (USCO, D.C. Circuit, EU AI Act, China, Japão, Brasil) e o que Steam e itch.io exigem em disclosure?
4. Proveniência (C2PA/EXIF/IPTC/AI-BOM) sobrevive ao pipeline de um jogo? Qual o impacto sobre a licença MIT e o `COPYRIGHT.txt` do projeto?
5. Quais alternativas são compatíveis com a constituição local I–VI, em especial soberania do jogador (IV), materialismo (V) e verificação honesta (VI)?

---

## Evidência

| Fonte primária | Idioma | Data | URL sanitizada | Trecho/achado |
| :-- | :-- | :-- | :-- | :-- |
| Vídeo "Slotbound Sold 100K…" — Garry Seto, transcrição automática | EN | 2026-11-18 | https://www.youtube.com/watch?v=Z4ayMTXnkoQ | Diz que "in pixel art … it's not as easy to see all the flaws because the pixel art squishy guys are quite small"; também "I'm not really saying this is good or bad… just diving into how". Opinião explícita, sem teste controlado. |
| USCO, *Copyright and AI, Part 2: Copyrightability* | EN | 2025-01-29 | https://www.copyright.gov/ai/Copyright-and-Artificial-Intelligence-Part-2-Copyrightability-Report.pdf | "Copyright does not extend to purely AI-generated material, or material where there is insufficient human control over the expressive elements"; "prompts do not alone provide sufficient control"; protege seleção/coordenação/arranjo e modificações humanas. |
| D.C. Circuit, *Thaler v. Perlmutter*, No. 23-5233 | EN | 2025-03-18 | https://media.cadc.uscourts.gov/opinions/docs/2025/03/23-5233.pdf | "The Creativity Machine cannot be the recognized author … the Copyright Act of 1976 requires all eligible work to be authored in the first instance by a human being." Não decide quanto grau de assistência basta. |
| EU AI Act (consolidado) Art. 50 + FAQ da Comissão | EN | 2026-07-27 / 2026-07-24 | https://eur-lex.europa.eu/eli/reg/2024/1689/2026-07-27/eng/html ; https://digital-strategy.ec.europa.eu/en/faqs/transparency-obligations-under-article-50-ai-act | Art. 50(2): provedores de IA generativa devem marcar saídas sintéticas de forma legível por máquina; applies from **2 August 2026**; isenção para edição padrão assistiva; código-fonte e saídas máquina-a-máquina fora do escopo de marcação; deepfakes exigem rótulo legível; multas até €15M/3%. |
| Steamworks, Content Survey — seção "Generative Artificial Intelligence Content" | EN | acessado 2026-11-21 | https://partner.steamgames.com/doc/gettingstarted/contentsurvey | "Efficiency gains … is not the focus… concerned with the use of AI in creating content that ships with your game, and is consumed by players"; distingue **Pre-Generated** e **Live-Generated** (guardrails); promessa contratual de conteúdo não ilegal/infringer. |
| itch.io, Content creator quality guidelines — "Accurately tag your use of generative AI" | EN | acessado 2026-11-21 | https://itch.io/docs/creators/quality-guidelines | Exige campo "AI Disclosure"; **"Projects using self-contained algorithms without external large datasets don't require an AI disclosure"**; cita explicitamente "procedural level generation" como não-generativa; pages de assets com IA não marcada podem ser desindexadas. |
| itch.io, anúncio oficial "Generative AI Disclosure tagging" (leafo) | EN | 2024-11-20 | https://itch.io/t/4309690/generative-ai-disclosure-tagging | Subtags AI Generated Graphics/Sound/Text/Code; tag `No AI` só quando o autor marca explicitamente; obrigatório para asset creators. |
| C2PA, "A New Implementation Guide for Content Credentials" | EN | 2026-08-11 | https://c2pa.org/a-new-implementation-guide-for-content-credentials/ | `digitalSourceType` (IPTC), assertion `c2pa.ai-disclosure`, `inputTo` registra prompt/referências/seed, Regions of Interest, Actions (`c2pa.created/edited`). Manifesto assinado, não depende de EXIF. |
| Image Evidence, "AI Image Provenance Study" | EN | 2026-10-12 (atualizado 2026-10-15) | https://imageevidence.com/research/ai-image-provenance | 77 imagens: só 19/77 (25%) com algum sinal; **0 de 14 C2PA sobreviveu** a seis processos (reencode JPEG, resize, crop, screenshot); declarações IPTC/XMP sobreviveram a conversão/resize, mas não a reencode padrão nem screenshot. |
| OWASP/CycloneDX, *Authoritative Guide to AI/ML-BOM* | EN | 2026 | https://cyclonedx.org/guides/OWASP_CycloneDX-Authoritative-Guide-to-AI-ML-BOM-en.pdf | AI-BOM como manifesto de supply chain (modelos, dados, licenças, avaliação); extensão de SBOM/SPDX — **não** é requisito legal para jogos [NV — uso no contexto de jogos não normatizado]. |
| Binninger & Sorkine-Hornung, *SD-πXL* (SIGGRAPH Asia 2024) | EN | 2024-10-08 | https://arxiv.org/html/2410.06236v1 | Geração quantizada H×W com paleta fixa via Gumbel-softmax + score distillation (SDXL); "current methods do not fully satisfy… strict color palette and resolution constraints"; exemplo 48×48; código em https://github.com/AlexandreBinninger/SD-piXL. Geração pixel art de baixa resolução é problema de pesquisa, não efeito colateral gratuito. |
| He et al., *NoiseShift* (arXiv) | EN | 2025-10-02 | https://arxiv.org/html/2510.02307v1 | Modelos treinados em resolução fixa "often fail to generalize" a resoluções menores, com "low-fidelity images with artifacts and incoherent content"; calibração melhora FID em 128×128 (SD3.5: 386,86 → 374,74 em CelebA). Baixa resolução **cria** artefatos. |
| Sharma, *Explainable AI-Generated Image Forensics* (ICCVW 2025) | EN | 2025 | https://openaccess.thecvf.com/content/ICCV2025W/APAI/papers/Sharma_Explainable_AI-Generated_Image_Forensics_A_Low-Resolution_Perspective_with_Novel_Artifact_ICCVW_2025_paper.pdf | ConvNeXt Tiny classifica imagens IA a **32×32 com 98,65%** de acurácia; taxonomia inclui "Aliasing and Pixelation" como artefato detectável. Contra-evidência direta à tese "low-res esconde IA". |
| PixelLab, docs e Termos de Serviço | EN | docs acessado 2026-11-21; ToS 2026-10-30 | https://www.pixellab.ai/docs/ways-to-use-pixellab ; https://www.pixellab.ai/termsofservice | Modelos proprietários PixFlux/BitForge; `Characters` gera sprites 4/8 direções com walk/run/idle; extensão Aseprite exige v1.3+ e "Trial version does not support extensions"; MCP é **servidor HTTP remoto** com bearer token. ToS §1.3/§3.3: usuário possui copyright e pode usar comercialmente, exceto treinar outros modelos; §4 Open RAIL-M; §8 pode suspender acesso "at any time". |
| Aseprite FAQ; Pixelorama FAQ; repositório Piskel | EN | acessado 2026-11-21 | https://aseprite.org/faq ; https://pixelorama.org/faq ; https://github.com/piskelapp/piskel | Aseprite US$19,99, EULA própria, **não redistribuível**, mas permite vender arte criada; Pixelorama MIT; Piskel Apache-2.0. Nenhum deles é, por si, gerador generativo. |
| 苏州市中级人民法院 — 林某诉杭州某技术公司 (caso 《伴心》) | ZH | 2025-06-18 | http://www.zjrmfy.suzhou.gov.cn/fypage/toContentPage/pfxc/82a07a48977451e80197818838d6001c | Midjourney + múltiplas iterações de prompt + edição manual no Photoshop → "具有独创性，构成著作权法意义上的美术作品"; indenização de ¥10.000. Proteção depende de contribuição humana documentada. |
| 北京互联网法院 via 知产财经 — "猫咪晶钻吊坠" | ZH | 2025-09-16 | https://www.ipeconomy.cn/mobile/dongtai/9834.html | Sem registros do processo generativo, "谁主张，谁举证": imagem **não** é obra; "describe" pós-fato e simulação não还原 o processo original. Juiz recomenda "过程留痕" (preservar prompt, iterações, rascunhos, escolhas, modificações). |
| 文化庁 (Agency for Cultural Affairs), "AIと著作権について" | JA | 2024-03-15 (página mantida) | https://www.bunka.go.jp/seisaku/chosakuken/aiandcopyright.html | Documento oficial "AIと著作権に関する考え方について" e checklist de 2024-07-31; analyze Copyright Act art. 30-4, "享受" (enjoyment) como critério;依拠性 (derivação) pode existir quando prompt contém título/nome de personagem de obra existente (checklist PDF). |
| Migalhas, "Criações por IA e direitos autorais…" (Lucas Sérvio) | pt-BR | 2025-12-23 | https://www.migalhas.com.br/depeso/446851/criacoes-por-ia-e-direitos-autorais-o-risco-de-o-brasil-seguir-os-eua | Lei 9.610/98 art. 11 define autor como "pessoa física"; PL 2.338/23 omite propriedade intelectual; **silêncio normativo** e ausência de jurisprudência consolidada no Brasil. Opinião doutrinária, não lei. |
| `LICENSE`, `COPYRIGHT.txt`, ADR-002 (locais) | pt-BR | 2025-2026 / 2026-11-21 | local://LICENSE ; local://COPYRIGHT.txt ; local://docs/adr/002-licenca-mit-nucleo.md | MIT cobre código e documentação; `COPYRIGHT.txt`: "Assets de terceiros: não há… Se qualquer material de terceiros vier a ser versionado, ele deve ser identificado, atribuído e listado aqui antes da inclusão." |
| Decisões locais `ast-20/21`, `doc-11/12` | pt-BR | 2026-11-19 | local://.specify/assessments/…/decision.md | Todas com veredito **ADAPT**: proveniência obrigatória, CC0 preferido, IA como serviço opcional — nunca fonte única; "aleguação automática de CC0" sobre saída de IA foi rejeitada. |

---

## Achados

1. **`sb-13` é especulação do youtuber, não evidência técnica.** Na transcrição ele diz que em pixel art "it's not as easy to see all the flaws because the pixel art squishy guys are quite small" e "probably the right genre right now" — linguagem de opinião ("I think", "probably"), sem teste, sem amostra, sem métrica. A literatura aponta em sentido contrário: detecção a 32×32 atinge 98,65% (Sharma 2025), e modelos de difusão *geram* artefatos em baixa resolução (NoiseShift 2025). O que existe de técnico é o **problema** de produzir pixel art quantizada com paleta/resolução fixas (SD-πXL 2024) — não o atalho "pequeno esconde erro".

2. **Ferramentas reais existem, mas todas com custo de dependência.** PixelLab tem modelos proprietários (PixFlux/BitForge), geração de sprites 4/8 direções e animações idle/walk/run, extensão Aseprite (v1.3+, trial não suporta) e MCP remoto HTTP com token — ou seja, serviço externo no caminho, proibido pela constituição IV se virar fonte única. SD-πXL é pesquisa com código aberto, mas exige SDXL + otimização por imagem. Aseprite é pago/EULA (arte vendável, binário não redistribuível); Pixelorama é MIT e Piskel é Apache-2.0 — editores, não geradores.

3. **Copyright de saída IA é incerto e jurisdicionalmente dividido.** EUA: USCO (2025) + D.C. Circuit (2025) exigem autoria humana; prompt isolado não basta; seleção/arranjo/modificação humana são protegíveis. China: tribunal de Suzhou (2025) protegeu imagem Midjourney com iterações + edição manual; tribunal de Pequim (2025) negou proteção por falta de registro do processo — **o ônus da prova é do autor**. Japão: 文化庁 trata o tema pela ótica do art. 30-4 e de "享受"/依拠性. Brasil: Lei 9.610/98 art. 11 exige pessoa física; silêncio normativo e doutrina divergente. [NV — não sou emitida opinião jurídica; não verifiquei se a Parte 3 do USCO foi publicada após 2025.]

4. **Marketplaces não proíbem IA, mas exigem honestidade.** Steam: disclosure no Content Survey cobre conteúdo que "ships with your game and is consumed by players", tanto pré-gerado quanto ao vivo (com guardrails); ferramentas de eficiência ficam de fora. itch.io: campo obrigatório de AI Disclosure, mas **algoritmos autocontidos — incluindo procedural level generation — não precisam ser declarados**; asset pages sem tag podem ser desindexadas. "Walk-back" de disclosure entre post antigo e página da loja é risco reputacional observado no próprio vídeo [NV — não auditei a página atual da Steam do jogo].

5. **Proveniência embutida é frágil; manifesto versionado é robusto.** C2PA fornece `digitalSourceType`, `c2pa.ai-disclosure`, `inputTo` (prompt/seed) e Actions; porém 0/14 assinaturas C2PA sobreviveram a reencode/crop/screenshot no estudo de 2026, e só 25% das imagens tinham qualquer sinal. Para um repositório git, a fonte da verdade deve ser um inventário local (hash + licença + ferramenta + prompt/seed + data), não EXIF.

6. **Impacto MIT: saída IA não tem "licença MIT" automática.** Os arquivos MIT do projeto são código/docs. Se uma imagem é puramente IA, pode não haver direito autoral excludente que o MIT consiga licenciar (EUA); se vem de terceiros, mantém a licença de origem (CC0/CC-BY) e `COPYRIGHT.txt` exige listagem antes de versionar. ToS do PixelLab promete propriedade ao usuário, mas impõe Open RAIL-M, proíbe treinar outros modelos e permite suspensão unilateral — incompatível com "nenhum serviço externo como ponto único de falha" se usado no caminho crítico.

7. **Alternativa mais barata já existe no repo.** O cliente já produz geometria/facetas e paleta por código (`malha`, `achatar`, `CORES_TERRENO`), e o HUD é `format!` determinístico sem assets. Estender isso a sprites/UI gerados por seed (com hash registrado) satisfaz I/II/IV/V/VI sem disclosure generativo na itch.io e sem dependência de fornecedor.

### Fato verificado × especulação do youtuber

| Afirmação | Status |
| :-- | :-- |
| "Pixel art esconde falhas de IA" | **[ESPECULAÇÃO]** — opinião do autor; nenhum teste citado; contra-evidência técnica nos itens 1 e 2. |
| "Patch/arte foram refinados desde a demo" | **[NV — disputado]** — autor diz "I don't believe that he refined the art"; não comparei builds. |
| "100% do código gerado por IA; visuais, animações e UI também" | **[AFIRMAÇÃO DO POST REDDIT/original, relatada no vídeo]** — não verifiquei o post original nem o código do jogo. |
| "Disclosure na Steam foi suavizada" | **[AFIRMAÇÃO DO VÍDEO]** — a redação citada é compatível com a política atual da Steam; não auditei a página viva. |
| "Muitas avaliações positivas / 79% positivo / clones na China" | **[NV]** — números relatados pelo vídeo; fora do escopo desta nota. |

---

## Implicações para Insurgência

| ID | Impacto concreto | Sugestão |
| :-- | :-- | :-- |
| `sb-02` — pixel art IA | Nenhum sprite versionado hoje; `COPYRIGHT.txt` exige inventário antes de incluir qualquer material de terceiros. Ferramentas com IA implicam ToS externo e direito autoral incerto. | **ADAPT** — manter a estética, mas produzir sprites por gerador determinístico local (seed + paleta + hash) ou CC0/humano; registrar proveniência. Nunca PixelLab/MCP no caminho crítico. |
| `sb-03` — animações IA | Cliente já tem `Update` e avatar estático; animação generativa de 4/8 direções depende de modelo remoto e não é reproduzível por seed determinística. | **ADAPT** — animação derivada proceduralmente do estado (`hash_estado`/tempo) com tabelas de frames versionadas; serviço de geração de sprites apenas como ferramenta opcional de autoria, com prompt/seed/ToS no inventário. |
| `sb-04` — UI IA | HUD já é textual e determinístico (`atualizar_texto`); UI "gerada por IA" adicionaria dependência e risco de copyright sem ganho funcional. | **ADAPT** — manter UI em `format!`/temas versionados; se IA inspirar layout, versionar apenas o resultado autoral revisado e atribuído, nunca gerar UI em runtime. |
| `sb-13` — low-res como máscara | Usar baixa resolução **para esconder falhas** é infalsificável e contradiz VI (verificação honesta) e a decisão local de proveniência; baixa resolução como escolha estética é legítima. | **REJECT** (a justificativa "mascarar artefatos"); manter ADOPT da estética low-poly/facetada por razões próprias — legibilidade, custo, determinismo — com inspeção e testes, não ocultação. |

Regra de ouro: nenhum ID depende de janela normal (regra `--minimizado` preservada), de sockets/servidor autoritativo ou de 5v5. Todos os artefatos visuais passam pelo mesmo gate de evidência que o código.

---

## Riscos, contra-evidência e [NV]

- **Risco jurídico:** outputs IA podem ser desprotegidos (EUA) ou protegidos condicionados a registro do processo (China); no Brasil,lei 9.610/98 art. 11 e silêncio normativo. Não há resposta única; por isso CC0/humano com contrato escrito é a via de menor risco.
- **Contra-evidência a `sb-13`:** forense a 32×32 com 98,65% de acurácia; NoiseShift mostra que baixa resolução *degrada* saída de difusão; SD-πXL exige constraints explícitos. A tese do vídeo não sobrevive ao escrutínio técnico disponível.
- **Contra-evidência a ferramentas IA:** mesmo o PixelLab, que alega propriedade ao usuário, exige Open RAIL-M e pode suspender acesso; logo falha a cláusula IV se virar fonte única.
- **[NV — acesso/verificação]:** não executei Stable Diffusion, PixelLab, Aseprite/Pixelorama/Piskel; não testei detecção C2PA em assets locais; não li o post Reddit original nem o build do Slotbound; não verifiquei a página viva da Steam/itch.io do jogo; não consultei a Parte 3 do USCO sobre treinamento; não validei whether a certiorari foi deferida (apenas fonte secundária não lida); nenhum número de vendas/wishlists/review foi verificado aqui.
- **Não é aconselhamento jurídico:** traduções/resumos de leis e decisões são descriptivos; aplicação ao caso depende de jurista competente.

---

## Alternativas/concorrentes mais viáveis

1. **Procedural determinístico (já no repo):** estender `malha`/`achatar`/`CORES_TERRENO` e `hash_estado()` a gerador de sprites e folhas de animação com seed e paleta versionadas; saída reproduzível por `verify.sh`. itch.io isenta algoritmos autocontidos de disclosure; evita serviço externo.
2. **CC0-first com inventário:** packs CC0 (ex.: Quaternius, já citado em `doc-11`) baixados uma vez, com SPDX, URL, data, hash e registro em `COPYRIGHT.txt`; sem obrigação de atribuição, mas com verificação de licença no ato do download.
3. **Arte humana com contrato escrito:** cessão/work-made-for-hire com cláusula de originalidade e proibição de treinar modelos com o material; atende Lei 9.610/98 art. 11 e dá titularidade clara para relicenciar sob MIT.
4. **Editores FOSS para autoria humana:** Pixelorama (MIT) e Piskel (Apache-2.0) como padrão; Aseprite (EULA pago) como opção individual, jamais redistribuído no repo.
5. **Se IA for usada mesmo assim (fora do caminho crítico):** modelo self-hosted com pesos fixados, prompt/seed/parâmetros e hash em `docs/evidencias/`; manifesto estilo AI-BOM com `digitalSourceType`/`inputTo`; disclosure em Steam e itch.io; C2PA como **sidecar** versionado, não confiando em EXIF que não sobrevive ao reencode.

---

## Fontes

1. USCO — https://www.copyright.gov/ai/Copyright-and-Artificial-Intelligence-Part-2-Copyrightability-Report.pdf
2. D.C. Circuit, *Thaler v. Perlmutter* — https://media.cadc.uscourts.gov/opinions/docs/2025/03/23-5233.pdf
3. Steamworks Content Survey — https://partner.steamgames.com/doc/gettingstarted/contentsurvey
4. itch.io quality guidelines / AI disclosure — https://itch.io/docs/creators/quality-guidelines ; https://itch.io/t/4309690/generative-ai-disclosure-tagging
5. EU AI Act Art. 50 + FAQ — https://digital-strategy.ec.europa.eu/en/faqs/transparency-obligations-under-article-50-ai-act
6. C2PA implementation guide — https://c2pa.org/a-new-implementation-guide-for-content-credentials/ ; estudo de proveniência — https://imageevidence.com/research/ai-image-provenance
7. SD-πXL — https://arxiv.org/html/2410.06236v1 ; NoiseShift — https://arxiv.org/html/2510.02307v1 ; forense 32×32 — https://openaccess.thecvf.com/content/ICCV2025W/APAI/papers/Sharma_Explainable_AI-Generated_Image_Forensics_A_Low-Resolution_Perspective_with_Novel_Artifact_ICCVW_2025_paper.pdf
8. PixelLab docs/ToS — https://www.pixellab.ai/docs/ways-to-use-pixellab ; https://www.pixellab.ai/termsofservice ; Aseprite FAQ — https://aseprite.org/faq ; Pixelorama — https://pixelorama.org/faq ; Piskel — https://github.com/piskelapp/piskel
9. 苏州市中级人民法院 (ZH) — http://www.zjrmfy.suzhou.gov.cn/fypage/toContentPage/pfxc/82a07a48977451e80197818838d6001c ; 北京互联网法院/知产财经 (ZH) — https://www.ipeconomy.cn/mobile/dongtai/9834.html
10. 文化庁 (JA) — https://www.bunka.go.jp/seisaku/chosakuken/aiandcopyright.html
11. Migalhas (pt-BR) — https://www.migalhas.com.br/depeso/446851/criacoes-por-ia-e-direitos-autorais-o-risco-de-o-brasil-seguir-os-eua
12. Vídeo analisado — https://www.youtube.com/watch?v=Z4ayMTXnkoQ
13. Locais — `LICENSE`, `COPYRIGHT.txt`, `docs/adr/002-licenca-mit-nucleo.md`, `.specify/assessments/{ast-20,ast-21,doc-11,doc-12}/decision.md`
