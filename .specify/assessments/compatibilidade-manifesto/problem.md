# Problem — Manifesto de compatibilidade e gate local de versão/hash

**Data:** 2026-11-20T03:15Z

## Problema
Após o ciclo 009 (servidor, validação e suítes Godot mesclados em `dev`, PR #14), o Núcleo já expõe `Protocolo.PROTOCOL_VERSION`, os quatro códigos de recusa (`VERSAO_PROTOCOLO_DIVERGENTE`, `HASH_MODULO_DIVERGENTE`, `MODULO_AUSENTE`, `MANIFEST_INVALID`) e o Módulo de exemplo (`modulos/exemplo/module.json`, `modulos/exemplo/regras.cfg`, hash `b2c11e6d…d746`), mas ainda **não possui manifesto de compatibilidade nem gate local**: ausentes `manifesto.json` (6 campos), `nucleo/compatibilidade/manifesto.gd`, `ferramentas/verificar_compatibilidade.gd`, `Makefile` com o alvo `check-compat` e `tests/test_manifesto.gd`. CE-4 (manifesto e gate local de compatibilidade) segue **sem implementação**; o STATE regista "Ilha Meio, manifesto, `Makefile`, `evidence.csv` pendentes" e 20/39 caminhos canônicos presentes. Sem esse gate, uma divergência de versão de protocolo ou de hash do Módulo passa despercebida até o handshake e o cliente não exibe versão/hash antes de entrar (CE-4.6). Toda saída dos comandos de verificação permanece **[NV]** nesta etapa de escrita.

## Por que é válido
- Impacto na missão: 5 — sem o manifesto versionado e a recusa local, o viabilizador NÚCLEO identificado em `ast-36` (fork sem recompilar exige compatibilidade explícita) não pode ser provado; CE-4 permanece bloqueado e a fila 80/20 volta a parar no mesmo ponto.
- Evidência: 5 — `docs/plans/2026-11-19-ciclo-80-20-omoba.md` Task 5 (linhas 1029–1215) dá os 6 campos literais, as três assinaturas de `ManifestoCompatibilidade`, os códigos de saída, as sete verificações e os comandos `--headless`/`make`; gate determinístico 5/5 PASS em `2026-11-20T03:08:52Z` (`.specify/memory/STATE.md:8`); `ast-36-compatibilidade-build` decidiu ADAPT para manifesto JSON + checagem local.
- Custo de não agir: nenhuma aceitação/recusa de versão, módulo ou hash pode ser verificada localmente, o cliente entra sem exibir versão/hash e as Tasks 6–7 seguem sem a evidência que este gate produce.

## Limites
Fora de escopo: sondagem de lobby/worker e publicação de release (`doc-26` DEFER, citado apenas como contexto), `evidence.csv`, partidas 5v5, Ilha Meio, vitória da Ilha Central, deploy, remote, factory, alteração de baselines, gates ou configuração de CI, e as portarias R4/R7. Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos) é feita aqui; o que depende de execução permanece **[NV]**. Esta nota não escreve código nem artefatos de validação/release.
