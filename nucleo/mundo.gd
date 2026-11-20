extends RefCounted
class_name Mundo

const PASSO_SEGUNDOS: float = 1.0 / 30.0

var tick: int = 0
var epoch: int = 1
var match_id: int = 1
var jogadores: Dictionary = {}
var comandos_pendentes: Array = []
var comandos_do_tick: Array = []

func adicionar_jogador(peer_id: int) -> void:
	jogadores[peer_id] = {"posicao": [0.0, 0.0], "vida": 100, "seq": 0, "request_id": 0}

func remover_jogador(peer_id: int) -> void:
	jogadores.erase(peer_id)

func agendar_comando(peer_id: int, cmd: Dictionary) -> void:
	var copia: Dictionary = cmd.duplicate(true)
	copia["peer_id"] = peer_id
	comandos_pendentes.append(copia)

func avancar_tick() -> void:
	_fase_prepare()
	_fase_skills()
	_fase_finish()
	tick += 1

func _fase_prepare() -> void:
	comandos_do_tick = comandos_pendentes.duplicate(true)
	comandos_pendentes.clear()

func _fase_skills() -> void:
	for cmd in comandos_do_tick:
		if cmd["acao"] == "mover":
			var jogador: Dictionary = jogadores.get(cmd["peer_id"], {})
			if jogador.is_empty():
				continue
			jogador["posicao"] = (cmd["destino"] as Array).duplicate(true)

func _fase_finish() -> void:
	comandos_do_tick.clear()

func hash_estado() -> String:
	var ctx := HashingContext.new()
	ctx.start(HashingContext.HASH_SHA256)
	ctx.update(str(jogadores, tick).to_utf8_buffer())
	return ctx.finish().hex_encode()
