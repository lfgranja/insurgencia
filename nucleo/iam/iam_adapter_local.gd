extends "res://nucleo/iam/iam_interface.gd"
class_name IAMAdapterLocal

const PERMISSOES: Dictionary = {
	"mover": "mundo",
	"sinalizar_sincronia": "ilha",
	"coletar_materiais": "ilha_inferior",
	"construir_barreira": "funcoes",
	"atacar_forte": "funcoes",
	"sinalizar_alavanca": "ilha_topo",
}

var _sessoes: Dictionary = {}
var _revogadas: Dictionary = {}
var _proximo_indice: int = 1

func autenticar(credenciais: Dictionary) -> Dictionary:
	var mensagem_recusa: String = "Credenciais recusadas."
	var valor: Variant = credenciais.get("jogador", null)
	if valor == null:
		return {"sucesso": false, "codigo": "CREDENCIAIS_AUSENTES", "mensagem": mensagem_recusa}
	if not (valor is String):
		return {"sucesso": false, "codigo": "CREDENCIAIS_INVALIDAS", "mensagem": mensagem_recusa}
	var nome: String = (valor as String).strip_edges()
	if nome.is_empty():
		return {"sucesso": false, "codigo": "CREDENCIAIS_AUSENTES", "mensagem": mensagem_recusa}
	if nome.to_lower() == "invalido":
		return {"sucesso": false, "codigo": "CREDENCIAIS_INVALIDAS", "mensagem": mensagem_recusa}
	var sessao: String = "sessao-local-%d" % _proximo_indice
	_proximo_indice += 1
	var identidade_local: Dictionary = {"jogador": nome}
	_sessoes[sessao] = identidade_local
	return {
		"sucesso": true,
		"codigo": "AUTENTICADO",
		"mensagem": "Autenticado.",
		"sessao": sessao,
		"identidade": identidade_local.duplicate(true),
	}

func identidade(sessao: String) -> Dictionary:
	if sessao.is_empty() or not _sessoes.has(sessao) or _revogadas.has(sessao):
		return {"sucesso": false, "codigo": "SESSAO_INVALIDA", "mensagem": "Sessao invalida."}
	return {
		"sucesso": true,
		"codigo": "SESSAO_VALIDA",
		"mensagem": "Sessao valida.",
		"sessao": sessao,
		"identidade": (_sessoes[sessao] as Dictionary).duplicate(true),
	}

func validar_permissao(identidade: Dictionary, acao: String, recurso: String) -> bool:
	if identidade.is_empty() or not PERMISSOES.has(acao):
		return false
	return str(PERMISSOES[acao]) == recurso

func revogar(sessao: String) -> bool:
	if sessao.is_empty() or not _sessoes.has(sessao) or _revogadas.has(sessao):
		return false
	_revogadas[sessao] = true
	return true
