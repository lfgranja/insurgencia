extends RefCounted
class_name CarregadorModulo

const OBRIGATORIOS: Array = [
	"contract_version", "module_id", "display_name", "protocol_version",
	"content_hash", "content_bytes", "content_files", "map", "characters", "rules",
]
const PERMITIDOS: Array = [
	"contract_version", "module_id", "display_name", "protocol_version",
	"content_hash", "content_bytes", "content_files", "map", "characters", "rules", "assets",
]
const TIPOS: Dictionary = {
	"contract_version": TYPE_STRING,
	"module_id": TYPE_STRING,
	"display_name": TYPE_STRING,
	"protocol_version": TYPE_INT,
	"content_hash": TYPE_STRING,
	"content_bytes": TYPE_INT,
	"content_files": TYPE_ARRAY,
	"map": TYPE_STRING,
	"characters": TYPE_STRING,
	"rules": TYPE_STRING,
	"assets": TYPE_ARRAY,
}

static func validar(dados: Dictionary) -> Dictionary:
	for campo in OBRIGATORIOS:
		if not dados.has(campo):
			return {"valido": false, "codigo": "CAMPO_OBRIGATORIO_AUSENTE", "campo": campo}
	for chave in dados.keys():
		if not PERMITIDOS.has(chave):
			return {"valido": false, "codigo": "CAMPO_DESCONHECIDO", "campo": chave}
	for chave in dados.keys():
		if not _tipo_valido(dados[chave], int(TIPOS.get(chave, TYPE_NIL))):
			return {"valido": false, "codigo": "TIPO_INVALIDO", "campo": chave}
	if str(dados["contract_version"]) != "1.0.0":
		return {"valido": false, "codigo": "CONTRACT_VERSION_INVALIDO", "campo": "contract_version"}
	if str(dados["content_hash"]).length() != 64:
		return {"valido": false, "codigo": "HASH_INVALIDO", "campo": "content_hash"}
	var bytes: int = int(dados["content_bytes"])
	if bytes <= 0 or bytes > 25000000:
		return {"valido": false, "codigo": "BYTES_INVALIDOS", "campo": "content_bytes"}
	return {"valido": true, "codigo": "OK", "campo": ""}

static func _tipo_valido(valor: Variant, esperado: int) -> bool:
	if esperado == TYPE_INT:
		return typeof(valor) == TYPE_INT or (
			typeof(valor) == TYPE_FLOAT and is_equal_approx(float(valor), floor(float(valor)))
		)
	return typeof(valor) == esperado
