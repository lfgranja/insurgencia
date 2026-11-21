# Concept: Streamers e algoritmo

- **Slug**: sb-19-streamers-algoritmo
- **Created**: 2026-11-21
- **Recommended option**: Option A — Pacote de imprensa com captura reproduzível por seed fixa

## Options

### Option A — Pacote de imprensa com captura reproduzível por seed fixa

- **Sketch**: congelar seed e cenário curto no exemplo `crates/sim/examples/demo_deterministica.rs` (sb-16), com `hash_estado()` impresso por tick; gerar a captura por um único comando, sem janela (ou, se for preciso HUD visual, sob `--minimizado`, sem perturbar a SE). Publicar pacote de imprensa com: comando exato, seed, hash esperado, saída executada de `verify.sh` e o clipe/quadro como artefato de CI. Criadores recebem o material e podem reproduzi-lo; a conversa é sobre o que o jogo faz, não sobre "algoritmo". Nenhuma afirmação de visibilidade algorítmica entra no material.
- **Appetite**: small (dias) — reusa sb-16, um script de captura e um artefato de CI.
- **Trade-offs**: troca uma promessa de alcance `[NV]` por evidência auditável e reutilizável; exige congelar seed/formato e disciplina de não editar a captura. Não mede audiência — isso permanece fora de escopo e `[NV]`.
- **Rabbit holes**: vender o pacote como "hack de algoritmo"; editar o clipe para parecer melhor que a execução; usar saída de LLM como texto do release sem revisão; transformar captura em KPI de sucesso.

### Option B — Estratégia centrada em streamers e algoritmo

- **Sketch**: perseguir cobertura de criadores e otimizar para recomendações da loja, assumindo que wishlists alimentam visibilidade.
- **Appetite**: large e contínuo — relacionamento com criadores, produção de vídeos, acompanhamento de métricas privadas.
- **Trade-offs**: é o que o caso parece sugerir, mas a causalidade quantificada é `[NV]` e o mecanismo é negado pela Valve; 7.240 demos em out/2026 e a aleatoriedade dos primeiros dias tornam a aposta de alto risco e baixa reprodutibilidade.
- **Rabbit holes**: confundir "houve vídeo" com "o vídeo causou o crescimento"; citar crenças de praticantes como evidência; medir sucesso por views de terceiros.

### Option C — Somente devlogs em texto, sem captura determinística

- **Sketch**: escrever devlogs descritivos, sem seed/comando/hash anexados.
- **Appetite**: small.
- **Trade-offs**: barato e honesto, mas não é falsificável: o leitor não consegue reproduzir o que foi descrito, e o material continua dependente da palavra do autor — o mesmo problema do vídeo editado, em outra forma.
- **Rabbit holes**: achar que "texto" já é verificação; omitir o comando exato.

## Recommendation

Option A, coerente com o veredito ADAPT: o que se adota é a **forma auditável do material** — captura reproduzível com seed fixa, comando e hash — e não a crença em streamers ou algoritmo. A documentação da Valve invalida a mecânica suposta; o que sobra do caso é que um clipe curto e legível ajudou a chamar atenção, e isso pode ser produzido internamente com determinismo e zero serviço externo. Guardrail: captura sempre com seed fixa sob `--minimizado`/headless; nenhuma afirmação de visibilidade algorítmica no pacote; nenhum número de alcance `[NV]` promovido a métrica.

## Out of Scope (for the recommended option)

- Perseguir recomendação algorítmica, wishlists ou views de terceiros.
- Contratação de streamers, SLA de cobertura ou métricas de alcance `[NV]`.
- Edição de vídeo com IA generativa no lugar da captura determinística.
- Abertura de janela normal durante o trabalho da SE; serviço externo no caminho crítico.
- Uso de saída de LLM como autoridade (constituição I/VI).

## Assumptions to Validate

- sb-16 entrega o exemplo headless com seed fixa e saída estável; validar com duas execuções antes de congelar o pacote.
- O artefato de CI pode conter a captura sem dependência de serviço externo (verificar no ato da implementação).
- [NEEDS CLARIFICATION: seed/cenário congelados e local do pacote de imprensa (`docs/evidencias/` vs artefato versionado).]
- [NEEDS CLARIFICATION: como acompanhar "canal" sem promover alcance `[NV]` a KPI.]
