# Concept: Emenda pontual do GDD a partir dos ADRs 001/002

- **Slug**: emenda-gdd-adrs
- **Created**: 2026-11-19
- **Recommended option**: Option A — Emenda pontual das 7 passagens com rastreabilidade explícita aos ADRs

## Options

### Option A — Emenda pontual das 7 passagens com rastreabilidade explícita aos ADRs

- **Sketch**: O GDD permanece o documento autoritativo, mas cada uma das sete passagens mapeadas (`docs/PROGRESSO.md:97`) deixa de contradizer a autoridade local: N3 passa a constar como decidida pela licença MIT única do ADR-002 (`:14`, `:37-40`), sem `COPYING`/AGPL e sem a alternativa "MIT ou GPL", e a promessa de "integração contínua desde o primeiro dia" (`docs/GDD - Insurgência.md:1594`) é reconciliada com o DEFER do gate remoto e com a independência de CI afirmada no ADR-001 (`:14`). Cada alteração carrega, no próprio GDD, a referência ao ADR que a decide, de modo que o leitor encontra uma versão coerente sem precisar cruzar fontes. O texto ao redor permanece intacto.

- **Appetite**: **small** (dias) — sete passagens documentais, sem código, build, gate, instalação ou autorização remota; alinhado ao candidato 1 do ranqueamento 002, escolhido justamente por "fechar divergências reais com evidência local, sem dependência externa, sem instalação e sem autorização remota" (`docs/governance/80-20-ranking-2026-11-19.md:31,37`).

- **Trade-offs**: Wins — elimina as 7 divergências e os 2 riscos ALTA (`docs/AVALIACAO-STACK-MDMV.md:437-438`), fecha a dívida 4 (`.specify/memory/STATE.md:27`) e preserva o trabalho alheio por ser estritamente pontual; exige apenas edição local, sem efeito remoto. Sacrifices — depende de aprovação humana dos ADRs, hoje `Pendente de aprovação` / `[NV]` (`docs/adr/001-excecao-linguagem-gdscript.md:6`; `docs/adr/002-licenca-mit-nucleo.md:6`; `.specify/memory/STATE.md:25`), podendo antecipar autorização ainda não obtida (`docs/adr/002-licenca-mit-nucleo.md:20,26`); a ressalva AGPL de terceiro (`:1389`) está fora da autoridade do ADR-002 (`:28,41`), o que restringe o que pode ser ajustado nessa passagem.

- **Rabbit holes**: (1) resolver a CI convertendo o DEFER em "DEFER com gatilho" no primeiro commit de `nucleo/` (saída de `docs/AVALIACAO-STACK-MDMV.md:380`) — conflita com o DEFER vigente e reabre a discussão; (2) tocar a ressalva de licença de terceiro em `docs/GDD - Insurgência.md:1389` além do que o ADR-002 autoriza; (3) citar a numeração provisória (ADR-002=CI, ADR-006=licenciamento, `docs/AVALIACAO-STACK-MDMV.md:406,410`) em vez da materializada (ADR-002=MIT-only), gerando rastreabilidade errônea; (4) deixar a emenda escorrer para as âncoras Godot-first `docs/GDD - Insurgência.md:1265,1678`, intocáveis; (5) tratar qualquer efeito remoto `[NV]` como verde.

### Option B — Reescrita ampla do GDD

- **Sketch**: Revisar e reescrever de forma ampla as seções de licenciamento, CI e decisões registradas do GDD, consolidando tudo em uma redação nova e unificada, com a oportunidade de limpar outras inconsistências além das sete passagens mapeadas.

- **Appetite**: **large** (meses) — revisão normativa de múltiplas seções de um documento de projeto, com múltiplos aprovadores e alto risco de reabrir decisões fechadas.

- **Trade-offs**: Wins — coerência potencialmente mais profunda e chance de resolver outras divergências latentes de uma só vez. Sacrifices — reescreve trabalho alheio e ameaça diretamente as âncoras Godot-first (`docs/GDD - Insurgência.md:1265,1678`; `docs/PROGRESSO.md:97`); esbarra no risco MÉDIO de "reabertura involuntária do GDD por ADRs de processo" (`docs/AVALIACAO-STACK-MDMV.md:443`); esforço e superfície de aprovação desproporcionais a um problema de 7 passagens. **Rejeitada** por esse risco de reescrever trabalho alheio.

- **Rabbit holes**: reabrir decisões já fechadas pelos ADRs; reescrever o roadmap condicionado a Godot-first (`:1678`); perder o histórico e a rastreabilidade das redações originais; escalada de aprovação para múltiplos aprovadores.

### Option C — Nenhuma emenda, apenas os ADRs

- **Sketch**: Manter os ADRs 001/002 como única autoridade e não tocar o GDD; a divergência fica registrada como conhecida e o documento normativo segue com N3 aberta e com a promessa de CI no dia 1 sem reconciliação com o DEFER.

