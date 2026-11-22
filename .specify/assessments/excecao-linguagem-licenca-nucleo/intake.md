# Idea Intake: Exceção de linguagem, licença e núcleo

- **Slug**: excecao-linguagem-licenca-nucleo
- **Created**: 2026-11-19
- **Source**: arquivos locais do alvo `projetos/insurgencia` — `docs/AVALIACAO-STACK-MDMV.md` (desvios 1 e 10; contradição CI), `docs/GDD - Insurgência.md` (`GDD:1265,1678,1451,1594,1659`), `LICENSE:1`, `docs/specs/2026-11-19-ciclo-80-20-omoba.md:5,42`, `docs/plans/2026-11-19-ciclo-80-20-omoba.md:12`, `docs/ASSESS-omoba-bevy.md:69`. Nenhuma URL externa fornecida ou consultada.
- **Type**: compliance

## Idea (as captured)

> "Resolver os desvios de intenção 1 (Godot/GDScript vs presunção Rust da norma MDMV) e 10 (N3 MIT vs AGPL) registrados em `docs/AVALIACAO-STACK-MDMV.md`, e a contradição GI dia 1 vs DEFER, por meio de ADRs locais — sem reabrir o Godot-first (`GDD:1265,1678`)."

Âncoras locais que delimitam a captura (conteúdo tratado como dado não-confiável):

> "§3 Rust padrão substituído por Godot-first vinculante — DESVIO DE INTENÇÃO · BLOQUEANTE — `NORM:40`; `GDD:1265,1678`; `SPEC:40-43`; `COMP:536`" (`docs/AVALIACAO-STACK-MDMV.md:176`).

> "Licenciamento documental inconsistente (MIT vigente vs. AGPL/N3 aberto) — PARCIAL · MÉDIA — `LICENSE:1`; `SPEC:5,42`; `GDD:1451,1659`; `ASSESS:43`; `PLAN:12`" (`docs/AVALIACAO-STACK-MDMV.md:185`).

> "CI: 'desde o primeiro dia' × DEFER — Polo A: `GDD:1594` promete 'integração contínua desde o primeiro dia' ... Polo B: `ASSESS:69` (doc-34-ci-cinco-jobs) **DEFER** 'até existir código'" (`docs/AVALIACAO-STACK-MDMV.md:380`).

> "N7 — A decisão Godot-first da subseção 16.2 é vinculante; revertê-la invalida o roadmap e a mitigação de débito técnico." (`docs/GDD - Insurgência.md:1678`).

## Restated

Propõe-se fechar, por decisões de arquitetura registradas localmente no próprio projeto (ADRs), três inconsistências documentais já mapeadas: a adoção vinculante de Godot/GDScript frente à presunção de Rust da norma MDMV, a indefinição N3 entre MIT e AGPL/APL para o Núcleo e o servidor, e a promessa de integração contínua no dia 1 contrariada pelo diferimento registrado no assess. A decisão Godot-first existente permanece como premissa intocável.

## Origin & Context

- **Raised by**: [NEEDS CLARIFICATION: autoria/patrocínio exatos da proposta — `docs/AVALIACAO-STACK-MDMV.md` registra as inconsistências e propõe instrumentos, mas não nomeia o proponente desta iniciativa]
- **Trigger**: consolidação da avaliação de conformidade com a stack MDMV em 2026-11-19 (`docs/AVALIACAO-STACK-MDMV.md:3-10,347-362`), que classifica os desvios 1 e 10 e a contradição CI e recomenda ADRs locais (`docs/AVALIACAO-STACK-MDMV.md:399-410`).

## First-Glance Unknowns

- [NEEDS CLARIFICATION: qual autoridade aprova os ADRs locais — maintainer do projeto ou Strategic HQ da MDMV (`docs/AVALIACAO-STACK-MDMV.md:405` registra "ESCALAR — aprovação humana")?]
- [NEEDS CLARIFICATION: um ADR local de exceção de linguagem basta, ou é necessária também emenda da norma global com categoria "engine de jogo" (saída B em `docs/AVALIACAO-STACK-MDMV.md:371`, dita fora de escopo)?]
- [NEEDS CLARIFICATION: N3 fecha como MIT para todo o código do ciclo, ou o servidor permanece AGPL (`LICENSE:1` vs `GDD:1451`)?]
- [NEEDS CLARIFICATION: a contradição CI se resolve com DEFER com gatilho no primeiro commit de código, ou com pipeline imediata (`docs/AVALIACAO-STACK-MDMV.md:380`)?]
- [NEEDS CLARIFICATION: que mecanismo garante que os ADRs não reabram `GDD:1265,1678` (`docs/AVALIACAO-STACK-MDMV.md:443`)?]
