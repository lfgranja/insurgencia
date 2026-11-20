#!/usr/bin/env bash
# Roteiro de validação solo do MVP — ver docs/ROTEIRO-TESTE-SOLO.md (2026-11-20).
# Não usa git, não altera arquivos do projeto e não instala nada.
set -euo pipefail

GODOT=${GODOT:-/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64}

RAIZ="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$RAIZ"

trap 'printf "INTERROMPIDO\n" >&2; exit 130' INT
trap 'printf "INTERROMPIDO\n" >&2; exit 143' TERM

RESULTADOS=()
STATUS_GERAL=0

registrar() {
	local etapa="$1" estado="$2"
	RESULTADOS+=("$(printf '%-4s %s' "$estado" "$etapa")")
	printf '%s %s\n' "$estado" "$etapa"
}

rodar() {
	local etapa="$1"
	shift
	printf '\n===== %s =====\n' "$etapa"
	if "$@"; then
		registrar "$etapa" PASS
	else
		registrar "$etapa" FAIL
		STATUS_GERAL=1
	fi
}

if ! test -x "$GODOT"; then
	printf 'FAIL GODOT_EXECUTAVEL: %s\n' "$GODOT" >&2
	exit 1
fi
registrar GODOT_EXECUTAVEL PASS

# 5 gates determinísticos (o gate de compatibilidade é exercitado por make check).
rodar gate_contrato        make test-contrato  GODOT="$GODOT"
rodar gate_servidor        make test-servidor  GODOT="$GODOT"
rodar gate_validacao       make test-validacao GODOT="$GODOT"
rodar gate_ilha            make test-ilha      GODOT="$GODOT"
rodar gate_manifesto       make test-manifesto GODOT="$GODOT"

# Suíte completa (5 gates + COMPAT_OK) e E2E de rede com 2 processos UDP em 127.0.0.1:7780.
rodar make_check           make check          GODOT="$GODOT"
rodar test_rede_e2e        make test-rede-e2e  GODOT="$GODOT"

printf '\n===== RESUMO =====\n'
for linha in "${RESULTADOS[@]}"; do
	printf '%s\n' "$linha"
done

if [ "$STATUS_GERAL" -ne 0 ]; then
	printf 'RESULTADO FINAL: FAIL\n' >&2
	exit 1
fi
printf 'RESULTADO FINAL: PASS\n'
exit 0
