extends RefCounted
class_name ValidacaoComandos

const ACOES_PERMITIDAS: Array = ["mover", "sinalizar_sincronia", "sinalizar_alavanca", "coletar_materiais"]
const LIMITE_MOVIMENTO_M: float = 1.0
const ALAVANCAS_TOPO: int = 3

static func validar(mundo, peer_id: int, cmd: Dictionary) -> Dictionary:
	if not mundo.jogadores.has(peer_id):
		return {"valido": false, "codigo": "PEER_NAO_AUTORIZADO"}
	var jogador: Dictionary = mundo.jogadores[peer_id]
	var seq: int = int(cmd.get("seq", 0))
	var request_id: int = int(cmd.get("request_id", 0))
	if seq <= int(jogador["seq"]) or request_id <= int(jogador["request_id"]):
		return {"valido": false, "codigo": "SEQ_OU_REQUEST_REPLAY"}
	var acao: String = str(cmd.get("acao", ""))
	if acao == "creditar_materiais":
		return {"valido": false, "codigo": "MATERIAIS_FORJADOS"}
	if not ACOES_PERMITIDAS.has(acao):
		return {"valido": false, "codigo": "ACAO_DESCONHECIDA"}
	if acao == "mover":
		var destino: Array = cmd.get("destino", [0.0, 0.0])
		var origem: Array = jogador["posicao"]
		var deslocamento: Vector2 = Vector2(destino[0] - origem[0], destino[1] - origem[1])
		if deslocamento.length() > LIMITE_MOVIMENTO_M:
			return {"valido": false, "codigo": "MOVIMENTO_FORA_DO_LIMITE"}
	if acao == "sinalizar_sincronia":
		var alvo: int = int(cmd.get("alvo", 0))
		if alvo < 1 or alvo > 2:
			return {"valido": false, "codigo": "PLACA_INVALIDA"}
	if acao == "sinalizar_alavanca":
		var alavanca: int = int(cmd.get("alvo", 0))
		if alavanca < 1 or alavanca > ALAVANCAS_TOPO:
			return {"valido": false, "codigo": "ALAVANCA_INVALIDA"}
	if acao == "coletar_materiais":
		for campo in ["materiais", "credito", "saldo", "quantidade", "valor", "materiais_creditados"]:
			if cmd.has(campo):
				return {"valido": false, "codigo": "MATERIAIS_FORJADOS"}
	return {"valido": true, "codigo": "OK"}
