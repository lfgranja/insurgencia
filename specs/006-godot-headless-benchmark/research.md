# Idea Research: Godot headless benchmark

- **Slug**: godot-headless-benchmark
- **Created**: 2026-11-20
- **Evidence confidence (overall)**: high — release, checksum, identidade do binário e números da medição foram obtidos por comandos com carimbos de data/hora e são citados abaixo. Nenhuma afirmação central repousa em `ASSUMPTION`; os pontos ainda não executados nesta etapa são identificados como tal, não usados como evidência a favor.

## Users & Demand

- O mantenedor `lfgranja` é quem autorizou a instalação verificada e a medição, e precisa de uma base de comparação para decisões futuras sobre desempenho da simulação. A demanda é explícita no pedido desta sessão, não inferida. [source: pedido do mantenedor, 2026-11-20] (confidence: high)
- Auditores e revisores precisam provar que a engine veio de release oficial e que o artefato não divergiu entre publicação e uso; o par esperado/calculado do checksum é a evidência que fecha essa cadeia. [source: evidência do orquestrador 2026-11-20T00:43:52Z] (confidence: high)
- Quem for implementar a simulação no futuro usa a mediana por tick como linha de base: sem ela, qualquer regressão de custo de simulação só seria percebida por impressão. [source: pedido do mantenedor; `tests/benchmark/headless_tick.gd:3-4`] (confidence: high)

## Prior Art

- O estado do alvo registra ausência de engine instalada, de código-fonte de jogo, de testes, de build, de CI e de deploy; todo estado avançado é `[NV]` até verificação ao vivo. [source: `AGENTS.md:3-5`] (confidence: high)
- Já existe carga de benchmark versionada: `tests/benchmark/headless_tick.gd` executa 600 ticks × 64 entidades, acumula `sin((tick+entity)*0.01)` e imprime uma linha `BENCHMARK` com `ticks`, `entities`, `elapsed_us`, `per_tick_us` e `accumulator`; `tests/benchmark/project.godot` declara `config/features=PackedStringArray("4.7")` e nenhuma cena principal. [source: leitura local dos dois arquivos, 2026-11-20] (confidence: high)
- A spec 005 estabeleceu o padrão do hub para evidência auditável: comando executado, saída colada e data/hora, com rótulo `[NV]` até confirmação, além do conjunto de dez artefatos de planejamento. [source: `specs/005-ci-protecao-dev/research.md:27-34`; `docs/governance/sdd-phase-contracts.md:24-27`] (confidence: high)
- Não há registro anterior de instalação de Godot no alvo nem de medição de tick neste repositório; a carga em `tests/benchmark/` é o único precedente local de benchmarking. [source: `AGENTS.md:3-5`; listagem de `tests/benchmark/`, 2026-11-20] (confidence: high)

## Market & Context

- Obter a engine de uma release upstream nomeada e conferir o checksum publicado é a alternativa a usar binário arbitrário, que não permitiria provar proveniência nem integridade. A consulta `gh api repos/godotengine/godot/releases/tags/4.7.2-stable --jq '.assets[].name'` listou `Godot_v4.7.2-stable_linux.x86_64.zip` e `SHA512-SUMS.txt`, baixados com `curl -fsSL`; é o mecanismo usado para fixar a identidade da release. [source: E1, 2026-11-20T00:43:52Z] (confidence: high)
- A alternativa atual é não ter engine nem medição, o que mantém o desempenho da simulação como incógnita e impede comparar mudanças futuras; o custo da inação é a ausência de linha de base e de prova de integridade do binário. [source: `AGENTS.md:3-5`; pedido do mantenedor] (confidence: high)
- O limiar de 16,7 ms por tick corresponde ao orçamento de um quadro a 60 FPS; é uma referência de comparação fornecida no pedido, não um requisito de produto nem um SLA de partida. [source: pedido do mantenedor, 2026-11-20] (confidence: high)

## Data & Constraints

