# Idea Research: Hotfixes rápidos

- **Slug**: sb-22-hotfixes-rapidos
- **Created**: 2026-11-21
- **Evidence confidence (overall)**: high para a prática documentada e para o canal local; medium para o número de reviews e a data de lançamento (duas linhas legítimas)
- **Notas consultadas**: `docs/research/slotbound-2026-11-21-steam-gtm.md`; `docs/research/slotbound-2026-11-21-alternativas.md`; `docs/research/slotbound-2026-11-21-fit-local.md`

## Achados

1. **Nove atualizações em três dias, com notas públicas e origem nos relatórios.** Steam Community registra v1.0.0 (4) em 13/11, (5)/(6)/(7) em 14/11, (8) em 15/11, (9) em 16/11, v1.0.1 em 17/11 e v1.0.2 em 19/11; o dev escreve que "a maioria veio de relatórios e sugestões" dos jogadores. [source: steam-gtm tabela Steam Community e achado 8; https://steamcommunity.com/app/4459590/allnews; confidence: high]
2. **A própria Valve endossa corrigir rápido — com método.** "Não há melhor resposta do que corrigir o problema o mais rápido possível (se razoável)"; anunciar no Community Hub; testar em branch antes do default; atualizar quando necessário. Reserva "Update Visibility Rounds" (5 por produto) para grandes updates, não para cada hotfix. [source: steam-gtm tabelas Updating/Update Rounds; https://partner.steamgames.com/doc/store/updates; https://partner.steamgames.com/doc/marketing/visibility/update_rounds; confidence: high]
3. **Triagem de reviews, não resposta integral.** A Valve orienta limitar o tempo de leitura, não responder toda review e não pedir avaliação dentro do jogo; o score só afeta visibilidade abaixo de 40% de positivas. O caso tinha 79% de 1.598 reviews em 21/11 (instantâneo; a demo tinha 82% — populações distintas). [source: steam-gtm C5 e achado 9; https://partner.steamgames.com/doc/store/reviews; https://store.steampowered.com/app/4459590/Slotbound/; confidence: high]
4. **O canal já existe no repo, sem nada a construir.** `verify.sh` roda check/clippy/nextest/fmt (com `--all-targets`); CI (`ci-gates.yml`) executa gates + `verify.sh` em toolchain fixada; verde observado ao vivo autoriza merge autônomo via `gh`. Os pré-requisitos de uma cadência de patches — tag, notas, gate executado — são documentais. [source: fit-local evidências `verify.sh`/CI e matriz sb-22; alternativas ALT-8/14; https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases; confidence: high]
5. **Datas e métricas do caso exigem rigor.** A loja registra lançamento em 12/11; o post "1.0 Is Out Now" e o primeiro hotfix em 13/11; a imprensa adota 13/11. 50k cópias/US$356k em 48 h e 100k em 3 dias são brutas e de posts do dev; nenhum foi reproduzido localmente. [source: steam-gtm C6/C7 e achado 2; https://slotbound.org/release-date; https://x.com/OptimaArchGames/status/2121818614571622489; confidence: high para divergência, `[NV]` para telemetria]

## Contra-evidência e `[NV]`

- Atribuir "9 updates em 3 dias" a uma estratégia deliberada de hotfix é inferência; o dev apenas relata os updates `[NV]`.
- Não há evidência de que responder reviews tenha causado vendas; a própria Valve desaconselha resposta integral `[NV]`.
- O vídeo original não foi recuperado (host youtu.be recusado); a frase "fila de reviews da Steam" vem do resumo da SE `[NV]`.
- Nenhum número de vendas/reviews foi medido localmente; constituição VI — nada vira "verificado por nós" sem comando executado.
- "Hotfix cego" (publicar sem gate) é o risco a mitigar, não prática documentada da Valve.

## Fontes

- `docs/research/slotbound-2026-11-21-steam-gtm.md` (local) — Steam Community, C5/C6/C7, achados 2/8/9, tabelas Valve.
- `docs/research/slotbound-2026-11-21-alternativas.md` (local) — ALT-8/14 (releases, CHANGELOG, atestação).
- `docs/research/slotbound-2026-11-21-fit-local.md` (local) — `verify.sh`, CI, canal CI→`gh`, matriz sb-22.
- URLs primárias nos achados 1–5; nenhuma fonte remota adicional buscada neste artefato.
