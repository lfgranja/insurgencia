extends SceneTree

const PontoColeta = preload("res://nucleo/desafios/ponto_coleta.gd")
const Mundo = preload("res://nucleo/mundo.gd")
const ValidacaoComandos = preload("res://nucleo/sim/validacao.gd")

var _falhas: int = 0

func _verificar(condicao: bool, rotulo: String) -> void:
	if condicao:
		print("PASS ", rotulo)
	else:
		_falhas += 1
		printerr("FAIL ", rotulo)

func _mundo_com_coleta() -> Mundo:
	var mundo: Mundo = Mundo.new()
	mundo.adicionar_jogador(2)
	for i in range(91):
		mundo.avancar_tick()
	mundo.agendar_comando(2, {
		"seq": 1, "request_id": 1, "acao": "coletar_materiais",
	})
	mundo.avancar_tick()
	return mundo

func _initialize() -> void:
	var ponto: PontoColeta = PontoColeta.new()
	var sem_estoque: Dictionary = ponto.coletar(2, 1.0)
	var antes_ciclo: Dictionary = ponto.avancar(2.9)
	_verificar(
		not bool(sem_estoque["aceito"]) and sem_estoque["motivo"] == "SEM_ESTOQUE"
			and int(antes_ciclo["materiais_disponiveis"]) == 0,
		"sem_estoque_antes_de_3s"
	)
	var ciclo1: Dictionary = ponto.avancar(3.0)
	_verificar(
		int(ciclo1["materiais_disponiveis"]) == 5 and int(ciclo1["ciclos_produzidos"]) == 1,
		"ciclo_3s_produz_5_materiais"
	)
	var coleta1: Dictionary = ponto.coletar(2, 3.0)
	_verificar(
		bool(coleta1["aceito"]) and int(coleta1["materiais"]) == 5 and ponto.total_creditado == 5,
		"credito_5_materiais_por_ciclo"
	)

	var estoque: PontoColeta = PontoColeta.new()
	estoque.avancar(9.0)
	_verificar(
		estoque.materiais_disponiveis == 15 and estoque.ciclos_produzidos == 3,
		"estoque_de_3_ciclos_15_materiais"
	)
	estoque.avancar(30.0)
	_verificar(estoque.materiais_disponiveis == 15, "estoque_nao_excede_3_ciclos")
	var c1: Dictionary = estoque.coletar(2, 30.0)
	var c2: Dictionary = estoque.coletar(2, 30.0)
	var c3: Dictionary = estoque.coletar(2, 30.0)
	_verificar(
		int(c1["materiais"]) == 5 and int(c2["materiais"]) == 5
			and bool(c3["esgotado"]) and absf(estoque.recarrega_ate - 50.0) < 0.0001,
		"tres_coletas_esgotam_e_agendam_recarga_20s"
	)
	var recusada: Dictionary = estoque.coletar(2, 31.0)
	_verificar(
		not bool(recusada["aceito"]) and recusada["motivo"] == "PONTO_ESGOTADO",
		"coleta_recusada_apos_esgotamento"
	)
	var carregando: Dictionary = estoque.avancar(49.9)
	_verificar(carregando["estado"] == "RECARGANDO", "avancar_durante_recarga")
	var recarregado: Dictionary = estoque.avancar(50.0)
	_verificar(
		recarregado["estado"] == "RECARREGADO" and estoque.recarrega_ate < 0.0,
		"recarga_concluida_em_20s"
	)
	estoque.avancar(53.0)
	var novo: Dictionary = estoque.coletar(2, 53.0)
	_verificar(
		bool(novo["aceito"]) and int(estoque.creditos_por_peer[2]) == 20
			and estoque.total_creditado == 20,
		"novo_ciclo_apos_recarga_e_creditos_acumulados"
	)
	estoque.reiniciar()
	_verificar(
		estoque.materiais_disponiveis == 0 and estoque.ciclos_produzidos == 0
			and estoque.total_creditado == 0 and estoque.recarrega_ate < 0.0
			and estoque.creditos_por_peer.is_empty(),
		"reiniciar_limpa_estado"
	)

	var mundo_val: Mundo = Mundo.new()
	mundo_val.adicionar_jogador(2)
	var aceita: Dictionary = ValidacaoComandos.validar(mundo_val, 2, {
		"seq": 1, "request_id": 1, "acao": "coletar_materiais",
	})
	_verificar(
		bool(aceita["valido"]) and aceita["codigo"] == "OK",
		"validacao_aceita_coletar_materiais"
	)
	var forjada: Dictionary = ValidacaoComandos.validar(mundo_val, 2, {
		"seq": 2, "request_id": 2, "acao": "coletar_materiais", "materiais": 999,
	})
	_verificar(
		not bool(forjada["valido"]) and forjada["codigo"] == "MATERIAIS_FORJADOS",
		"validacao_recusa_credito_forjado"
	)
	var direta: Dictionary = ValidacaoComandos.validar(mundo_val, 2, {
		"seq": 3, "request_id": 3, "acao": "creditar_materiais",
	})
	_verificar(
		not bool(direta["valido"]) and direta["codigo"] == "MATERIAIS_FORJADOS",
		"validacao_recusa_acao_credito_direto"
	)

	var mundo_a: Mundo = _mundo_com_coleta()
	_verificar(
		mundo_a.ponto_coleta.total_creditado == 5
			and int(mundo_a.jogadores[2]["materiais"]) == 5,
		"integracao_tick_produz_e_credita"
	)
	var mundo_b: Mundo = _mundo_com_coleta()
	var ctx_a := HashingContext.new()
	ctx_a.start(HashingContext.HASH_SHA256)
	ctx_a.update(str(mundo_a.jogadores, mundo_a.tick, mundo_a.ponto_coleta.total_creditado).to_utf8_buffer())
	var ctx_b := HashingContext.new()
	ctx_b.start(HashingContext.HASH_SHA256)
	ctx_b.update(str(mundo_b.jogadores, mundo_b.tick, mundo_b.ponto_coleta.total_creditado).to_utf8_buffer())
	_verificar(
		ctx_a.finish().hex_encode() == ctx_b.finish().hex_encode()
			and mundo_a.hash_estado() == mundo_b.hash_estado(),
		"hash_estado_paridade_com_coleta"
	)

	if _falhas == 0:
		print("OK test_ilha_inferior")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)
