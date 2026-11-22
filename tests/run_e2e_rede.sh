#!/usr/bin/env bash
set -u

GODOT=${GODOT:-godot}
PORTA=7780
LOG=/tmp/servidor-e2e.log
SERVIDOR_PID=""

limpeza() {
	if [ -n "$SERVIDOR_PID" ] && kill -0 "$SERVIDOR_PID" 2>/dev/null; then
		kill "$SERVIDOR_PID" 2>/dev/null || true
		wait "$SERVIDOR_PID" 2>/dev/null || true
	fi
}
trap limpeza EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

: > "$LOG"
timeout 15s "$GODOT" --headless --path . --script res://nucleo/main_server.gd -- --porta=$PORTA > "$LOG" 2>&1 &
SERVIDOR_PID=$!

pronto=0
for _ in $(seq 1 50); do
	if grep -q "SERVIDOR_PRONTO" "$LOG" 2>/dev/null; then
		pronto=1
		break
	fi
	if ! kill -0 "$SERVIDOR_PID" 2>/dev/null; then
		break
	fi
	sleep 0.1
done
if [ "$pronto" -ne 1 ]; then
	echo "FAIL servidor_pronto" >&2
	cat "$LOG" >&2
	exit 1
fi
echo "PASS servidor_pronto"

for peer in 1 2; do
	if ! timeout 10s "$GODOT" --headless --path . --script res://tests/test_cliente_rede.gd -- --porta=$PORTA --peer=$peer; then
		echo "FAIL cliente_$peer" >&2
		exit 1
	fi
	echo "PASS cliente_$peer"
done

echo "OK test_e2e_rede"
exit 0
