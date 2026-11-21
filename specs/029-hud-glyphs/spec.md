# Feature Specification: HUD glyphs ausentes

**Branch**: `feature/029-hud-glyphs` · **Created**: 2026-11-21 · **Status**: Draft · **R1**: go

## User Scenarios
### US1 — HUD legível (P1)
A SE vê `INSURGENCIA - low-poly` e `Limites: X +/-580 / Y +/-310` sem caracteres "▯".

**Acceptance Scenarios**: 1) strings do HUD contêm somente ASCII; 2) `Ê`, `—`, `·` não aparecem em nenhum `Text::new`/`format!` do HUD.

## Requirements
- **FR-001**: `main.rs:458` → `Text::new("INSURGENCIA - low-poly")`.
- **FR-002**: `main.rs:555` → `format!("INSURGENCIA - low-poly\n...Limites: X +/-{LIMITE_X:.0} / Y +/-{LIMITE_Y:.0}\n...")`.
- **FR-003**: nenhuma fonte/asset externo adicionado.
- **FR-004**: lógica, testes e regra da janela preservados; agente nunca abre janela normal.
- **FR-005**: `verify.sh` ALL STAGES OK + 5/5 gates PASS.

## Success Criteria
SC-001: ASCII-only no HUD (verificado por grep). SC-002: 34 testes PASS. SC-003: 5/5 gates PASS. SC-004: nenhuma janela normal aberta por agente. SC-005: confirmação visual da SE **[NV]** até observação.
