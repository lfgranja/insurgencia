# Decision — Servidor headless determinístico

- **Slug**: `servidor-headless`
- **Decided**: 2026-11-23T04:30Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | O `sim` já tem relógio fixo, fila ordenada e `hash_estado`, mas não há binário que exponha isso como evidência comparável; sem `diff` vazio não há prova auditável de determinismo nem base para netcode. |
| Força da evidência | strong | Contrato imutável fecha crate, dependências, CLI, parser, agendamento, saída, erro e testes; checkout lido (`Cargo.toml:1-5`, `crates/sim/src/lib.rs:15,38-46,95-105,110-127,138-148,199-215,430,480,520-545`); constituição v1.2.2 lida. |
| Valor vs. inação | strong | 256/625 contra 250/625 (só dívida documental), 48/625 (combate/Fôlego) e 10/625 (rede Lightyear); inação mantém o determinismo afirmado por testes internos, sem recibo executável. |
| Viabilidade / appetite | strong | `small`: crate único, dependência única `insurgencia-sim`, sem contrato, protocolo, dependência externa ou infraestrutura novos; fronteira textual já especificada. |
| Alinhamento estratégico | strong | Constituição I (determinismo primeiro), II (adaptadores/portas — nenhum domínio chama rede diretamente), IV (soberania) e VI (verificação honesta); R1 `go`, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY antes de `implement`. |
| Postura de risco | strong | Riscos nomeados e mitigados: parser sem pânico via `Result`/`ExitCode`, ordem preservada por `push_back`, hash único em `hash_estado`, proibição de coordenadas no parser; R6 registrada como N/A — dívida de automação, sem issue simulada nem `gh` improvisado. |

## 80/20 (scorecard único do ciclo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Servidor headless (escolhida) | 4×4×4×4 = 256/625 | **256/625** |
| Só fechar a dívida documental | 2×5×5×5 = 250/625 | 250/625 |
| Combate / Fôlego | 4×2×2×3 = 48/625 | 48/625 |
| Rede Lightyear (netcode) | 5×2×1×1 = 10/625 | 10/625 |

Critério do contrato — impacto × confiança × facilidade ÷ risco, escala 1–5, denominador 625 (5⁴); o quarto fator é o risco invertido (segurança): 5 = risco mínimo, 1 = risco máximo. Este é o único scorecard do assessment; os mesmos valores aparecem em `intake.md`, `research.md` e `shape.md`, sem pontuação paralela.

O servidor headless vence porque é a única opção que produz **capacidade verificável nova**: impacto 4 (recibo executável e comparável), confiança 4 (contrato fechado, integração ainda não executada), facilidade 4 (um crate, uma dependência) e risco 4 (sem rede, sem `async`). A dívida documental tem confiança/facilidade/segurança máximas, mas impacto 2 — daí a margem estreita de 6 pontos e a escolha pelo impacto. Combate/Fôlego contradiz o MVP de desafios em vez de combate puro (GDD §16.3:1509); Lightyear tem impacto nominal 5, porém risco máximo e é vetado pelo contrato. Contraste entre ciclos: 039 = 180/625 (coleta no Núcleo), 040 = 400/625 (barricada), 041 = 500/625 (coleta jogável), 042 = 256/625 (servidor headless) — pontuações são máximos **dentro de cada conjunto de opções**, não uma série comparável.

## Veredito e justificativa

