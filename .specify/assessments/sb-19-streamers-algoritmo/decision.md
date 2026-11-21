# Decision — Streamers e algoritmo

- **Slug**: sb-19-streamers-algoritmo
- **Decided**: 2026-11-21
- **Veredito**: ADAPT
- **Artifacts reviewed**: intake.md; research.md; problem.md; concept.md

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | A vontade de ser visto é legítima, mas o problema formulado (mecanismo de algoritmo) não é o problema real. |
| Força da evidência | weak | Ocorrência de streamers é documentada; causalidade quantificada é `[NV]` e a tese do algoritmo é contrariada pela Valve. |
| Valor vs. inação | strong | Sem material verificável não há aprendizado nem resposta auditável; a alternativa é barata e reutilizável. |
| Viabilidade / Appetite | strong | Reaproveita sb-16, `--minimizado`/headless e artefatos de CI; nenhum serviço externo no caminho crítico. |
| Alinhamento estratégico | strong | Substitui crença por evidência (I/VI) e preserva soberania (IV) e a SE sem janela. |
| Postura de risco | adequate | Risco de promover alcance `[NV]` a KPI ou de editar a captura; mitigado por seed fixa, hash e comando publicado. |

## Racional

ADAPT: rejeita-se o **mecanismo** proposto — wishlists e "recomendações algorítmicas" como motor — porque a própria Valve documenta que wishlists não são fator de visibilidade (exceto Popular Upcoming), que tráfego e conversão não contam e que os primeiros dias de festival são aleatórios; a cobertura de streamers existiu, mas sua causalidade quantificada é `[NV]`. Adota-se a **forma** do material que funcionou como canal: um clipe curto e legível, porém produzido internamente como captura reproduzível — seed fixa, comando único, `hash_estado()` e saída de `verify.sh` anexados, gerada pelo exemplo headless de sb-16 ou sob `--minimizado`, sem perturbar a SE e sem serviço externo. Criadores são tratados como canal, não como promessa de algoritmo. Nenhum número de alcance ou crença de praticante entra como evidência de decisão.

## Condição/responsável

- Responsável: Main (exemplo/seed/artefato de CI) + SE (texto do pacote de imprensa e contato com criadores).
- Condição de validade: duas execuções com o mesmo hash antes de congelar a seed; captura nunca editada; pacote sempre com comando, seed e saída do gate. Se a captura deixar de ser reproduzível ou virar KPI de alcance, a adaptação converte em AVOID para aquele uso.

## Handoff

- **Problema**: não existe material de imprensa verificável, e a narrativa de "algoritmo" é falsificável e não orienta decisão.
- **Abordagem**: captura reproduzível com seed fixa (exemplo headless de sb-16 ou `--minimizado`), hash por tick, comando exato e saída do gate anexados; pacote de imprensa como artefato de CI; criadores como canal, nunca como mecanismo.
- **In**: seed e cenário congelados; script de captura; pacote com comando/hash/saída; artefato de CI.
- **Out**: afirmações de visibilidade algorítmica, metas de wishlist/views `[NV]`, contratação de streamers, edição IA do clipe, janela normal durante a SE, saída de LLM como autoridade.
- **Métricas**: hashes idênticos em duas execuções; um pacote com seed/comando/saída; artefato de CI sem serviço externo; zero afirmações de causalidade algorítmica em atas; nenhuma janela aberta.
- **Perguntas**: qual seed/cenário congelar; onde fica o pacote (`docs/evidencias/` vs artefato versionado); como acompanhar "canal" sem promover alcance `[NV]` a KPI.
