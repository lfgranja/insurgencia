extends SceneTree

## Suíte determinística da tela de resultado. Nenhum tipo customizado é anotado
## por class_name: em --headless a resolução falha, então o script vem de preload.

const TelaResultadoScript = preload("res://nucleo/apresentacao/tela_resultado.gd")

const CAMINHO_PROJETO: String = "res://project.godot"
const CAMINHO_SCRIPT: String = "res://nucleo/apresentacao/tela_resultado.gd"
const CAMINHO_APRESENTACAO: String = "res://nucleo/apresentacao"
const EVENTO_SERVIDOR: String = "vitoria_ilha_central"
const DECLARADOR_SERVIDOR: String = "servidor"
const TEMPO_VITORIA: float = 12.5

const TOKENS_REDE: Array = [
	"StreamPeer", "PacketPeer", "ENet", "UDP", "TCP", "HTTPRequest",
	"HTTPClient", "WebSocket", "Socket", "Multiplayer",
]
const TOKENS_ARQUIVO: Array = [
	"FileAccess.open", "FileAccess.get_file_as_string", "DirectoryAccess",
	"ConfigFile", "save_file", "store_string", "store_var",
]
const TOKENS_EXTERNO: Array = [
	"Time.get", "OS.get", "randf", "randi", "randomize", "PackedScene", "change_scene",
]

var _falhas: int = 0

func _verificar(condicao: bool, rotulo: String) -> void:
	if condicao:
		print("PASS ", rotulo)
	else:
		_falhas += 1
		printerr("FAIL ", rotulo)

