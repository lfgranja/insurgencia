# Implementation Plan — 029-hud-glyphs

1. Editar `crates/cliente/src/main.rs:458`: `Text::new("INSURGENCIA - low-poly")`.
2. Editar `:555`: `format!("INSURGENCIA - low-poly\n...Limites: X +/-{LIMITE_X:.0} / Y +/-{LIMITE_Y:.0}\n...")`.
3. Verificar por `grep` que não há `Ê`, `—`, `·` no HUD.
4. Rodar `./verify.sh` e 5 gates. NUNCA abrir janela normal; validação em `--minimizado` se necessário.
5. converge (append-only) → validate (R4) → release (R7) → PR → CI verde → merge.
