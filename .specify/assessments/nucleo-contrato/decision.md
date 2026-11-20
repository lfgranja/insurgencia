# Decision — Núcleo, contrato versionado e Módulo de exemplo

**Data:** 2026-11-20T02:01Z · **R1:** `go`

## Veredito
**go** — problema válido (impacto 5, evidência 5), opção recomendada = Opção 1 (Task 1 literal do plano).

## Gate de entrada
- Gate determinístico do alvo: 5/5 PASS em `2026-11-20T01:56:13Z`.
- Nenhum ciclo em voo; `feature.json` apontava para 007 (fechado); NNN resolvido por `create-new-feature.sh --dry-run` = 008.
- Branch `feature/008-nucleo-contrato` criada; âncora agora `specs/008-nucleo-contrato`.

## Restrições de execução
- Diff de `project.godot` registrado antes de qualquer edição.
- Comandos com o binário absoluto do Godot; saídas coladas no validate (R4).
- Sem tocar em paths de outras sessões, factory, remote, baselines ou `converge.md`.
- `analyze` é READ-ONLY; se apontar conflito, corrigir na fonte e rodar de novo.
