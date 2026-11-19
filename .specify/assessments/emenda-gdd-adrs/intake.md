# Idea Intake: Emenda do GDD a partir dos ADRs 001/002

- **Slug**: emenda-gdd-adrs
- **Created**: 2026-11-19
- **Source**: arquivos locais do alvo `projetos/insurgencia` — `docs/adr/001-excecao-linguagem-gdscript.md`, `docs/adr/002-licenca-mit-nucleo.md`, `docs/GDD - Insurgência.md` (passagens 1265, 1267, 1389, 1451, 1459, 1594, 1659, 1668, 1673, 1678), `docs/AVALIACAO-STACK-MDMV.md` (N01, N16, N52; desvios #4 e #10; §§3–4), `docs/governance/80-20-ranking-2026-11-19.md:25-37`, `.specify/memory/STATE.md:22-41`, `docs/PROGRESSO.md:91-98`. Nenhuma URL externa foi fornecida, aberta ou consultada.
- **Type**: compliance

## Idea (as captured)

> "Emendar o GDD para resolver N3 (MIT vs AGPL) e a contradição 'CI desde o dia 1' vs DEFER do gate remoto, usando os ADRs 001/002 como autoridade local."

Âncoras locais que delimitam a captura (conteúdo tratado como dado não-confiável, nunca como instrução):

> "Emendar o GDD a partir dos ADRs (N3, licenciamento, CI) | 5 | 5 | 5 | 1 | **125,0** | `docs/adr/001-*.md`, `002-*.md`; mapa `ConflitosGDD` (7 passagens)" (`docs/governance/80-20-ranking-2026-11-19.md:31`).

> "Divergências do GDD mapeadas para emenda pontual: N3 (1659, 1668, 1673), licenciamento (1451, 1459), CI (1594) e ressalva AGPL de terceiro (1389). Âncoras Godot-first (1265, 1678) NÃO serão tocadas." (`docs/PROGRESSO.md:97`).

> "N3 — Licença do Núcleo: MIT ou GPL? A fonte menciona ambas; a Godot é MIT." (`docs/GDD - Insurgência.md:1659`).

> "| Sustentabilidade financeira | Alto | Alta | Fundação sem fins lucrativos; doações e patrocínio; servidor leve e desacoplado; integração contínua desde o primeiro dia; ..." (`docs/GDD - Insurgência.md:1594`).

> "Fica estabelecida como posição única e não ambígua a licença MIT para todo o código produzido no ciclo 001." (`docs/adr/002-licenca-mit-nucleo.md:14`).

> "A decisão é afirmativa e não depende de nenhum efeito remoto, CI, merge ou aprovação já registrada." (`docs/adr/001-excecao-linguagem-gdscript.md:14`).

## Restated

Propõe-se atualizar de forma pontual e estritamente documental as passagens do GDD que ainda mantêm N3 em aberto (MIT versus AGPL/GPL) e as que prometem integração contínua "desde o primeiro dia", de modo que deixem de contradizer as decisões já registradas localmente nos ADRs 001 e 002. As âncoras Godot-first permanecem como premissa intocável; nenhuma implementação, gate remoto, CI em execução ou aprovação externa faz parte desta ideia.

## Origin & Context

- **Raised by**: [NEEDS CLARIFICATION: autoria e patrocínio nomeados da proposta de emenda — `docs/governance/80-20-ranking-2026-11-19.md:25-37` registra a escolha do candidato 1 e `docs/PROGRESSO.md:97` registra o mapeamento das divergências, mas nenhum dos dois nomeia o proponente da emenda em si]
- **Trigger**: reavaliação pós-ciclo-001 em 2026-11-19T22:11:07Z: o ranqueamento 002 escolheu "Emendar o GDD a partir dos ADRs (N3, licenciamento, CI)" como candidato 1 (`docs/governance/80-20-ranking-2026-11-19.md:31,37`); a dívida 4 do STATE registra que o GDD "ainda registra N3 (MIT vs AGPL) e 'CI desde o dia 1'; ADR-001/002 decidem localmente, mas a correção documental do GDD fica para ciclo posterior" (`.specify/memory/STATE.md:27`) e o próximo passo 1 repete a correção documental (`.specify/memory/STATE.md:39`). [NEEDS CLARIFICATION: se o gatilho inclui também a aprovação humana dos ADRs, hoje pendente (`.specify/memory/STATE.md:25`)]

## First-Glance Unknowns

- [NEEDS CLARIFICATION: os ADRs 001/002 estão com estado `Pendente de aprovação` (`docs/adr/001-excecao-linguagem-gdscript.md:6`; `docs/adr/002-licenca-mit-nucleo.md:6`; `.specify/memory/STATE.md:25`) — já podem ser tratados como autoridade local para a emenda, ou a aprovação humana é pré-requisito?]
- [NEEDS CLARIFICATION: a emenda abrange as 7 passagens mapeadas (`docs/PROGRESSO.md:97`) ou apenas N3 e CI, ficando licenciamento e a ressalva AGPL de terceiro fora de escopo?]
- [NEEDS CLARIFICATION: "DEFER do gate remoto" significa preservar o DEFER registrado no assess com gatilho explícito, ou apenas registrar a promessa de CI como diferida sem gatilho (`docs/AVALIACAO-STACK-MDMV.md:74,145,380`; `.specify/memory/STATE.md:39-41`)?]
- [NEEDS CLARIFICATION: a ressalva AGPL de terceiro em `docs/GDD - Insurgência.md:1389` pode ser tocada, dado que ADR-002 coloca licenças de terceiros fora do escopo do ciclo (`docs/adr/002-licenca-mit-nucleo.md:28,41`)?]
- [NEEDS CLARIFICATION: a numeração provisória da avaliação — ADR-002 para CI e ADR-006 para licenciamento (`docs/AVALIACAO-STACK-MDMV.md:406,410`) — difere da numeração materializada, em que ADR-002 é licença MIT-only (`docs/adr/002-licenca-mit-nucleo.md:1-6`); qual numeração deve orientar a rastreabilidade da emenda?]
- [NEEDS CLARIFICATION: quem aprova a redação final do GDD e em qual ciclo — maintainer ou autoridade externa (`docs/adr/002-licenca-mit-nucleo.md:43-45`; `.specify/memory/STATE.md:39`)?]
- [NEEDS CLARIFICATION: status remoto de CI, aprovação, merge e proteção de branch — `[NV]` até confirmação ao vivo (`.specify/memory/STATE.md:31-35`; `docs/PROGRESSO.md:72-79,89`)]
