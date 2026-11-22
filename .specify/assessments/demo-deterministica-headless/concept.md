# Concept — Demo determinística headless

- **Opção 1 (recomendada)**: `crates/sim/examples/demo_deterministica.rs`, usando só `insurgencia_sim`; N ticks fixos, hash por tick. Custo mínimo; entra no gate por `--all-targets`.
- **Opção 2**: novo crate `crates/demo-headless/`. Mais isolamento, mais manutenção; sem ganho verificado.
- **Opção 3**: demo Steam/itch.io com build próprio. Exige plataforma e janela; depende de `sb-17` (DEFER).

**Recomendação**: Opção 1, com saída textual para `diff` e posterior captura (`sb-20`).
