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

func _initialize() -> void:
	var servidor := ServidorAutoritativo.new()
	root.add_child(servidor)
	servidor.abrir(7778)
	var hash_valido: String = "b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746"
	for id in [2, 3]:
		var r: Dictionary = servidor.handle_join(id, {
			"protocol_version": Protocolo.PROTOCOL_VERSION,
			"module_id": Protocolo.MODULO_ID,
			"content_hash": hash_valido,
		})
		_verificar(bool(r["aceito"]), "cliente_loopback_%d_aceito" % id)
	servidor.avancar_com_snapshots()
	_verificar(servidor.snapshots.size() == 20, "20_snapshots_em_1s")
	var snap: Dictionary = servidor.snapshots[0]
	for campo in ["protocol_version", "epoch", "match_id", "tick", "content_hash"]:
		_verificar(snap.has(campo), "snapshot_campo_%s" % campo)
	if _falhas == 0:
		print("OK test_loopback")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)
