# Problem — Núcleo, contrato versionado e Módulo de exemplo

**Data:** 2026-11-20T02:00Z

## Problema
Após 7 ciclos de governança, o repositório tem aparato SDD e CI funcional, mas **nenhum código de produto**: 0/39 caminhos canônicos, 0 testes GDScript, 0 `Makefile`, 0 `manifesto.json`, 0 evidência. CE-1 não pode ser avaliado.

## Por que é válido
- Impacto na missão: 5 — sem Núcleo e contrato, F2–F5 e o MVP não podem existir.
- Evidência: 5 — spec/plano com caminhos literais, campos, hashes e comandos binários; engine executável confirmada agora.
- Custo de não agir: a fila 80/20 fica permanentemente bloqueada no mesmo ponto.

## Limites
Não inclui servidor, snapshots, Ilha, manifesto ou evidência ( Tasks 2–6 ). Não altera ADRs, factory, remote ou baselines de governança.
