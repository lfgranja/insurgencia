extends Node
class_name ServidorAutoritativo

const PORTA_PADRAO: int = 7777
const MAX_CLIENTES: int = 4

const Protocolo = preload("res://nucleo/contratos/protocolo.gd")
const Mundo = preload("res://nucleo/mundo.gd")
const ValidacaoComandos = preload("res://nucleo/sim/validacao.gd")

const SNAPSHOT_HZ: int = 20
const HASH_MODULO: String = "b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746"

var mundo = Mundo.new()
var peers_aceitos: Dictionary = {}
var _peer_de_rede: ENetMultiplayerPeer
var snapshots: Array = []
var _snapshot_acumulador: float = 0.0

func abrir(porta: int = PORTA_PADRAO) -> void:
	_peer_de_rede = ENetMultiplayerPeer.new()
	var erro: int = _peer_de_rede.create_server(porta, MAX_CLIENTES)
	if erro != OK:
		push_error("Falha ao abrir servidor na porta %d (erro %d)" % [porta, erro])
		return
	var arvore: MainLoop = Engine.get_main_loop()
	if arvore is SceneTree:
		var api: MultiplayerAPI = (arvore as SceneTree).get_multiplayer()
		if api != null:
			api.multiplayer_peer = _peer_de_rede
			api.peer_connected.connect(_on_peer_connected)
			api.peer_disconnected.connect(_on_peer_disconnected)
	print("SERVIDOR_PRONTO porta=%d" % porta)

func _on_peer_connected(peer_id: int) -> void:
	print("PEER_CONECTADO id=%d aguardando_JOIN" % peer_id)

func _on_peer_disconnected(peer_id: int) -> void:
	aceitar_saida(peer_id)

func handle_join(peer_id: int, dados: Dictionary) -> Dictionary:
	var join: Dictionary = Protocolo.normalizar_pacote(Protocolo.TipoPacote.JOIN, dados)
	if int(join["protocol_version"]) != Protocolo.PROTOCOL_VERSION:
		return {"aceito": false, "codigo": Protocolo.REJEICAO_VERSAO, "peer_id": peer_id}
	if str(join["module_id"]).is_empty():
		return {"aceito": false, "codigo": Protocolo.REJEICAO_MODULO, "peer_id": peer_id}
	if str(join["content_hash"]).length() != 64:
		return {"aceito": false, "codigo": Protocolo.REJEICAO_HASH, "peer_id": peer_id}
	peers_aceitos[peer_id] = {
		"module_id": str(join["module_id"]),
		"content_hash": str(join["content_hash"]),
	}
	mundo.adicionar_jogador(peer_id)
	return {"aceito": true, "codigo": "OK", "peer_id": peer_id}

func handle_command(peer_id: int, dados: Dictionary) -> Dictionary:
	if not peers_aceitos.has(peer_id):
		return {"aceito": false, "codigo": "PEER_NAO_AUTORIZADO"}
	var cmd: Dictionary = Protocolo.normalizar_pacote(Protocolo.TipoPacote.COMANDO, dados)
	var resultado: Dictionary = ValidacaoComandos.validar(mundo, peer_id, cmd)
	if not bool(resultado["valido"]):
		return {"aceito": false, "codigo": resultado["codigo"]}
	mundo.jogadores[peer_id]["seq"] = int(cmd["seq"])
	mundo.jogadores[peer_id]["request_id"] = int(cmd["request_id"])
	mundo.agendar_comando(peer_id, cmd)
	return {"aceito": true, "codigo": "OK"}

func aceitar_saida(peer_id: int) -> void:
	peers_aceitos.erase(peer_id)
	mundo.remover_jogador(peer_id)

func emitir_snapshot() -> Dictionary:
	var meta := Protocolo.SnapshotMeta.new(
		Protocolo.PROTOCOL_VERSION, mundo.epoch, mundo.match_id, mundo.tick, HASH_MODULO
	)
	var snap: Dictionary = meta.to_dict()
	snap["jogadores"] = mundo.jogadores.duplicate(true)
	snapshots.append(snap)
	return snap

func avancar_com_snapshots() -> void:
	for i in range(30):
		mundo.avancar_tick()
		_snapshot_acumulador += Mundo.PASSO_SEGUNDOS
		if _snapshot_acumulador + 0.0000001 >= 1.0 / float(SNAPSHOT_HZ):
			_snapshot_acumulador -= 1.0 / float(SNAPSHOT_HZ)
			emitir_snapshot()
