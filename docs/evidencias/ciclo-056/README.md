# Evidência visual — ciclo 056

Esta pasta contém a evidência visual solicitada para o PR #72 (`dev → main`).

- `servidor-validacao.png`: screenshot renderizado em navegador headless (WebKit gerenciado pelo harness) a partir da saída real capturada em `2026-11-24T11:37:21Z`. Nenhuma janela foi aberta na área de trabalho do usuário.
- `terminal-output.txt`: transcrição integral usada para gerar o screenshot.

## Comandos executados

```bash
cargo test -q -p insurgencia-servidor --test cli
printf '1 mover 1 1 0.5 0.0\n' | cargo run -q -p insurgencia-servidor -- --ticks 1
printf '1 voar 1 1\n' | cargo run -q -p insurgencia-servidor -- --ticks 1
```

## Resultado observado

- `cargo test -p insurgencia-servidor --test cli`: **10 passed, 0 failed**; exit 0.
- Comando válido: `0<TAB>7b134fe2…`; exit 0.
- Ação desconhecida: exit 2, stdout vazia, stderr `linha 1: ação desconhecida: voar`.

Âncora: `origin/dev = d2219b6`, sessão `20261124T102734Z-servidor-validacao`. A captura visual do cliente Bevy permanece **[NV]**; esta é evidência da superfície CLI/headless, que é a superfície automatizável do alvo.
