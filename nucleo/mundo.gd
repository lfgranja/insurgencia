extends RefCounted
class_name Mundo

const PASSO_SEGUNDOS: float = 1.0 / 30.0
const IlhaSincronia = preload("res://nucleo/desafios/ilha_sincronia.gd")
const IlhaTopo = preload("res://nucleo/desafios/ilha_topo.gd")
const PontoColeta = preload("res://nucleo/desafios/ponto_coleta.gd")
const Funcoes = preload("res://nucleo/sim/funcoes.gd")

var tick: int = 0
var epoch: int = 1
var match_id: int = 1
var jogadores: Dictionary = {}
var comandos_pendentes: Array = []
var comandos_do_tick: Array = []

var ilha_sincronia = IlhaSincronia.new()
var ilha_topo = IlhaTopo.new()
var ponto_coleta = PontoColeta.new()
var funcoes = Funcoes.new()
var estado_persistido: String = ""

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
		elif cmd["acao"] == "sinalizar_sincronia":
			ilha_sincronia.sinalizar(int(cmd["peer_id"]), int(cmd["alvo"]), float(tick) * PASSO_SEGUNDOS)
		elif cmd["acao"] == "sinalizar_alavanca":
			ilha_topo.sinalizar_alavanca(int(cmd["peer_id"]), int(cmd["alvo"]), float(tick) * PASSO_SEGUNDOS)
		elif cmd["acao"] == "coletar_materiais":
			var resultado: Dictionary = ponto_coleta.coletar(int(cmd["peer_id"]), float(tick) * PASSO_SEGUNDOS)
			if bool(resultado["aceito"]):
				var jogador: Dictionary = jogadores.get(int(cmd["peer_id"]), {})
				if not jogador.is_empty():
					jogador["materiais"] = int(jogador.get("materiais", 0)) + int(resultado["materiais"])
		elif cmd["acao"] == "construir_barreira":
			var jogador_barreira: Dictionary = jogadores.get(int(cmd["peer_id"]), {})
			if jogador_barreira.is_empty():
				continue
			var materiais: int = int(jogador_barreira.get("materiais", 0))
			var resultado_barreira: Dictionary = funcoes.construir_barreira(
				int(cmd["peer_id"]), float(tick) * PASSO_SEGUNDOS, materiais
			)
			if bool(resultado_barreira["aceito"]):
				jogador_barreira["materiais"] = materiais - int(resultado_barreira["custo"])
		elif cmd["acao"] == "atacar_forte":
			funcoes.atacar(int(cmd["peer_id"]), float(tick) * PASSO_SEGUNDOS)

func _fase_finish() -> void:
	var gas: Dictionary = ilha_sincronia.aplicar_gas(PASSO_SEGUNDOS)
	for peer_id in gas["dano"].keys():
		if jogadores.has(int(peer_id)):
			var dano: Dictionary = funcoes.receber_dano(
				int(peer_id), int(gas["dano"][peer_id]), float(tick) * PASSO_SEGUNDOS
			)
			jogadores[int(peer_id)]["vida"] -= int(dano["dano_restante"])
			if int(dano["reembolso"]) > 0:
				var atingido: Dictionary = jogadores[int(peer_id)]
				atingido["materiais"] = int(atingido.get("materiais", 0)) + int(dano["reembolso"])
	for peer_id in gas["expulsos"]:
		remover_jogador(int(peer_id))
	if ilha_sincronia.capturada:
		estado_persistido = ilha_sincronia.estado_persistido
	if ilha_topo.capturada:
		estado_persistido = ilha_topo.estado_persistido
	funcoes.avancar(float(tick) * PASSO_SEGUNDOS)
	ponto_coleta.avancar(float(tick) * PASSO_SEGUNDOS)
	comandos_do_tick.clear()

func hash_estado() -> String:
	var ctx := HashingContext.new()
	ctx.start(HashingContext.HASH_SHA256)
	ctx.update(str(jogadores, tick).to_utf8_buffer())
	ctx.update(str(funcoes.canalizacao, funcoes.barreiras, funcoes.recarga_ate).to_utf8_buffer())
	return ctx.finish().hex_encode()
