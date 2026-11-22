extends SceneTree

# Benchmark determinístico de simulação headless: mede o custo por tick de uma
# carga representativa de entidades, sem renderização, rede ou serviços externos.
const TICKS: int = 600
const ENTITIES: int = 64

func _initialize() -> void:
	var started_us: int = Time.get_ticks_usec()
	var accumulator: float = 0.0
	for tick in range(TICKS):
		for entity in range(ENTITIES):
			accumulator += sin(float(tick + entity) * 0.01)
	var elapsed_us: int = Time.get_ticks_usec() - started_us
	var per_tick_us: float = float(elapsed_us) / float(TICKS)
	print("BENCHMARK ticks=%d entities=%d elapsed_us=%d per_tick_us=%.2f accumulator=%.6f" % [
		TICKS, ENTITIES, elapsed_us, per_tick_us, accumulator
	])
	quit()