| Fato observado | Valor registrado | Carimbo / origem |
|---|---|---|
| Release upstream | `4.7.2-stable`; `gh api repos/godotengine/godot/releases/tags/4.7.2-stable --jq '.assets[].name'` → `Godot_v4.7.2-stable_linux.x86_64.zip` e `SHA512-SUMS.txt`; download com `curl -fsSL` | E1, 2026-11-20T00:43:52Z |
| Checksum esperado | `9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65` (exibição: `9aa00f7a...c65`), linha obtida com `grep 'Godot_v4.7.2-stable_linux.x86_64.zip' SHA512-SUMS.txt` | E2, 2026-11-20T00:43:52Z |
| Checksum calculado | `sha512sum Godot_v4.7.2-stable_linux.x86_64.zip` → mesmo valor 128-hex com nome local; esperado == calculado → `CHECKSUM_OK` | E2, 2026-11-20T00:43:52Z |
| Identidade do binário | `Godot_v4.7.2-stable_linux.x86_64 --headless --version` → `4.7.2.stable.official.ed1daf0bf` | E3, 2026-11-20T00:44:11Z |
| Carga do benchmark | 600 ticks × 64 entidades, `sin((tick+entity)*0.01)`, sem renderização, rede ou serviços externos | `tests/benchmark/headless_tick.gd:5-19` |
| Medição | `$G --headless --path tests/benchmark --script headless_tick.gd`: 9,43 / 15,50 / 17,48 (3 iniciais) e 25,49 / 42,95 / 22,71 / 27,64 / 39,45 / 39,99 / 37,48 (7) µs/tick; mediana 37,48; `accumulator=-297.626094` | E4, 2026-11-20T00:44:35Z e 2026-11-20T00:44:48Z |
| Referência de comparação | 16,7 ms/tick | pedido do mantenedor, 2026-11-20 |

- A mediana registrada, 37,48 µs/tick, equivale a 0,03748 ms/tick — abaixo da referência de 16,7 ms/tick. A conversão é aritmética sobre o valor medido e não altera a natureza da carga. [source: evidência 2026-11-20T00:44:48Z; pedido do mantenedor] (confidence: high)
- A constituição do alvo exige determinismo (I), fluxo SDD ordenado (III), soberania do jogador sem dependência proprietária no caminho crítico (IV) e verificação honesta com `[NV]` até confirmação ao vivo (VI). [source: `.specify/memory/constitution.md:7-25`; `AGENTS.md:37-46`] (confidence: high)
- Escopo de escrita desta etapa: somente os dez artefatos em `specs/006-godot-headless-benchmark/`; nenhum gate é executado, nenhum stage ou commit é feito, e nenhuma portaria posterior a tasks é escrita. [source: atribuição do mantenedor, 2026-11-20] (confidence: high)

### Evidência bruta (comando + saída + carimbo)

Os quatro blocos abaixo substituem as abreviações de origem: comando executado, saída observada e carimbo aparecem juntos. Nenhum bloco foi reexecutado nesta etapa; todos são citação da evidência do orquestrador.

**E1 — release e ativos (2026-11-20T00:43:52Z)**

```
$ gh api repos/godotengine/godot/releases/tags/4.7.2-stable --jq '.assets[].name'
Godot_v4.7.2-stable_linux.x86_64.zip
SHA512-SUMS.txt
```

Saída transcrita nas entradas relevantes; download dos dois ativos com `curl -fsSL` a partir dos campos `browser_download_url` da resposta da API, com sucesso silencioso e arquivos presentes no alvo.

**E2 — integridade (2026-11-20T00:43:52Z)**

```
$ grep 'Godot_v4.7.2-stable_linux.x86_64.zip' SHA512-SUMS.txt
9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65  Godot_v4.7.2-stable_linux.x86_64.zip
$ sha512sum Godot_v4.7.2-stable_linux.x86_64.zip
9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65  Godot_v4.7.2-stable_linux.x86_64.zip
```

Comparação esperado == calculado → `CHECKSUM_OK`.

**E3 — identidade do binário (2026-11-20T00:44:11Z)**

```
$ Godot_v4.7.2-stable_linux.x86_64 --headless --version
4.7.2.stable.official.ed1daf0bf
```

**E4 — carga sintética (2026-11-20T00:44:35Z e 2026-11-20T00:44:48Z)**

```
$ G=Godot_v4.7.2-stable_linux.x86_64
$ $G --headless --path tests/benchmark --script headless_tick.gd
```

