extends SceneTree

const Funcoes = preload("res://nucleo/sim/funcoes.gd")
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
	var f: Funcoes = Funcoes.new()
	var pobre: Dictionary = f.construir_barreira(2, 0.0, Funcoes.CUSTO_BARREIRA_MATERIAIS - 1)
	_verificar(
		Funcoes.CUSTO_BARREIRA_MATERIAIS == 20 and not bool(pobre["aceito"])
			and pobre["motivo"] == "MATERIAIS_INSUFICIENTES",
		"custo_20_e_recusa_sem_materiais"
	)

	var inicio: Dictionary = f.construir_barreira(2, 0.0, 20)
	var antes: Dictionary = f.avancar(1.49)
	var pronto: Dictionary = f.avancar(1.5)
	var barreira: Dictionary = f.barreiras[2]
	_verificar(
		bool(inicio["aceito"]) and absf(float(inicio["conclui_em"]) - 1.5) < 0.000001
			and antes["barreiras_criadas"].is_empty() and pronto["barreiras_criadas"].size() == 1
			and int(barreira["pv"]) == 200,
		"canalizacao_1_5s_e_pv_200"
	)

	var ainda_ativa: Dictionary = f.avancar(13.49)
	var expirada: Dictionary = f.avancar(13.5)
	_verificar(
		not ainda_ativa["barreiras_expiradas"].has(2)
			and expirada["barreiras_expiradas"].has(2) and not f.barreiras.has(2),
		"duracao_12s"
	)

	var g: Funcoes = Funcoes.new()
	g.construir_barreira(3, 0.0, 20)
	g.avancar(1.5)
	var parcial: Dictionary = g.receber_dano(3, 50, 2.0)
	_verificar(
		int(parcial["absorvido"]) == 50 and int(parcial["dano_restante"]) == 0
			and int(g.barreiras[3]["pv"]) == 150,
		"dano_parcial_absorvido_pela_barreira"
	)
	var total: Dictionary = g.receber_dano(3, 150, 2.1)
	_verificar(
		bool(total["barreira_destruida"]) and not g.barreiras.has(3)
			and int(total["dano_restante"]) == 0,
		"200_pv_destruidos"
	)

	var h: Funcoes = Funcoes.new()
	h.construir_barreira(4, 0.0, 20)
	h.avancar(1.5)
	var excesso: Dictionary = h.receber_dano(4, 250, 2.0)
	_verificar(
		bool(excesso["barreira_destruida"]) and int(excesso["absorvido"]) == 200
			and int(excesso["dano_restante"]) == 50,
		"excedente_repassado_ao_jogador"
	)

	var funcoes: Funcoes = Funcoes.new()
	funcoes.construir_barreira(5, 0.0, 20)
	var interrompida: Dictionary = funcoes.receber_dano(5, 1, 0.5)
	var depois: Dictionary = funcoes.avancar(1.5)
	_verificar(
		bool(interrompida["interrompida"]) and interrompida["tipo_interrompido"] == "barreira"
			and int(interrompida["reembolso"]) == 20 and funcoes.canalizacao.is_empty()
			and depois["barreiras_criadas"].is_empty(),
		"dano_interrompe_canalizacao_e_reembolsa"
	)

	var j: Funcoes = Funcoes.new()
	var ini_ataque: Dictionary = j.atacar(6, 0.0)
	var cedo: Dictionary = j.avancar(1.19)
	var golpe: Dictionary = j.avancar(1.2)
	_verificar(
		bool(ini_ataque["aceito"]) and cedo["ataques_concluidos"].is_empty()
			and golpe["ataques_concluidos"].size() == 1
			and int(golpe["ataques_concluidos"][0]["dano"]) == 2 * Funcoes.DANO_BASE,
		"ataque_2x_dano_base_apos_1_2s"
	)
	_verificar(
		absf(float(j.recarga_ate[6]) - (1.2 + Funcoes.RECARGA_ATAQUE_SEGUNDOS)) < 0.000001,
		"recarga_6s_apos_conclusao"
	)
	var na_recarga: Dictionary = j.atacar(6, 7.19)
	var liberado: Dictionary = j.atacar(6, 7.2)
	_verificar(
		not bool(na_recarga["aceito"]) and na_recarga["motivo"] == "EM_RECARGA"
			and bool(liberado["aceito"]),
		"servidor_recusa_ataque_durante_recarga"
	)

	var mundo_val: Mundo = Mundo.new()
	mundo_val.adicionar_jogador(2)
	var v1: Dictionary = ValidacaoComandos.validar(mundo_val, 2, {
		"seq": 1, "request_id": 1, "acao": "construir_barreira",
	})
	var v2: Dictionary = ValidacaoComandos.validar(mundo_val, 2, {
		"seq": 2, "request_id": 2, "acao": "atacar_forte",
	})
	_verificar(v1["codigo"] == "OK" and v2["codigo"] == "OK", "validacao_aceita_funcoes")
	var forjado1: Dictionary = ValidacaoComandos.validar(mundo_val, 2, {
		"seq": 3, "request_id": 3, "acao": "construir_barreira", "materiais": 999,
	})
	var forjado2: Dictionary = ValidacaoComandos.validar(mundo_val, 2, {
		"seq": 4, "request_id": 4, "acao": "atacar_forte", "dano": 9999,
	})
	_verificar(
		forjado1["codigo"] == "RECURSO_FORJADO" and forjado2["codigo"] == "RECURSO_FORJADO",
		"validacao_recusa_recursos_forjados"
	)

	var mundo: Mundo = Mundo.new()
	mundo.adicionar_jogador(2)
	mundo.jogadores[2]["materiais"] = 20
	mundo.agendar_comando(2, {"seq": 1, "request_id": 1, "acao": "construir_barreira"})
	mundo.avancar_tick()
	_verificar(
		int(mundo.jogadores[2]["materiais"]) == 0 and mundo.funcoes.canalizacao.has(2),
		"mundo_consome_20_materiais_no_tick"
	)
	for i in range(45):
		mundo.avancar_tick()
	_verificar(
		mundo.funcoes.barreiras.has(2) and int(mundo.funcoes.barreiras[2]["pv"]) == 200,
		"mundo_cria_barreira_apos_1_5s"
	)

	var mundo_sem: Mundo = Mundo.new()
	mundo_sem.adicionar_jogador(3)
	mundo_sem.agendar_comando(3, {"seq": 1, "request_id": 1, "acao": "construir_barreira"})
	mundo_sem.avancar_tick()
	_verificar(
		mundo_sem.funcoes.canalizacao.is_empty()
			and int(mundo_sem.jogadores[3].get("materiais", 0)) == 0,
		"mundo_recusa_sem_materiais"
	)

	var mundo_ataque: Mundo = Mundo.new()
	mundo_ataque.adicionar_jogador(4)
	mundo_ataque.agendar_comando(4, {"seq": 1, "request_id": 1, "acao": "atacar_forte"})
	mundo_ataque.avancar_tick()
	for i in range(37):
		mundo_ataque.avancar_tick()
	_verificar(mundo_ataque.funcoes.recarga_ate.has(4), "mundo_aplica_recarga_do_ataque")

	var p_a: Mundo = Mundo.new()
	var p_b: Mundo = Mundo.new()
	for mundo_par in [p_a, p_b]:
		mundo_par.adicionar_jogador(2)
		mundo_par.jogadores[2]["materiais"] = 20
		mundo_par.agendar_comando(2, {"seq": 1, "request_id": 1, "acao": "construir_barreira"})
		for i in range(50):
			mundo_par.avancar_tick()
	var ctx_a: HashingContext = HashingContext.new()
	ctx_a.start(HashingContext.HASH_SHA256)
	ctx_a.update(str(p_a.funcoes.canalizacao, p_a.funcoes.barreiras, p_a.funcoes.recarga_ate).to_utf8_buffer())
	var ctx_b: HashingContext = HashingContext.new()
	ctx_b.start(HashingContext.HASH_SHA256)
	ctx_b.update(str(p_b.funcoes.canalizacao, p_b.funcoes.barreiras, p_b.funcoes.recarga_ate).to_utf8_buffer())
	_verificar(
		p_a.hash_estado() == p_b.hash_estado()
			and ctx_a.finish().hex_encode() == ctx_b.finish().hex_encode(),
		"paridade_hash_com_funcoes"
	)

	j.reiniciar()
	_verificar(
		j.canalizacao.is_empty() and j.barreiras.is_empty() and j.recarga_ate.is_empty(),
		"reiniciar_limpa_estado"
	)

	if _falhas == 0:
		print("OK test_funcoes")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)
