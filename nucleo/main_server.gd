extends SceneTree

const ServidorAutoritativo = preload("res://nucleo/servidor.gd")

func _initialize() -> void:
	var servidor := ServidorAutoritativo.new()
	root.add_child(servidor)
	var porta: int = 7777
	for arg in OS.get_cmdline_user_args():
		if arg.begins_with("--porta="):
			porta = int(arg.substr(len("--porta=")))
	servidor.abrir(porta)
