extends SceneTree

const Protocolo = preload("res://nucleo/contratos/protocolo.gd")
const CarregadorModulo = preload("res://nucleo/contratos/carregador_modulo.gd")

var _falhas: int = 0

func _verificar(condicao: bool, rotulo: String) -> void:
	if condicao:
		print("PASS ", rotulo)
	else:
		_falhas += 1
		printerr("FAIL ", rotulo)

func _initialize() -> void:
	_verificar(Protocolo.PROTOCOL_VERSION == 1, "protocolo_versao_1")
	var pacote: Dictionary = Protocolo.normalizar_pacote(Protocolo.TipoPacote.COMANDO, {"seq": 7, "vida": 999})
	_verificar(pacote["seq"] == 7, "campo_presente_preservado")
	_verificar(pacote["request_id"] == 0, "campo_ausente_usa_padrao")
	_verificar(not pacote.has("vida"), "campo_forjado_descartado")
	_verificar(pacote["tipo"] == Protocolo.TipoPacote.COMANDO, "tag_discriminante_presente")
	var meta := Protocolo.SnapshotMeta.new(1, 2, 3, 4, "abc")
	_verificar(meta.to_dict()["tick"] == 4, "snapshot_meta_to_dict")
	var modulo: Dictionary = _ler_json("res://modulos/exemplo/module.json")
	_verificar(int(modulo["protocol_version"]) == Protocolo.PROTOCOL_VERSION, "module_versao_igual")
	_verificar(modulo["module_id"] == Protocolo.MODULO_ID, "module_id")
	_verificar(str(modulo["content_hash"]).length() == 64, "hash_sha256_64_hex")
	_verificar(bool(CarregadorModulo.validar(modulo)["valido"]), "module_11_campos_conforme")
	var sem_obrigatorio: Dictionary = modulo.duplicate(true)
	sem_obrigatorio.erase("content_bytes")
	_verificar(CarregadorModulo.validar(sem_obrigatorio)["codigo"] == "CAMPO_OBRIGATORIO_AUSENTE", "campo_obrigatorio_ausente_recusado")
	var desconhecido: Dictionary = modulo.duplicate(true)
	desconhecido["forjado"] = 1
	_verificar(CarregadorModulo.validar(desconhecido)["codigo"] == "CAMPO_DESCONHECIDO", "campo_desconhecido_recusado")
	var tipo_errado: Dictionary = modulo.duplicate(true)
	tipo_errado["protocol_version"] = "1"
	_verificar(CarregadorModulo.validar(tipo_errado)["codigo"] == "TIPO_INVALIDO", "int_como_texto_recusado")
	for tamanho in [63, 65]:
		var hash_errado: Dictionary = modulo.duplicate(true)
		hash_errado["content_hash"] = "a".repeat(int(tamanho))
		_verificar(CarregadorModulo.validar(hash_errado)["codigo"] == "HASH_INVALIDO", "hash_%d_recusado" % tamanho)
	var bytes_excessivos: Dictionary = modulo.duplicate(true)
	bytes_excessivos["content_bytes"] = 25000001
	_verificar(CarregadorModulo.validar(bytes_excessivos)["codigo"] == "BYTES_INVALIDOS", "content_bytes_acima_do_limite_recusado")
	if _falhas == 0:
		print("OK test_contrato")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)

func _ler_json(caminho: String) -> Dictionary:
	var arquivo: FileAccess = FileAccess.open(caminho, FileAccess.READ)
	var dados: Variant = JSON.parse_string(arquivo.get_as_text())
	return dados as Dictionary
