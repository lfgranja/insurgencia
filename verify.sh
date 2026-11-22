#!/usr/bin/env bash
# verify.sh — a única fonte de verdade do gate local da Insurgência.
#
# Estágios, na ordem (AGENTS.md):
#   1. cargo check   --workspace --all-targets --locked
#   2. cargo clippy  --workspace --all-targets --all-features -- -D warnings
#   3. cargo nextest run --workspace  (fallback: cargo test --locked --workspace)
#   4. cargo fmt     --all -- --check
#
# Modos:
#   (padrão)   gate completo; exit 1 no primeiro estágio vermelho
#   --fast     todos os estágios menos o de testes (tight loop; NÃO substitui o gate)
#   --install-hook   instala .git/hooks/pre-commit (backup do existente em .bak)
#   --uninstall-hook remove o pre-commit instalado (nenhuma outra modificação)
#   --help / -h     mostra este texto
#
# Codificação: POSIX-ish bash; nenhum side-effect fora da worktree além dos modos explícitos.

set -euo pipefail

cd "$(dirname "$(readlink -f "$0")")"

now() { date +%s; }
elapsed() { echo $(( $(now) - $1 )); }

STAGE_START=$(now)
warn() { printf 'verify.sh: %s\n' "$*" >&2; }

if [[ "${1:-}" = "--help" || "${1:-}" = "-h" ]]; then
    sed -n '1,40p' "$0"
    exit 0
fi

FAST=0
if [[ "${1:-}" = "--fast" ]]; then
    FAST=1
    shift
fi

case "${1:-}" in
    "") ;;
    --install-hook)
        HOOK=.git/hooks/pre-commit
        mkdir -p .git/hooks
        if [[ -f "$HOOK" && ! -f "$HOOK.bak" ]]; then
            cp -p "$HOOK" "$HOOK.bak"
            warn "existing $HOOK backed up to $HOOK.bak"
        fi
        cat > "$HOOK" <<'HOOKBODY'
#!/usr/bin/env bash
# Instalado por verify.sh --install-hook.
exec ./verify.sh --fast
HOOKBODY
        chmod +x "$HOOK"
        printf 'verify.sh: installed .git/hooks/pre-commit → ./verify.sh --fast\n'
        exit 0
        ;;
    --uninstall-hook)
        rm -f .git/hooks/pre-commit
        printf 'verify.sh: removed .git/hooks/pre-commit (backup at .git/hooks/pre-commit.bak preserved)\n'
        exit 0
        ;;
    *)
        warn "unknown argument: $1 (see --help)"
        exit 2
        ;;
esac

printf 'verify.sh: stage 1/4 — cargo check\n'
cargo check --locked --workspace --all-targets
printf 'verify.sh: stage 1 ok (%ss)\n' "$(elapsed "$STAGE_START")"

STAGE_START=$(now)
printf 'verify.sh: stage 2/4 — cargo clippy\n'
cargo clippy --workspace --all-targets --all-features -- -D warnings
printf 'verify.sh: stage 2 ok (%ss)\n' "$(elapsed "$STAGE_START")"

STAGE_START=$(now)
if [[ "$FAST" = "1" ]]; then
    printf 'verify.sh: stage 3/4 — SKIPPED (--fast)\n'
else
    if cargo nextest --version >/dev/null 2>&1; then
        printf 'verify.sh: stage 3/4 — cargo nextest run --workspace\n'
        cargo nextest run --workspace
    else
        warn 'cargo nextest not available; falling back to cargo test --locked --workspace'
        cargo test --locked --workspace
    fi
    printf 'verify.sh: stage 3 ok (%ss)\n' "$(elapsed "$STAGE_START")"
fi

STAGE_START=$(now)
printf 'verify.sh: stage 4/4 — cargo fmt --all -- --check\n'
cargo fmt --all -- --check
printf 'verify.sh: stage 4 ok (%ss)\n' "$(elapsed "$STAGE_START")"

printf 'verify.sh: ALL STAGES OK\n'
