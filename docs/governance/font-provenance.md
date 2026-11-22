# Proveniência da fonte da HUD

- Nome: Noto Sans (Regular)
- Arquivo embarcado: `assets/fonts/NotoSans-Regular.ttf`
- Origem do binário: `/usr/share/fonts/google-noto/NotoSans-Regular.ttf` (pacote de fontes Google Noto instalado no sistema)
- Tamanho: 621572 bytes
- SHA-256: `478c558ea716033cd60c03438f628dfa75694dcf6b5f6d505a2f05fd2b4f3823`
- Licença: SIL Open Font License 1.1, texto integral em `assets/fonts/OFL.txt`
- URL primária: https://github.com/notofonts/noto-fonts/blob/main/LICENSE

## Cobertura Unicode

`fc-match` com `charset=00ca` (Ê), `charset=2014` (—) e `charset=00b7` (·) resolve para `Noto Sans` em `/usr/share/fonts/google-noto/NotoSans-Regular.ttf`. O campo `charset` da fonte inclui os intervalos `00a0-0377` e `2000-2064`, cobrindo U+00CA, U+2014 e U+00B7.

## Contexto

Esta fonte supera o paliativo ASCII do ciclo 029: a HUD volta a exibir `INSURGÊNCIA — low-poly` e o separador `·` em `Limites: X +/-{LIMITE_X:.0} · Y +/-{LIMITE_Y:.0}` sem substituição por caracteres ASCII.
