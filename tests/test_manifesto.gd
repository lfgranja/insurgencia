extends SceneTree

const ManifestoCompatibilidade = preload("res://nucleo/compatibilidade/manifesto.gd")
const Protocolo = preload("res://nucleo/contratos/protocolo.gd")

var _falhas: int = 0

func _verificar(condicao: bool, rotulo: String) -> void:
	if condicao:
		print("PASS ", rotulo)
	else:
		_falhas += 1
		printerr("FAIL ", rotulo)

func _initialize() -> void:
	var manifesto: Dictionary = ManifestoCompatibilidade.carregar("res://manifesto.json")
	var modulo: Dictionary = ManifestoCompatibilidade.carregar("res://modulos/exemplo/module.json")
	var regras: String = "res://modulos/exemplo/regras.cfg"
	var valido: Dictionary = ManifestoCompatibilidade.conferir(manifesto, modulo, regras, Protocolo.PROTOCOL_VERSION)
	_verificar(bool(valido["ok"]) and valido["codigo"] == "OK", "manifesto_original_aceito")
	var sem_manifesto: Dictionary = ManifestoCompatibilidade.conferir({}, modulo, regras, Protocolo.PROTOCOL_VERSION)
	_verificar(sem_manifesto["codigo"] == Protocolo.REJEICAO_MANIFESTO, "manifesto_ausente_recusado")
	var incompleto: Dictionary = manifesto.duplicate(true)
	incompleto.erase("checked_at")
	var r_incompleto: Dictionary = ManifestoCompatibilidade.conferir(incompleto, modulo, regras, Protocolo.PROTOCOL_VERSION)
	_verificar(r_incompleto["codigo"] == Protocolo.REJEICAO_MANIFESTO, "manifesto_incompleto_recusado")
	var versao: Dictionary = manifesto.duplicate(true)
	versao["protocol_version"] = Protocolo.PROTOCOL_VERSION + 1
	var r_versao: Dictionary = ManifestoCompatibilidade.conferir(versao, modulo, regras, Protocolo.PROTOCOL_VERSION)
	_verificar(r_versao["codigo"] == Protocolo.REJEICAO_VERSAO, "divergencia_de_versao_falha")
	var hash_errado: Dictionary = manifesto.duplicate(true)
	hash_errado["content_hash"] = "0000000000000000000000000000000000000000000000000000000000000000"
	var r_hash: Dictionary = ManifestoCompatibilidade.conferir(hash_errado, modulo, regras, Protocolo.PROTOCOL_VERSION)
	_verificar(r_hash["codigo"] == Protocolo.REJEICAO_HASH, "divergencia_de_hash_falha")
	var sem_modulo: Dictionary = manifesto.duplicate(true)
	sem_modulo["module_id"] = ""
	var r_modulo: Dictionary = ManifestoCompatibilidade.conferir(sem_modulo, modulo, regras, Protocolo.PROTOCOL_VERSION)
	_verificar(r_modulo["codigo"] == Protocolo.REJEICAO_MODULO, "modulo_ausente_falha")
	var r_sem_regras: Dictionary = ManifestoCompatibilidade.conferir(manifesto, modulo, "res://modulos/exemplo/inexistente.cfg", Protocolo.PROTOCOL_VERSION)
	_verificar(r_sem_regras["codigo"] == "REGRAS_AUSENTES", "arquivo_de_regras_ausente_falha")
	if _falhas == 0:
		print("OK test_manifesto")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)
