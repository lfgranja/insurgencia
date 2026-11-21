# Research — Demo determinística headless

Fontes: `docs/research/slotbound-2026-11-21-{fit-local,steam-gtm,alternativas,ai-promo}.md`.

1. `insurgencia-sim` é headless por design (`avancar_tick`, `hash_estado` SHA-256, `BTreeMap`/`VecDeque`); nenhum plugin Bevy é necessário.
2. `verify.sh` roda `cargo check/clippy --all-targets`, então `examples/` entra no gate sem novo job.
3. O gatilho de wishlists de Slotbound foi a **demo**, não o Next Fest (~600 → ~7.000 em 3 dias); Valve documenta que wishlists "mostly aren't a factor" para visibilidade.
4. Captura com Bevy headless + FFmpeg `-bitexact` permite clipes reproduzíveis; saída textual do `sim` é ainda mais barata e auditável.
5. `[NV]`: `verify.sh` completo e CI remoto não observados por timeout de `cargo` concorrente de outros repos (180 s e 600 s).
