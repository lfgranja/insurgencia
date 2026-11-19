# Problem Definition: Inconsistências documentais de linguagem, licença e CI no Núcleo

- **Slug**: excecao-linguagem-licenca-nucleo
- **Created**: 2026-11-19
- **Inputs used**: intake.md | research.md

## Problem Statement

Em um repositório ainda 100% documental (`docs/AVALIACAO-STACK-MDMV.md:354`; `docs/specs/2026-11-19-ciclo-80-20-omoba.md:6`), mantenedores do Núcleo e operadores de servidores/forks convivem com três posições documentais mutuamente contraditórias — a linguagem Godot/GDScript vinculante frente à presunção Rust-only da norma MDMV (`docs/AVALIACAO-STACK-MDMV.md:176`; `docs/GDD - Insurgência.md:1265,1678`), o licenciamento N3 indefinido entre MIT vigente e AGPL/COPYING (`LICENSE:1`; `docs/GDD - Insurgência.md:1451,1659`) e a promessa de integração contínua "desde o primeiro dia" contrariada por um diferimento sem gatilho (`docs/GDD - Insurgência.md:1594`; `docs/ASSESS-omoba-bevy.md:69`). Isso dói porque a conformidade integral é impossível sem autorizar a linguagem e fechar a licença (`docs/AVALIACAO-STACK-MDMV.md:362`), e os desvios foram classificados BLOQUEANTE/ALTA e precisam ser decididos "agora, antes que o código exista" (`docs/AVALIACAO-STACK-MDMV.md:28-30`).

## Affected Users & Stakeholders

- **Users**: mantenedores do Núcleo — precisam de uma posição documental coerente sobre a linguagem e a licença antes de escrever código (`docs/AVALIACAO-STACK-MDMV.md:28-30`); operadores de servidores e forks — dependem de regras inspecionáveis, modificáveis e redistribuíveis (`.specify/memory/constitution.md:13`) e sofrem com o risco de fragmentação por forks (`docs/GDD - Insurgência.md:1595`).
- **Stakeholders**: maintainer do projeto — decide e aprova decisões locais no repositório (`docs/AVALIACAO-STACK-MDMV.md:405`); Strategic HQ da MDMV — detém autoridade sobre a norma global e eventual emenda (`docs/AVALIACAO-STACK-MDMV.md:371,405`); comunidade de jogadores — interessada em regras auditáveis e na preservação do pilar Godot-first (`docs/GDD - Insurgência.md:1678`).

## Goals

- Alcançar coerência entre a norma MDMV e a linguagem efetivamente adotada (Godot/GDScript), preservando a decisão Godot-first como premissa intocável (`docs/GDD - Insurgência.md:1265,1678`).
- Estabelecer uma posição documental única e não ambígua sobre o licenciamento do Núcleo e do servidor, substituindo a indefinição N3 (`LICENSE:1`; `docs/GDD - Insurgência.md:1451,1659`).
- Estabelecer uma posição documental única sobre integração contínua que elimine a contradição dia-1 × DEFER (`docs/GDD - Insurgência.md:1594`; `docs/ASSESS-omoba-bevy.md:69`).
- Reduzir de três para zero as inconsistências documentais classificadas como BLOQUEANTE/ALTA (`docs/AVALIACAO-STACK-MDMV.md:176,185,380,436-438`).
- Registrar a autoridade de aprovação das decisões e explicitar que nenhuma delas altera o roadmap ancorado em Godot-first (`docs/AVALIACAO-STACK-MDMV.md:405,443`).

## Non-Goals

- Escrever código, runtime, gates, build, testes, lint ou compilação — o repositório é documental e nenhum gate foi executado (`docs/AVALIACAO-STACK-MDMV.md:354,447`).
- Reabrir, revisar ou reverter a decisão Godot-first (`docs/GDD - Insurgência.md:1265,1678`; `docs/AVALIACAO-STACK-MDMV.md:443`).
- Emendar a norma global MDMV neste escopo — saída classificada como fora de escopo (`docs/AVALIACAO-STACK-MDMV.md:371`).
- Introduzir uma segunda linguagem ou trocar a engine adotada (inclusive a alternativa de servidor Rust, `docs/AVALIACAO-STACK-MDMV.md:372`).
- Verificar status remoto — aprovação humana, execução de CI, merge e branch protection permanecem `[NV]` (`.specify/memory/constitution.md:17`).
- Medir adoção de mercado, telemetria ou pesquisa de usuário — não há dados nem runtime no alvo (`docs/AVALIACAO-STACK-MDMV.md:350,354`).

