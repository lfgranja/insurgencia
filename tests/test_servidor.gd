extends SceneTree

const ServidorAutoritativo = preload("res://nucleo/servidor.gd")
const Protocolo = preload("res://nucleo/contratos/protocolo.gd")

var _falhas: int = 0

func _verificar(condicao: bool, rotulo: String) -> void:
	if condicao:
		print("PASS ", rotulo)
	else:
		_falhas += 1
		printerr("FAIL ", rotulo)

func _join(versao: int, modulo: String, hash_conteudo: String) -> Dictionary:
	return {
		"protocol_version": versao,
		"module_id": modulo,
		"content_hash": hash_conteudo,
	}

func _initialize() -> void:
	var servidor := ServidorAutoritativo.new()
	var hash_valido: String = "b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746"
	var r: Dictionary = servidor.handle_join(2, _join(Protocolo.PROTOCOL_VERSION, Protocolo.MODULO_ID, hash_valido))
	_verificar(bool(r["aceito"]) and r["codigo"] == "OK", "join_aceito")
	_verificar(servidor.peers_aceitos.has(2) and servidor.mundo.jogadores.has(2), "peer_registrado_no_mundo")
	var versao_errada: Dictionary = servidor.handle_join(3, _join(Protocolo.PROTOCOL_VERSION + 1, Protocolo.MODULO_ID, hash_valido))
	_verificar(not bool(versao_errada["aceito"]) and versao_errada["codigo"] == Protocolo.REJEICAO_VERSAO, "join_recusa_versao")
	_verificar_modulo_vazio(servidor, hash_valido)
	var hash_errado: Dictionary = servidor.handle_join(5, _join(Protocolo.PROTOCOL_VERSION, Protocolo.MODULO_ID, "curto"))
	_verificar(not bool(hash_errado["aceito"]) and hash_errado["codigo"] == Protocolo.REJEICAO_HASH, "join_recusa_hash")
	servidor.aceitar_saida(2)
	_verificar(not servidor.peers_aceitos.has(2) and not servidor.mundo.jogadores.has(2), "saida_remove_peer")
	if _falhas == 0:
		print("OK test_servidor")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)

func _verificar_modulo_vazio(servidor, hash_valido: String) -> void:
	var r: Dictionary = servidor.handle_join(4, _join(Protocolo.PROTOCOL_VERSION, "", hash_valido))
	_verificar(not bool(r["aceito"]) and r["codigo"] == Protocolo.REJEICAO_MODULO, "join_recusa_modulo_ausente")
