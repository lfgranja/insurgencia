# Implementation Plan — Soberania e Exportação (`crates/exportacao`, `COPYRIGHT.txt`, DCO)

**Data:** 2026-11-21T01:19Z · **Fonte:** `.specify/feature.json` → `specs/025-soberania-exportacao`; `.specify/memory/STATE.md` (estado vivo 2026-11-21T00:22Z) fixa o item 4 da prioridade da SE: exportação de conta/personagem, `COPYRIGHT.txt` e commits assinados (GDD §15.9). A GDD §15.5 torna a exportação requisito do Núcleo, não cortesia do anfitrião (GDD:1395); a §15.7 exige arquivo legível e verificável por ferramenta independente; a §15.9 exige `LICENSE`/`COPYRIGHT.txt`, DCO com "todo *commit* assinado; contribuição sem assinatura é recusada" (GDD:1452) e recusa de versão divergente no *handshake*. Ambiente verificado: sem chave GPG, `commit.gpgsign`/`user.signingkey` ausentes — DCO via `Signed-off-by` é a assinatura de origem adotada (Q2). Escopo: exclusivamente os quatro documentos de especificação; nenhum `cargo`, gate, teste, build, formatador, código Rust, commit ou push. Toda saída permanece **[NV]** até registro datado em R4.

## Arquivos (exatamente os do plano)

Materializar na implementação: `crates/exportacao/Cargo.toml` (pacote `insurgencia-exportacao`, `edition = "2024"`), os módulos de `crates/exportacao/src/` para o formato versionado, o exportador e o importador com recusa de divergência, `scripts/dco-check.sh` (gate do trailer `Signed-off-by`) e `COPYRIGHT.txt` na raiz (fonte/licença/data por arquivo e asset). Leitura sem alteração: `specs/025-soberania-exportacao/spec.md`, `.specify/feature.json`, `.specify/memory/STATE.md`, `docs/ADR` aplicáveis (ADR-002 MIT, ADR-004 toolchain), specs 011 e 021–024, `docs/GDD - Insurgência.md` (§14.2, §14.7, §15.5–§15.9, GDD:1395/1451/1452), `Cargo.toml` (workspace), `verify.sh`, `clippy.toml`, `scripts/ci/`, `projetos/AGENTS.md` e a skill `mdmv-rust-thin`.

## Steps

