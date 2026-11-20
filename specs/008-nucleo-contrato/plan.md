# Implementation Plan — Núcleo, contrato versionado e Módulo de exemplo

**Data:** 2026-11-20 · **Fonte:** `docs/plans/2026-11-19-ciclo-80-20-omoba.md` Task 1, Steps 1–9.

## Arquivos (exatamente os do plano)
`project.godot`, `nucleo/contratos/protocolo.gd`, `nucleo/contratos/carregador_modulo.gd`, `modulos/exemplo/regras.cfg`, `modulos/exemplo/module.json`, `modulos/exemplo/personagens.json`, `tests/test_contrato.gd`.

## Steps
1. Diff de `project.godot` (existente) registrado; em seguida aplicar o conteúdo do Step 1 do plano.
2. `protocolo.gd`: código literal do Step 2 do plano.
3. `regras.cfg`: 7 linhas literais do Step 3 (218 bytes, newline final); conferir `sha256sum`.
4. `module.json`: 11 campos do Step 4; `content_hash` e `content_bytes` recalculados se o hash divergir.
5. `personagens.json`: `{"personagens": []}`.
6. `carregador_modulo.gd`: código literal do Step 5.
7. `test_contrato.gd`: código literal do Step 6 (`extends SceneTree`).
8. Verificação: 14/39 caminhos presentes (25 pendentes das Tasks 2–6), teste, hash, isolamento e gates — saídas coladas no R4.
9. Commit atômico em PT-BR com staging explícito dos paths criados.

## Comandos de verificação
```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
"$G" --headless --path . --script res://tests/test_contrato.gd
sha256sum modulos/exemplo/regras.cfg
git diff --stat -- nucleo/
grep -c '"assets": \[\]' modulos/exemplo/module.json
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```

## Riscos e mitigação
- `project.godot` existente → diff registrado antes da edição (FR-002).
- Hash dependente de bytes exatos → `sha256sum` antes de fechar o `module.json`.
- `--script` exige `extends SceneTree` → teste já conforme.
- Árvore suja com trabalho alheio → staging explícito, nunca `git add -A`.
