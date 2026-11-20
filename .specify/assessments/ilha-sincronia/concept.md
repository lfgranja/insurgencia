# Concept — Desafio da Ilha: sincronia reduzida em 5 s

**Data:** 2026-11-20T03:10Z

## Opções consideradas
1. **Implementar a Task 4 literalmente do plano** (escolhida) — `nucleo/desafios/ilha_sincronia.gd`, `modulos/exemplo/mapa.json`, `modulos/base/regras.cfg`, integração em `nucleo/mundo.gd`, validação da placa em `nucleo/sim/validacao.gd` e as duas suítes `extends SceneTree`, com constantes, motivos, contagens de `PASS` e comandos exatamente como em `docs/plans/2026-11-19-ciclo-80-20-omoba.md` (Task 4, Steps 1–8, linhas 723–1025).
2. **PR único com Tasks 4–6** (rejeitada) — mistura o desafio com manifesto de compatibilidade, gate `make check`, `evidence.csv`, 5v5 e vitória Central num diff grande e de baixa revisabilidade; rompe a fatia vertical de um ciclo e antecipa evidência ainda **[NV]**.
3. **Mudar a engine (Rust/Bevy ou equivalente)** (rejeitada) — ADR-001 e a restrição GDScript/Godot 4.7.2 seguem válidas; o plano fixa `--headless --script` sobre o binário absoluto, sem etapa de build.
4. **Escrever apenas os testes, sem a regra da Ilha** (rejeitada) — não destrava CE-3 nem produz captura, penalidade por amostra ou evento `ilha_concluida`.

## Consequências
- Arquivos novos e modificados restritos a `nucleo/`, `modulos/` e `tests/`; nenhuma dependência externa e nenhuma alteração de gate, baseline ou configuração remota.
- A janela é medida com `tick * PASSO_SEGUNDOS` fornecido pelo servidor, nunca com o relógio do cliente; o gás é debitado por amostra de 1,0 s (8 pontos; 24 em 3 s), não por quadro.
- A regra de produção (3 jogadores) fica em `modulos/base/regras.cfg`, separada do Módulo de exemplo (2 jogadores); o único estado persistido é `ilha_concluida`, limpo por `reiniciar()`.
- Evidência de execução permanece **[NV]** até comando com saída e data/hora; portarias de validação/release pertencem a fases posteriores, ausentes destes artefatos.
- Cada suíte tem contagem de `PASS` e código de saída binários, o que permite Convergence READ-ONLY sem reinterpretar intenção.
