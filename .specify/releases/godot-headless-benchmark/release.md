# Release (R7) — godot-headless-benchmark

- **Versão**: 1.0.0 (SemVer)
- **Data**: a preencher pelo orquestrador no ato do release (ISO 8601); referência documental da feature: 2026-11-20
- **Feature**: `specs/006-godot-headless-benchmark`
- **R4**: registrado em `.specify/validations/godot-headless-benchmark/validate.md` antes deste release
- **Branch**: `feature/006-godot-headless-benchmark` (HEAD base `56e9237`) — sem merge, sem auto-merge, sem force-push
- **Remote/CI/PR/proteções/deploy**: `[NV]` até confirmação ao vivo via `gh`

## Changelog (1.0.0)

| ID | Mudança | Origem |
|---|---|---|
| FR-001 | Release upstream fixada como `4.7.2-stable`, rastreável a `gh api repos/godotengine/godot/releases/tags/4.7.2-stable`, antes de qualquer afirmação de instalação | `spec.md` §FR-001; `plan.md` item 1 |
| FR-002 | Par esperado/calculado do SHA-512 registrado: esperado `9aa00f7a...c65` (128-hex) de `SHA512-SUMS.txt` e calculado por `sha512sum`, com exigência de igualdade exata | `spec.md` §FR-002; `research.md` §E2 |
| FR-003 | Igualdade registrada como `CHECKSUM_OK` em 2026-11-20T00:43:52Z; divergência tratada como falha, sem afirmar identidade de binário | `spec.md` §FR-003; `plan.md` itens 2–3 |
| FR-004 | Identidade do binário fixada pela string auto-reportada `4.7.2.stable.official.ed1daf0bf` (2026-11-20T00:44:11Z), sem inferência por nome de arquivo | `spec.md` §FR-004; `research.md` §E3 |
| FR-005 | Medição delimitada à carga sintética headless de `tests/benchmark/headless_tick.gd`, 600 ticks × 64 entidades, sem renderização, rede, serviços externos ou jogadores | `spec.md` §FR-005; `research.md` §E4 |
| FR-006 | Saída do benchmark registrada com `ticks`, `entities`, `elapsed_us`, `per_tick_us` e `accumulator`, preservando o formato da carga versionada | `spec.md` §FR-006; `tests/benchmark/headless_tick.gd:5-19` |
| FR-007 | 7 amostras registradas com mediana 37,48 µs/tick, mínimo 22,71 e máximo 42,95; nenhuma amostra descartada | `spec.md` §FR-007; `research.md` §E4 |
| FR-008 | Mediana convertida para 0,03748 ms/tick e comparada à referência inalterada de 16,7 ms/tick, com registro de conformidade e sem SLA | `spec.md` §FR-008; `plan.md` item 7 |
| FR-009 | Toda descrição identificada como simulação headless sintética; nenhum artefato classifica a carga como benchmark 5v5 real fora de negação explícita | `spec.md` §FR-009, §Clarifications Q1 |
| FR-010 | Toda afirmação sobre release, checksum, binário ou medição com origem e data/hora; o não verificado permanece `[NV]` | `spec.md` §FR-010; `plan.md` §D8 |
| FR-011 | Nenhum segredo, token ou credencial registrado; nenhuma instrução de conteúdo de release, issue ou web executada como diretriz | `spec.md` §FR-011; `plan.md` §D9 |
| FR-012 | Escrita restrita aos artefatos da feature; nenhum gate executado e nenhum stage, commit, PR, merge ou reescrita de histórico nesta etapa | `spec.md` §FR-012; `plan.md` §D10 |
| FR-013 | Instalação, reexecução do benchmark e portarias R4/R7 mantidas fora dos artefatos de planejamento; término em tasks | `spec.md` §FR-013; `tasks.md` §Convenção |
| FR-014 | Dez artefatos presentes e rastreáveis, sem seção vazia nem marcador de pendência, com checklist `[ ]` e tarefas `T###` | `spec.md` §FR-014; `checklists/requirements.md`, `checklists/benchmark-quality.md` |
| T001–T030 | 30 tarefas de rastreabilidade, inventário de evidência, rotulagem e auditoria concluídas (`[X]`), preservando os IDs | `specs/006-godot-headless-benchmark/tasks.md` |
| SC-001–SC-010 | Conformidade registrada no R4: release nomeada, par de checksums idêntico, 1 string de versão, 1 carga 600×64, 7 amostras com mediana/dispersão, comparação sem SLA, 100% rotulagem, origem/carimbo em 100%, 10/10 artefatos, 0 gates | `.specify/validations/godot-headless-benchmark/validate.md` |
| CHK001–CHK022, CHK001–CHK020 | Rastreabilidade FR↔SC↔CHK nos dois checklists (22/22 e 20/20 itens rastreáveis; marcadores reviewer-owned permanecem `[ ]`) | `checklists/benchmark-quality.md`; `checklists/requirements.md` |
| DEC-R1 | Veredito `go` da Opção A (release oficial verificada por checksum e benchmark sintético documentado), com evidência `strong` e nenhum `unknown` glossado | `.specify/assessments/godot-headless-benchmark/decision.md` |
| BUG-### | Nenhum bug reportado neste ciclo | — (nenhum registro `BUG-###` no ciclo) |

