# Concept: Substituir marcadores `[NV]` de pesquisa por fatos verificados ao vivo

- **Slug**: verifica-nv-externa
- **Created**: 2026-11-19
- **Recommended option**: A — substituição cirúrgica com ressalvas e notas de superação

## Options

### Option A — Substituição cirúrgica com ressalvas e notas de superação

- **Sketch**: o leitor encontra, no próprio documento integrado, o fato antes marcado `[NV]` acompanhado de título, URL, data e duas fontes, com a ressalva de escopo escrita junto; nos relatórios de pesquisa datados, o corpo permanece como estava e ganha um bloco curto de "nota de superação" apontando a evidência de 19/11/2026. Nada mais muda: notas, índices e recomendações seguem intactos.
- **Appetite**: medium — 12 fatos espalhados por cerca de dez documentos, com duas modalidades de escrita (substituição e nota de superação) e uma auditoria por contagem. Dias de trabalho documental, não semanas de engenharia.
- **Trade-offs**: ganha auditabilidade e preserva o histórico; sacrifica a homogeneidade visual dos relatórios (corpo datado + nota) e exige disciplina para não reescrever frases antigas. Risco: um revisor tratar a nota de superação como edição e reescrever o corpo.
- **Rabbit holes**: refazer scores e índices "por coerência"; perseguir marcadores de classe C achando que são pendências; tentar provar empiricamente a issue #1666; buscar novas URLs além das três evidências.

### Option B — Reescrita integral dos documentos de avaliação

- **Sketch**: todos os documentos seriam reescritos com a evidência de 19/11/2026 incorporada de forma uniforme, eliminando marcadores e notas antigas.
- **Appetite**: large — reformatação de vários arquivos, recálculo de quadros e revisão de recomendações.
- **Trade-offs**: ganha consistência estética; destrói snapshots históricos, contraria a regra de append-only e coloca em risco notas e índices já validados. Risco alto de regressão documental e de alegações de benchmark por redação nova.
- **Rabbit holes**: recalcular índices ponderados, reescrever `03-recomendacao.md`, alterar `docs/governance/**`.

### Option C — Errata centralizada única

- **Sketch**: um único arquivo de errata listaria os 12 fatos e suas fontes, sem tocar nos documentos de destino.
- **Appetite**: small — um arquivo novo e uma tabela.
- **Trade-offs**: ganha rapidez; falha o objeto da feature ("substituir marcadores nos documentos integrados"), deixa os `[NV]` antigos no texto e cria uma segunda fonte de verdade que o leitor precisa cruzar.
- **Rabbit holes**: transformar a errata em novo documento canônico; duplicar citações; perder o vínculo com o arquivo/linha original.

## Recommendation

Escolher **Option A**. Ela é a única que satisfaz simultaneamente os três objetivos medidos em `problem.md`: aplicar os 12 fatos com duas fontes e ressalvas (SC-001, SC-002, SC-008), preservar marcadores de classe B/C (SC-004, SC-005) e manter snapshots históricos por nota de superação (SC-006). A evidência em `research.md` já fornece título, URL, data e pelo menos duas fontes para cada fato, e a constituição (Princípio VI) exige verificação honesta sem invenção — o que descarta B (reescrita) e C (marcador permanece). Nenhuma opção é descartada por falta de evidência; a reescrita é descartada por risco, a errata por inadequação ao objeto.

## Out of Scope (for the recommended option)

- Execução de engine, build, protótipo, benchmark ou custo por tick; nenhuma afirmação empírica nova.
- Reescrita de snapshots históricos datados e de qualquer arquivo de governança, ADR, `PROGRESSO.md` ou `converge.md`.
- Recálculo de notas, índices, scores e recomendações; substituição de marcadores de classe B e C.
- Resolução de fatos sem evidência nos três arquivos (reconexão, hash, serviços proprietários, CI/merge/proteção, publicação/deploy).
- Busca de novas URLs, rodada de gates, stage ou commit.

## Assumptions to Validate

- Os três arquivos de evidência são suficientes e estão datados de 19/11/2026; nenhuma fonte precisa ser refetchada.
- `docs/avaliacao-engines/pesquisa/**` é o conjunto de snapshots históricos; os demais caminhos listados no pedido são documentos integrados.
- A contagem de 12 fatos é derivável dos quadros "Resumo dos marcadores" dos três arquivos de evidência e não conflita com a contagem canônica de governança.
- A ressalva de cada fato pode ser escrita sem alterar o significado das notas e recomendações existentes.
- O trabalho é documental: nenhum gate, build ou efeito remoto é necessário para concluir a feature de planejamento.