1. Criar `crates/exportacao/Cargo.toml` com o pacote `insurgencia-exportacao`, `edition = "2024"` e alvo de biblioteca, membro do workspace `members = ["crates/*"]`. Declarar lints: `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, `unwrap_used`/`expect_used` em `deny` e avisos fatais via `-D warnings`; erros com `Result<T, E>`, `?` e `thiserror` onde aplicável, sem desativar lint em `Cargo.toml`/`clippy.toml` (única exceção: `#[expect(clippy::lint_name, reason = "Auditable technical rationale")]` para proc-macro isolado). Nenhuma dependência de rede, banco, HTTP, IAM remoto, servidor ou chave de instância; nenhum crate mesclado (`contratos`, `sim`, `cliente`, `metricas`) é alterado em sua responsabilidade. (FR-001, Q1, skill `mdmv-rust-thin`)
2. Definir em `crates/exportacao/src/` o formato versionado com exatamente cinco campos obrigatórios: `conta_id`, `personagem_id`, `criado_em` (ISO-8601 UTC explícito), `versao` (inteiro ≥ 1) e `rules_hash` (RULES hash — SHA-256 de 64 hex do conjunto de regras ativo, na forma do *hash* de conteúdo do *handshake* da GDD §14.2/spec 011). O tipo DEVE ser legível por ferramenta independente e determinístico para a mesma entrada e `criado_em`; nenhum campo aleatório, nenhum carimbo de tempo implícito e nenhuma chave de exportação guardada por instância. (FR-002, US1, Q3, GDD:1395)
3. Implementar o exportador local em `crates/exportacao/src/`: serializar conta, personagem e histórico para o JSON versionado do passo 2, sem chamada de rede, banco, IAM remoto ou servidor. A exportação DEVE estar disponível antes do lançamento do MVP (GDD §15.9) e produzir bytes idênticos quando repetida com a mesma entrada e o mesmo `criado_em`. (FR-002, FR-004, US1, Q1)
4. Implementar o importador em `crates/exportacao/src/` com recusa de formato divergente e códigos distintos, nesta ordem: JSON inválido → versão divergente → campo obrigatório ausente/malformado → `rules_hash` com comprimento diferente de 64 hex → `rules_hash` divergente → `OK`. Nenhuma conversão silenciosa, upgrade automático de versão, aceitação de estado pronto ou acesso a servidor; erro legível em cada recusa, espelhando a política de `REJEICAO_MANIFESTO`/`REJEICAO_VERSAO`/`REJEICAO_HASH` da spec 011. (FR-003, US2, Q3)
5. Verificar o roundtrip exportar → importar por leitura da suíte: os 5 de 5 campos preservados, 0 perdas, 0 divergências e saída byte-idêntica na repetição; nenhuma suíte é executada nesta etapa. (FR-004, SC-002)
6. Materializar `COPYRIGHT.txt` na raiz com uma entrada por arquivo e asset declarando fonte, licença MIT e data (e restrições quando houver), licença MIT para o Núcleo e o servidor headless (ADR-002, GDD:1451), 0 arquivos sem licença declarada e validação antes de empacotar cada Módulo (GDD §14.7). A AGPL de terceiros da GDD §15.5 NÃO é atribuída ao Insurgência. (FR-005, US3, Q4)
7. Materializar `scripts/dco-check.sh`: verificar o trailer `Signed-off-by: Nome <email>` no(s) commit(s), aceitar (exit 0) somente trailer bem formado e falhar com exit não zero e mensagem legível quando ausente ou malformado. Nenhuma chave GPG, `commit.gpgsign`, `user.signingkey`, servidor de chaves ou segredo é introduzido; a assinatura de origem é o DCO conforme GDD:1452. (FR-006, FR-007, US4, Q2)
8. Integrar o gate DCO ao pre-commit por `verify.sh --install-hook` SEM alterar os quatro estágios de `verify.sh` nem os modos `--fast`/`--uninstall-hook`, e sem modificar `Makefile`, baselines ou `evidence.csv`. `verify.sh` permanece a única fonte de verdade do gate local, terminando com `verify.sh: ALL STAGES OK`. (FR-007, FR-008, SC-001)
9. Preservar os cinco gates determinísticos alvo — `check_sdd_completeness`, `check_work_order_dag`, `check_task_id_uniqueness`, `check_converge_append_only`, `hub_docs_gate` — PASS e inalterados, com `Makefile`, baselines e `evidence.csv` intactos; nenhum gate é reescrito para acomodar a feature. (FR-008, SC-005)
10. Verificar por leitura (sem executar comando): crate e formato versionado (FR-001, FR-002), recusa de divergência (FR-003), roundtrip (SC-002), `COPYRIGHT.txt` (SC-003), gate DCO (SC-004), integridade de `verify.sh` (4 de 4 estágios) e dos 5 gates determinísticos (SC-001, SC-005), ausência de `unsafe`/`unwrap`/`expect`, de rede e de chave GPG. Toda saída permanece **[NV]**. (FR-001–FR-008, SC-001–SC-005)
11. Na implementação/R4, executar os comandos da seção abaixo e colar a saída datada somente em R4; R4 (`validate`) precede R7 (`release`: SemVer, changelog com IDs FR/T e rollback). Até lá, nenhuma medição, roundtrip, commit ou saída de gate é afirmada como observada nesta etapa. (SC-001–SC-005)
12. Convergência READ-ONLY: revisar `crates/exportacao`, `scripts/dco-check.sh`, `COPYRIGHT.txt`, `spec.md`, este plano, `tasks.md` e o checklist contra STATE.md, GDD §14.2/§14.7/§15.5–§15.9, spec 011, specs 021–024, `projetos/AGENTS.md` e a skill `mdmv-rust-thin`, corrigindo divergências sempre na fonte (código/documento de origem), nunca no artefato derivado. Manter zero servidor, zero GPG, zero alteração de gate/baseline/`evidence.csv`/`Makefile`, zero commit executado e zero alegação de execução observada. (FR-005, FR-008, SC-001–SC-005)

## Comandos de verificação

```bash
# Gate Rust (única fonte de verdade, quatro estágios na ordem)
./verify.sh
# Equivalente explícito dos estágios (execução pertence à implementação/R4, não a esta etapa):
cargo check --locked --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo nextest run --workspace          # fallback: cargo test --locked --workspace
cargo fmt --all -- --check

# Gate DCO (assinatura de origem; execução pertence à implementação/R4)
./scripts/dco-check.sh                 # exit 0 com Signed-off-by, não zero sem trailer

# Roundtrip/atribuição (conferidos por leitura nesta etapa; executados em R4)
# exportar conta/personagem -> JSON versionado -> importar -> comparar os 5 campos
# conferir COPYRIGHT.txt na raiz: fonte, licença e data por arquivo/asset

# Gates determinísticos alvo — preservados, não alterados
for s in check_sdd_completeness check_work_order_dag check_task_id_uniqueness check_converge_append_only hub_docs_gate; do python3 scripts/ci/$s.py; done
```

Nenhum desses comandos foi executado nesta etapa de escrita; toda saída permanece **[NV]** e só entra com data/hora em R4. `verify.sh` deve terminar com `verify.sh: ALL STAGES OK` e 0 estágios vermelhos (SC-001); o roundtrip deve preservar os 5 de 5 campos (SC-002); `COPYRIGHT.txt` deve estar presente com fonte/licença/data (SC-003); o gate DCO deve recusar commit sem trailer (SC-004); os 5 gates determinísticos devem permanecer PASS e inalterados (SC-005). Nenhuma medição é tratada como resultado observado até R4.
