# Decision — Stills animados por IA em vídeos promocionais

- **Slug**: sb-06-stills-animados-ia
- **Decided**: 2026-11-21
- **Veredito**: ADAPT
- **Artifacts reviewed**: intake.md; research.md; problem.md; concept.md; `docs/research/slotbound-2026-11-21-ai-promo.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | Risco real de falsa evidência visual e disclosure, mas a premissa "Slotbound fez isso" é inferência `[NV]`. |
| Força da evidência | weak | "I guess" na transcrição; imagens originais não inspecionadas; políticas 2026 primárias, caso alheio não. |
| Valor vs. inação | adequate | Captura real mostra progresso; inação evita risco, mas deixa sem critério o que conta como "gameplay" em vídeo. |
| Viabilidade/Appetite | strong | `sim` headless + exemplo Bevy + FFmpeg `-bitexact`; fora dos 5 crates; ADR-004 intacto. |
| Alinhamento estratégico | strong | Substituir still sintético por quadro real serve I/IV/VI (auditoria, soberania, honestidade). |
| Postura de risco | adequate | Rótulo + declaração mitigam disclosure; bit-exatidão não medida `[NV]`; postar `DEFER` até conta. |

## Racional

ADAPT: o valor é mostrar o jogo; a origem IA do still é inferência não verificada `[NV]` e, se apresentada como gameplay, sugeriria recursos inexistentes no MVP — incompatível com verificação honesta (Constituição VI) e com disclosure de marketing na Steam (Content Survey §3). Substituir por **quadros reais do `sim`**, com semente + commit no artefato e hash publicado, transforma promoção em evidência auditável, sem serviço externo (IV) e sem janela normal (regra `--minimizado`). Se um still IA for usado, a condição é declará-lo como marketing e rotulá-lo. Publicar em plataforma fica **DEFER** até existir conta/estreia (guardrail do orquestrador); produzir o quadro é imediato. Nada aqui toca os 5 crates, os 34 testes ou o CI; captura/encode não foram executados nesta etapa.

## Condição/responsável

- **Responsável**: SE/mantenedor — autoridade sobre o que é publicado; coordenação com `sb-24`/`DisclosureLegalEvidence` para a checklist de rótulos.
- **Condição para avançar**: (1) origem de cada quadro rastreável (semente + commit + hash); (2) qualquer still IA declarado como marketing e rotulado; (3) nenhuma afirmação de que Slotbound usou IA tratada como fato; (4) conta/estreia definida antes de postar — enquanto não houver, `DEFER`.
- **Gate**: CI verde autoriza merge, nunca auto-merge cego; nenhuma alegação de verde sem comando executado (VI). Nenhum build/teste/captura/encode rodado aqui.

## Handoff

- **Problema**: risco de still sintético ser lido como evidência de um MVP inexistente, com passivo de disclosure e violação da regra da janela se gravada a janela normal.
- **Abordagem**: Opção 1 do concept — captura determinística de quadros reais do `sim` (ou exemplo headless do Bevy), rótulo de origem, hash publicado; still IA só se declarado e rotulado; postar `DEFER` até conta.
- **In scope (handoff)**: escolher o ponto de captura (exemplo `crates/sim/examples/` ou `headless_renderer`), registrar semente/commit/hash, codificar com `-bitexact`/`-map_metadata -1`, documentar disclosure e gerar o clipe rotulado (coordenar com `sb-20`).
- **Out of scope**: ferramenta de geração de imagem, serviço de nuvem, conteúdo realista enganoso, publicação sem conta, janela normal, alteração em crates/gates/testes/engine (ADR-004 fixo).
- **Métricas**: 100% dos quadros com origem rastreável e rotulada; zero stills IA não declarados; reprodução do hash entre execuções com a mesma semente; medição de campanha por UTM/72 h quando houver conta — não views.
- **Perguntas remanescentes**: `[NV]` vídeo original inspecionado em imagem? `[NV]` bit-exatidão entre versões do FFmpeg? `[NV]` conta/estreia definida? `[NV]` o `verify.sh --all-targets` cobre o exemplo de captura sem novo job?