**Veredito: go** — Opção A (servidor headless determinístico), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 `1.0.0-local`, R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. Crate `crates/servidor` com binário `insurgencia-servidor`, dependência única `insurgencia-sim`, sem novas dependências externas, sem `async`, sem sockets e sem Lightyear; `Cargo.toml` da workspace ganha o membro e `Cargo.lock` é gerado pelo Cargo.
2. CLI com `--ticks N` (inteiro positivo) e comandos pela stdin, um por linha, `peer_id acao seq request_id [x y]`; coordenadas obrigatórias para `mover`, `coletar_materiais` e `construir_barreira`, proibidas para as demais ações conhecidas; comandos agendaados em ordem antes do tick 0.
3. Saída de exatamente uma linha por tick, `<tick>\t<hash sha256>`, começando em `0`, com o hash obtido exclusivamente de `Mundo::hash_estado()`.
4. Entrada malformada e ação desconhecida → erro determinístico, código de saída `2`, sem pânico; uso de `Result`/`ExitCode`.
5. Disciplina Rust: `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, documentação `///` em itens públicos.
6. Testes: unitários do parser e de integração CLI — saída idêntica entre duas execuções, contagem de linhas igual a `N` e rejeição de entrada inválida.
7. Fronteira preservada: nada em `crates/contratos`, `PROTOCOL_VERSION`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`; nenhum código-fonte alterado neste assessment.
8. Constituição v1.2.2 aplicada no ciclo: R1 `go`, R4 antes de R7 `1.0.0-local`, `analyze` READ-ONLY antes de `implement`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Parser sem pânico**: conversão de `peer_id`/`seq`/`request_id`/`x`/`y` e contagem de campos devem retornar `Result`; nenhum `unwrap`/`expect` — erro determinístico código 2 é requisito, não detalhe.
- **Regra de coordenadas**: `validar_comando` do `sim` só exige destino para algumas ações; a proibição nas demais cinco ações conhecidas é do parser da CLI e deve ser aplicada antes de montar `Comando`.
- **Ordem e contagem de ticks**: `fase_prepare` drena todos os pendentes no primeiro tick; a especificação deve fixar o instante de impressão do hash no tick 0 para garantir N linhas numeradas de 0 a N-1.
- **Existência do peer**: `CodigoRecusa::PeerNaoAutorizado` surge se o jogador não existe; definir `adicionar_jogador` antes de agendar.
- **Fonte do hash**: usar somente `Mundo::hash_estado()`; reimplementar a serialização criaria duas verdades e invalidaria o `diff`.
- **Dívida R6 (automação, N/A)**: fora da sequência executável na v1.2.2; nenhuma issue é criada e nenhum `gh` é improvisado. Não é dívida de produto.
- **Verificação**: gates, build, testes, R4 e R7 permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: o determinismo do Núcleo existe, mas não há fronteira executável e comparável que o prove; sem recibo textual, netcode viria antes da verdade que precisa sincronizar.
- **Chosen approach**: Opção A — crate `crates/servidor` com binário `insurgencia-servidor`, dependência única `insurgencia-sim`, CLI `--ticks N` + comandos na stdin, saída `<tick>\t<hash sha256>` e erro determinístico código 2.
- **In scope**: crate e membro da workspace, parser de argumentos e de comandos, regra de coordenadas, agendamento em ordem, loop de N ticks, emissão do hash do `sim`, `Result`/`ExitCode`, testes unitários e de integração. **Out of scope**: rede/Lightyear, `async`, sockets, dependências externas, contratos/protocolo, combate, R6 e qualquer execução de gates/build/testes neste assessment.
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: membro via glob ou entrada explícita em `Cargo.toml`, instante de impressão no tick 0, garantia de existência do `peer_id` e nomes definitivos de T001..T012 em `specs/042-servidor-headless/tasks.md` — validar no `/speckit.specify`.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | crate `crates/servidor`, binário `insurgencia-servidor`, só `insurgencia-sim`; membro da workspace; `Cargo.lock` gerado pelo Cargo | contrato; `intake.md` |
| FR-002 | `--ticks N` com inteiro positivo | contrato; `define.md` |
| FR-003 | parser `peer_id acao seq request_id [x y]`; coordenadas obrigatórias para `mover`/`coletar_materiais`/`construir_barreira`, proibidas para as demais ações conhecidas | contrato; `shape.md` · Opção A |
| FR-004 | comandos agendaados em ordem antes do tick 0 | `crates/sim/src/lib.rs:199-215`; `shape.md` · Opção A |
| FR-005 | uma linha por tick `<tick>\t<hash sha256>` a partir de 0, via `hash_estado()` | `crates/sim/src/lib.rs:430`; `define.md` |
| FR-006 | erro determinístico, código 2, sem pânico; `Result`/`ExitCode`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, `///` | contrato; `shape.md` · Opção A |
| SC-001 | saída idêntica entre duas execuções | `define.md` · Success Metrics 1 |
| SC-002 | exatamente N linhas, numeradas de 0 a N-1 | `define.md` · Success Metrics 2 |
| SC-003 | `--ticks` rejeita 0, negativo e não inteiro | `define.md` · Success Metrics 3 |
| SC-004 | coordenadas ausentes/extras rejeitadas conforme a ação | `define.md` · Success Metrics 4 |
| SC-005 | linha malformada e ação desconhecida → código 2, sem pânico | `define.md` · Success Metrics 5 |
| SC-006 | hash exclusivo de `hash_estado()`; disciplina Rust e testes | `define.md` · Success Metrics 6-8 |
| T001 | `crates/servidor/Cargo.toml`: binário e dependência única; membro da workspace | `shape.md` · Opção A |
| T002 | `main.rs` com `forbid(unsafe_code)`, `deny(clippy::pedantic)` e itens documentados | `shape.md` · Opção A |
| T003 | parser de argumentos `--ticks N` com validação de inteiro positivo | `shape.md` · Opção A |
| T004 | parser de linha de comando `peer_id acao seq request_id [x y]` | `shape.md` · Opção A |
| T005 | validação de coordenadas obrigatórias/proibidas por ação | `shape.md` · Opção A |
| T006 | agendamento em ordem na fila do `Mundo` antes do tick 0 | `shape.md` · Opção A |
| T007 | loop de N ticks e impressão `<tick>\t<hash sha256>` via `hash_estado()` | `shape.md` · Opção A |
| T008 | erro determinístico com `Result`/`ExitCode` código 2, sem pânico | `shape.md` · Opção A |
| T009 | testes unitários do parser (válidos, inválidos, coordenadas, ações desconhecidas) | `shape.md` · Opção A |
| T010 | teste de integração CLI: saída idêntica entre duas execuções | `shape.md` · Opção A |
| T011 | teste de integração CLI: contagem de linhas igual a N | `shape.md` · Opção A |
| T012 | teste de integração CLI: rejeição de entrada inválida; artefatos do ciclo | `shape.md` · Assumptions |
| R6 | N/A — constituição v1.2.2; dívida de automação, nunca issue simulada ou `gh` improvisado | `.specify/memory/constitution.md` |

Os nomes definitivos de T001..T012 são os de `specs/042-servidor-headless/tasks.md`; a coluna "Eco" apenas rastreia a cobertura deste assessment. Nada aqui é especificação, plano ou implementação.

Veredito: go
