extends RefCounted
class_name Funcoes

const CUSTO_BARREIRA_MATERIAIS: int = 20
const CANALIZACAO_BARREIRA_SEGUNDOS: float = 1.5
const PV_BARREIRA: int = 200
const DURACAO_BARREIRA_SEGUNDOS: float = 12.0
const CANALIZACAO_ATAQUE_SEGUNDOS: float = 1.2
const RECARGA_ATAQUE_SEGUNDOS: float = 6.0
const DANO_BASE: int = 100
const DANO_ATAQUE_FORTE: int = 2 * DANO_BASE
const TOLERANCIA: float = 0.000001

var canalizacao: Dictionary = {}
var barreiras: Dictionary = {}
var recarga_ate: Dictionary = {}

func construir_barreira(peer_id: int, agora: float, materiais: int) -> Dictionary:
	if materiais < CUSTO_BARREIRA_MATERIAIS:
		return {
			"aceito": false, "motivo": "MATERIAIS_INSUFICIENTES", "peer_id": peer_id,
			"custo": CUSTO_BARREIRA_MATERIAIS, "materiais": materiais,
		}
	if canalizacao.has(peer_id) or barreiras.has(peer_id):
		return {
			"aceito": false, "motivo": "CANALIZACAO_OU_BARREIRA_ATIVA", "peer_id": peer_id,
		}
	var conclui_em: float = agora + CANALIZACAO_BARREIRA_SEGUNDOS
	canalizacao[peer_id] = {
		"tipo": "barreira", "inicio": agora, "conclui_em": conclui_em,
		"custo": CUSTO_BARREIRA_MATERIAIS,
	}
	return {
		"aceito": true, "motivo": "CANALIZACAO_BARREIRA_INICIADA", "peer_id": peer_id,
		"custo": CUSTO_BARREIRA_MATERIAIS, "pv": PV_BARREIRA, "conclui_em": conclui_em,
	}

func atacar(peer_id: int, agora: float) -> Dictionary:
	var fim_recarga: float = float(recarga_ate.get(peer_id, -1.0))
	if fim_recarga >= 0.0 and agora < fim_recarga - TOLERANCIA:
		return {
			"aceito": false, "motivo": "EM_RECARGA", "peer_id": peer_id,
			"recarga_ate": fim_recarga,
		}
	if canalizacao.has(peer_id):
		return {"aceito": false, "motivo": "CANALIZACAO_EM_ANDAMENTO", "peer_id": peer_id}
	var conclui_em: float = agora + CANALIZACAO_ATAQUE_SEGUNDOS
	canalizacao[peer_id] = {
		"tipo": "ataque", "inicio": agora, "conclui_em": conclui_em,
	}
	return {
		"aceito": true, "motivo": "CANALIZACAO_ATAQUE_INICIADA", "peer_id": peer_id,
		"dano": DANO_ATAQUE_FORTE, "conclui_em": conclui_em,
	}

func receber_dano(peer_id: int, dano: int, agora: float) -> Dictionary:
	var restante: int = maxi(0, dano)
	var absorvido: int = 0
	var barreira_destruida: bool = false
	if barreiras.has(peer_id):
		var barreira: Dictionary = barreiras[peer_id]
		absorvido = mini(int(barreira["pv"]), restante)
		barreira["pv"] = int(barreira["pv"]) - absorvido
		restante -= absorvido
		if int(barreira["pv"]) <= 0:
			barreiras.erase(peer_id)
			barreira_destruida = true
	var interrompida: bool = false
	var tipo_interrompido: String = ""
	var reembolso: int = 0
	if canalizacao.has(peer_id):
		var estado: Dictionary = canalizacao[peer_id]
		tipo_interrompido = str(estado["tipo"])
		canalizacao.erase(peer_id)
		interrompida = true
		if tipo_interrompido == "barreira":
			reembolso = int(estado.get("custo", 0))
	return {
		"peer_id": peer_id, "dano": dano, "absorvido": absorvido,
		"dano_restante": restante, "barreira_destruida": barreira_destruida,
		"interrompida": interrompida, "tipo_interrompido": tipo_interrompido,
		"reembolso": reembolso, "agora": agora,
	}

func avancar(agora: float) -> Dictionary:
	var criadas: Array = []
	var expiradas: Array = []
	var ataques: Array = []
	for peer_id in canalizacao.keys():
		var estado: Dictionary = canalizacao[peer_id]
		if agora + TOLERANCIA < float(estado["conclui_em"]):
			continue
		canalizacao.erase(peer_id)
		if str(estado["tipo"]) == "barreira":
			var expira_em: float = agora + DURACAO_BARREIRA_SEGUNDOS
			barreiras[peer_id] = {
				"pv": PV_BARREIRA, "criada_em": agora, "expira_em": expira_em,
			}
			criadas.append({"peer_id": peer_id, "pv": PV_BARREIRA, "expira_em": expira_em})
		else:
			var fim_recarga: float = agora + RECARGA_ATAQUE_SEGUNDOS
			recarga_ate[peer_id] = fim_recarga
			ataques.append({
				"peer_id": peer_id, "dano": DANO_ATAQUE_FORTE, "recarga_ate": fim_recarga,
			})
	for peer_id in barreiras.keys():
		if agora + TOLERANCIA >= float(barreiras[peer_id]["expira_em"]):
			barreiras.erase(peer_id)
			expiradas.append(peer_id)
	for peer_id in recarga_ate.keys():
		if float(recarga_ate[peer_id]) + TOLERANCIA < agora:
			recarga_ate.erase(peer_id)
	return {
		"estado": "ATUALIZADO", "barreiras_criadas": criadas,
		"barreiras_expiradas": expiradas, "ataques_concluidos": ataques,
		"agora": agora,
	}

func reiniciar() -> void:
	canalizacao.clear()
	barreiras.clear()
	recarga_ate.clear()
