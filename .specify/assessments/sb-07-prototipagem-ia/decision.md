# Decision — Prototipagem rápida com IA

- **Slug**: sb-07-prototipagem-ia
- **Decided**: 2026-11-21
- **Veredito**: ADAPT

| Critério | Avaliação | Justificativa (1 linha) |
|---|---|---|
| Validade do problema | strong | Necessidade real de iteração rápida sob `--minimizado` sem contaminar o caminho crítico. |
| Força da evidência | strong | Fonte primária (E1/E2/E5) + fit local: já é prática do SDD; LLM nunca roteia estado. |
| Valor vs. inação | strong | Mantém a velocidade do primeiro rascunho com risco baixo e sem plataforma. |
| Viabilidade/Appetite | strong | Ferramentas open-source e `verify.sh` já disponíveis; nenhuma mudança de engine. |
| Alinhamento estratégico | strong | I (LLM informativa), IV (sem dependência), V (decisão de design humana). |
| Postura de risco | strong | Protótipo descartável, sem janela normal e sem engine nova; reversível. |

## Racional
A conclusão do vídeo é plausível no primeiro rascunho, mas a fonte primária mostra ~6–12 meses de trabalho e desaceleração quando a complexidade cresce. Na Insurgência, prototipar com IA é legítimo como insumo informativo, desde que a saída de LLM nunca roteie estado (`avancar_tick`/`hash_estado`) e que nada entre no caminho crítico sem `verify.sh`. ADR-004 (Bevy+Lightyear) e `--minimizado` são inegociáveis.

## Condição/responsável
Responsável: SE e o agente que produz o protótipo. Condição: protótipo marcado como descartável; agente somente-leitura em exploração; nenhum diff em `sim`/`contratos` sem gate verde; nenhuma janela normal; decisão de "fun/balance/legibility" humana.

## Handoff
Registrar no fluxo SDD que IA é ferramenta de prototipagem informativa: usar OpenCode `plan`/Aider como opcionais, manter ADR-004, exigir `verify.sh` ALL STAGES OK antes de promover qualquer rascunho a código de produção.
