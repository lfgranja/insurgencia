# Implementation Plan — 039-coleta-materiais-deterministica

**Ordem normativa (constituição v1.2.2 do alvo, ratificada 2026-11-19, última emenda 2026-11-23)**: 00 constituição → 01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze (READ-ONLY) → 12 implement → 13 converge → 27 validate → 26 release. **`tasks` executa DEPOIS da revisão do checklist e ANTES de `analyze`**; R1 `go`; R4 precede R7 `1.0.0`; `converge` append-only. **R6 suspensa por decisão ratificada**: fora da sequência executável até `resolve-tasks.sh`/adaptador existirem; PARE se invocada, nunca simular issues.

## Passos

1. **00 Constituição**: v1.2.2 lida e aplicada; sequência acima; R6 não executada nem simulada.
2. **01 Intake**: sessão `20261123T015356Z-coleta-materiais`; 80/20 = 5×4×3×3 = 180/625 (fila documental esgotada); base `990ab06`.
3. **02 Research**: GDD §7.1 e `crates/sim/src/lib.rs` lidos; contrato técnico auditado registrado com fonte; R6 suspensa.
4. **03 Define (problem)**: regra canônica ausente no sim, critérios observáveis e custo de inação.
5. **04 Shape (concept)**: 6 opções; recomendação documental; 180/625.
6. **05 Decide**: scorecard 6/6 strong, `Veredito: go` (R1), R3 = 0, R6 suspensa.
7. **06 Specify / 07 Clarify**: substituir o stub por `spec.md` (US1–US5, FR-001..FR-008, SC-001..SC-006); clarificar destino como ID do ponto, acúmulo zerado, recusa sem destino, hash divergente, implementação **[NV]**, R6 suspensa.
8. **08 Plan**: este arquivo; fronteira explícita (só 14 artefatos), sem build/gate/commit.
9. **09 Checklist**: um item por FR em `checklists/requirements.md`; revisão registrada na seção "Revisão" e aprovada para `tasks`; `[x]` = revisão de requisitos, não implementação.
10. **10 Tasks**: `tasks.md` com T-IDs únicos e DAG linear; `analyze` vem **depois** de `tasks`, conforme v1.2.2.
11. **11 Analyze (READ-ONLY)**: `analyze.md` com A1..A8, convergência e veredito; nenhuma edição de spec/plano/checklist/código nesta etapa.
12. **12 Implement (documental)**: redação dos artefatos SDD na fronteira, incluindo o roteiro de testes **antes do PR**; nenhuma alteração de código, build, gate ou teste.
13. **13 Converge (append-only)**: primeira entrada em `specs/039-coleta-materiais-deterministica/converge.md`, datada, sem tocar `converge.md` de outros ciclos.
14. **27 Validate (R4)**: `validate.md` aprovando documentalmente, com execução pendente marcada **[NV]**; R4 antes de R7.
15. **26 Release (R7)**: `release.md` versão `1.0.0`, changelog por FR/T, rastreabilidade e rollback (nunca executado), após R4.

## Desenho da implementação futura (fora da fronteira SDD — **[NV]**)
Escopo: **somente `crates/sim/src/lib.rs`**.
- Constante `TICKS_COLETA: i64 = 180` (180 × `PASSO_SEGUNDOS` = 6 s).
- `Jogador`: novos campos `materiais: i64`, `suprimentos: i64`, `coleta_ticks: i64`, `coleta_ponto: Option<(f64, f64)>`; inicializados zerados/None em `adicionar_jogador`.
- `ValidacaoComandos::validar`: para `acao == "coletar_materiais"`, exigir `destino` e exigir igualdade com `jogador.posicao`, sob pena de `CodigoRecusa::DestinoInvalido` (mantidas as checagens de par, replay e ação; `mover` inalterado).
- `fase_skills`: para comando de coleta válido, acumular `coleta_ticks` enquanto o destino/ponto permanecer igual à posição; ao completar 180 ticks, creditar **atomicamente** `+3` suprimentos e `+2` materiais e zerar o contador; saída antes do prazo ou mudança de posição zera `coleta_ticks`/`coleta_ponto` sem crédito parcial; nenhum consumo de recursos nesta fase.
- `hash_estado`: incluir `materiais`, `suprimentos`, `coleta_ticks`, `coleta_ponto` na serialização por jogador, mantendo o formato canônico determinístico.
- **Não tocar** `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, GDScript, GUI, rede, dano, barricada nem R6. Ponto neutro = `Comando.destino`, sem nova mensagem.

## Roteiro de testes (antes do PR)
`.specify/validations/coleta-materiais-deterministica/test-roadmap.md` — escopo/FR/T, pré-condições, 10 passos numerados, superfície documental/CLI, capturas de terminal, coluna `automatizável?`, rollback. Casos exigidos: **180 ticks** (crédito exato +3/+2), **saída antes** (zera parcial), **recusa sem destino** (`DestinoInvalido`, inclusive destino ≠ posição), **hash divergente** (campos novos no resumo), **ausência de consumo parcial** (nenhum débito de recursos) e **determinismo** (mesma sequência → mesmo hash em mundos distintos).

## Handoff ao SWE / orquestrador (fora da fronteira SDD)
- Implementar o desenho acima em `crates/sim/src/lib.rs`, executar gates e o roteiro, revisar e mesclar PR quando aplicável; nada disso é feito por este agente (**[NV]**).
- Regra canônica: GDD §7.1 — 6 s em ponto neutro, +3 Suprimentos/+2 Materiais; sair antes zera acúmulo. Barricada, dano/emboscada, GUI, rede e GDScript ficam para ciclos futuros.
- R6 permanece suspensa até emenda ratificada após a ferramenta existir; nenhuma issue é simulada. `.specify/memory/STATE.md` é memória local do SWE — não versionar nem editar; alteração preexistente em `.specify/feature.json` pertence ao usuário.

## Verificação futura (pelo orquestrador, **[NV]** neste ciclo)
- `cargo test`/gates do hub e execução dos 10 passos do roteiro, com capturas em diretório temporário fora do worktree.
- Conferir que `hash_estado` muda com estoques/acúmulo distintos e que duas execuções idênticas produzem o mesmo resumo.
- Confirmar que nenhum arquivo de protocolo/Cargo/GDD foi alterado e que R6 não foi adotada nem simulada.

## Riscos e mitigação
| Risco | Mitigação |
|---|---|
| Crédito antes de 180 ticks ou em saída antecipada | Contador explícito e crédito atômico em `fase_skills`; caso "saída antes" no roteiro (FR-001/FR-002) |
| Coleta fora do ponto ou sem destino | `DestinoInvalido` para `destino=None` ou destino ≠ posição (FR-002; passo 4 do roteiro) |
| Hash não refletir estoques/acúmulo | Incluir os quatro campos em `hash_estado`; caso "hash divergente" (FR-002; passo 6) |
| Consumo parcial ou débito indevido | `fase_skills` apenas credita ao completar o prazo; caso "ausência de consumo parcial" (passo 7) |
| Mudança de protocolo ou escopo | Fronteira `lib.rs`; `destino` como ID do ponto; sem `crates/contratos`/`PROTOCOL_VERSION`/`Cargo.toml` (FR-003) |
| Afirmar verde sem execução | Pendências **[NV]** (Princípio VI); nenhum build/gate/teste/commit neste agente (FR-008) |
| Ordem invertida, R6 ativada ou fronteira violada | `tasks` antes de `analyze`, R4 antes de R7, `converge` novo, R6 suspensa, 14 artefatos (FR-004/FR-006) |
