extends RefCounted
class_name IlhaTopo

const JANELA_LIMITE_SEGUNDOS: float = 45.0
const TEMPO_ALAVANCA_SEGUNDOS: float = 8.0
const REINICIO_SEGUNDOS: float = 2.0
const ALAVANCAS: int = 3
const EVENTO_CONCLUIDA: String = "ilha_topo_concluida"

var proxima_alavanca: int = 1
var janela_iniciada_em: float = -1.0
var ultima_alavanca_em: float = -1.0
var reinicio_ate: float = -1.0
var capturada: bool = false
var estado_persistido: String = ""
var eventos: Array = []

func sinalizar_alavanca(peer_id: int, alavanca: int, agora: float) -> Dictionary:
	if capturada:
		return {"aceito": false, "motivo": "DESAFIO_JA_RESOLVIDO", "capturada": true}
	if alavanca < 1 or alavanca > ALAVANCAS:
		return {"aceito": false, "motivo": "ALAVANCA_INVALIDA", "capturada": false}
	if reinicio_ate >= 0.0:
		if agora < reinicio_ate:
			return {
				"aceito": false, "motivo": "REINICIO_EM_ANDAMENTO",
				"capturada": false, "reinicio_ate": reinicio_ate,
			}
		_concluir_reinicio()
	if janela_iniciada_em >= 0.0:
		if agora - janela_iniciada_em > JANELA_LIMITE_SEGUNDOS:
			return _registrar_erro(agora, "JANELA_EXPIRADA")
		if ultima_alavanca_em >= 0.0 and agora - ultima_alavanca_em > TEMPO_ALAVANCA_SEGUNDOS:
			return _registrar_erro(agora, "TEMPO_ALAVANCA_EXPIRADO")
	if alavanca != proxima_alavanca:
		return _registrar_erro(agora, "ORDEM_INCORRETA")
	if janela_iniciada_em < 0.0:
		janela_iniciada_em = agora
	ultima_alavanca_em = agora
	proxima_alavanca += 1
	if proxima_alavanca <= ALAVANCAS:
		return {
			"aceito": true, "motivo": "AGUARDANDO_PROXIMA_ALAVANCA",
			"capturada": false, "proxima_alavanca": proxima_alavanca,
		}
	capturada = true
	estado_persistido = EVENTO_CONCLUIDA
	var evento: Dictionary = {
		"evento": EVENTO_CONCLUIDA, "peer_id": peer_id, "tempo_segundos": agora,
	}
	eventos.append(evento)
	return {
		"aceito": true, "motivo": "SEQUENCIA_CONCLUIDA", "capturada": true,
		"evento": EVENTO_CONCLUIDA, "estado_persistido": estado_persistido,
	}

func _concluir_reinicio() -> void:
	reinicio_ate = -1.0
	proxima_alavanca = 1
	ultima_alavanca_em = -1.0

func _registrar_erro(agora: float, motivo: String) -> Dictionary:
	proxima_alavanca = 1
	ultima_alavanca_em = -1.0
	reinicio_ate = agora + REINICIO_SEGUNDOS
	return {
		"aceito": false, "motivo": motivo, "capturada": false,
		"aviso": "REINICIO", "reinicio_ate": reinicio_ate,
	}

func reiniciar() -> void:
	proxima_alavanca = 1
	janela_iniciada_em = -1.0
	ultima_alavanca_em = -1.0
	reinicio_ate = -1.0
	capturada = false
	estado_persistido = ""
	eventos.clear()
