extends RefCounted
class_name IAMInterface

func autenticar(credenciais: Dictionary) -> Dictionary:
	push_error("IAMInterface: metodo deve ser sobrescrito")
	return {"sucesso": false, "codigo": "CREDENCIAIS_INVALIDAS", "mensagem": "Credenciais recusadas."}

func identidade(sessao: String) -> Dictionary:
	push_error("IAMInterface: metodo deve ser sobrescrito")
	return {"sucesso": false, "codigo": "SESSAO_INVALIDA", "mensagem": "Sessao invalida."}

func validar_permissao(identidade: Dictionary, acao: String, recurso: String) -> bool:
	push_error("IAMInterface: metodo deve ser sobrescrito")
	return false

func revogar(sessao: String) -> bool:
	push_error("IAMInterface: metodo deve ser sobrescrito")
	return false
