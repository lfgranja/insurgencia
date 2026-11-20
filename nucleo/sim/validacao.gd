extends RefCounted
class_name ValidacaoComandos

const ACOES_PERMITIDAS: Array = ["mover", "sinalizar_sincronia"]
const LIMITE_MOVIMENTO_M: float = 1.0

static func validar(mundo, peer_id: int, cmd: Dictionary) -> Dictionary:
	if not mundo.jogadores.has(peer_id):
		return {"valido": false, "codigo": "PEER_NAO_AUTORIZADO"}
	var jogador: Dictionary = mundo.jogadores[peer_id]
	var seq: int = int(cmd.get("seq", 0))
	var request_id: int = int(cmd.get("request_id", 0))
	if seq <= int(jogador["seq"]) or request_id <= int(jogador["request_id"]):
		return {"valido": false, "codigo": "SEQ_OU_REQUEST_REPLAY"}
	var acao: String = str(cmd.get("acao", ""))
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
	return {"valido": true, "codigo": "OK"}
