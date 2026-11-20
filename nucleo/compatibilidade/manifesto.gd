extends RefCounted
class_name ManifestoCompatibilidade

const Protocolo = preload("res://nucleo/contratos/protocolo.gd")
const CAMPOS: Array = [
	"manifest_version", "protocol_version", "module_id",
	"content_hash", "checked_at", "tool_version",
]

static func carregar(caminho: String) -> Dictionary:
	var arquivo: FileAccess = FileAccess.open(caminho, FileAccess.READ)
	if arquivo == null:
		return {}
	var dados: Variant = JSON.parse_string(arquivo.get_as_text())
	return dados if dados is Dictionary else {}

static func validar_formato(manifesto: Dictionary) -> Dictionary:
	for campo in CAMPOS:
		if not manifesto.has(campo):
			return {"ok": false, "codigo": Protocolo.REJEICAO_MANIFESTO}
	return {"ok": true, "codigo": "OK"}

static func conferir(manifesto: Dictionary, modulo: Dictionary, caminho_regras: String, versao_local: int) -> Dictionary:
	var formato: Dictionary = validar_formato(manifesto)
	if not bool(formato["ok"]):
		return formato
	if int(manifesto["protocol_version"]) != versao_local:
		return {"ok": false, "codigo": Protocolo.REJEICAO_VERSAO}
	if str(manifesto["module_id"]) != str(modulo.get("module_id", "")):
		return {"ok": false, "codigo": Protocolo.REJEICAO_MODULO}
	if str(manifesto["content_hash"]).length() != 64:
		return {"ok": false, "codigo": Protocolo.REJEICAO_MANIFESTO}
	var arquivo: FileAccess = FileAccess.open(caminho_regras, FileAccess.READ)
	if arquivo == null:
		return {"ok": false, "codigo": "REGRAS_AUSENTES"}
	var ctx := HashingContext.new()
	ctx.start(HashingContext.HASH_SHA256)
	ctx.update(arquivo.get_buffer(arquivo.get_length()))
	var hash_calculado: String = ctx.finish().hex_encode()
	if hash_calculado != str(manifesto["content_hash"]):
		return {"ok": false, "codigo": Protocolo.REJEICAO_HASH}
	return {"ok": true, "codigo": "OK"}
