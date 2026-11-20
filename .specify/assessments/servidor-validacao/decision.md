# Decision — Servidor headless e validação autoritativa

**Data:** 2026-11-20T03:00Z · **R1:** `go`

## Veredito
**go** — problema válido (impacto 5, evidência 5), opção recomendada = Opção 1 (Tasks 2–3 literais do plano).

## Gate de entrada
- Gate determinístico do alvo: 5/5 PASS em `2026-11-20T02:02:49Z` (`spec_dirs=8`, `tasks=222`, `cycles=0`, `0 erros`); `pytest -q` N/A (exit 5) e `check_production_gate.py` N/A por Bloco 0.a.
- `.specify/feature.json` → `specs/009-servidor-validacao`; branch `feature/009-servidor-validacao`, HEAD `0e26179`. Estado remoto nesta sessão é **[NV]**.
- Binário fixado: `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`; a versão só é conferida na fase de implementação **[NV]**.

## Restrições de execução
- Código literal das Tasks 2–3: constantes, assinaturas, cinco códigos de validação e contagens de `PASS` inalterados; nenhuma reescrita "melhorada".
- Comandos sempre com `--headless` e o binário absoluto; saídas coladas com data/hora na portaria de validação, sem afirmar execução não observada (**[NV]**).
- Sem tocar em Ilha, manifesto, `Makefile`, `evidence.csv`, factory, remote, baselines ou `converge.md`; sem branches, commits, pushes ou gates nesta etapa de escrita.
- Convergence/analyze é READ-ONLY: divergência corrigida na fonte e revalidada, nunca mascarada no artefato derivado.
