# Problem Definition: GDD divergente dos ADRs 001/002 (N3 e CI dia 1)

- **Slug**: emenda-gdd-adrs
- **Created**: 2026-11-19
- **Inputs used**: intake.md | research.md (ambos; nenhum input adicional do usuário)

## Problem Statement

O GDD (`docs/GDD - Insurgência.md`) é o documento normativo/autoritativo do Núcleo, mas ainda mantém a pergunta N3 aberta — "MIT ou GPL?", com exigência AGPL-3.0 para o servidor e `LICENSE`/`COPYING` (`:1451`, `:1459`, `:1659`, `:1668`, `:1673`) — e promete "integração contínua desde o primeiro dia" (`:1594`), em contradição direta com as decisões já materializadas localmente: ADR-002 fixa MIT único, sem AGPL e sem cláusula alternativa (`:14`, `:37-40`), e ADR-001 afirma que a decisão não depende de CI ou efeito remoto (`:14`), enquanto o gate remoto foi DEFERido (`.specify/memory/STATE.md:39-41`; `docs/AVALIACAO-STACK-MDMV.md:74,145,380`). Mantenedores, operadores de forks/servidores e o próprio processo de governança convivem hoje com um documento que se contradiz com a autoridade que deveria orientá-lo — divergência classificada como risco ALTA e dívida 4 pendente (`docs/AVALIACAO-STACK-MDMV.md:437-438`; `.specify/memory/STATE.md:27`).

## Affected Users & Stakeholders

- **Users**: mantenedores do Núcleo — leem o GDD como fonte autoritativa e são expostos à divergência MIT↔AGPL e à promessa de CI não honrada (`docs/adr/002-licenca-mit-nucleo.md:10`; `.specify/memory/STATE.md:27`).
- **Users**: operadores de forks e servidores — dependem da definição da licença do servidor para avaliar risco jurídico e direito de saída (`docs/adr/002-licenca-mit-nucleo.md:10`; `docs/GDD - Insurgência.md:1595`).
- **Stakeholders**: autoridade de aprovação dos ADRs (maintainer) — detém a sanção da redação final do GDD e a aprovação dos ADRs, hoje `Pendente de aprovação` / `[NV]` (`docs/adr/001-excecao-linguagem-gdscript.md:6`; `docs/adr/002-licenca-mit-nucleo.md:6`; `.specify/memory/STATE.md:25`).
- **Stakeholders**: processo de governança SDD / ranqueamento 002 — elegeu a correção documental como candidato 1 (score 125,0) e registra a dívida 4 como próximo passo (`docs/governance/80-20-ranking-2026-11-19.md:31,37`; `.specify/memory/STATE.md:39`).

## Goals

- Restaurar a coerência entre o GDD e os ADRs 001/002, de modo que o documento autoritativo não mantenha N3 em aberto nem uma promessa de CI que contradiga o DEFER do gate remoto.
- Permitir que qualquer leitor determine a licença vigente e o status da CI a partir do próprio GDD, sem precisar cruzar com ADRs ou fontes remotas.
- Preservar as âncoras Godot-first e o trabalho alheio já registrado no GDD, limitando-se às passagens divergentes (`docs/GDD - Insurgência.md:1265,1678`; `docs/PROGRESSO.md:97`).
- Eliminar a dívida 4 do STATE, que hoje adia a correção documental para ciclo posterior (`.specify/memory/STATE.md:27,39`).

## Non-Goals

- Não é objetivo implementar código, gate, build, pipeline, telemetria, serviço externo ou runtime — os ADRs decretam que nenhum efeito desses decorre dos registros (`docs/adr/001-excecao-linguagem-gdscript.md:28`; `docs/adr/002-licenca-mit-nucleo.md:29`).
- Não é objetivo alterar a decisão Godot-first nem as âncoras intocáveis `docs/GDD - Insurgência.md:1265,1678` (`docs/PROGRESSO.md:97`; `docs/adr/001-excecao-linguagem-gdscript.md:32`; `docs/adr/002-licenca-mit-nucleo.md:33`).
- Não é objetivo implementar CI remota / GitHub Actions ou reverter o DEFER do gate remoto (`docs/governance/80-20-ranking-2026-11-19.md:34-37`).
- Não é objetivo instalar Godot ou mudar engine/linguagem de implementação.
- Não é objetivo reescrever o GDD além das passagens divergentes mapeadas (`docs/PROGRESSO.md:97`).
- Não é objetivo obter ou confirmar efeito remoto — aprovação em plataforma, execução de CI, merge, proteção de branch —, que permanece `[NV]` até verificação ao vivo (`docs/adr/001-excecao-linguagem-gdscript.md:40`; `docs/adr/002-licenca-mit-nucleo.md:45`).

