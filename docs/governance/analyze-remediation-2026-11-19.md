# Contrato de remediação do analyze — Feature 001 (2026-11-19)

Fonte: relatório de analyze (somente leitura) com 14 achados. Decisões canônicas do orquestrador:

1. **C1 — caminho canônico dos ADRs:** `docs/adr/001-excecao-linguagem-gdscript.md` e
   `docs/adr/002-licenca-mit-nucleo.md`. Atualizar `plan.md`, `research.md`, `data-model.md`,
   `contracts/adr-contract.md` e `quickstart.md` para esses caminhos. `tasks.md` já os usa.
2. **C2 — divergência R1 ↔ spec:** registrar explicitamente em `research.md` que R1 fala em
   "ADRs aprovados no ciclo" e a spec veta aprovação verde; reconciliação: entregável =
   ADRs materializados e revisados localmente; aprovação humana é `[NV]`. Métrica do gate
   `check_sdd_completeness PASS` é evidência local, não afirmação de merge/CI.
3. **C3/H5 — tasks.md:** manter apenas fases Setup → Foundational → User Stories → Polish
   (T001–T035). Remover T036–T046 (analyze/implement/converge/validate/release são portarias
   separadas, não tarefas). A regra de STOP por checkbox `[ ]` vale somente para o checklist de
   domínio em revisão humana, não para bloquear a implementação documental.
4. **C4 — T032:** remover a escrita em `quickstart.md`; saída/evidência vai para
   `.specify/validations/<slug>/validate.md` na fase R4.
5. **H6 — `[NV]` literal:** substituir prosa "não verificado/não confirmado" por `[NV]` em
   afirmações remotas de `spec.md`.
6. **M7 — Clarifications:** acrescentar a 4ª Q/A (autoria/histórico da ideia) em
   `## Clarifications`, com resolução por premissa, e registrar que R3 foi aplicado uma pergunta
   por vez.
7. **M8 — IDs:** completar listas de IDs em T015/T025 ou relaxar a regra para "IDs afetados";
   definir formato `BUG-###` para IDs de bug no contrato de ADR.
8. **M9 — escopo temporal:** alinhar "nenhum gate neste ciclo" para "nenhum gate na fase de
   planejamento; R4/R7 executados como portarias do ciclo completo".
9. **L10–L14:** D1–D8; prefixar IDs de checklist por domínio ao agregar (REQ-/ADR-); headings
   `###` para versões; explicitar que a regra de `[x]` vale só para `adr-quality.md`; registrar
   resultado esperado do `check_sdd_completeness` e quem materializa baseline, sem afirmar verde
   antecipado.

Depois das correções, **rodar analyze novamente** (somente leitura) e só então prosseguir para
implement → converge → validate → release.
