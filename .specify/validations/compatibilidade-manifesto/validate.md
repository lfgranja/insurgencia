# Validação (R4) — 011-compatibilidade-manifesto

**Data:** 2026-11-20T03:18Z · **Branch:** `feature/011-compatibilidade-manifesto` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
$ timeout 25s $G --headless --path . --script res://tests/test_manifesto.gd
PASS manifesto_original_aceito / manifesto_ausente_recusado / manifesto_incompleto_recusado /
divergencia_de_versao_falha / divergencia_de_hash_falha / modulo_ausente_falha /
arquivo_de_regras_ausente_falha
OK test_manifesto — 7 PASS, 0 FAIL, exit=0

$ make check-compat GODOT=$G
COMPAT_OK module_id=insurgencia.exemplo  exit=0

$ 3 execucoes de test_manifesto.gd -> /tmp/mf-{1,2,3}.log
diff -q /tmp/mf-1.log /tmp/mf-2.log && diff -q /tmp/mf-2.log /tmp/mf-3.log -> IDENTICO

$ cp regras.cfg /tmp/regras-original.cfg; printf '\n' >> regras.cfg; make check-compat GODOT=$G; cp /tmp/regras-original.cfg regras.cfg
COMPAT_FALHA HASH_MODULO_DIVERGENTE
CLIENTE_ERRO HASH_MODULO_DIVERGENTE
make_exit=2   (script Godot retorna 1; GNU make reporta 2 na receita que falha — SC-004 corrigido)
hash_restaurado=b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746
```

## Correção ao plano
`manifesto.gd` usa `HashingContext.HASH_SHA256` sobre os bytes de `regras.cfg`; o plano repetia `sha256_string()`, inexistente no Godot 4.7 (já corrigido no ciclo 009). Nenhuma outra divergência.

## Conformidade
SC-001 7 PASS/exit 0 · SC-002 COMPAT_OK/exit 0 · SC-003 3 execuções idênticas · SC-004 falha por hash com restauração e código de saída observado · 5 gates anexados abaixo.

## Ressalva
Sondagem de lobby/worker e publicação de release permanecem fora (`doc-26` DEFER); nenhum deploy/remote é afirmado.

## Gates finais (2026-11-20T03:17:25Z)

```text
sdd-completeness:     PASS (spec_dirs=11 gaps=0 baselined=0)  exit=0
work-order-dag:       PASS (files=11 tasks=222 cycles=0)      exit=0
task-id-uniqueness:   PASS (tasks_files=11 duplicates=0)      exit=0
converge-append-only: PASS (files=0 baselined=0)              exit=0
hub-docs-gate:        PASS (0 errors, 0 warnings)             exit=0
```

`check_sdd_completeness` esteve RED (`gaps=1: release, validate`) até a criação destes artefatos; a saída acima é a reexecução.
