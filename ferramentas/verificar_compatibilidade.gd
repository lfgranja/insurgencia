extends SceneTree

const ManifestoCompatibilidade = preload("res://nucleo/compatibilidade/manifesto.gd")
const Protocolo = preload("res://nucleo/contratos/protocolo.gd")

func _initialize() -> void:
	var manifesto: Dictionary = ManifestoCompatibilidade.carregar("res://manifesto.json")
	var modulo: Dictionary = ManifestoCompatibilidade.carregar("res://modulos/exemplo/module.json")
	var resultado: Dictionary = ManifestoCompatibilidade.conferir(
		manifesto, modulo, "res://modulos/exemplo/regras.cfg", Protocolo.PROTOCOL_VERSION
	)
	if bool(resultado["ok"]):
		print("COMPAT_OK module_id=%s" % manifesto["module_id"])
		quit(0)
	else:
		printerr("COMPAT_FALHA %s" % resultado["codigo"])
		_exibir_no_cliente(manifesto, str(resultado["codigo"]))
		quit(1)

func _exibir_no_cliente(manifesto: Dictionary, codigo: String) -> void:
	print("CLIENTE_VERSAO_PROTOCOLO ", int(manifesto.get("protocol_version", -1)))
	print("CLIENTE_HASH_MODULO ", str(manifesto.get("content_hash", "")))
	print("CLIENTE_ERRO ", codigo)
