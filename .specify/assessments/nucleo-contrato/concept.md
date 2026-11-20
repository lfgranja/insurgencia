# Concept — Núcleo, contrato versionado e Módulo de exemplo

**Data:** 2026-11-20T02:00Z

## Opções consideradas
1. **Implementar Task 1 literalmente do plano** (escolhida) — contrato puro em GDScript + Módulo JSON + teste `extends SceneTree`.
2. Implementar Task 1 + Tasks 2–6 num único ciclo — rejeitada: PR de ~2000 LOC, baixa revisabilidade, mistura responsabilidades e exige netcode/loopback ainda não validados.
3. Criar apenas `Makefile`/`manifesto.json` — rejeitada: depende do contrato e não destrava CE-1.
4. Migrar para Rust/Bevy — rejeitada: ADR-001 permanece válida (`docs/research/2026-11-20-alternativas-netcode-oficial.md`).

## Consequências
- Arquivos novos apenas em `nucleo/`, `modulos/exemplo/`, `tests/`; `project.godot` existente é modificado com diff registrado.
- Nenhuma dependência externa, nenhuma alteração de gate, nenhuma transição de estado remota.
