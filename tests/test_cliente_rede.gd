extends SceneTree

const Protocolo = preload("res://nucleo/contratos/protocolo.gd")

const HASH_MODULO: String = "b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746"
const TIMEOUT_MS: int = 5000
const SERVIDOR_ID: int = 1

var _api: SceneMultiplayer
var _porta: int = 7780
var _peer: int = 1
var _inicio_ms: int = 0
var _concluido: bool = false
var _codigo_saida: int = 0
var _quadros: int = 0

func _initialize() -> void:
	for arg in OS.get_cmdline_user_args():
		if arg.begins_with("--porta="):
			_porta = int(arg.substr(len("--porta=")))
		elif arg.begins_with("--peer="):
			_peer = int(arg.substr(len("--peer=")))
	_api = get_multiplayer() as SceneMultiplayer
	if _api == null:
		printerr("FAIL api_multijogador_indisponivel")
		_finalizar(1)
		return
	_api.peer_connected.connect(_on_peer_connected)
	_api.peer_packet.connect(_on_peer_packet)
	_api.connection_failed.connect(_on_connection_failed)
	var cliente := ENetMultiplayerPeer.new()
	var erro: int = cliente.create_client("127.0.0.1", _porta)
	if erro != OK:
		printerr("FAIL criar_cliente_erro=%d" % erro)
		_finalizar(1)
		return
	_api.multiplayer_peer = cliente
	_inicio_ms = Time.get_ticks_msec()

func _enviar(envelope: Dictionary) -> bool:
	var erro: int = _api.send_bytes(JSON.stringify(envelope).to_utf8_buffer(), SERVIDOR_ID)
	if erro != OK:
		printerr("FAIL send_bytes_erro=%d" % erro)
		return false
	return true

func _on_peer_connected(id: int) -> void:
	if id != SERVIDOR_ID or _concluido:
		return
	var enviado: bool = _enviar({
		"tipo": "join_request",
		"protocol_version": Protocolo.PROTOCOL_VERSION,
		"module_id": Protocolo.MODULO_ID,
		"content_hash": HASH_MODULO,
		"client_build": "e2e",
	})
	if not enviado:
		_finalizar(1)

func _on_connection_failed() -> void:
	printerr("FAIL conexao_com_servidor peer=%d" % _peer)
	_finalizar(1)

func _on_peer_packet(id: int, packet: PackedByteArray) -> void:
	var json := JSON.new()
	var erro: int = json.parse(packet.get_string_from_utf8())
	if erro != OK or not (json.data is Dictionary):
		printerr("FAIL resposta_invalida peer=%d erro=%d" % [_peer, erro])
		_finalizar(1)
		return
	var resposta: Dictionary = json.data
	match str(resposta.get("tipo", "")):
		"join_accept":
			print("PASS join_aceito_%d" % _peer)
			var enviado: bool = _enviar({
				"tipo": "comando",
				"seq": 1,
				"request_id": 1,
				"acao": "mover",
				"destino": [0.5, 0.0],
			})
			if not enviado:
				_finalizar(1)
				return
			print("PASS comando_enviado_%d" % _peer)
			print("OK test_cliente_rede")
			_finalizar(0)
		"join_reject":
			printerr("FAIL join_rejeitado peer=%d reason=%s" % [_peer, str(resposta.get("reason", ""))])
			_finalizar(1)
		_:
			printerr("FAIL resposta_desconhecida peer=%d origem=%d" % [_peer, id])
			_finalizar(1)

func _finalizar(codigo: int) -> void:
	if _concluido:
		return
	_concluido = true
	_codigo_saida = codigo

func _process(_delta: float) -> bool:
	if _concluido:
		_quadros += 1
		if _quadros >= 2:
			quit(_codigo_saida)
		return false
	if Time.get_ticks_msec() - _inicio_ms >= TIMEOUT_MS:
		printerr("FAIL timeout peer=%d" % _peer)
		_finalizar(1)
	return false
