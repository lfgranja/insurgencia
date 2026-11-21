# Checklist de Requisitos — 030-demo-deterministica-headless

- [x] FR-001 exemplo headless apenas `insurgencia_sim`
- [x] FR-002 sequência fixa (2 jogadores, 30 ticks, hash por tick)
- [x] FR-003 saída determinística (`diff` vazio, `DETERMINISTIC_OK`)
- [x] FR-004 regra da janela (`--minimizado`; sem janela normal)
- [x] FR-005 princípio I/VI (LLM informativa; gate determinístico autoridade)
- [x] FR-006 cobertura por `./verify.sh` (que executa `--all-targets` internamente)
- [x] FR-007 release `1.0.0` + changelog `sb-16` + rollback; R4 precede R7
- [x] FR-008 converge append-only datado (2026-11-21)

## Revisão
Requisitos verificáveis por comando/inspeção. Checklist `[x]` = revisão de requisitos, não implementação; evidências observadas registradas no R4. Aprovado para tasks.
