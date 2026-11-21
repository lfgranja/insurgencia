# Problem Definition: Streamers e algoritmo

- **Slug**: sb-19-streamers-algoritmo
- **Created**: 2026-11-21
- **Inputs used**: intake.md + research.md (steam-gtm, alternativas, fit-local)

## Problem Statement

A proposta atribui a aquisição de wishlists a "cobertura de streamers e recomendações algorítmicas da loja" e trata essa crença como motor replicável. Dois problemas concretos: (1) a própria Valve documenta que wishlists não são fator de visibilidade (exceto Popular Upcoming) e que os primeiros dias de festival são aleatórios — logo "o algoritmo fez tudo" é falsificável e não orienta decisão; (2) a Insurgência não tem hoje material de imprensa que um terceiro possa verificar: não há captura com seed fixa, comando publicado ou hash anexado, e a saída de LLM é apenas informativa. Sem isso, qualquer conversa com criadores depende de confiança cega e de uma causalidade `[NV]`, em vez de um artefato auditável que a própria CI produz.

## Affected Users & Stakeholders

- **Users**: criadores de conteúdo que precisam de um clipe curto e reproduzível para mostrar, sem Steam e sem SDK — [source: alternativas ALT-13]
- **Users**: engenheiros sem estúdio (GDD §3.2), que não têm equipe de marketing para sustentar narrativa de algoritmo — [source: fit-local]
- **Stakeholders**: SE, que trabalha com janelas minimizadas e não pode ter a sessão perturbada por captura visual — [source: `AGENTS.md` via fit-local]
- **Stakeholders**: autoridade do gate, que só aceita "verde" com comando executado (constituição VI) — [source: fit-local achado 6]

## Goals

- Produzir captura reproduzível da sim (seed fixa, comando único, hash) derivada do exemplo headless ou do HUD sob `--minimizado`.
- Anexar a captura a um pacote de imprensa com seed, comando exato e saída de `verify.sh`, gerado como artefato de CI.
- Tratar criadores como canal de relação, não como mecanismo comprovado de visibilidade.
- Eliminar afirmações de causalidade algorítmica não verificadas dos documentos de planejamento.

## Non-Goals

- Perseguir recomendações algorítmicas da loja ou otimizar para wishlists.
- Contratar streamers, prometer views ou medir sucesso por alcance de terceiros `[NV]`.
- Usar a crença de praticantes ("o algoritmo nota no primeiro dia") como evidência.
- Editar vídeo com saída de IA como substituto de captura determinística.
- Abrir janela normal durante o trabalho da SE ou introduzir serviço externo no caminho crítico.

## Success Metrics

- Captura gerada por um comando com seed fixa, com `hash_estado()` idêntico em duas execuções (baseline: não existe).
- Pacote de imprensa com seed, comando e saída do gate anexados (baseline: nenhum).
- Artefato de CI contendo a captura, produzido sem serviço externo no caminho crítico (baseline: canal CI→`gh` existe, artefato de captura não).
- Zero afirmações de causalidade "algoritmo por wishlists" em atas/decisões (baseline: presente no resumo do vídeo).
- Nenhuma janela aberta durante a SE (baseline: regra `AGENTS.md`).

## Cost of Inaction

Sem captura reproduzível, a equipe não consegue mostrar a tese nem responder a relatórios com evidência; fica refém de narrativas de terceiros sobre "o algoritmo", que a documentação oficial desmente. O custo é duplo: aprendizado bloqueado (não se sabe o que funcionou) e risco VI (afirmar como verificado o que é observação alheia). O material determinístico já é possível com o que existe — a inação é escolha, não falta de ferramenta.

## Open Questions

- [NEEDS CLARIFICATION: qual seed e qual cenário mínimo da demo headless serão congelados para a captura?]
- [NEEDS CLARIFICATION: o pacote de imprensa fica em `docs/evidencias/` ou em artefato de CI versionado?]
- [NEEDS CLARIFICATION: como medir "canal" sem transformar alcance `[NV]` em KPI?]
