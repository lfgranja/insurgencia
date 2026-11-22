# Research — HUD com acento e fonte livre

**Data**: 2026-11-22T21:16Z

## Evidência primária
- [source: código, 21:18Z] `crates/cliente/src/main.rs:458` — `Text::new("INSURGENCIA - low-poly")` (paliativo ASCII do ciclo 029); `:555` — `format!("INSURGENCIA - low-poly\n...Limites: X +/-{LIMITE_X:.0} / Y +/-{LIMITE_Y:.0}\n...")`; `:459-462` — `TextFont { font_size: FontSize::Px(20.0), ..default() }`, sem fonte externa.
- [source: specs/029-hud-glyphs/*, lido] A fonte padrão do Bevy não cobre `Ê`, `—` (U+2014) nem `·` (U+00B7); o HUD exibia `▯` (tofu) antes do paliativo ASCII. O usuário rejeitou explicitamente o paliativo.
- [source: captura, 21:15Z] `docs/evidencias/hud-confirmacao-visual-2026-11-22.png` (Xvfb + lavapipe, na árvore principal; fora da fronteira deste agente) — HUD ASCII legível, sem tofu, com grafia incorreta.
- [source: sistema, 21:17Z] `/usr/share/fonts/google-noto/NotoSans-Regular.ttf` — 621572 bytes, SHA-256 `478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823`.
- [source: `fc-match charset:00ca|2014|00b7`, 21:17Z] As três consultas resolvem para `Noto Sans` no arquivo acima; o campo `charset` da fonte inclui `00a0-0377` e `2000-2064`, cobrindo U+00CA (`Ê`), U+2014 (`—`) e U+00B7 (`·`).
- [source: licença, 21:17Z] SIL Open Font License 1.1. Texto integral embarcado pelo agente de código em `assets/fonts/OFL.txt` (SHA-256 `4b0e3cf405082dc00cc396e53af77cd6ba6cfe02537e19786e5d113c9158d107`, cabeçalho `Copyright 2022 The Noto Project Authors` + `SIL OPEN FONT LICENSE Version 1.1 - 26 February 2007`). Texto canônico local de referência: `/usr/share/doc/anicraft/OFL-noto-sans-arabic.txt`; fonte primária: https://github.com/notofonts/noto-fonts/blob/main/LICENSE . O hash difere do arquivo `anicraft` porque o cabeçalho de copyright é específico do pacote Noto Sans — o corpo da licença OFL 1.1 é o mesmo.
- [source: código do agente de código, 21:18Z] `main.rs` passa a receber `fontes: Res<AssetServer>`, carrega `fonts/NotoSans-Regular.ttf` via `Handle<Font>` e aplica `font: fonte.into()` no `TextFont`; `assets/fonts/{NotoSans-Regular.ttf,OFL.txt}` e `docs/governance/font-provenance.md` criados. Implementação em curso, fora da fronteira SDD: **[NV]** até verificação pelo orquestrador.

## Interpretação
A causa raiz não é lógica, é tipográfica: a fonte padrão do Bevy carece dos três glifos. A correção soberana e rastreável é embarcar Noto Sans Regular (OFL 1.1, fonte livre) e vinculá-la ao `TextFont` do HUD, restaurando as strings unicode. Isso supera o ciclo 029 sem reintroduzir tofu, mantendo histórico append-only.

## Alternativas rejeitadas
1. Manter ASCII do ciclo 029 — rejeitada pelo próprio SWE (“Agora quero o Ê sim”).
2. Usar fonte do sistema sem embarcar — não rastreável nem reproduzível em outro host; rejeitada.
3. Modificar o atlas/fonte padrão do Bevy — acoplamento a detalhe interno da engine, sem proveniência auditável; rejeitada.
4. Substituir `—` por `-` e `·` por `/` mantendo só `Ê` — correção parcial, contraria o pedido; rejeitada.

## Rastreabilidade
`docs/governance/font-provenance.md` registra origem, tamanho, SHA-256, licença, URL primária e cobertura Unicode; `assets/fonts/OFL.txt` contém o texto integral. Nenhum download de rede e nenhuma dependência `Cargo` nova são necessários: o TTF é copiado de pacote já instalado no sistema.
