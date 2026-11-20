.PHONY: check check-compat test-contrato test-servidor test-validacao test-ilha test-manifesto test-rede-e2e

GODOT ?= godot
GODOT_FLAGS := --headless --path . --script

check: test-contrato test-servidor test-validacao test-ilha test-manifesto check-compat

test-contrato:
	$(GODOT) $(GODOT_FLAGS) res://tests/test_contrato.gd

test-servidor:
	$(GODOT) $(GODOT_FLAGS) res://tests/test_servidor.gd

test-validacao:
	$(GODOT) $(GODOT_FLAGS) res://tests/test_validacao.gd

test-ilha:
	$(GODOT) $(GODOT_FLAGS) res://tests/test_ilha_sincronia.gd

test-manifesto:
	$(GODOT) $(GODOT_FLAGS) res://tests/test_manifesto.gd

check-compat:
	$(GODOT) $(GODOT_FLAGS) res://ferramentas/verificar_compatibilidade.gd

test-rede-e2e:
	GODOT=$(GODOT) bash tests/run_e2e_rede.sh
