.PHONY: check-compat

GODOT ?= godot
GODOT_FLAGS := --headless --path . --script

# Gate local de compatibilidade (ast-36): manifesto.json + versao + SHA-256 de 32 bytes.
# Fora do escopo (doc-26 DEFER): sondagem de lobby/worker e publicacao de release.
check-compat:
	$(GODOT) $(GODOT_FLAGS) res://ferramentas/verificar_compatibilidade.gd
