# Implementation Plan — 027-lowpoly-real

1. Inspecionar assinaturas de `Mesh::new`, `PrimitiveTopology::TriangleList`, `insert_attribute`, `compute_normals` em `bevy_mesh-0.20.1`.
2. Substituir terreno plano por malha irregular com alturas determinísticas (≥20 triângulos).
3. Converter cada ilha em plataforma irregular com taludo (≥30 triângulos), preservando cores e silhueta.
4. Adicionar 2–3 árvores (tronco 5 lados + copa 5–6 lados) e 2 rochas por ilha.
5. Reforçar avatar com corpo e cabeça low-poly.
6. Ajustar iluminação para realçar facetas (normais por triângulo).
7. Preservar HUD, regra da janela e constantes/testes de `lib.rs`.
8. Rodar `cargo fmt --all` e `./verify.sh`; corrigir lints sem relaxar `deny`.
9. Executar minizado para inspeção técnica; inspeção visual fica com a SE.
10. Convergência READ-ONLY, R4, R7.

## Comandos
```bash
./verify.sh
cargo run -p insurgencia-cliente                  # SE
cargo run -p insurgencia-cliente -- --minimizado  # agente
```
