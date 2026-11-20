extends SceneTree

const IlhaCentral = preload("res://nucleo/desafios/ilha_central.gd")
const Mundo = preload("res://nucleo/mundo.gd")

var _falhas: int = 0

func _verificar(condicao: bool, rotulo: String) -> void:
	if condicao:
		print("PASS ", rotulo)
	else:
		_falhas += 1
		printerr("FAIL ", rotulo)

func _dupla(artesao_vivo: bool = true, guerreiro_vivo: bool = true, guerreiro_presente: bool = true) -> Array:
	return [
		{"peer_id": 2, "funcao": "Artesao", "viva": artesao_vivo, "presente": true},
		{"peer_id": 3, "funcao": "Guerreiro", "viva": guerreiro_vivo, "presente": guerreiro_presente},
	]

func _mundo_com_ilhas_e_dupla() -> Mundo:
	var mundo: Mundo = Mundo.new()
	mundo.ilha_sincronia.capturada = true
	mundo.ilha_topo.capturada = true
	mundo.ponto_coleta.total_creditado = 5
	mundo.adicionar_jogador(2)
	mundo.definir_funcao(2, "Artesao")
	mundo.definir_presenca_central(2, true)
	mundo.adicionar_jogador(3)
	mundo.definir_funcao(3, "Guerreiro")
	mundo.definir_presenca_central(3, true)
	return mundo