- **Appetite**: **small** (nenhum esforço de emenda) — é o estado atual do repositório.

- **Trade-offs**: Wins — risco zero de reescrever trabalho alheio e nenhuma dependência de aprovação de redação; preserva o status quo. Sacrifices — deixa o documento autoritativo contraditório, mantendo os 2 riscos ALTA (`docs/AVALIACAO-STACK-MDMV.md:437-438`), o risco jurídico para forks e operadores (`docs/adr/002-licenca-mit-nucleo.md:10`), a dívida 4 aberta (`.specify/memory/STATE.md:27`) e N3 bloqueando a "Primeira publicação" (`docs/GDD - Insurgência.md:1668`) — ou seja, é o próprio custo de inação. **Rejeitada** porque deixa o documento autoritativo contraditório.

- **Rabbit holes**: confundir "os ADRs decidiram" com "o problema está resolvido"; a promessa de CI dia 1 continuar a ser lida como compromisso (`docs/AVALIACAO-STACK-MDMV.md:22`); publicação bloqueada indefinidamente por N3.

## Recommendation

Recomenda-se a **Option A**. É a única que reduz a métrica principal de 7 passagens contraditórias para 0 e fecha a dívida 4, sem reescrever trabalho alheio e sem depender de efeito remoto — exatamente o critério com que o ranqueamento 002 a elegeu como candidato 1 (score 125,0; `docs/governance/80-20-ranking-2026-11-19.md:31,37`). A Option B é desproporcional e arrisca as âncoras intocáveis; a Option C é o custo de inação medido em `problem.md`. A recomendação está condicionada à validação da aprovação humana dos ADRs, hoje `[NV]`: se for confirmado que ainda não há autoridade para emendar, a Option C torna-se o fallback, não a recomendação.

## Out of Scope (for the recommended option)

- Âncoras Godot-first `docs/GDD - Insurgência.md:1265,1678` — intocáveis (`docs/PROGRESSO.md:97`; `docs/adr/001-excecao-linguagem-gdscript.md:32`; `docs/adr/002-licenca-mit-nucleo.md:33`).
- CI remota / GitHub Actions / gate remoto e a reversão do DEFER (`docs/governance/80-20-ranking-2026-11-19.md:34-37`; `docs/AVALIACAO-STACK-MDMV.md:74,145,380`) — efeito remoto `[NV]`.
- Instalação de Godot ou qualquer mudança de engine, linguagem, build ou runtime.
- Implementação de código, gate, build, teste, lint, compilação, telemetria ou serviço externo (`docs/adr/001-excecao-linguagem-gdscript.md:28`; `docs/adr/002-licenca-mit-nucleo.md:29`; `docs/PROGRESSO.md:89`).
- Reescrita ampla do GDD (Option B) e decisão de mérito sobre licenças de terceiros, fora da autoridade do ADR-002 (`docs/adr/002-licenca-mit-nucleo.md:28,41`).
- Confirmação de aprovação em plataforma, execução de CI, merge, proteção de branch e publicação externa — `[NV]` até verificação ao vivo (`docs/adr/001-excecao-linguagem-gdscript.md:40`; `docs/adr/002-licenca-mit-nucleo.md:45`; `.specify/memory/STATE.md:31-35`).

## Assumptions to Validate

- Os ADRs 001/002 podem ser tratados como autoridade local para a emenda embora estejam `Pendente de aprovação` (`docs/adr/001-excecao-linguagem-gdscript.md:6`; `docs/adr/002-licenca-mit-nucleo.md:6`; `.specify/memory/STATE.md:25`); aprovação humana efetiva = `[NV]`.
- A autoria/patrocínio da proposta e quem sanciona a redação final do GDD, e em qual ciclo (`.specify/memory/STATE.md:39`; `docs/adr/002-licenca-mit-nucleo.md:43-45`).
- A cobertura efetiva é das 7 passagens (`docs/PROGRESSO.md:97`), e não apenas de N3 e CI — em particular, se `:1389` e `:1459` entram (`docs/adr/002-licenca-mit-nucleo.md:28,41`).
- A numeração de ADR a usar na rastreabilidade é a materializada (ADR-002=MIT-only), não a provisória (`docs/AVALIACAO-STACK-MDMV.md:406,410`).
- A contradição de CI se resolve alterando a promessa em `docs/GDD - Insurgência.md:1594` (e não registrando DEFER com gatilho, saída de `docs/AVALIACAO-STACK-MDMV.md:380`, que conflita com o DEFER vigente).
- N3 deixar de bloquear a "Primeira publicação" (`docs/GDD - Insurgência.md:1668`) não depende de publicação externa ainda não verificada (`[NV]`).
- Nenhum efeito remoto (CI, aprovação, merge, proteção de branch) é prerequisite para a emenda documental (`docs/adr/001-excecao-linguagem-gdscript.md:14,40`; `docs/adr/002-licenca-mit-nucleo.md:14,45`).
