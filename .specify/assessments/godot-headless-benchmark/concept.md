# Concept: Godot headless benchmark

- **Slug**: godot-headless-benchmark
- **Created**: 2026-11-20
- **Recommended option**: Opção A — release oficial verificada por checksum e benchmark sintético documentado

## Options

Três opções são consideradas, da instalação verificada com medição documentada à ausência de medição; a recomendação é nomeada ao final com base nos objetivos de `problem.md`.

### Option A — Release oficial verificada por checksum e benchmark sintético documentado

- **Sketch**: a engine é obtida de uma release upstream nomeada, o artefato tem seu checksum comparado ao valor publicado e o binário instalado reporta sua própria identidade. Em seguida, a carga headless já versionada é executada e o resultado — mediana, dispersão e carimbos — é registrado como linha de base explicitamente sintética. Quem lê o resultado consegue provar de onde veio o binário e o que exatamente foi medido.
- **Appetite**: small — horas de instalação, verificação e documentação, usando comandos já executados e arquivos já versionados; sem desenvolvimento de produto.
- **Trade-offs**: ganha proveniência, integridade e linha de base comparável; sacrifica cobertura de cenários de jogo reais, pois a carga é apenas um laço de simulação. Riscos: a medição pode ser mal citada como desempenho de partida, e a dispersão entre amostras pode ser confundida com garantia de desempenho.
- **Rabbit holes**: tentar ampliar a carga para 10 jogadores, renderização ou rede para "parecer real"; perseguir otimizações de engine para melhorar o número; transformar a mediana em meta de produto.

### Option B — Binário arbitrário sem verificação de checksum

- **Sketch**: usar qualquer build disponível no ambiente, sem fixar release e sem comparar checksum, e medir a carga sintética da mesma forma.
- **Appetite**: small — menos passos iniciais.
- **Trade-offs**: ganha rapidez; sacrifica a capacidade de provar integridade e reprodutibilidade da medição, deixando a identidade do binário como suposição. Risco: uma medição válida, porém atribuída a versão errada, invalida comparações futuras.
- **Rabbit holes**: deduzir a versão do nome do arquivo ou de saída parcial, em vez de registrar a string reportada pelo executável.

### Option C — Nenhuma instalação nem medição nesta rodada

- **Sketch**: manter o alvo sem engine e sem linha de base, registrando apenas a intenção de medir no futuro.
- **Appetite**: small — nenhuma ação.
- **Trade-offs**: ganha zero exposição a mudanças no ambiente; não resolve o problema, pois não produz prova de integridade nem referência de custo por tick.
- **Rabbit holes**: substituir a medição por números publicados por terceiros e apresentá-los como se fossem do alvo.

## Recommendation

Opção A. Ela atende diretamente aos objetivos de maior valor: provar a origem e a integridade da engine e estabelecer uma linha de base de custo por tick auditável. É a opção com menor superfície de erro — uma release nomeada, um par de checksums, uma string de versão e uma carga sintética já conhecida — e preserva a hierarquia de autoridade do hub: a instalação e a medição são ações autorizadas, enquanto reclassificar a carga e transformar o número em SLA não são. A Opção B é preterida porque economiza minutos e destrói a cadeia de evidência; a Opção C não resolve o problema. Nenhuma opção inclui medição de partida real, portarias posteriores a tasks ou uso de segredos.

## Out of Scope (for the recommended option)

- Reclassificação da carga sintética como benchmark 5v5 real ou como prova de desempenho de jogo multiplayer.
- Medição de renderização, rede, serviços externos, IA de jogadores ou qualquer carga além do laço de `tests/benchmark/headless_tick.gd`.
- Transformação da mediana em SLA, garantia de hardware ou meta de produto.
- Execução de gates, stage, commit, PR, merge ou reescrita de histórico nesta etapa de planejamento.
- Portarias posteriores a tasks e escrita de arquivos fora de `specs/006-godot-headless-benchmark/`.

## Assumptions to Validate

- A evidência obtida pelo orquestrador é o registro fidedigno desta rodada: `CHECKSUM_OK` em 2026-11-20T00:43:52Z, binário `4.7.2.stable.official.ed1daf0bf` em 00:44:11Z e mediana de 37,48 µs/tick em 7 amostras em 00:44:48Z.
- A referência de 16,7 ms/tick fornecida no pedido é um critério de comparação, não um requisito de produto.
- A série 4.7 declarada em `tests/benchmark/project.godot` é compatível com a release `4.7.2-stable`.
- A carga de 600 ticks × 64 entidades permanece a carga definida para a linha de base; nenhuma outra é introduzida nesta feature.
- Nenhuma credencial é necessária para consultar a release pública ou para executar o benchmark headless.
