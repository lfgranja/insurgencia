extends Control
class_name TelaResultado

## Tela de resultado do protótipo. A vitória só é exibida quando o evento
## recebido foi emitido pelo servidor — nunca por inferência local.

const EVENTO_VITORIA: String = "vitoria_ilha_central"
const DECLARADOR_SERVIDOR: String = "servidor"
const TITULO_VITORIA: String = "VITORIA"
const TITULO_AGUARDANDO: String = "AGUARDANDO"
const DETALHES_AGUARDANDO: String = "aguardando evento do servidor"

var rotulo_titulo: Label = null
var rotulo_detalhes: Label = null

func _init() -> void:
	_garantir_labels()

func definir_resultado(evento: Dictionary) -> void:
	_garantir_labels()
	if not _evento_valido(evento):
		rotulo_titulo.text = TITULO_AGUARDANDO
		rotulo_detalhes.text = DETALHES_AGUARDANDO
		return
	rotulo_titulo.text = TITULO_VITORIA
	rotulo_detalhes.text = _detalhes_vitoria(evento)

func obter_titulo() -> String:
	_garantir_labels()
	return rotulo_titulo.text

func obter_detalhes() -> String:
	_garantir_labels()
	return rotulo_detalhes.text

func _garantir_labels() -> void:
	if rotulo_titulo == null:
		rotulo_titulo = Label.new()
		rotulo_titulo.name = "Titulo"
		rotulo_titulo.text = TITULO_AGUARDANDO
		add_child(rotulo_titulo)
	if rotulo_detalhes == null:
		rotulo_detalhes = Label.new()
		rotulo_detalhes.name = "Detalhes"
		rotulo_detalhes.text = DETALHES_AGUARDANDO
		add_child(rotulo_detalhes)

func _evento_valido(evento: Dictionary) -> bool:
	var nome_evento: String = str(evento.get("evento", ""))
	var declarador: String = str(evento.get("declarada_por", ""))
	return nome_evento == EVENTO_VITORIA and declarador == DECLARADOR_SERVIDOR

func _detalhes_vitoria(evento: Dictionary) -> String:
	var detalhes: String = "vitoria declarada pelo %s" % DECLARADOR_SERVIDOR
	if _tempo_valido(evento):
		detalhes += " em %s s" % str(evento["tempo_segundos"])
	return detalhes

func _tempo_valido(evento: Dictionary) -> bool:
	if not evento.has("tempo_segundos"):
		return false
	var tempo: Variant = evento["tempo_segundos"]
	if typeof(tempo) != TYPE_INT and typeof(tempo) != TYPE_FLOAT:
		return false
	return float(tempo) >= 0.0
