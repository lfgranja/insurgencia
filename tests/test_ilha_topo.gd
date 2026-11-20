extends SceneTree

const IlhaTopo = preload("res://nucleo/desafios/ilha_topo.gd")
const Mundo = preload("res://nucleo/mundo.gd")
const ValidacaoComandos = preload("res://nucleo/sim/validacao.gd")

var _falhas: int = 0

func _verificar(condicao: bool, rotulo: String) -> void:
	if condicao:
		print("PASS ", rotulo)
	else:
		_falhas += 1
		printerr("FAIL ", rotulo)

func _mundo_com_topo() -> Mundo:
	var mundo: Mundo = Mundo.new()
	mundo.adicionar_jogador(2)
	for alavanca in [1, 2, 3]:
		mundo.agendar_comando(2, {
			"seq": int(alavanca), "request_id": int(alavanca),
			"acao": "sinalizar_alavanca", "alvo": int(alavanca),
		})
		mundo.avancar_tick()
	return mundo

func _initialize() -> void:
	var ilha := IlhaTopo.new()
	var primeira: Dictionary = ilha.sinalizar_alavanca(2, 1, 0.0)
	_verificar(
		bool(primeira["aceito"]) and int(primeira["proxima_alavanca"]) == 2,
		"primeira_alavanca_aceita"
	)
	var segunda: Dictionary = ilha.sinalizar_alavanca(2, 2, 3.0)
	_verificar(
		bool(segunda["aceito"]) and not bool(segunda["capturada"]),
		"segunda_alavanca_aguarda_terceira"
	)
	var terceira: Dictionary = ilha.sinalizar_alavanca(2, 3, 6.0)
	_verificar(
		bool(terceira["capturada"]) and ilha.capturada,
		"ordem_1_2_3_captura"
	)
	_verificar(
		str(terceira["evento"]) == "ilha_topo_concluida"
			and ilha.estado_persistido == "ilha_topo_concluida"
			and ilha.eventos.size() == 1,
		"evento_e_estado_persistido"
	)
	var repetida: Dictionary = ilha.sinalizar_alavanca(3, 1, 7.0)
	_verificar(
		not bool(repetida["aceito"]) and repetida["motivo"] == "DESAFIO_JA_RESOLVIDO",
		"sinal_apos_captura_recusado"
	)
	ilha.reiniciar()
	_verificar(
		not ilha.capturada and ilha.estado_persistido == ""
			and ilha.proxima_alavanca == 1 and ilha.eventos.is_empty(),
		"reiniciar_limpa_estado"
	)

	var erro := IlhaTopo.new()
	erro.sinalizar_alavanca(2, 1, 0.0)
	var fora_de_ordem: Dictionary = erro.sinalizar_alavanca(2, 3, 1.0)
	_verificar(
		not bool(fora_de_ordem["aceito"]) and fora_de_ordem["motivo"] == "ORDEM_INCORRETA"
			and not erro.capturada,
		"alavanca_fora_de_ordem_recusada"
	)
	_verificar(
		absf(float(fora_de_ordem["reinicio_ate"]) - 3.0) < 0.0001,
		"erro_agenda_reinicio_em_2s"
	)
	var bloqueada: Dictionary = erro.sinalizar_alavanca(2, 1, 2.9)
	_verificar(
		bloqueada["motivo"] == "REINICIO_EM_ANDAMENTO" and not erro.capturada,
		"sinal_durante_reinicio_bloqueado"
	)
	var apos_reinicio: Dictionary = erro.sinalizar_alavanca(2, 1, 3.0)
	_verificar(
		bool(apos_reinicio["aceito"]) and erro.proxima_alavanca == 2,
		"sequencia_reiniciada_apos_2s"
	)

	var lenta := IlhaTopo.new()
	lenta.sinalizar_alavanca(2, 1, 0.0)
	var expirada: Dictionary = lenta.sinalizar_alavanca(2, 2, 8.1)
	_verificar(
		expirada["motivo"] == "TEMPO_ALAVANCA_EXPIRADO" and not lenta.capturada,
		"tempo_por_alavanca_8s_exigido"
	)

	var tardia := IlhaTopo.new()
	tardia.sinalizar_alavanca(2, 1, 0.0)
	tardia.sinalizar_alavanca(2, 2, 1.0)
	var fora_da_janela: Dictionary = tardia.sinalizar_alavanca(2, 3, 46.0)
	_verificar(
		fora_da_janela["motivo"] == "JANELA_EXPIRADA"
			and not tardia.capturada and tardia.eventos.is_empty(),
		"janela_45s_excedida_sem_captura"
	)

	var mundo_val: Mundo = Mundo.new()
	mundo_val.adicionar_jogador(2)
	var aceita: Dictionary = ValidacaoComandos.validar(mundo_val, 2, {
		"seq": 1, "request_id": 1, "acao": "sinalizar_alavanca", "alvo": 2,
	})
	_verificar(
		bool(aceita["valido"]) and aceita["codigo"] == "OK",
		"validacao_aceita_sinalizar_alavanca"
	)
	var invalida: Dictionary = ValidacaoComandos.validar(mundo_val, 2, {
		"seq": 2, "request_id": 2, "acao": "sinalizar_alavanca", "alvo": 4,
	})
	_verificar(
		not bool(invalida["valido"]) and invalida["codigo"] == "ALAVANCA_INVALIDA",
		"validacao_recusa_alavanca_fora_de_1_3"
	)

	var mundo: Mundo = _mundo_com_topo()
	_verificar(
		mundo.ilha_topo.capturada and mundo.estado_persistido == "ilha_topo_concluida",
		"integracao_tick_e_estado_persistido"
	)
	var mundo2: Mundo = _mundo_com_topo()
	var ctx_a := HashingContext.new()
	ctx_a.start(HashingContext.HASH_SHA256)
	ctx_a.update(str(mundo.jogadores, mundo.tick, mundo.estado_persistido).to_utf8_buffer())
	var ctx_b := HashingContext.new()
	ctx_b.start(HashingContext.HASH_SHA256)
	ctx_b.update(str(mundo2.jogadores, mundo2.tick, mundo2.estado_persistido).to_utf8_buffer())
	_verificar(
		ctx_a.finish().hex_encode() == ctx_b.finish().hex_encode()
			and mundo.hash_estado() == mundo2.hash_estado(),
		"hash_estado_paridade_com_captura"
	)

	if _falhas == 0:
		print("OK test_ilha_topo")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)