- 2026-11-20T00:44:35Z — 3 amostras iniciais: 9,43 / 15,50 / 17,48 µs/tick.
- 2026-11-20T00:44:48Z — 7 amostras: 25,49 / 42,95 / 22,71 / 27,64 / 39,45 / 39,99 / 37,48 µs/tick; mediana 37,48 µs/tick; trecho da linha `BENCHMARK`: `accumulator=-297.626094`.

## Evidence Against the Idea

- A carga é sintética: mede o custo de um laço de simulação com `sin()`, sem renderização, rede, IA de jogadores ou serviços externos. Apresentá-la como benchmark de partida 5v5 real geraria conclusão inválida. Mitigação: rotular explicitamente a carga como simulação headless sintética em todos os artefatos e vedar a reclassificação. [source: `tests/benchmark/headless_tick.gd:3-19`; pedido do mantenedor] (confidence: high)
- A medição vem de uma única sessão, em uma única máquina, com 7 amostras e dispersão de 22,71 a 42,95 µs/tick. Isso estabelece uma linha de base, não uma garantia de desempenho em hardware diverso nem um limite superior. Mitigação: registrar mediana, mínimo, máximo e contagem de amostras, sem converter o resultado em SLA. [source: evidência 2026-11-20T00:44:48Z] (confidence: high)
- A verificação de checksum prova integridade entre o artefato publicado e o artefato usado; ela não prova identidade de mantenedor upstream nem ausência de comprometimento da conta que publicou a release. Mitigação: citar origem e valores registrados e tratar qualquer conteúdo obtido da web como dado não confiável. [source: lógica da própria verificação; `AGENTS.md:41-45`] (confidence: high)
- Nesta etapa de planejamento a instalação e a execução não são reproduzidas: a evidência é do orquestrador. Afirmar que o planejamento "rodou" o benchmark seria desonesto. Mitigação: citar carimbos e deixar reexecução e validação para as fases próprias. [source: atribuição do mantenedor; `docs/governance/sdd-phase-contracts.md:24-27`] (confidence: high)
- O `project.godot` do benchmark declara `config/features` para a série 4.7 e nenhuma cena principal; uma versão de engine diferente poderia invalidar a comparação. Mitigação: fixar a identidade `4.7.2.stable.official.ed1daf0bf` como parte da evidência. [source: `tests/benchmark/project.godot:1-5`; evidência 2026-11-20T00:44:11Z] (confidence: high)

## Gaps & Open Questions

- Como a carga do benchmark deve ser classificada para não induzir conclusão de desempenho de partida real? (respondido em `spec.md` / `## Clarifications` Q1)
- Qual referência numérica usar para julgar a medição e quais estatísticas registrar? (respondido em `spec.md` / `## Clarifications` Q2)
- Como tratar a evidência já obtida pelo orquestrador nesta etapa de planejamento? (respondido em `spec.md` / `## Clarifications` Q3)

## Sources

- `gh api repos/godotengine/godot/releases/tags/4.7.2-stable --jq '.assets[].name'` (host: github.com via CLI `gh`, policy: allowlisted) — listou `Godot_v4.7.2-stable_linux.x86_64.zip` e `SHA512-SUMS.txt`; download com `curl -fsSL`. Comando e saída transcritos em E1; nenhuma credencial, parâmetro assinado ou URL com userinfo foi persistido.
- Evidência de execução do orquestrador transcrita em E1–E4 (política: dado do pedido, não buscado na web): `CHECKSUM_OK` em 2026-11-20T00:43:52Z (E2); binário `4.7.2.stable.official.ed1daf0bf` em 2026-11-20T00:44:11Z (E3); 3 amostras em 2026-11-20T00:44:35Z e 7 amostras com mediana 37,48 µs/tick em 2026-11-20T00:44:48Z (E4).
- Arquivos locais do alvo (política: `local-only`): `tests/benchmark/headless_tick.gd`, `tests/benchmark/project.godot`, `AGENTS.md`, `.specify/memory/constitution.md`, `docs/governance/sdd-phase-contracts.md`, `specs/005-ci-protecao-dev/research.md`.