func _initialize() -> void:
	var tela: TelaResultadoScript = TelaResultadoScript.new()
	_verificar(tela is Control, "instancia_e_control")

	var rotulo_titulo: Node = tela.get_node_or_null("Titulo")
	var rotulo_detalhes: Node = tela.get_node_or_null("Detalhes")
	_verificar(
		rotulo_titulo is Label and rotulo_detalhes is Label
			and tela.rotulo_titulo == rotulo_titulo
			and tela.rotulo_detalhes == rotulo_detalhes,
		"rotulos_titulo_e_detalhes_criados_sem_cena"
	)

	var vitoria_valida: Dictionary = {
		"evento": EVENTO_SERVIDOR,
		"declarada_por": DECLARADOR_SERVIDOR,
		"tempo_segundos": TEMPO_VITORIA,
	}
	tela.definir_resultado(vitoria_valida)
	_verificar(tela.obter_titulo() == TelaResultadoScript.TITULO_VITORIA, "evento_valido_gera_vitoria")

	var detalhes_vitoria: String = tela.obter_detalhes()
	_verificar(
		detalhes_vitoria.contains(DECLARADOR_SERVIDOR)
			and detalhes_vitoria.contains(str(TEMPO_VITORIA))
			and not detalhes_vitoria.contains(TelaResultadoScript.TITULO_AGUARDANDO),
		"detalhes_contem_tempo_segundos_e_servidor"
	)

	var evento_errado: Dictionary = {
		"evento": "vitoria_ilha_inferior",
		"declarada_por": DECLARADOR_SERVIDOR,
		"tempo_segundos": TEMPO_VITORIA,
	}
	tela.definir_resultado(evento_errado)
	_verificar(
		tela.obter_titulo() == TelaResultadoScript.TITULO_AGUARDANDO,
		"evento_diverso_gera_aguardando"
	)

	var declaracao_cliente: Dictionary = {
		"evento": EVENTO_SERVIDOR,
		"declarada_por": "cliente",
		"tempo_segundos": TEMPO_VITORIA,
	}
	tela.definir_resultado(declaracao_cliente)
	_verificar(
		tela.obter_titulo() == TelaResultadoScript.TITULO_AGUARDANDO,
		"declarada_por_cliente_gera_aguardando"
	)

	tela.definir_resultado({})
	_verificar(
		tela.obter_titulo() == TelaResultadoScript.TITULO_AGUARDANDO
			and tela.obter_detalhes() == TelaResultadoScript.DETALHES_AGUARDANDO,
		"dicionario_vazio_gera_aguardando"
	)

	var sem_declarador: Dictionary = {
		"evento": EVENTO_SERVIDOR,
		"tempo_segundos": TEMPO_VITORIA,
	}
	tela.definir_resultado(sem_declarador)
	_verificar(
		tela.obter_titulo() == TelaResultadoScript.TITULO_AGUARDANDO,
		"evento_vitoria_sem_declarada_por_gera_aguardando"
	)

	var sem_evento: Dictionary = {
		"declarada_por": DECLARADOR_SERVIDOR,
		"tempo_segundos": TEMPO_VITORIA,
	}
	tela.definir_resultado(sem_evento)
	_verificar(
		tela.obter_titulo() == TelaResultadoScript.TITULO_AGUARDANDO,
		"chave_evento_ausente_gera_aguardando"
	)

	var grafia_errada: Dictionary = {
		"evento": "VITORIA_ILHA_CENTRAL",
		"declarada_por": "Servidor",
		"tempo_segundos": TEMPO_VITORIA,
	}
	tela.definir_resultado(grafia_errada)
	_verificar(
		tela.obter_titulo() == TelaResultadoScript.TITULO_AGUARDANDO,
		"comparacao_literal_case_sensitive"
	)

	var sem_tempo: Dictionary = {
		"evento": EVENTO_SERVIDOR,
		"declarada_por": DECLARADOR_SERVIDOR,
	}
	tela.definir_resultado(sem_tempo)
	var detalhes_sem_tempo: String = tela.obter_detalhes()
	_verificar(
		tela.obter_titulo() == TelaResultadoScript.TITULO_VITORIA
			and detalhes_sem_tempo.contains(DECLARADOR_SERVIDOR)
			and not _tem_digito(detalhes_sem_tempo)
			and not detalhes_sem_tempo.contains("null"),
		"tempo_ausente_nao_inventa_valor"
	)

	tela.definir_resultado(vitoria_valida)
	var titulo_apos_valido: String = tela.obter_titulo()
	tela.definir_resultado({})
	var titulo_apos_invalido: String = tela.obter_titulo()
	tela.definir_resultado(vitoria_valida)
	_verificar(
		titulo_apos_valido == TelaResultadoScript.TITULO_VITORIA
			and titulo_apos_invalido == TelaResultadoScript.TITULO_AGUARDANDO
			and tela.obter_titulo() == TelaResultadoScript.TITULO_VITORIA,
		"segunda_chamada_troca_estado_nos_dois_sentidos"
	)

	var outra_tela: TelaResultadoScript = TelaResultadoScript.new()
	outra_tela.definir_resultado(vitoria_valida)
	_verificar(
		tela.obter_titulo() == outra_tela.obter_titulo()
			and tela.obter_detalhes() == outra_tela.obter_detalhes(),
		"determinismo_entre_instancias"
	)

	var codigo_componente: String = _ler_arquivo(CAMINHO_SCRIPT)
	_verificar(_nao_contem_token(codigo_componente, TOKENS_REDE), "componente_sem_chamadas_de_rede")
	_verificar(_nao_contem_token(codigo_componente, TOKENS_ARQUIVO), "componente_sem_gravacao_de_arquivo")
	_verificar(
		_nao_contem_token(codigo_componente, TOKENS_EXTERNO),
		"componente_sem_relogio_aleatoriedade_ou_cena"
	)

	var projeto: String = _ler_arquivo(CAMINHO_PROJETO)
	_verificar(
		not projeto.contains("run/main_scene") and not projeto.contains(".tscn"),
		"project_godot_sem_main_scene_nem_cena"
	)

	var diretorio: DirAccess = DirAccess.open(CAMINHO_APRESENTACAO)
	var sem_cena_nova: bool = diretorio != null
	if diretorio != null:
		for nome_arquivo in diretorio.get_files():
			if String(nome_arquivo).ends_with(".tscn"):
				sem_cena_nova = false
	_verificar(sem_cena_nova, "nenhum_tscn_novo_em_apresentacao")

	_verificar(
		TelaResultadoScript.EVENTO_VITORIA == EVENTO_SERVIDOR
			and TelaResultadoScript.DECLARADOR_SERVIDOR == DECLARADOR_SERVIDOR,
		"constantes_espelham_evento_do_servidor"
	)

	if _falhas == 0:
		print("OK test_tela_resultado")
		quit(0)
	else:
		printerr("FALHAS ", _falhas)
		quit(1)

func _tem_digito(texto: String) -> bool:
	for indice in range(texto.length()):
		if texto.substr(indice, 1).is_valid_int():
			return true
	return false

func _nao_contem_token(texto: String, tokens: Array) -> bool:
	for token in tokens:
		if texto.contains(String(token)):
			return false
	return true

func _ler_arquivo(caminho: String) -> String:
	var arquivo: FileAccess = FileAccess.open(caminho, FileAccess.READ)
	var conteudo: String = ""
	if arquivo != null:
		conteudo = arquivo.get_as_text()
		arquivo.close()
	return conteudo
