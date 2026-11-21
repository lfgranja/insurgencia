# Decision — Demo versionada no itch.io

- **Slug**: sb-16-demo-itch-io
- **Decided**: 2026-11-21
- **Veredito**: ADOPT
- **Artifacts reviewed**: intake.md; research.md; problem.md; concept.md

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Falta ponto de entrada executável e versionado entre a sim headless existente e qualquer público externo. |
| Força da evidência | adequate | Sequência demo→wishlists e custos de distribuição documentados; mês no itch.io e acesso direto ficaram `[NV]`. |
| Valor vs. inação | strong | Sem artefato, a tese não pode ser mostrada nem falsificada e a proveniência anti-clone não é ancorada. |
| Viabilidade / Appetite | strong | Um exemplo em `crates/sim/examples/` já entra em `verify.sh --all-targets`; zero ferramenta nova. |
| Alinhamento estratégico | strong | Realiza IV (sem dependência proprietária), I/VI (gate é autoridade) e a tese FOSS de proveniência aberta. |
| Postura de risco | adequate | Risco de o espelho virar fonte única ou de demo visual reintroduzir janela/SDK; mitigado por GitHub como fonte e gate verde. |

## Racional

ADOPT com guardrail explícito: adota-se o padrão **demo versionada**, não a dependência de plataforma. O caso estudado mostra que a demo — não o Next Fest — foi o gatilho documentado, e que o canal de descoberta precedeu a Steam; porém o número de wishlists é de um outlier (mediana de 806–1.079 em out/2026), então o canal é experimento, não promessa. O repo já possui o mecanismo: sim determinística headless e `verify.sh --all-targets`. A decisão é produzir `crates/sim/examples/demo_deterministica.rs` com seed fixa, publicar tag semver com checksum e notas em CHANGELOG/GitHub Releases e espelhar no itch.io. Nenhum App ID, SDK ou janela entra no caminho crítico; a saída do gate é a única prova de "verde".

## Condição/responsável

- Responsável: Main (exemplo e tag) + SE (conta/espelho e texto do devlog).
- Condição de validade: duas execuções com o mesmo hash antes da tag; `verify.sh` executado com saída colada; GitHub Releases como fonte, itch.io apenas cópia. Se o espelho virar dependência, a decisão converte em AVOID para aquele canal.

## Handoff

- **Problema**: não existe demo executável, versionada e verificável que terceiros possam rodar sem Steam.
- **Abordagem**: exemplo headless determinístico em `crates/sim/examples/demo_deterministica.rs`, seed fixa, `hash_estado()` por tick, tag semver + checksum + changelog e espelho opcional no itch.io.
- **In**: novo exemplo Rust usando apenas `insurgencia_sim`; release versionada; notas curtas; saída de `verify.sh`.
- **Out**: Steam, App ID, SDK, janela gráfica, metas de wishlist, alterações na sim ou nos gates, canal externo como fonte única.
- **Métricas**: exemplo coberto pelo gate (baseline 0); hashes idênticos em duas máquinas; uma release com checksum e notas; downloads via API do GitHub; zero serviço externo no caminho crítico.
- **Perguntas**: qual sessão mínima da sim vira demo; qual seed e formato de saída são congelados; quem opera a conta itch.io e com que política de espelhamento.
