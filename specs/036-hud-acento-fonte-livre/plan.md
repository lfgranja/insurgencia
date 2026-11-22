# Implementation Plan — 036-hud-acento-fonte-livre

**Ordem normativa**: assess.intake → research → define(problem) → shape(concept) → decide(`go`) → specify → clarify → plan → checklist → **revisar checklist** → **analyze (READ-ONLY)** → tasks → implement → converge (append-only) → validate (R4) → release (R7). `analyze` executa **antes** de `tasks`; R4 aprova **antes** de R7 `1.0.0`.

## Passos

1. **Assess**: registrar intake (`21:15Z`), research com fontes e grau de evidência, problem, concept (5 opções, 80/20 = 240/625) e decision com `Veredito: go` (R1), 0 clarificações (R3).
2. **Specify/clarify**: substituir o stub por `spec.md` com US1–US4, FR-001..FR-009, SC-001..SC-006, clarifying que a escolha do SWE revoga o paliativo 029 para os três glifos.
3. **Plan**: este arquivo; fronteira explícita (só 14 artefatos SDD), sem build/gate/commit.
4. **Checklist**: um item por FR em `checklists/requirements.md`; revisão registrada na própria seção “Revisão” e aprovada para `analyze`. `[x]` = revisão de requisitos, não implementação.
5. **Analyze (READ-ONLY)**: `analyze.md` com achados A1..A8, convergência e veredito; nenhuma edição de spec/plano/checklist/código nesta etapa; executa antes de `tasks`.
6. **Tasks**: `tasks.md` com T-IDs únicos e DAG linear; T018 (implement) pertence ao agente de código e fica **[NV]**.
7. **Roteiro de testes (antes do PR)**: `.specify/validations/hud-acento-fonte-livre/test-roadmap.md` — escopo/FR/T, pré-condições, passos numerados, superfície CLI/visual isolada, capturas Xvfb sem janela no desktop, coluna `automatizável?`, rollback; passos explícitos para os três glifos, ausência de `▯`, hash e licença da fonte.
8. **Converge (append-only)**: primeira entrada em `specs/036-hud-acento-fonte-livre/converge.md`, datada, sem tocar `converge.md` de outros ciclos.
9. **Validate (R4)**: `validate.md` aprovando documentalmente, com toda execução pendente marcada **[NV]**.
10. **Release (R7)**: `release.md` versão `1.0.0`, changelog por FR/T, rastreabilidade e rollback (nunca executado), após R4.

## Handoff ao agente de código (fora da fronteira SDD)
- Aplicar FR-001/FR-002 nas duas strings de `crates/cliente/src/main.rs` e FR-003 no `TextFont` (`AssetServer` → `Handle<Font>` → `font: fonte.into()`).
- Embarcar `assets/fonts/NotoSans-Regular.ttf` (621572 bytes, SHA-256 `478c…3823`), `assets/fonts/OFL.txt` (OFL 1.1 integral) e `docs/governance/font-provenance.md`.
- Observado às `21:18Z`: essas mudanças já estão em curso no worktree (`main.rs` +10/−5, assets e proveniência criados). Verificação pertence ao orquestrador; nada é reexecutado por este agente.

## Verificação (pelo orquestrador, **[NV]** neste ciclo)
- `./verify.sh` → ALL STAGES OK, 41/41 testes; 5 gates Python (SDD, DAG, task-id, converge append-only, hub-docs).
- Captura Xvfb + lavapipe, cliente em `--minimizado`: `INSURGÊNCIA — low-poly`, `·` nos limites, nenhum `▯`.
- `sha256sum` do TTF e leitura da licença conferem com a proveniência; `fc-match` para os três codepoints.
- CI/PR e merge ficam com o SWE; nenhum agente abre janela normal nem mescla `main`.

## Riscos e mitigação
| Risco | Mitigação |
|---|---|
| Tofu retorne por fonte não carregada | `AssetServer` + `font` no `TextFont`; passo de captura no roteiro |
| Fonte sem licença rastreável | OFL 1.1 integral + hash + URL em `font-provenance.md`; passo de hash/licença no roteiro |
| Ordem normativa invertida | `analyze` antes de `tasks`; R4 antes de R7; checklist revisado |
| Janela aberta no desktop da SE | Cliente em `--minimizado`; capturas apenas em Xvfb |
| Evidências afirmadas sem execução | Tudo o que não foi observado marcado **[NV]** |
