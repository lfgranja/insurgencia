# Concept — Servidor headless e validação autoritativa

**Data:** 2026-11-20T03:00Z

## Opções consideradas
1. **Implementar Tasks 2 e 3 literalmente do plano** (escolhida) — `nucleo/mundo.gd`, `nucleo/servidor.gd`, `nucleo/main_server.gd`, `nucleo/sim/validacao.gd` e três testes `extends SceneTree`, com códigos, constantes e comandos exatamente como em `docs/plans/2026-11-19-ciclo-80-20-omoba.md` (Task 2 Steps 1–7, Task 3 Steps 1–6).
2. **PR único com Tasks 2–6** — rejeitada: mistura servidor, Ilha, manifesto, `Makefile` e evidência num diff grande, de baixa revisabilidade, exigindo netcode e loopback ainda não validados; rompe a fatia vertical de um ciclo.
3. **Migrar a simulação para Rust/Bevy** — rejeitada: ADR-001 e a restrição GDScript/Godot 4.7.2 seguem válidas; o plano fixa `--headless --script`, sem etapa de build.
4. **Escrever apenas os testes, sem servidor** — rejeitada: não destrava CE-2 e não produz `SERVIDOR_PRONTO`, snapshots nem paridade.

## Consequências
- Arquivos novos restritos a `nucleo/` e `tests/`; nenhuma dependência externa, nenhum serviço de descoberta e nenhuma alteração de gate ou configuração remota.
- Evidência de execução permanece **[NV]** até comando com saída e data/hora; portarias de validação e release pertencem a fases posteriores, ausentes destes artefatos.
- Cada suíte tem contagem de `PASS` e código de saída binários, o que permite Convergence READ-ONLY sem reinterpretar intenção.
