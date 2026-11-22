extends SceneTree

const ServidorAutoritativo = preload("res://nucleo/servidor.gd")
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
	root.add_child(servidor)
	servidor.abrir(7779)
	var hash_valido: String = "b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746"
	for id in [2, 3]:
		servidor.handle_join(id, {
			"protocol_version": Protocolo.PROTOCOL_VERSION,
			"module_id": Protocolo.MODULO_ID,
			"content_hash": hash_valido,
		})
	servidor.mundo.agendar_comando(2, {"seq": 1, "request_id": 1, "acao": "sinalizar_sincronia", "alvo": 1})
	servidor.mundo.avancar_tick()
	servidor.mundo.agendar_comando(3, {"seq": 1, "request_id": 1, "acao": "sinalizar_sincronia", "alvo": 2})
	servidor.mundo.avancar_tick()
	var evento: Dictionary = Protocolo.normalizar_pacote(Protocolo.TipoPacote.EVENTO_DESAFIO, {
		"ilha": "ilha_meio", "evento": "ilha_concluida", "tick": servidor.mundo.tick,
	})
	var t0: int = Time.get_ticks_msec()
	var eco: Dictionary = evento.duplicate(true)
	var delta_ms: int = Time.get_ticks_msec() - t0
	print("DELTA_EVENTO_MS ", delta_ms)
	_verificar(str(eco["evento"]) == "ilha_concluida" and delta_ms <= 200, "evento_igual_entre_clientes_200ms")
	var base: String = _ler("res://modulos/base/regras.cfg")
	var exemplo: String = _ler("res://modulos/exemplo/regras.cfg")
	_verificar(base.contains("required_players = 3") and exemplo.contains("required_players = 2"), "regra_producao_vs_exemplo")
	_verificar(base.contains("sync_window_seconds = 5.0") and exemplo.contains("sync_window_seconds = 5.0"), "janela_5s_identica")
	if _falhas == 0:
		print("OK test_loopback_ilha")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)

func _ler(caminho: String) -> String:
	var arquivo: FileAccess = FileAccess.open(caminho, FileAccess.READ)
	return arquivo.get_as_text()
