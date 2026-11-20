extends RefCounted
class_name Protocolo

const PROTOCOL_VERSION: int = 1
const MODULO_ID: String = "insurgencia.exemplo"

enum TipoPacote {
	JOIN = 1,
	JOIN_ACEITO = 2,
	JOIN_RECUSADO = 3,
	COMANDO = 4,
	SNAPSHOT = 5,
	EVENTO_DESAFIO = 6,
}

const REJEICAO_VERSAO: String = "VERSAO_PROTOCOLO_DIVERGENTE"
const REJEICAO_HASH: String = "HASH_MODULO_DIVERGENTE"
const REJEICAO_MODULO: String = "MODULO_AUSENTE"
const REJEICAO_MANIFESTO: String = "MANIFEST_INVALID"

const PADROES: Dictionary = {
	TipoPacote.JOIN: {"protocol_version": 0, "module_id": "", "content_hash": ""},
	TipoPacote.COMANDO: {"request_id": 0, "seq": 0, "acao": "", "alvo": 0, "destino": [0.0, 0.0]},
	TipoPacote.SNAPSHOT: {"tick": 0, "epoch": 0, "match_id": 0, "jogadores": []},
	TipoPacote.EVENTO_DESAFIO: {"ilha": "", "evento": "", "tick": 0},
}

class SnapshotMeta extends RefCounted:
	var protocol_version: int
	var epoch: int
	var match_id: int
	var tick: int
	var content_hash: String

	func _init(p_protocol_version: int, p_epoch: int, p_match_id: int, p_tick: int, p_content_hash: String) -> void:
		protocol_version = p_protocol_version
		epoch = p_epoch
		match_id = p_match_id
		tick = p_tick
		content_hash = p_content_hash

	func to_dict() -> Dictionary:
		return {
			"protocol_version": protocol_version,
			"epoch": epoch,
			"match_id": match_id,
			"tick": tick,
			"content_hash": content_hash,
		}

static func normalizar_pacote(tipo: int, dados: Dictionary) -> Dictionary:
	var out: Dictionary = PADROES.get(tipo, {}).duplicate(true)
	for chave in dados.keys():
		if out.has(chave):
			out[chave] = dados[chave]
	out["tipo"] = tipo
	return out
