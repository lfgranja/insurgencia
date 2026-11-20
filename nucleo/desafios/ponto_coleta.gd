extends RefCounted
class_name PontoColeta

const CICLO_SEGUNDOS: float = 3.0
const MATERIAIS_POR_CICLO: int = 5
const CICLOS_ESTOQUE: int = 3
const RECARGA_SEGUNDOS: float = 20.0

var ciclo_iniciado_em: float = 0.0
var ciclos_produzidos: int = 0
var materiais_disponiveis: int = 0
var recarrega_ate: float = -1.0
var total_creditado: int = 0
var creditos_por_peer: Dictionary = {}

func coletar(peer_id: int, agora: float) -> Dictionary:
	if recarrega_ate >= 0.0:
		return {
			"aceito": false, "motivo": "PONTO_ESGOTADO", "materiais": 0,
			"peer_id": peer_id, "recarrega_ate": recarrega_ate,
		}
	if materiais_disponiveis < MATERIAIS_POR_CICLO:
		return {
			"aceito": false, "motivo": "SEM_ESTOQUE", "materiais": 0,
			"peer_id": peer_id, "materiais_disponiveis": materiais_disponiveis,
		}
	materiais_disponiveis -= MATERIAIS_POR_CICLO
	creditos_por_peer[peer_id] = int(creditos_por_peer.get(peer_id, 0)) + MATERIAIS_POR_CICLO
	total_creditado += MATERIAIS_POR_CICLO
	var esgotado: bool = materiais_disponiveis == 0 and ciclos_produzidos >= CICLOS_ESTOQUE
	if esgotado:
		recarrega_ate = agora + RECARGA_SEGUNDOS
	return {
		"aceito": true, "motivo": "MATERIAIS_CREDITADOS", "peer_id": peer_id,
		"materiais": MATERIAIS_POR_CICLO, "total_creditado": total_creditado,
		"esgotado": esgotado,
	}

func avancar(agora: float) -> Dictionary:
	if recarrega_ate >= 0.0:
		if agora < recarrega_ate:
			return {
				"estado": "RECARGANDO", "recarrega_ate": recarrega_ate,
				"materiais_disponiveis": 0,
			}
		recarrega_ate = -1.0
		ciclos_produzidos = 0
		materiais_disponiveis = 0
		ciclo_iniciado_em = agora
		return {
			"estado": "RECARREGADO", "recarrega_em": agora + RECARGA_SEGUNDOS,
			"materiais_disponiveis": 0,
		}
	if materiais_disponiveis == 0 and ciclos_produzidos >= CICLOS_ESTOQUE and recarrega_ate < 0.0:
		recarrega_ate = agora + RECARGA_SEGUNDOS
		return {
			"estado": "ESGOTADO", "recarrega_ate": recarrega_ate,
			"materiais_disponiveis": 0,
		}
	var ciclos_novos: int = 0
	while agora - ciclo_iniciado_em >= CICLO_SEGUNDOS and ciclos_produzidos < CICLOS_ESTOQUE:
		ciclo_iniciado_em += CICLO_SEGUNDOS
		ciclos_produzidos += 1
		materiais_disponiveis += MATERIAIS_POR_CICLO
		ciclos_novos += 1
	return {
		"estado": "CICLO", "ciclos_novos": ciclos_novos,
		"ciclos_produzidos": ciclos_produzidos,
		"materiais_disponiveis": materiais_disponiveis,
	}

func reiniciar() -> void:
	ciclo_iniciado_em = 0.0
	ciclos_produzidos = 0
	materiais_disponiveis = 0
	recarrega_ate = -1.0
	total_creditado = 0
	creditos_por_peer.clear()
