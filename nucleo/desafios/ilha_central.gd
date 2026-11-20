extends RefCounted
class_name IlhaCentral

## Condição de vitória do protótipo (GDD §16.3): três ilhas de rota sob controle
## da mesma equipe e 4 s de canalização conjunta contínua, apenas por Artesão e
## Guerreiro vivos e presentes. A vitória é sempre declarada pelo servidor.
## A regra de produção (seis ilhas + Plano Mestre, 20 s) permanece inalterada.

const ILHAS_NECESSARIAS: int = 3
const CANALIZACAO_SEGUNDOS: float = 4.0
const EVENTO_VITORIA: String = "vitoria_ilha_central"
const FUNCOES_EXIGIDAS: Array = ["artesao", "guerreiro"]

signal vitoria_ilha_central(resultado: Dictionary)

var canalizando: bool = false
var canalizacao_iniciada_em: float = -1.0
var canalizadores: Dictionary = {}
var _funcoes_referencia: Array = []
var progresso: float = 0.0
var vitoria_declarada: bool = false
var estado_persistido: String = ""
var eventos: Array = []

func iniciar_canalizacao(ilhas_controladas: int, funcoes_presentes: Array, agora: float) -> Dictionary:
	if vitoria_declarada:
		return _recusa("VITORIA_JA_DECLARADA")
	if ilhas_controladas < ILHAS_NECESSARIAS:
		interromper()
		return _recusa("ILHAS_INSUFICIENTES", {"ilhas_controladas": ilhas_controladas})
	var mapa_funcoes: Dictionary = _mapear_funcoes(funcoes_presentes)
	var faltantes: Array = []
	for exigida in FUNCOES_EXIGIDAS:
		if not mapa_funcoes.has(exigida):
			faltantes.append(exigida)
	if not faltantes.is_empty():
		interromper()
		return _recusa("FUNCOES_AUSENTES", {"faltantes": faltantes})
	canalizadores = mapa_funcoes.duplicate(true)
	canalizando = true
	canalizacao_iniciada_em = agora
	progresso = 0.0
	_funcoes_referencia = funcoes_presentes
	return {
		"aceito": true, "motivo": "CANALIZACAO_INICIADA", "progresso": 0.0,
		"conclui_em": agora + CANALIZACAO_SEGUNDOS,
	}

func avancar(agora: float) -> Dictionary:
	if vitoria_declarada:
		return {
			"estado": "VITORIA", "vitoria": true,
			"progresso": CANALIZACAO_SEGUNDOS, "evento": EVENTO_VITORIA,
		}
	if not canalizando:
		return {"estado": "INATIVA", "vitoria": false, "progresso": 0.0}
	if not _funcoes_referencia.is_empty() and not verificar_funcoes(_funcoes_referencia):
		interromper()
		return {
			"estado": "INTERROMPIDA", "motivo": "FUNCAO_AUSENTE",
			"vitoria": false, "progresso": 0.0,
		}
	var decorrido: float = maxf(0.0, agora - canalizacao_iniciada_em)
	if decorrido + 0.000001 < CANALIZACAO_SEGUNDOS:
		progresso = decorrido
		return {
			"estado": "CANALIZANDO", "vitoria": false, "progresso": decorrido,
			"conclui_em": canalizacao_iniciada_em + CANALIZACAO_SEGUNDOS,
		}
	vitoria_declarada = true
	canalizando = false
	progresso = CANALIZACAO_SEGUNDOS
	estado_persistido = EVENTO_VITORIA
	var resultado: Dictionary = {
		"evento": EVENTO_VITORIA, "declarada_por": "servidor", "tempo_segundos": agora,
	}
	eventos.append(resultado)
	vitoria_ilha_central.emit(resultado)
	return {
		"estado": "VITORIA", "vitoria": true, "progresso": CANALIZACAO_SEGUNDOS,
		"evento": EVENTO_VITORIA, "resultado": resultado,
	}

func verificar_funcoes(funcoes_presentes: Array) -> bool:
	if not canalizando:
		return false
	var mapa_funcoes: Dictionary = _mapear_funcoes(funcoes_presentes)
	for exigida in FUNCOES_EXIGIDAS:
		if not mapa_funcoes.has(exigida):
			return false
		if int(mapa_funcoes[exigida]) != int(canalizadores.get(exigida, -1)):
			return false
	return true

func interromper() -> Dictionary:
	var estava_ativa: bool = canalizando or canalizacao_iniciada_em >= 0.0
	canalizando = false
	canalizacao_iniciada_em = -1.0
	canalizadores.clear()
	progresso = 0.0
	_funcoes_referencia.clear()
	return {"interrompida": estava_ativa, "progresso": 0.0}

func reiniciar() -> void:
	interromper()
	vitoria_declarada = false
	estado_persistido = ""
	eventos.clear()

func _recusa(motivo: String, detalhes: Dictionary = {}) -> Dictionary:
	var resultado: Dictionary = {
		"aceito": false, "motivo": motivo, "vitoria": vitoria_declarada,
		"progresso": 0.0,
	}
	for chave in detalhes.keys():
		resultado[chave] = detalhes[chave]
	return resultado

func _mapear_funcoes(funcoes_presentes: Array) -> Dictionary:
	var mapa_funcoes: Dictionary = {}
	for indice in range(funcoes_presentes.size()):
		var entrada = funcoes_presentes[indice]
		var nome_funcao: String = ""
		var peer_id: int = indice
		var viva: bool = true
		var presente: bool = true
		if entrada is Dictionary:
			var dados: Dictionary = entrada
			nome_funcao = str(dados.get("funcao", dados.get("papel", "")))
			peer_id = int(dados.get("peer_id", indice))
			viva = bool(dados.get("viva", dados.get("vivo", true))) and int(dados.get("vida", 1)) > 0
			presente = bool(dados.get("presente", dados.get("presente_central", true)))
		elif entrada is String:
			nome_funcao = str(entrada)
		else:
			continue
		var chave: String = _normalizar_funcao(nome_funcao)
		if not FUNCOES_EXIGIDAS.has(chave) or not viva or not presente:
			continue
		mapa_funcoes[chave] = peer_id
	return mapa_funcoes

func _normalizar_funcao(valor: String) -> String:
	var texto: String = valor.strip_edges().to_lower()
	texto = texto.replace("ã", "a").replace("á", "a").replace("à", "a").replace("â", "a")
	texto = texto.replace("é", "e").replace("ê", "e")
	texto = texto.replace("í", "i").replace("ó", "o").replace("ô", "o")
	texto = texto.replace("ú", "u").replace("ç", "c")
	return texto
