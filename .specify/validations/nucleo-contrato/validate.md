# Validação (R4) — 008-nucleo-contrato

**Data:** 2026-11-20T02:02Z · **Branch:** `feature/008-nucleo-contrato` · **HEAD:** `2af7537` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ /home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64 --headless --version
4.7.2.stable.official.ed1daf0bf

$ sha256sum modulos/exemplo/regras.cfg
b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746  modulos/exemplo/regras.cfg

$ /home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64 --headless --path . --script res://tests/test_contrato.gd
PASS protocolo_versao_1
PASS campo_presente_preservado
PASS campo_ausente_usa_padrao
PASS campo_forjado_descartado
PASS tag_discriminante_presente
PASS snapshot_meta_to_dict
PASS module_versao_igual
PASS module_id
PASS hash_sha256_64_hex
PASS module_11_campos_conforme
PASS campo_obrigatorio_ausente_recusado
PASS campo_desconhecido_recusado
PASS int_como_texto_recusado
PASS hash_63_recusado
PASS hash_65_recusado
PASS content_bytes_acima_do_limite_recusado
OK test_contrato
exit=0

$ git diff --stat -- nucleo/
(vazio)

$ grep -c '"assets": \[\]' modulos/exemplo/module.json
1

$ laço dos 39 caminhos canônicos
TOTAL=39 PRESENTES=14 AUSENTES=25
```

## Conformidade com SCs

| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | 14/39 presentes; 25 pendências das Tasks 2–6 registradas | PASS no escopo do ciclo |
| SC-002 | 16 `PASS` + `OK test_contrato` + exit 0 | PASS |
| SC-003 | hash SHA-256 idêntico ao do plano | PASS |
| SC-004 | 5 recusas exercitadas (63/65 = 2 hashs), 10 verificações no total | PASS |
| SC-005 | diff vazio; `assets` vazio contado = 1 | PASS |

## Ressalva
`OK_CAMINHOS` completo (39/39) **não** é reivindicado neste ciclo; 25 caminhos dependem das Tasks 2–6. Gates finais após a criação destes artefatos R4/R7: ver saída colada em `docs/PROGRESSO.md` e Bloco 7 do relatório.

**[NV]:** PR, CI remoto, merge e proteção não são afirmados aqui; verificação remota anterior é datada de 2026-11-20T01:42Z.

## Gates finais (2026-11-20T02:02:06Z, após R4/R7)

```text
check_sdd_completeness: PASS (spec_dirs=8 gaps=0 baselined=0 strict_from=88)  exit=0
work-order-dag:         PASS (files=8 tasks=222 edges=0 cycles=0)             exit=0
task-id-uniqueness:     PASS (tasks_files=8 duplicates=0 baselined=0)        exit=0
converge-append-only:   PASS (files=0 baselined=0)                            exit=0
hub-docs-gate:          PASS (0 errors, 0 warnings)                           exit=0
```

Nota: entre a abertura do ciclo e a criação destes artefatos, `check_sdd_completeness` esteve
RED (`gaps=1: release, validate`). A saída acima é a reexecução após materializá-los.
