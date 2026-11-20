extends SceneTree

const ServidorAutoritativo = preload("res://nucleo/servidor.gd")
const Mundo = preload("res://nucleo/mundo.gd")
const ValidacaoComandos = preload("res://nucleo/sim/validacao.gd")
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
	var hash_valido: String = "b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746"
	servidor.handle_join(2, {"protocol_version": 1, "module_id": Protocolo.MODULO_ID, "content_hash": hash_valido})
	var nao_autorizado: Dictionary = ValidacaoComandos.validar(servidor.mundo, 9, {"seq": 1, "acao": "mover"})
	_verificar(nao_autorizado["codigo"] == "PEER_NAO_AUTORIZADO", "peer_nao_autorizado_recusado")
	var falso_mundo := Mundo.new()
	falso_mundo.jogadores[9] = {"posicao": [0.0, 0.0], "vida": 100, "seq": 0, "request_id": 0}
	var replay: Dictionary = ValidacaoComandos.validar(falso_mundo, 9, {"seq": 0, "request_id": 0, "acao": "mover"})
	_verificar(replay["codigo"] == "SEQ_OU_REQUEST_REPLAY", "replay_recusado")
	var desconhecida: Dictionary = ValidacaoComandos.validar(falso_mundo, 9, {"seq": 1, "request_id": 1, "acao": "capturar_ilha"})
	_verificar(desconhecida["codigo"] == "ACAO_DESCONHECIDA", "captura_forjada_recusada")
	var salto: Dictionary = ValidacaoComandos.validar(falso_mundo, 9, {"seq": 1, "request_id": 1, "acao": "mover", "destino": [50.0, 50.0]})
	_verificar(salto["codigo"] == "MOVIMENTO_FORA_DO_LIMITE", "salto_recusado")
	var aceito: Dictionary = servidor.handle_command(2, {
		"seq": 1, "request_id": 1, "acao": "mover", "destino": [0.5, 0.0], "vida": 999,
	})
	_verificar(bool(aceito["aceito"]), "movimento_valido_aceito")
	_verificar(int(servidor.mundo.jogadores[2]["vida"]) == 100, "vida_forjada_ignorada")
	servidor.mundo.avancar_tick()
	_verificar(servidor.mundo.jogadores[2]["posicao"][0] == 0.5, "movimento_aplicado_no_tick")
	var repetido: Dictionary = servidor.handle_command(2, {"seq": 1, "request_id": 1, "acao": "mover", "destino": [0.9, 0.0]})
	_verificar(not bool(repetido["aceito"]) and repetido["codigo"] == "SEQ_OU_REQUEST_REPLAY", "replay_bloqueado_no_servidor")
	var a := Mundo.new()
	var b := Mundo.new()
	for mundo in [a, b]:
		mundo.adicionar_jogador(2)
		mundo.agendar_comando(2, {"seq": 1, "acao": "mover", "destino": [0.5, 0.0]})
		mundo.avancar_tick()
	_verificar(a.hash_estado() == b.hash_estado(), "paridade_hash_de_estado")
	if _falhas == 0:
		print("OK test_validacao")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)
