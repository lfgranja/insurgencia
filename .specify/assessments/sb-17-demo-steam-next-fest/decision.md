# Decision — Demo Steam + Next Fest

- **Slug**: sb-17-demo-steam-next-fest
- **Decided**: 2026-11-21
- **Veredito**: DEFER
- **Artifacts reviewed**: intake.md; research.md; problem.md; concept.md

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | adequate | Alcance é legítimo, mas o problema real é sequenciamento: faltam demo versionada, página e App ID. |
| Força da evidência | weak | Janelas e pré-requisitos são sólidos; o efeito alegado do festival é `[NV]` e os 100k wishlists precederam o evento. |
| Valor vs. inação | adequate | Adiado, o valor é preservar a opção; entrar cedo queima a janela única sem tração medida. |
| Viabilidade / Appetite | weak | US$100 por app, SDK, build review de 3–5 semanas e prazo de registro até 10/01/2027. |
| Alinhamento estratégico | weak | Steam como canal adicional é permitido, mas como caminho crítico conflita com IV; o mecanismo de visibilidade negado pela Valve enfraquece a tese. |
| Postura de risco | weak | Participação é única por título; erro de timing é irreversível dentro do ciclo e a diluição é extrema (7.240 demos). |

## Racional

DEFER, não REJECT: o canal pode vir a ser útil, mas hoje não há nem Steam page, nem App ID, nem demo versionada que tenha passado no gate. O próprio caso estudado alcançou 100k wishlists **antes** do Next Fest de outubro, e a documentação da Valve afirma que wishlists não movem visibilidade algorítmica (exceto Popular Upcoming) — a causalidade afirmada no vídeo é `[NV]`/contradizida. A janela de fevereiro de 2027 (22/02–01/03, registro até 10/01) é a primeira revisável, com build review de 3–5 semanas; a de junho/2027 é a seguinte. Como a participação é única por título, antecipar sem artefato validado troca uma opção real por uma aposta em outlier. O pré-requisito local é sb-16 (demo determinística publicada), não uma meta de wishlists.

## Condição/responsável

- Responsável: Main (gate e release de sb-16) + SE (conta Steam, pagamento e inscrição, se autorizado).
- Condição de reavaliação: sb-16 publicada com tag/checksum e `verify.sh` verde executado; janela com ≥5 semanas de folga; registro até 10/01/2027 para Feb/2027 ou antes de jun/2027. Sem isso, o diferimento se renova automaticamente e nenhuma conta é criada.

## Reavaliação

Não se aplica (DEFER). Reavaliação registrada para até 10/01/2027 (Feb/2027) ou antes de jun/2027, condicionada aos pré-requisitos acima. Nenhuma abertura de Steam page, pagamento de US$100, build review ou inscrição neste ciclo. A tese de "visibilidade algorítmica por wishlists" não deve ser usada como justificativa de go; substituí-la por métricas locais é condição para reabrir.
