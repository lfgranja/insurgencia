# Validação (R4) — 023-cliente-greybox

**Data:** 2026-11-21T01:01Z · **Branch:** `feature/023-cliente-greybox` · **Veredito:** aprovado

## Comandos executados e saídas

```text
$ ./verify.sh
stage 1/4 cargo check ... ok
stage 2/4 cargo clippy pedantic -D warnings ... ok
stage 3/4 cargo nextest run --workspace
  20 tests run: 20 passed, 0 skipped
  (7 contratos + 8 sim + 5 cliente)
stage 4/4 cargo fmt --all -- --check ... ok
verify.sh: ALL STAGES OK

$ cargo build -p insurgencia-cliente
Finished dev profile

$ ./target/debug/insurgencia-cliente  (8s, janela minizada)
exit=0 (processo permaneceu vivo ate encerramento externo)
INFO bevy_render::renderer: AdapterInfo { name: "AMD Radeon Vega 10 Graphics (RADV RAVEN)", ... backend: Vulkan }
INFO bevy_winit::system: Creating new window insurgencia-cliente
```

## Conformidade
| SC | Evidência | Resultado |
|:--|:--|:--|
| SC-001 | `verify.sh` ALL STAGES OK | PASS |
| SC-002 | binário inicia e permanece rodando; GPU Vulkan detectada | PASS |
| SC-003 | 4 ilhas geométricas (Topo/Meio/Inferior/Central) com cores e formas testadas | PASS |
| SC-004 | `Camera2d` + input WASD/setas + painel de texto | PASS |
| SC-005 | 5 gates do alvo preservados (saída na entrega) | PASS |

## Correções determinísticas
`thiserror` unificado em `=2.0.21` para resolver a cadeia Bevy; lints pedantic corrigidos; `&Res<T>` revertido para `Res<T>` com `#[expect]` isolado (Bevy exige SystemParam por valor); janela passou a iniciar minizada via `Window::set_minimized(true)`.

## Ressalva
A janela **foi criada** e o renderer Vulkan inicializou sem erro, mas a inspeção visual do conteúdo (ilhas/avatar) ainda não foi confirmada por screenshot legível — o ambiente é Wayland/KDE e a captura da ponte Xwayland saiu preta. Não afirmo que o greybox foi visto; **[NV]** a inspeção visual. Nenhum servidor/rede neste ciclo.
