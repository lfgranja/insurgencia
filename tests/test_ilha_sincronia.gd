extends SceneTree

const IlhaSincronia = preload("res://nucleo/desafios/ilha_sincronia.gd")
const Mundo = preload("res://nucleo/mundo.gd")
const ValidacaoComandos = preload("res://nucleo/sim/validacao.gd")

var _falhas: int = 0

func _verificar(condicao: bool, rotulo: String) -> void:
	if condicao:
		print("PASS ", rotulo)
	else:
		_falhas += 1
		printerr("FAIL ", rotulo)

func _initialize() -> void:
	var ilha := IlhaSincronia.new()
	var primeiro: Dictionary = ilha.sinalizar(2, 1, 0.0)
	_verificar(bool(primeiro["aceito"]) and not bool(primeiro["capturada"]), "primeira_placa_registrada")
	var segundo: Dictionary = ilha.sinalizar(3, 2, 4.8)
	_verificar(bool(segundo["capturada"]) and ilha.capturada, "dois_jogadores_em_4_8s_capturam")
	_verificar(ilha.estado_persistido == "ilha_concluida", "estado_persistido_unico")
	var gas_apos_captura: Dictionary = ilha.aplicar_gas(1.0)
	_verificar(gas_apos_captura["dano"].is_empty(), "sem_gas_apos_captura")
	var segundo_sinal: Dictionary = ilha.sinalizar(4, 1, 6.9)
	_verificar(not bool(segundo_sinal["aceito"]) and segundo_sinal["motivo"] == "DESAFIO_JA_RESOLVIDO", "segundo_sinal_descartado")
	ilha.reiniciar()
	_verificar(ilha.estado_persistido == "" and not ilha.capturada, "reinicio_para_reconexao")
	ilha.sinalizar(2, 1, 0.0)
	var expirado: Dictionary = ilha.sinalizar(3, 2, 5.3)
	_verificar(expirado["motivo"] == "JANELA_EXPIRADA" and not ilha.capturada, "entrada_em_5_3s_nao_captura")
	var dano_jogador_2: int = 0
	var vidas: Array = [100]
	var expulsos: Array = []
	for i in range(3):
		var resultado: Dictionary = ilha.aplicar_gas(1.0)
		dano_jogador_2 += int(resultado["dano"].get(2, 0))
		vidas.append(100 - dano_jogador_2)
		expulsos = resultado["expulsos"]
	_verificar(dano_jogador_2 == 24, "gas_8_por_amostra_3_amostras_24_total")
	_verificar(vidas == [100, 92, 84, 76], "vida_100_92_84_76")
	_verificar(expulsos.size() == 2, "expulsao_apos_3_s")
	var ilha_49 := IlhaSincronia.new()
	ilha_49.sinalizar(2, 1, 0.0)
	_verificar(bool(ilha_49.sinalizar(3, 2, 4.9)["capturada"]), "captura_em_4_9s")
	var ilha_unica := IlhaSincronia.new()
	ilha_unica.sinalizar(2, 1, 0.0)
	var mesmo: Dictionary = ilha_unica.sinalizar(2, 2, 1.0)
	_verificar(mesmo["motivo"] == "MESMO_JOGADOR_NAS_DUAS_PLACAS" and not ilha_unica.capturada, "um_jogador_nao_captura")
	var mundo := Mundo.new()
	mundo.adicionar_jogador(2)
	mundo.adicionar_jogador(3)
	var placa_invalida: Dictionary = ValidacaoComandos.validar(mundo, 2, {"seq": 1, "request_id": 1, "acao": "sinalizar_sincronia", "alvo": 9})
	_verificar(placa_invalida["codigo"] == "PLACA_INVALIDA", "placa_fora_de_1_e_2_recusada")
	mundo.agendar_comando(2, {"seq": 1, "request_id": 1, "acao": "sinalizar_sincronia", "alvo": 1})
	mundo.avancar_tick()
	mundo.agendar_comando(3, {"seq": 1, "request_id": 1, "acao": "sinalizar_sincronia", "alvo": 2})
	mundo.avancar_tick()
	_verificar(mundo.ilha_sincronia.capturada and mundo.estado_persistido == "ilha_concluida", "captura_e_estado_no_servidor")
	if _falhas == 0:
		print("OK test_ilha_sincronia")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)
