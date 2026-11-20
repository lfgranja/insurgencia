# Problem — Servidor headless e validação autoritativa

**Data:** 2026-11-20T03:00Z

## Problema
Após o ciclo 008, o Núcleo tem contrato versionado, Módulo de exemplo e `test_contrato.gd`, mas **nenhum servidor e nenhuma validação de intenções**: ausência de `nucleo/mundo.gd`, `nucleo/servidor.gd`, `nucleo/main_server.gd`, `nucleo/sim/validacao.gd` e das suítes `test_servidor.gd`, `test_loopback.gd`, `test_validacao.gd`. CE-2 (servidor headless e conexão direta) não pode ser avaliado; `SERVIDOR_PRONTO`, snapshots a 20 Hz, recusa de comandos forjados e paridade por hash seguem sem evidência. Toda execução de servidor, handshake ou validação é **[NV]** até a data desta nota.

## Por que é válido
- Impacto na missão: 5 — sem servidor autoritativo e validação, a tese do Núcleo (passo fixo determinístico, intenções nunca aceitas cegas, gate por versão/hash) não pode ser provada e as Tasks 4–6 ficam bloqueadas.
- Evidência: 5 — `docs/plans/2026-11-19-ciclo-80-20-omoba.md` Tasks 2–3 (linhas 304–719) dão caminhos literais, constantes, cinco códigos, contagens de `PASS` e comandos `--headless` com binário absoluto; gate anterior 5/5 PASS em `2026-11-20T02:02:49Z` (`.specify/memory/STATE.md:8`).
- Custo de não agir: a fila 80/20 para novamente no mesmo ponto; nenhuma conexão, snapshot ou prova de paridade pode ser produzida.

## Limites
Não inclui Desafio da Ilha Meio, `manifesto.json`, `Makefile`, `evidence.csv`, deploy, remote, CI remoto, factory ou alteração de baselines de governança. Não antecipa portarias de validação/release nem escreve código nesta nota; o que depende de execução permanece **[NV]**.
