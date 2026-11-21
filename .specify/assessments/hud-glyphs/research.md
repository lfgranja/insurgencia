# Research — HUD glyphs ausentes

**Data**: 2026-11-21T04:59Z

## Evidência primária
- Imagem anexada pela SE (1366×768, `local://pasted-image-d438e8e7af1a5414.png`): HUD com `INSURG▯NCIA ▯ low-poly` e `Limites: X +/-580 ▯ Y +/-310`.
- `crates/cliente/src/main.rs:458` — `Text::new("INSURGÊNCIA — low-poly")` (mesmos caracteres problemáticos).
- `crates/cliente/src/main.rs:554-556` — `format!("INSURGÊNCIA — low-poly\n...Limites: X +/-{LIMITE_X:.0} · Y +/-{LIMITE_Y:.0}\n...")`.
- `crates/cliente/src/main.rs:459-462` — `TextFont { font_size: FontSize::Px(20.0), ..default() }`: nenhuma fonte externa configurada → fonte padrão do Bevy.

## Interpretação
A fonte padrão do Bevy não contém `Ê`, `—` (em dash) nem `·` (ponto médio). A correção mais simples e soberana é usar ASCII seguro (`INSURGENCIA - low-poly`, ` / `), sem adicionar assets/fontes externas — coerente com GDD §13.2 (sem assets externos) e com a regra da janela (verificação por código; inspeção visual com a SE).

## Alternativa rejeitada
Incluir arquivo de fonte TTF com os glyphs: exigiria asset externo/licença e auditoria — fora do escopo e da allowlist.
