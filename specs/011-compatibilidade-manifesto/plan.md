# Implementation Plan — Manifesto de compatibilidade e gate local de versão/hash

**Data:** 2026-11-20T03:15Z · **Fonte:** `docs/plans/2026-11-19-ciclo-80-20-omoba.md` Task 5 (Steps 1–6, linhas 1029–1215).

## Arquivos (exatamente os do plano)
Criação: `manifesto.json`, `nucleo/compatibilidade/manifesto.gd`, `ferramentas/verificar_compatibilidade.gd`, `Makefile`, `tests/test_manifesto.gd`. Leitura (sem alteração): `nucleo/contratos/protocolo.gd`, `modulos/exemplo/module.json`, `modulos/exemplo/regras.cfg`. Nenhum outro path é tocado.

## Steps
1. Task 5 / Step 1 — escrever `manifesto.json` com os 6 campos obrigatórios e `nucleo/compatibilidade/manifesto.gd` com `CAMPOS`, `carregar`, `validar_formato` e `conferir` (ordem formato→versão→módulo→comprimento do hash→arquivo de regras→hash), usando os códigos literais de `Protocolo` (FR-001, FR-002, FR-003).
2. Task 5 / Step 2 — escrever `ferramentas/verificar_compatibilidade.gd` (`extends SceneTree`) com `COMPAT_OK module_id=<id>`/`COMPAT_FALHA <codigo>`, `quit(0)`/`quit(1)` e as linhas de exibição no cliente (FR-004, CE-4.4/CE-4.6).
3. Task 5 / Step 3 — criar o alvo `check-compat` no `Makefile`, com `GODOT ?= godot` e `--headless --path . --script` (FR-005, SC-002).
4. Task 5 / Step 4 — escrever `tests/test_manifesto.gd` com as 7 verificações literais e o encerramento `OK test_manifesto`/`FALHAS <n>` (FR-006, SC-001).
5. Task 5 / Step 5 — marco CE-4: executar a suíte (7 `PASS` + `OK` + exit 0), `make check-compat` (`COMPAT_OK`), 3 execuções com `diff -q` idêntico e o teste de divergência de hash com captura de status e restauração de `modulos/exemplo/regras.cfg`; conferir os 5 gates (SC-001..SC-004).
6. Task 5 / Step 6 — commit atômico com staging explícito dos cinco paths (a executar na implementação; não realizado nesta etapa de escrita).
7. Convergência READ-ONLY: revisar spec/plano/tasks contra a Task 5 do plano, corrigir divergências na fonte e reexecutar os 5 gates (SC-004).

## Comandos de verificação
```bash
G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
"$G" --headless --version
"$G" --headless --path . --script res://tests/test_manifesto.gd
make check-compat
for i in 1 2 3; do "$G" --headless --path . --script res://tests/test_manifesto.gd >/tmp/manifesto-$i.log 2>&1; done
diff -q /tmp/manifesto-1.log /tmp/manifesto-2.log && diff -q /tmp/manifesto-2.log /tmp/manifesto-3.log
cp modulos/exemplo/regras.cfg /tmp/regras-original.cfg && printf '\n' >> modulos/exemplo/regras.cfg && make check-compat; codigo=$?; cp /tmp/regras-original.cfg modulos/exemplo/regras.cfg; test $codigo -eq 1
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```
Saídas não executadas nesta etapa permanecem **[NV]** e só entram com data/hora na portaria de validação (R4); versão/changelog/rollback em R7. Nenhum artefato das portarias é escrito aqui.

## Riscos e mitigação
- `--script` exige `extends SceneTree` → o executável do gate e a suíte já seguem essa forma.
- O JSON converte números para `float` → comparar `protocol_version` com `int(...)`, sem alterar o campo no arquivo.
- Hash malformado aceito como divergente → recusar comprimento diferente de 64 hex com `MANIFEST_INVALID` antes de abrir o arquivo de regras.
- Vereditos embaralhados → checar versão antes do hash, preservando os quatro códigos do handshake (cor-06).
- Cálculo do SHA-256 de 32 bytes → o plano escreve `sha256_string()`, mas o ciclo 009 registou em R4 que esse método não existe no Godot 4.7.2 e exigiu `HashingContext`; usar a primitiva disponível sem mudar o contrato de 64 hex.
- Teste de divergência suja o repositório → copiar `regras.cfg` antes de alterar, capturar o status e restaurar sempre, mesmo após falha.
- Não determinismo entre execuções → nenhum carimbo de tempo no caminho de saída do gate; só o campo `checked_at` do manifesto é datado e fixo.
- Árvore suja com trabalho alheio → staging explícito dos cinco paths, nunca `git add -A`.