## Success Metrics

- Passagens do GDD em contradição com ADRs 001/002 (baseline: **7** mapeadas em `docs/PROGRESSO.md:97`, conferidas em `docs/GDD - Insurgência.md:1389,1451,1459,1594,1659,1668,1673`) → alvo 0.
- N3 constar como pergunta aberta / bloqueante no GDD (baseline: **sim**, `:1659,:1668,:1673`) → alvo: não.
- Promessa de "integração contínua desde o primeiro dia" sem reconciliação com o DEFER (baseline: **1** em `:1594`) → alvo 0.
- Dívida 4 do STATE aberta (baseline: **sim**, `.specify/memory/STATE.md:27`) → alvo: fechada.
- Riscos ALTA registrados de "N3 aberta" e "CI prometida no dia 1 entregue apenas como gate local" (baseline: **2**, `docs/AVALIACAO-STACK-MDMV.md:437-438`) → alvo 0 no documento.
- Passagens alteradas com origem ADR identificável no próprio GDD (baseline: **0**; rastreabilidade hoje só fora do GDD) → alvo: todas as passagens alteradas.

## Cost of Inaction

Sem a correção, o GDD segue como documento autoritativo contradizendo a autoridade local: os riscos ALTA de "N3 aberta: `LICENSE` MIT vs AGPL/`COPYING` no GDD" e de "CI prometida no dia 1 entregue apenas como gate local" permanecem (`docs/AVALIACAO-STACK-MDMV.md:437-438,362`); o risco jurídico para forks e operadores de servidores descrito no ADR-002 não é removido (`docs/adr/002-licenca-mit-nucleo.md:10`); N3 continua a bloquear a "Primeira publicação" (`docs/GDD - Insurgência.md:1668`); a dívida 4 e o próximo passo 1 do STATE seguem abertos (`.specify/memory/STATE.md:27,39`) e a promessa de CI dia 1 continua a ser lida como compromisso (`docs/AVALIACAO-STACK-MDMV.md:22`). Qualquer efeito remoto (aprovação, CI, merge, publicação) segue `[NV]` até confirmação ao vivo.

## Open Questions

- [NEEDS CLARIFICATION: autoria e patrocínio nomeados da proposta de emenda — `docs/governance/80-20-ranking-2026-11-19.md:31,37` e `docs/PROGRESSO.md:97` registram a escolha, não o proponente.]
- [NEEDS CLARIFICATION: a aprovação humana dos ADRs (hoje `Pendente de aprovação`, `docs/adr/001-excecao-linguagem-gdscript.md:6`; `docs/adr/002-licenca-mit-nucleo.md:6`; `.specify/memory/STATE.md:25`) é pré-requisito para a emenda?]
- [NEEDS CLARIFICATION: a emenda cobre as 7 passagens mapeadas (`docs/PROGRESSO.md:97`) ou apenas N3 e CI, ficando `docs/GDD - Insurgência.md:1389` e `:1459` fora de escopo (`docs/adr/002-licenca-mit-nucleo.md:28,41`)?]
- [NEEDS CLARIFICATION: a contradição de CI se resolve registrando o DEFER com gatilho (saída de `docs/AVALIACAO-STACK-MDMV.md:380`) ou alterando a promessa em `docs/GDD - Insurgência.md:1594`? As fontes divergem sobre emendar ou não.]
- [NEEDS CLARIFICATION: qual numeração de ADR usar na rastreabilidade — a provisória (`docs/AVALIACAO-STACK-MDMV.md:406,410`: ADR-002=CI, ADR-006=licenciamento) ou a materializada (`docs/adr/002-licenca-mit-nucleo.md:1-6`: ADR-002=MIT-only)?]
- [NEEDS CLARIFICATION: N3 deixar de bloquear a "Primeira publicação" (`docs/GDD - Insurgência.md:1668`) depende de alguma publicação externa ainda não verificada?]
- [NEEDS CLARIFICATION: quem sanciona a redação final do GDD e em qual ciclo (`.specify/memory/STATE.md:39`; `docs/adr/002-licenca-mit-nucleo.md:43-45`)?]
- `[NV]`: aprovação em plataforma remota, execução de CI, merge, proteção de branch e qualquer efeito externo — não verificados ao vivo (`docs/adr/001-excecao-linguagem-gdscript.md:40`; `docs/adr/002-licenca-mit-nucleo.md:45`; `.specify/memory/STATE.md:31-35`; `docs/PROGRESSO.md:72-79`).
