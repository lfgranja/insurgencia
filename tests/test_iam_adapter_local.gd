extends SceneTree

const IAMInterfaceLocal = preload("res://nucleo/iam/iam_interface.gd")
const IAMAdapterLocalScript = preload("res://nucleo/iam/iam_adapter_local.gd")

# Os nove codigos legiveis e imutaveis do contrato (docs/architecture/iam-interface.md).
const CODIGOS: Array = [
	"AUTENTICADO",
	"CREDENCIAIS_INVALIDAS",
	"CREDENCIAIS_AUSENTES",
	"SESSAO_VALIDA",
	"SESSAO_INVALIDA",
	"PERMISSAO_CONCEDIDA",
	"PERMISSAO_NEGADA",
	"REVOGADA",
	"NAO_REVOGADA",
]
const CHAVES_PROIBIDAS: Array = ["token", "segredo", "chave_privada", "endpoint"]

var _falhas: int = 0

func _verificar(condicao: bool, rotulo: String) -> void:
	if condicao:
		print("PASS ", rotulo)
	else:
		_falhas += 1
		printerr("FAIL ", rotulo)

func _initialize() -> void:
	var adaptador: IAMAdapterLocalScript = IAMAdapterLocalScript.new()
	var porta: IAMInterfaceLocal = IAMInterfaceLocal.new()

	_verificar(porta.has_method("autenticar"), "contrato_autenticar")
	_verificar(porta.has_method("identidade"), "contrato_identidade")
	_verificar(porta.has_method("validar_permissao"), "contrato_validar_permissao")
	_verificar(porta.has_method("revogar"), "contrato_revogar")
	_verificar(CODIGOS.size() == 9, "contrato_nove_codigos")

	var credenciais_vazias: Dictionary = {}
	var resultado_vazio: Dictionary = adaptador.autenticar(credenciais_vazias)
	_verificar(resultado_vazio["codigo"] == "CREDENCIAIS_AUSENTES", "credenciais_ausentes_vazias")
	_verificar(not resultado_vazio.has("sessao"), "credenciais_ausentes_sem_sessao")

	var credenciais_incompletas: Dictionary = {"outro": "ana"}
	var resultado_incompleto: Dictionary = adaptador.autenticar(credenciais_incompletas)
	_verificar(resultado_incompleto["codigo"] == "CREDENCIAIS_AUSENTES", "credenciais_ausentes_incompletas")

	var credenciais_nome_vazio: Dictionary = {"jogador": ""}
	var resultado_nome_vazio: Dictionary = adaptador.autenticar(credenciais_nome_vazio)
	_verificar(resultado_nome_vazio["codigo"] == "CREDENCIAIS_AUSENTES", "credenciais_ausentes_nome_vazio")

	var credenciais_invalidas: Dictionary = {"jogador": "invalido"}
	var resultado_invalido: Dictionary = adaptador.autenticar(credenciais_invalidas)
	_verificar(resultado_invalido["codigo"] == "CREDENCIAIS_INVALIDAS", "credenciais_invalidas_codigo")
	_verificar(not resultado_invalido.has("sessao"), "credenciais_invalidas_sem_sessao")
	_verificar(not resultado_invalido.has("identidade"), "credenciais_invalidas_sem_identidade")

	var credenciais_tipo_errado: Dictionary = {"jogador": 42}
	var resultado_tipo_errado: Dictionary = adaptador.autenticar(credenciais_tipo_errado)
	_verificar(resultado_tipo_errado["codigo"] == "CREDENCIAIS_INVALIDAS", "credenciais_tipo_invalido")

	var credenciais_validas: Dictionary = {"jogador": "ana"}
	var autenticado: Dictionary = adaptador.autenticar(credenciais_validas)
	_verificar(autenticado["codigo"] == "AUTENTICADO", "autenticado_codigo")
	_verificar(bool(autenticado["sucesso"]), "autenticado_sucesso")
	var sessao: String = autenticado["sessao"] as String
	_verificar(not sessao.is_empty() and sessao.begins_with("sessao-local-"), "autenticado_sessao")
	var identidade_autenticada: Dictionary = autenticado["identidade"] as Dictionary
	_verificar(not identidade_autenticada.is_empty(), "autenticado_identidade")
	_verificar(str(identidade_autenticada["jogador"]) == "ana", "autenticado_identidade_jogador")

	var consulta_valida: Dictionary = adaptador.identidade(sessao)
	_verificar(consulta_valida["codigo"] == "SESSAO_VALIDA", "sessao_valida_codigo")
	_verificar(consulta_valida.has("identidade"), "sessao_valida_com_identidade")
	var identidade_valida: Dictionary = consulta_valida["identidade"] as Dictionary
	_verificar(identidade_valida.has("jogador"), "sessao_valida_identidade_jogador")

	var consulta_desconhecida: Dictionary = adaptador.identidade("sessao-desconhecida")
	_verificar(consulta_desconhecida["codigo"] == "SESSAO_INVALIDA", "sessao_desconhecida_invalida")
	_verificar(not consulta_desconhecida.has("identidade"), "sessao_invalida_sem_identidade")

	_verificar(adaptador.validar_permissao(identidade_valida, "mover", "mundo"), "permissao_mover_mundo_concedida")
	_verificar(adaptador.validar_permissao(identidade_valida, "sinalizar_sincronia", "ilha"), "permissao_sinalizar_sincronia_concedida")
	_verificar(not adaptador.validar_permissao(identidade_valida, "capturar_ilha", "ilha"), "permissao_capturar_ilha_negada")
	_verificar(not adaptador.validar_permissao(identidade_valida, "mover", "recurso_desconhecido"), "permissao_recurso_desconhecido_negada")
	_verificar(not adaptador.validar_permissao({}, "mover", "mundo"), "permissao_identidade_vazia_negada")

	_verificar(adaptador.revogar(sessao), "revogar_ativa_true")
	var consulta_revogada: Dictionary = adaptador.identidade(sessao)
	_verificar(consulta_revogada["codigo"] == "SESSAO_INVALIDA", "identidade_apos_revogar_invalida")
	_verificar(not adaptador.revogar("sessao-inexistente"), "revogar_inexistente_false")
	_verificar(not adaptador.revogar(sessao), "segunda_revogacao_false")
	_verificar(CODIGOS.has("REVOGADA") and CODIGOS.has("NAO_REVOGADA"), "codigos_revogacao_presentes")

	var det_a: IAMAdapterLocalScript = IAMAdapterLocalScript.new()
	var det_b: IAMAdapterLocalScript = IAMAdapterLocalScript.new()
	var det_a_auth: Dictionary = det_a.autenticar({"jogador": "ana"})
	var det_b_auth: Dictionary = det_b.autenticar({"jogador": "ana"})
	_verificar(det_a_auth["sessao"] == det_b_auth["sessao"], "determinismo_sessao")
	_verificar(det_a_auth["identidade"] == det_b_auth["identidade"], "determinismo_identidade")

	var retornos: Array = [
		resultado_vazio,
		resultado_incompleto,
		resultado_nome_vazio,
		resultado_invalido,
		resultado_tipo_errado,
		autenticado,
		consulta_valida,
		consulta_desconhecida,
		consulta_revogada,
	]
	var chave_sensivel_encontrada: bool = false
	for retorno in retornos:
		var resultado: Dictionary = retorno as Dictionary
		for chave in CHAVES_PROIBIDAS:
			if resultado.has(str(chave)):
				chave_sensivel_encontrada = true
	_verificar(not chave_sensivel_encontrada, "retornos_sem_token_segredo_chave_endpoint")

	if _falhas == 0:
		print("OK test_iam_adapter_local")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)