## Success Metrics

- Inconsistências documentais abertas (desvio 1, desvio 10 e contradição CI): baseline 3 (`docs/AVALIACAO-STACK-MDMV.md:176,185,380`); meta 0.
- Posições conflitantes sobre N3: baseline 2 (MIT vigente em `LICENSE:1` × AGPL/`COPYING` em `docs/GDD - Insurgência.md:1451,1659`); meta 1.
- Posições conflitantes sobre CI: baseline 2 (`docs/GDD - Insurgência.md:1594` × `docs/ASSESS-omoba-bevy.md:69`); meta 1.
- Mudanças no pilar Godot-first: baseline 0 (`docs/GDD - Insurgência.md:1265,1678` vinculante); meta 0.
- Autoridade de aprovação explicitada: baseline 0 (questão em aberto em `docs/AVALIACAO-STACK-MDMV.md:405`); meta 1.
- Qualitativa (rótulo): percepção de "sem reabertura involuntária do GDD" por ADRs de processo — baseline: risco MÉDIO registrado (`docs/AVALIACAO-STACK-MDMV.md:443`); meta: risco mitigado por declaração verificável.

## Cost of Inaction

Sem decisões escritas, persistem o risco BLOQUEANTE de governança sem autorização para GDScript (`docs/AVALIACAO-STACK-MDMV.md:436`), o risco ALTA de licenciamento N3 aberto entre `LICENSE` MIT e AGPL/`COPYING` (`docs/AVALIACAO-STACK-MDMV.md:437`) e o risco ALTA de CI prometida no dia 1 e entregue apenas como gate local (`docs/AVALIACAO-STACK-MDMV.md:438`), todos repetidos na síntese (`docs/AVALIACAO-STACK-MDMV.md:362`). A fragmentação por forks do pilar de soberania permanece sem mitigação (`docs/GDD - Insurgência.md:1595`) e qualquer reversão não intencional de Godot-first invalidaria roadmap e mitigação de débito técnico (`docs/GDD - Insurgência.md:1678`). Como o repositório não tem runtime, os ADRs podem se tornar intenção sem evidência de execução, e aprovação, CI, merge e branch protection seguem `[NV]` (`.specify/memory/constitution.md:17`).

## Open Questions

- [NEEDS CLARIFICATION: autoria e patrocínio exatos da proposta — `docs/AVALIACAO-STACK-MDMV.md:399-410` nomeia os ADRs, não o proponente.]
- [NEEDS CLARIFICATION: autoridade de aprovação — maintainer do projeto ou Strategic HQ da MDMV (`docs/AVALIACAO-STACK-MDMV.md:405`)?]
- [NEEDS CLARIFICATION: uma decisão local basta para GDScript, ou é exigida emenda normativa com categoria "engine de jogo" (saída B em `docs/AVALIACAO-STACK-MDMV.md:371`)?]
- [NEEDS CLARIFICATION: destino final de N3 — MIT para Núcleo e servidor (`docs/plans/2026-11-19-ciclo-80-20-omoba.md:12`) ou AGPL no servidor (`docs/GDD - Insurgência.md:1451`)?]
- [NEEDS CLARIFICATION: gatilho exato da CI — "primeiro commit de código em `nucleo/`" (`docs/AVALIACAO-STACK-MDMV.md:380,406`)?]
- [NEEDS CLARIFICATION: como medir "sem reabrir Godot-first" — declaração textual em cada decisão (`docs/AVALIACAO-STACK-MDMV.md:443`) ou gate de checagem documental?]
- [NV]: aprovação humana, execução de CI, status de merge e branch protection — não verificados nesta rodada (`.specify/memory/constitution.md:17`).