## Entregáveis versionados

- Assess canônico 5/5 em `.specify/assessments/godot-headless-benchmark/`: `intake.md`,
  `research.md`, `problem.md`, `concept.md`, `decision.md` — idênticos aos de
  `specs/006-godot-headless-benchmark/`.
- `.specify/validations/godot-headless-benchmark/validate.md` — R4 com `## Evidência final` para o
  orquestrador colar as saídas e a data/hora.
- Carga sintética de benchmark versionada: `tests/benchmark/headless_tick.gd` e
  `tests/benchmark/project.godot` — linha de base de 600 ticks × 64 entidades, rotulada como
  simulação headless sintética, não como partida 5v5 real.
- Evidência de integridade citada com origem e carimbo: `CHECKSUM_OK` em
  2026-11-20T00:43:52Z, binário `4.7.2.stable.official.ed1daf0bf` em 2026-11-20T00:44:11Z e
  mediana 37,48 µs/tick em 7 amostras em 2026-11-20T00:44:48Z.

## Plano de rollback (documental)

Procedimento aplicável aos artefatos desta feature: a entrega é de evidência documental e de uma
carga de benchmark versionada; nenhuma mudança de runtime de jogo, banco, migração ou serviço
externo é introduzida. A reversão ocorre em novo ciclo documentado, com nova versão SemVer e data —
nunca apagando ou reescrevendo o histórico.

1. Registrar o achado com IDs FR e, quando existirem, `T###`, `SC-###` e `BUG-###` (gatilhos:
   carga reclassificada como 5v5 real, amostra descartada, checksum divergente sem falha
   registrada, carimbo alterado, segredo introduzido, estereótipo de verde remoto sem
   confirmação ao vivo).
2. **Desinstalar o binário**: remover do alvo o executável
   `Godot_v4.7.2-stable_linux.x86_64` instalado a partir da release `4.7.2-stable`, sem apagar
   o registro histórico de sua identidade (`4.7.2.stable.official.ed1daf0bf`) e do
   `CHECKSUM_OK` em `research.md`/`plan.md`.
3. **Remover a carga de benchmark**: retirar `tests/benchmark/headless_tick.gd` e
   `tests/benchmark/project.godot` (e o diretório `tests/benchmark/` se ficar vazio), revertendo
   a linha de base de 600 ticks × 64 entidades documentada nesta feature.
4. **Reverter os documentos**: remover os cinco artefatos do assess canônico em
   `.specify/assessments/godot-headless-benchmark/`, o R4 em
   `.specify/validations/godot-headless-benchmark/validate.md` e este R7 em
   `.specify/releases/godot-headless-benchmark/release.md`, e desfazer a marcação `[X]` em
   T001–T030 de `specs/006-godot-headless-benchmark/tasks.md`, preservando os IDs.
5. Recuperar os artefatos de planejamento de `specs/006-godot-headless-benchmark/` a partir do
   estado anterior registrado no histórico, sem reescrever o histórico existente; manter a
   distinção entre evidência citada do orquestrador e reexecução.
6. Reconfirmar 0 gates, 0 stage/commit/PR/merge e 0 efeitos remotos; reexecutar a auditoria
   documental por contagens e conferir as saídas coladas no R4; registrar a reversão no
   changelog com versão, data e IDs FR/`T###`/`BUG-###`.

**Exemplo de reversão**: remoção de `tests/benchmark/headless_tick.gd` e do binário
`Godot_v4.7.2-stable_linux.x86_64` do alvo, com os documentos de `specs/006-godot-headless-benchmark/`
restaurados ao estado anterior e os registros de integridade mantidos como evidência histórica.

**Proibido**: apagar histórico, `force-push`, merge/auto-merge, reescrever snapshots ou
governança, afirmar CI/merge/aprovação/deploy obtidos. **Não aplicável**: rollback de runtime de
jogo, banco, migração, configuração de CI ou serviço externo — a feature é documental e de
benchmark headless.

## Restrições

- Nenhum merge, auto-merge, force-push ou deleção de branch executada.
- Nenhuma afirmação de CI/check verde remoto; `[NV]` até confirmação ao vivo via `gh`.
- Publicação, release e deploy no remoto permanecem `[NV]`; nenhum efeito remoto executado.
- Nenhum gate determinístico, stage ou commit executado por esta portaria.
- Nenhum código de produto, dependência proprietária no caminho crítico ou emenda da norma
  externa introduzido.
- A carga permanece rotulada como simulação headless sintética de 600 ticks × 64 entidades;
  nenhum artefato a converte em benchmark 5v5 real, SLA ou garantia de hardware.
