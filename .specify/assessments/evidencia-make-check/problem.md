# Problem — Evidência auditável do gate `make check` e registro do não verificado

**Data:** 2026-11-20T03:19Z

## Problema
Após o ciclo 009 (servidor e validação mesclados em `dev`, PR #14), o Núcleo já expõe as cinco suítes Godot e o gate local de compatibilidade: `test_contrato` 16 `PASS`, `test_servidor` 6, `test_validacao` 9, `test_ilha_sincronia` 14, `test_loopback` 8, `test_loopback_ilha` 3 e `test_manifesto` 7, com `COMPAT_OK` e `SERVIDOR_PRONTO porta=7777` observados nesta sessão. Ainda **não existe, porém, porta única nem evidência auditável**: ausentes o alvo `check` no `Makefile`, `docs/progress/evidence.csv` (9 colunas, cabeçalho literal) e `docs/progress/2026-11-19-ciclo-80-20.md` (registro datado, CE-5.5). CE-5 (evidência comandada, histórico datado, métricas e orçamento de verificação) segue **sem artefatos**; o STATE regista `evidence.csv` e `Makefile` como pendentes.

## Por que é válido
- Impacto na missão: 5 — sem `make check` e sem a evidência ligada aos ids, os cinco gates já executados não podem ser reexecutados por uma única porta nem auditados com origem, comando, resultado, confiança e ressalvas (doc-38, doc-16, ast-01); a fila 80/20 volta a parar na mesma pessoa que precisa reconstruir o que foi verificado.
- Evidência: 5 — `docs/plans/2026-11-19-ciclo-80-20-omoba.md` Task 6 (linhas 1219–1304) dá o `Makefile` literal, o cabeçalho de 9 colunas, as sete linhas de evidência, o registro datado e os quatro comandos de verificação; gate determinístico 5/5 PASS observado nesta sessão, com contagens de `PASS` e `COMPAT_OK` acima diretamente observadas.
- Custo de não agir: nenhuma reexecução repetível, nenhum registro do que ficou sem executar e nenhum histórico datado que ligue CE-1..CE-5 ao que realmente rodou; o trabalho das Tasks 1–5 perde a porta de verificação que o ciclo prometeu.

## Limites
Fora de escopo: portarias R4/R7, sondagem de lobby/worker e publicação de release (`doc-26` DEFER, citado apenas como contexto), partidas 5v5, Ilha Meio, vitória da Ilha Central, elenco Artesão/Guerreiro, deploy, remote, factory, alteração de baselines, gates ou configuração de CI. Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos) é feita aqui: `SERVIDOR_PRONTO porta=7777` e as contagens observadas cobrem as suítes executadas, não a árvore inteira; o restante permanece **[NV]**. Esta nota não escreve `Makefile`, `evidence.csv`, registro datado, código nem artefatos de validação/release.
