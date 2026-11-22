# Checklist de requisitos — 025-soberania-exportacao

**Data:** 2026-11-21T01:19Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação do crate `crates/exportacao`, escrita de código Rust, criação de `scripts/dco-check.sh`, materialização de `COPYRIGHT.txt`, execução de `cargo`/`verify.sh`/gates/testes/build/formatadores, criação de commit com trailer, stage, commit ou push.

- [x] Revisão de requisitos concluída sobre `.specify/feature.json`, `.specify/memory/STATE.md` (item 4 da prioridade da SE), `docs/GDD - Insurgência.md` (§14.2, §14.7, §15.5–§15.9, GDD:1395/1451/1452), spec 011, specs 021–024, ADR-002 (MIT) e ADR-004 (toolchain), `Cargo.toml` do workspace, `verify.sh`, `clippy.toml`, `scripts/ci/`, `projetos/AGENTS.md` e skill `mdmv-rust-thin`
- [x] Saída desta etapa é exclusivamente documental: `spec.md`, `plan.md`, `checklists/requirements.md` e `tasks.md`; nenhum `.rs`, `Cargo.toml` de crate, `scripts/dco-check.sh`, `COPYRIGHT.txt`, gate, baseline, `evidence.csv`, `Makefile` ou artefato R4/R7 é escrito ou alterado aqui (FR-001, FR-005, FR-007, FR-008)
- [x] US1 (P1) fixa a exportação local de conta/personagem em JSON versionado com os cinco campos `conta_id`, `personagem_id`, `criado_em`, `versao`, `rules_hash` (RULES hash), legível por ferramenta independente e sem servidor (GDD:1395, FR-002, Q1, Q3)
- [x] US2 (P1) fixa a importação com recusa de formato divergente — versão divergente, campo ausente/malformado, `rules_hash` malformado ou divergente, JSON inválido — com códigos distintos e erro legível, sem conversão silenciosa (FR-003, Q3)
- [x] US3 (P2) fixa `COPYRIGHT.txt` na raiz com fonte, licença e data por arquivo/asset, licença MIT para Núcleo/servidor headless e 0 arquivos sem licença declarada (GDD:1451, FR-005, Q4)
- [x] US4 (P2) fixa DCO em todo commit via trailer `Signed-off-by` e gate `scripts/dco-check.sh` que recusa commit sem trailer, sem GPG (GDD:1452, FR-006, FR-007, Q2)
- [x] **Sem servidor (Q1):** exportação/importação locais no Núcleo, sem rede, banco, HTTP, IAM remoto, federação, servidor comunitário, fork publicado ou chave guardada por instância; itens 5 e 6 do STATE.md ficam DEFER
- [x] **DCO, não GPG (Q2):** verificado que o ambiente não tem chave GPG nem `commit.gpgsign`/`user.signingkey`; GDD:1452 atendida como assinatura de origem via `Signed-off-by`, verificável por `scripts/dco-check.sh`; GPG fica **[NV]**/fora de escopo
- [x] **Formato versionado (Q3):** `versao` inteiro ≥ 1, `criado_em` ISO-8601 UTC explícito, `rules_hash` SHA-256 de 64 hex do conjunto de regras ativo; recusa de divergência espelha a política de *handshake* da GDD §14.2/spec 011; determinismo sem relógio de parede implícito
- [x] **MIT (Q4):** licença MIT para Núcleo e servidor headless (ADR-002, GDD:1451) com `LICENSE` e `COPYRIGHT.txt` presentes; AGPL-3.0 da GDD §15.5 é de terceiros (Lichess) e não é atribuída ao Insurgência
- [x] FR-001 exige o crate `insurgencia-exportacao` em `crates/exportacao`, `edition = "2024"`, membro do workspace, com `unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, sem `unwrap`/`expect`, `Result` + `thiserror` e nenhuma dependência de rede/servidor
- [x] FR-002 exige os exatamente cinco campos obrigatórios do JSON versionado, com `rules_hash` (RULES hash) de 64 hex, legibilidade por ferramenta independente e determinismo, sem chave de instância
- [x] FR-003 exige recusa de formato divergente com códigos distintos, na ordem JSON inválido → versão → campo → comprimento do `rules_hash` → divergência do `rules_hash`, sem conversão silenciosa nem estado pronto
- [x] FR-004 exige roundtrip exportar → importar com 5 de 5 campos preservados, 0 perdas, 0 divergências e saída byte-idêntica na repetição, antes do lançamento do MVP
- [x] FR-005 exige `COPYRIGHT.txt` com fonte/licença/data, MIT, 0 arquivos sem licença declarada e validação antes de empacotar Módulos (GDD §14.7)
- [x] FR-006 exige trailer `Signed-off-by: Nome <email>` em todo commit como DCO, sem chave GPG, segredo ou servidor de chaves
- [x] FR-007 exige `scripts/dco-check.sh` com exit 0 para trailer bem formado e exit não zero com mensagem legível para trailer ausente/malformado, integrado ao pre-commit sem alterar os quatro estágios de `verify.sh`
- [x] FR-008 exige `verify.sh` com quatro estágios na ordem terminando em `verify.sh: ALL STAGES OK` e os cinco gates determinísticos alvo PASS e inalterados
- [x] Critérios de sucesso mensuráveis e binários: 4 de 4 estágios `verify.sh` (SC-001); roundtrip 1 de 1 com 5 de 5 campos (SC-002); 1 de 1 `COPYRIGHT.txt` com 100% de atribuição (SC-003); 100% de commits com DCO e gate recusando ausência (SC-004); 5 de 5 gates determinísticos PASS e inalterados (SC-005)
- [x] Nenhum marcador `[NEEDS CLARIFICATION]` em aberto: as quatro ambiguidades foram resolvidas em `## Clarifications` com fonte local (GDD §15.5/§15.9/§14.2, ADR-002, STATE.md, verificação do ambiente)
- [x] Plano de escrita fixa criação de `crates/exportacao`, `scripts/dco-check.sh` e `COPYRIGHT.txt`, com FRs/SCs mapeados, nenhum código Rust nesta etapa e ordem de R4 antes de R7
- [x] Tasks T001..T010 com marcas `[X]` nas fases Implementação, Verificação e Convergência, cobrindo todas as histórias, requisitos e critérios de sucesso
- [x] Edge cases cobertos: versão divergente, `rules_hash` malformado e divergente, `criado_em` ausente/fora de ISO-8601, JSON inválido, exportação repetida, commit sem trailer e sem GPG, AGPL de terceiros, servidores comunitários e alegação de execução
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado, nenhum comando executado, nenhum commit criado e nenhuma alegação de execução observada — todo resultado permanece **[NV]** até registro datado em R4