func _initialize() -> void:
	_verificar(
		IlhaCentral.ILHAS_NECESSARIAS == 3
			and absf(IlhaCentral.CANALIZACAO_SEGUNDOS - 4.0) < 0.000001,
		"constantes_3_ilhas_e_4_segundos"
	)

	var recusa_ilhas: IlhaCentral = IlhaCentral.new()
	var sem_ilhas: Dictionary = recusa_ilhas.iniciar_canalizacao(2, _dupla(), 0.0)
	_verificar(
		not bool(sem_ilhas["aceito"]) and sem_ilhas["motivo"] == "ILHAS_INSUFICIENTES"
			and not recusa_ilhas.canalizando,
		"recusa_sem_3_ilhas"
	)

	var recusa_funcoes: IlhaCentral = IlhaCentral.new()
	var so_artesao: Array = [{"peer_id": 2, "funcao": "Artesao", "viva": true, "presente": true}]
	var sem_guerreiro: Dictionary = recusa_funcoes.iniciar_canalizacao(3, so_artesao, 0.0)
	_verificar(
		not bool(sem_guerreiro["aceito"]) and sem_guerreiro["motivo"] == "FUNCOES_AUSENTES"
			and (sem_guerreiro["faltantes"] as Array).has("guerreiro"),
		"recusa_sem_ambas_as_funcoes"
	)

	var funcao_morta: IlhaCentral = IlhaCentral.new()
	var morta: Dictionary = funcao_morta.iniciar_canalizacao(3, _dupla(true, false), 0.0)
	_verificar(not bool(morta["aceito"]) and not funcao_morta.canalizando, "funcao_morta_impede_inicio")

	var funcao_fora: IlhaCentral = IlhaCentral.new()
	var fora: Dictionary = funcao_fora.iniciar_canalizacao(3, _dupla(true, true, false), 0.0)
	_verificar(not bool(fora["aceito"]) and not funcao_fora.canalizando, "funcao_fora_impede_inicio")

	var canalizacao: IlhaCentral = IlhaCentral.new()
	var inicio: Dictionary = canalizacao.iniciar_canalizacao(3, _dupla(), 0.0)
	_verificar(
		bool(inicio["aceito"]) and canalizacao.canalizando
			and absf(float(inicio["conclui_em"]) - 4.0) < 0.000001,
		"inicia_com_3_ilhas_e_dupla_viva_presente"
	)

	var substitutos: IlhaCentral = IlhaCentral.new()
	var outros: Array = [
		{"peer_id": 2, "funcao": "Artesao", "viva": true, "presente": true},
		{"peer_id": 8, "funcao": "Caudillo", "viva": true, "presente": true},
		{"peer_id": 9, "funcao": "Campones", "viva": true, "presente": true},
	]
	var nao_substitui: Dictionary = substitutos.iniciar_canalizacao(3, outros, 0.0)
	_verificar(
		not bool(nao_substitui["aceito"]) and nao_substitui["motivo"] == "FUNCOES_AUSENTES",
		"outros_jogadores_nao_substituem_as_funcoes"
	)

	var quase: Dictionary = canalizacao.avancar(3.99)
	_verificar(
		not bool(quase["vitoria"]) and quase["estado"] == "CANALIZANDO"
			and absf(float(quase["progresso"]) - 3.99) < 0.000001 and not canalizacao.vitoria_declarada,
		"3_99s_de_canalizacao_nao_vence"
	)

	var vitoria: Dictionary = canalizacao.avancar(4.0)
	_verificar(
		bool(vitoria["vitoria"]) and canalizacao.vitoria_declarada
			and absf(float(vitoria["progresso"]) - 4.0) < 0.000001,
		"4s_contínuos_declaram_vitoria"
	)
	var resultado: Dictionary = vitoria["resultado"]
	_verificar(
		resultado["declarada_por"] == "servidor" and vitoria["evento"] == "vitoria_ilha_central"
			and canalizacao.estado_persistido == "vitoria_ilha_central"
			and canalizacao.eventos.size() == 1 and not canalizacao.canalizando,
		"vitoria_declarada_pelo_servidor_com_evento"
	)

	var interrompida: IlhaCentral = IlhaCentral.new()
	interrompida.iniciar_canalizacao(3, _dupla(), 0.0)
	interrompida.avancar(2.5)
	interrompida.interromper()
	var apos_interrupcao: Dictionary = interrompida.avancar(3.0)
	_verificar(
		absf(interrompida.progresso) < 0.000001 and apos_interrupcao["estado"] == "INATIVA"
			and not bool(apos_interrupcao["vitoria"]) and not interrompida.canalizando,
		"interrupcao_zera_progresso"
	)

	interrompida.iniciar_canalizacao(3, _dupla(), 3.0)
	var novo_parcial: Dictionary = interrompida.avancar(6.0)
	var novo_completo: Dictionary = interrompida.avancar(7.0)
	_verificar(
		absf(float(novo_parcial["progresso"]) - 3.0) < 0.000001
			and not bool(novo_parcial["vitoria"]) and bool(novo_completo["vitoria"]),
		"novo_periodo_exige_4s_sem_acumular_interrupcao"
	)

	var identidade: IlhaCentral = IlhaCentral.new()
	identidade.iniciar_canalizacao(3, _dupla(), 0.0)
	var outro_guerreiro: Array = [
		{"peer_id": 2, "funcao": "Artesao", "viva": true, "presente": true},
		{"peer_id": 9, "funcao": "Guerreiro", "viva": true, "presente": true},
	]
	var substituido: bool = identidade.verificar_funcoes(outro_guerreiro)
	identidade.interromper()
	_verificar(not substituido, "outro_guerreiro_nao_substitui_quem_saiu")

	var roster_vivo: Array = _dupla()
	var monitorada: IlhaCentral = IlhaCentral.new()
	monitorada.iniciar_canalizacao(3, roster_vivo, 0.0)
	monitorada.avancar(1.0)
	roster_vivo[1]["viva"] = false
	var apos_morte: Dictionary = monitorada.avancar(5.0)
	_verificar(
		apos_morte["estado"] == "INTERROMPIDA" and absf(monitorada.progresso) < 0.000001
			and not monitorada.vitoria_declarada and not monitorada.canalizando,
		"morte_durante_canalizacao_interrompe"
	)

	canalizacao.reiniciar()
	_verificar(
		not canalizacao.vitoria_declarada and not canalizacao.canalizando
			and canalizacao.estado_persistido.is_empty() and canalizacao.eventos.is_empty()
			and absf(canalizacao.progresso) < 0.000001,
		"reiniciar_limpa_estado_de_vitoria"
	)

	var mundo: Mundo = _mundo_com_ilhas_e_dupla()
	var mundo_par: Mundo = _mundo_com_ilhas_e_dupla()
	for passo in range(130):
		mundo.avancar_tick()
		mundo_par.avancar_tick()
	var ctx_a: HashingContext = HashingContext.new()
	ctx_a.start(HashingContext.HASH_SHA256)
	ctx_a.update(str(mundo.jogadores, mundo.tick, mundo.ilha_central.progresso).to_utf8_buffer())
	var ctx_b: HashingContext = HashingContext.new()
	ctx_b.start(HashingContext.HASH_SHA256)
	ctx_b.update(str(mundo_par.jogadores, mundo_par.tick, mundo_par.ilha_central.progresso).to_utf8_buffer())
	_verificar(
		mundo.ilha_central.vitoria_declarada
			and mundo.estado_persistido == "vitoria_ilha_central"
			and mundo.vitoria_declarada_persistida()
			and mundo.hash_estado() == mundo_par.hash_estado()
			and ctx_a.finish().hex_encode() == ctx_b.finish().hex_encode(),
		"integracao_mundo_declara_vitoria_com_paridade_de_hash"
	)

	var mundo_sem_ilhas: Mundo = Mundo.new()
	mundo_sem_ilhas.ilha_topo.capturada = true
	mundo_sem_ilhas.adicionar_jogador(2)
	mundo_sem_ilhas.definir_funcao(2, "Artesao")
	mundo_sem_ilhas.definir_presenca_central(2, true)
	mundo_sem_ilhas.adicionar_jogador(3)
	mundo_sem_ilhas.definir_funcao(3, "Guerreiro")
	mundo_sem_ilhas.definir_presenca_central(3, true)
	for passo in range(130):
		mundo_sem_ilhas.avancar_tick()
	_verificar(
		not mundo_sem_ilhas.ilha_central.vitoria_declarada
			and not mundo_sem_ilhas.vitoria_declarada_persistida(),
		"integracao_mundo_exige_3_ilhas"
	)

	var mundo_sem_dupla: Mundo = Mundo.new()
	mundo_sem_dupla.ilha_sincronia.capturada = true
	mundo_sem_dupla.ilha_topo.capturada = true
	mundo_sem_dupla.ponto_coleta.total_creditado = 5
	mundo_sem_dupla.adicionar_jogador(2)
	mundo_sem_dupla.definir_funcao(2, "Artesao")
	mundo_sem_dupla.definir_presenca_central(2, true)
	for passo in range(130):
		mundo_sem_dupla.avancar_tick()
	_verificar(
		not mundo_sem_dupla.ilha_central.vitoria_declarada
			and mundo_sem_dupla.estado_persistido != "vitoria_ilha_central",
		"integracao_mundo_exige_as_duas_funcoes"
	)

	if _falhas == 0:
		print("OK test_vitoria_central")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)
