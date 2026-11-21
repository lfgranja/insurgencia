#!/usr/bin/env bash
# Verifica o DCO (Developer Certificate of Origin) no commit mais recente (GDD §15.9).
set -euo pipefail

mensagem="$(git log -1 --format=%B)"

if [[ "$mensagem" == *"Signed-off-by:"* ]]; then
    printf 'DCO OK: o commit mais recente contem Signed-off-by:.\n'
    exit 0
fi

printf 'DCO FALHOU: o commit mais recente nao contem Signed-off-by:.\n' >&2
printf 'Inclua na mensagem do commit: Signed-off-by: Nome <email@example.com>\n' >&2
exit 1
