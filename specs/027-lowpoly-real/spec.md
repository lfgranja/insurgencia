# Feature Specification: Low-poly visual real no cliente

**Branch**: `feature/027-lowpoly-real` · **Created**: 2026-11-21 · **Status**: Draft

## User Scenarios

### US1 — Visual low-poly reconhecível (P1)
A SE abre o jogo e vê terreno irregular, ilhas com relevo, árvores e rochas low-poly — não apenas cubos e cones.

**Acceptance Scenarios**: 1) terreno com ≥20 triângulos e alturas variadas; 2) cada ilha com ≥30 triângulos e topo/taludo irregulares; 3) 2–3 árvores e 2 rochas por ilha; 4) avatar com corpo e cabeça.

### US2 — Identidade preservada (P1)
As quatro ilhas continuam reconhecíveis por silhueta e cor (GDD:1045), e o HUD reporta posição/ilha.

### US3 — Regra da janela (P2)
Janela normal quando a SE pede; `--minimizado` em validação de agente.

## Requirements
- **FR-001**: malhas procedurais irregulares via `Mesh::new(TriangleList)` + `compute_normals()`.
- **FR-002**: terreno com altura variável determinística.
- **FR-003**: ilhas com plataforma irregular e taludo.
- **FR-004**: árvores e rochas low-poly por ilha.
- **FR-005**: avatar low-poly com corpo e cabeça.
- **FR-006**: sem texturas e sem assets externos (GDD §13.2; host fora da allowlist).
- **FR-007**: constantes e 5 testes de `lib.rs` preservados.
- **FR-008**: `verify.sh` ALL STAGES OK; 5 gates do alvo.

## Clarifications
Q1: low-poly = geometria irregular poucos polígonos, não primitivas isoladas. Q2: Kenney CC0 é referência futura, não baixado. Q3: silhueta+cor preservadas. Q4: regra da janela inalterada.

## Success Criteria
SC-001: visual low-poly confirmado por inspeção da SE **[NV]** até validação. SC-002: ≥20 triângulos no terreno e ≥30 por ilha no código. SC-003: 34 testes preserved. SC-004: gates verdes. SC-005: janela normal/minimizada conforme regra.
