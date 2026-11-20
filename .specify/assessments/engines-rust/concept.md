# Concept — Como fechar a lacuna da avaliação Rust

**Data:** 2026-11-20

## Contexto

A avaliação de 19/11 fixou 13 opções, só 3 Rust, com versões defasadas e sem Fyrox, Ambient, gdext ou Rapier (`docs/avaliacao-engines/01-matriz.md:47-61`; `docs/avaliacao-engines/pesquisa/rust-protocolo.md:13`). Os quatro relatórios de 20/11 entregam versões observadas ao vivo, metadados de primeira parte e notas C1–C10 recalculadas (Bevy 0.20.1, Lightyear 0.31, Fyrox 65,2, gdext 68,6*, Rapier 60,4, entre outras). O protocolo factory 019/040 exige matriz com ≥10 opções, critérios/pesos/escala formais, Fase 0 com C1–C3 invioláveis, recomendação e evidência (`rust-protocolo.md:5-7`). A ADR-001 mantém Godot-first como vinculante e **não reaberta**: a saída é documental, sem troca de motor, sem Rust no Núcleo e sem autorização de "servidor Rust como Plano B" (`docs/adr/001-excecao-linguagem-gdscript.md:19,30-36`).

## Opções consideradas

1. **Atualizar a matriz canônica com 10 opções — escolhida** — revisar a matriz em `docs/avaliacao-engines/01-matriz.md` (e seus documentos de apoio `00-criterios.md`, `02-analise-por-opcao.md`, `03-recomendacao.md` e `AVALIACAO-COMPARATIVA-ENGINES.md`) para a versão de 20/11: corrigir as versões defasadas (Bevy 0.19.1 → 0.20.1; Lightyear 0.28–0.30 → 0.31), incluir as opções ausentes (Fyrox, Ambient, gdext, Rapier, ggez) e reaplicar C1–C10 com `Σ(nota×peso)/10`, de modo que a matriz revisitada apresente **≥10 opções** (mínimo do protocolo) com notas e justificativas rastreáveis. A recomendação permanece documental: O1 Godot segue ADOPT Vinculante; opções Rust recebem ADAPT/DEFER/REJECT conforme a trava inviolável, acompanhadas de *spikes* (reconexão, replay, custo/tick 5v5+bots) e de tabela de evidência/limitações. Nenhuma conclusão é revertida nesta etapa: a atualização é o objeto do handoff, não um arquivo escrito aqui.
2. **Anexar os relatórios ao dossiê sem atualizar a matriz (rejeitada)** — deixar os quatro `pesquisa/*.md` como apêndice e manter `01-matriz.md:47-61` intacto. Rejeitada porque preserva exatamente a lacuna diagnosticada: versões defasadas e amostra 3/13 continuariam sendo a base scores da conclusão, e a evidência nova não entraria na comparação ponderada. A condição de reavaliação da ADR-001 continuaria sem dado consolidado, tornando o esforço de pesquisa estéril.
3. **Trocar o Godot por uma engine Rust (rejeitada por ADR-001)** — substituir O1 Godot por Bevy/Lightyear ou Fyrox como motor-base. Rejeitada de plano: a ADR-001 (Aprovado, 19/11/2026) declara Godot-first vinculante e **não reabre, não revê e não reverte** o motor, rejeita explicitamente "Servidor Rust como Plano B" e exige evidência prévia + aprovação formal do maintainer para qualquer troca (`docs/adr/001-excecao-linguagem-gdscript.md:14,19,30-36`); a ADR-003 não reabre motor, licença MIT-only nem escopo solo/single. Nenhuma avaliação documental autoriza Rust no Núcleo.

## Recomendação

**Opção 1.** É a única que fecha a lacuna com rastreabilidade: incorpora as versões observadas e as opções ausentes, reaplica critérios/pesos formais e gera a evidência prévia exigida pela ADR-001 — sem tocar na decisão Godot-first. A Opção 2 é rejeitada porque mantém a matriz desatualizada; a Opção 3, porque viola a ADR-001 e o escopo documental. Nenhuma opção inclui build, benchmark, gate, teste, `make`, Godot, spec, plano, avaliação final, stage/commit/push ou escrita fora dos cinco artefatos de assessment.

## Consequências

- A escrita desta etapa fica restrita a `.specify/assessments/engines-rust/{intake,research,problem,concept,decision}.md`; a matriz canônica e a avaliação final **não são escritas aqui** — pertencem ao handoff do ciclo seguinte.
- O dossiê preserva a distinção de categoria (engine escrita em Rust ≠ biblioteca/framework ≠ binding): gdext e Rapier entram como contexto/camada sobre O1, não como engines-base, e macroquad não é reintroduzida como "nova" opção (`rust-outras.md:24-32,173-176`).
- Toda nota C6 segue como proxy; reconexão, replay, custo por tick e 5v5+bots permanecem **[NV]** e viram *spikes* explícitos, não afirmações de desempenho.
- Host policy GitHub-only e cobertura linguística real (inglês apenas, sem triangulação) são declaradas como limitação, não presumidas como completude.
- Godot-first, ADR-001/003, baseline de converge, `Makefile`, `evidence.csv`, factory e portarias R4/R7 permanecem intocados; nenhum artefato de execução é gerado nesta etapa.
