extends RefCounted
class_name IlhaSincronia

const JANELA_SEGUNDOS: float = 5.0
const JOGADORES_NECESSARIOS: int = 2
const DANO_POR_AMOSTRA: int = 8
const INTERVALO_AMOSTRA_SEGUNDOS: float = 1.0
const EXPOSICAO_EXPULSAO_SEGUNDOS: float = 3.0

var janela_iniciada_em: float = -1.0
var placas: Dictionary = {}
var capturada: bool = false
var exposicao: Dictionary = {}
var amostras_aplicadas: Dictionary = {}
var estado_persistido: String = ""

func sinalizar(peer_id: int, placa: int, agora: float) -> Dictionary:
	if capturada:
		return {"aceito": false, "motivo": "DESAFIO_JA_RESOLVIDO", "capturada": true}
	if placa < 1 or placa > 2:
		return {"aceito": false, "motivo": "PLACA_INVALIDA", "capturada": false}
	if placas.has(placa) and int(placas[placa]) != peer_id:
		return {"aceito": false, "motivo": "PLACA_OCUPADA", "capturada": false}
	for outra in placas.values():
		if int(outra) == peer_id:
			return {"aceito": false, "motivo": "MESMO_JOGADOR_NAS_DUAS_PLACAS", "capturada": false}
	if janela_iniciada_em < 0.0:
		janela_iniciada_em = agora
	placas[placa] = peer_id
	if placas.size() == JOGADORES_NECESSARIOS:
		if agora - janela_iniciada_em <= JANELA_SEGUNDOS:
			capturada = true
			estado_persistido = "ilha_concluida"
			exposicao.clear()
			amostras_aplicadas.clear()
			return {"aceito": true, "motivo": "JANELA_CUMPRIDA", "capturada": true}
		_iniciar_gas()
		return {"aceito": false, "motivo": "JANELA_EXPIRADA", "capturada": false}
	return {"aceito": true, "motivo": "AGUARDANDO_SEGUNDO_JOGADOR", "capturada": false}

func _iniciar_gas() -> void:
	exposicao.clear()
	amostras_aplicadas.clear()
	for id in placas.values():
		exposicao[int(id)] = 0.0
		amostras_aplicadas[int(id)] = 0
	placas.clear()
	janela_iniciada_em = -1.0

func aplicar_gas(dt: float) -> Dictionary:
	var dano: Dictionary = {}
	var expulsos: Array = []
	if capturada:
		return {"dano": dano, "expulsos": expulsos}
	for peer_id in exposicao.keys():
		exposicao[peer_id] = float(exposicao[peer_id]) + dt
		var devidas: int = int(floor(float(exposicao[peer_id]) / INTERVALO_AMOSTRA_SEGUNDOS))
		var ja_aplicadas: int = int(amostras_aplicadas.get(peer_id, 0))
		if devidas > ja_aplicadas:
			amostras_aplicadas[peer_id] = devidas
			dano[peer_id] = DANO_POR_AMOSTRA * (devidas - ja_aplicadas)
		if float(exposicao[peer_id]) >= EXPOSICAO_EXPULSAO_SEGUNDOS:
			expulsos.append(int(peer_id))
	for peer_id in expulsos:
		exposicao.erase(peer_id)
		amostras_aplicadas.erase(peer_id)
	return {"dano": dano, "expulsos": expulsos}

func reiniciar() -> void:
	janela_iniciada_em = -1.0
	placas.clear()
	exposicao.clear()
	amostras_aplicadas.clear()
	capturada = false
	estado_persistido = ""
