# Diff registrado — project.godot (FR-002)

**Data:** 2026-11-20T02:03Z · **Motivo:** arquivo já existia no alvo; conformidade com a regra "não sobrescrever sem diff explícito e registrado".

## Antes (HEAD `2af7537`)
```ini
; Engine configuration file.
; ... (comentários de cabeçalho)
config_version=5

[application]

config/name="Insurgência"
config/features=PackedStringArray("4.5", "GL Compatibility")
config/icon="res://icon.svg"

[rendering]

renderer/rendering_method="gl_compatibility"
renderer/rendering_method.mobile="gl_compatibility"
```

## Depois (Step 1 do plano)
```ini
[application]
config/name="Insurgencia Nucleo"
config/features=PackedStringArray("4.7")

[rendering]
renderer/rendering_method="gl_compatibility"
```

## Justificativa
Removidos `config/icon` (asset legado), `renderer/rendering_method.mobile` e o cabeçalho de comentários; `config/name` e `config/features` alinhados ao Núcleo 4.7.2 sem `run/main_scene` (inicialização por `--script`). Nenhuma cena principal é introduzida.
