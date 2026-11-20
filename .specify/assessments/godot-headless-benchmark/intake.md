# Idea Intake: Godot headless benchmark

- **Slug**: godot-headless-benchmark
- **Created**: 2026-11-20
- **Source**: pedido do mantenedor nesta sessão (texto autorizado); evidência de execução do orquestrador com carimbos entre 2026-11-20T00:43:52Z e 2026-11-20T00:44:48Z; arquivos locais `tests/benchmark/project.godot` e `tests/benchmark/headless_tick.gd`
- **Type**: exploration

## Idea (as captured)

> "Objeto (autorizado): instalar Godot 4.7.2 de release oficial com verificação de checksum e executar benchmark headless no alvo. Evidência já obtida pelo orquestrador (cite em research.md): release `4.7.2-stable` via `gh api repos/godotengine/godot/releases/tags/4.7.2-stable`; `SHA512-SUMS.txt` esperado `9aa00f7a...c65`; calculado idêntico → `CHECKSUM_OK` às 2026-11-20T00:43:52Z; binário `4.7.2.stable.official.ed1daf0bf` às 00:44:11Z; benchmark `tests/benchmark/headless_tick.gd` 600 ticks × 64 entidades, mediana 37,48 µs/tick em 7 amostras às 00:44:48Z (mín 22,71; máx 42,95) — abaixo de 16,7 ms/tick. NÃO reclassificar como benchmark 5v5 real: é carga sintética de simulação headless. Execute EM ORDEM intake→research→define→shape→decide(go)→specify→clarify→plan→checklist→tasks, FRs/SCs mensuráveis, checklist `[ ]`, T### sem portarias posteriores."

> Aceitação solicitada: dez artefatos, sem seção vazia e sem marcador de pendência; R1 go.

## Restated

Propor que a instalação da engine Godot no alvo use uma release oficial identificada e verificada por checksum, e que o custo de um tick de simulação headless seja medido com a carga sintética já versionada em `tests/benchmark/`, produzindo uma linha de base auditável. A proposta é de exploração e verificação: ela estabelece identidade do binário e uma medição de referência, sem entregar funcionalidade de jogo e sem transformar a carga sintética em prova de desempenho de uma partida real.

## Origin & Context

- **Raised by**: mantenedor `lfgranja`, pedido explícito nesta sessão (2026-11-20).
- **Trigger**: necessidade de substituir a condição registrada em `AGENTS.md` — nenhuma engine instalada e nenhum teste ou build no alvo — por evidência concreta de uma release verificada e de uma medição de referência, com os resultados já obtidos pelo orquestrador e entregues como fatos a citar.
- **Contexto de localização**: os artefatos de assess e de entrega ficam colocalizados em `specs/006-godot-headless-benchmark/`, conforme o escopo de escrita da atribuição. Nenhum arquivo fora desse diretório é escrito nesta etapa de planejamento.
- **Limites de autoridade**: a instalação com verificação de checksum e a execução do benchmark headless foram autorizadas; reclassificar a medição como benchmark 5v5 real não foi autorizado e está explicitamente vedado. Nenhuma portaria posterior a tasks é parte deste pedido.

## First-Glance Unknowns

- Resolvido (dado do pedido): release `4.7.2-stable`; `CHECKSUM_OK` em 2026-11-20T00:43:52Z; binário `4.7.2.stable.official.ed1daf0bf` em 00:44:11Z; mediana 37,48 µs/tick em 7 amostras em 00:44:48Z; carga sintética, não 5v5 real; ordem obrigatória até tasks; dez artefatos; R1 `go`.
- Como a natureza da carga deve ser rotulada para que a medição não seja lida como desempenho de partida multiplayer real? (respondido em `spec.md` / `## Clarifications` Q1)
- Qual referência numérica deve ser usada para julgar a medição e quais estatísticas devem ser registradas junto com ela? (respondido em `spec.md` / `## Clarifications` Q2)
- Que evidência de origem e data/hora deve acompanhar release, checksum, binário e medição para tornar o resultado auditável? (respondido em `spec.md` / `## Clarifications` Q3)
