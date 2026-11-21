# Decision — Porte para Rust do contrato de manifesto de compatibilidade

- **Slug**: manifesto-compatibilidade-rust
- **Decided**: 2026-11-21T21:24:08Z
- **Veredito**: **go** · **R1**: `go`

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Lacuna verificável: `crates/contratos/src/lib.rs` expõe constantes de recusa mas nenhum validador de manifesto e nenhum `REGRAS_AUSENTES`, enquanto 011 + `manifesto.json` + GDScript já definem o contrato (GDD §14.2). |
| Força da evidência | strong | Âncora primária lida diretamente: `manifesto.json:1-8`, `specs/011` FR-001..FR-006, `manifesto.gd:5-42`, `tests/test_manifesto.gd:16-46`, `lib.rs:1-31`, GDD §1/§14.2. Gates 5/5 PASS em `21:24:08Z`; `./verify.sh` **[NV]** por contenção de `cargo`. |
| Valor vs. inação | strong | Evita segundo esquema e devolve ao Núcleo Rust a mesma recusa legível do handshake, com 6 campos e 5 códigos idênticos; sem dependências, sem I/O e sem plataforma nova. |
| Viabilidade / appetite | strong | Porte literal em 3 arquivos de código + 13 artefatos SDD, com fronteiras disjuntas e dono único por arquivo; nenhum build/gate/commit autorizado neste ciclo. |
| Alinhamento estratégico | strong | GDD §1 (soberania das regras) e §14.2 (versão/módulo/hash com constantes separadas); Constituição II (sem I/O na validação); SDD ordenado R1 → R4 → R7. |
| Postura de risco | strong | Risco baixo e contido: factory `b1deab1` somente fonte, `docs/GDD*`/baselines/gates/CI/194 paths alheios intactos, `converge.md` append-only, rollback documentado e nunca executado; `[NV]` preservado para `./verify.sh`. |

## Scorecard
6/6 critérios **strong** → **go**. Nenhum critério `weak`; nenhum marcador `[NEEDS CLARIFICATION]` em aberto.

## Racional
Não há decisão de design a tomar: o contrato é o de 011. O porte apenas o expressa em Rust, com a única adição literal obrigatória (`REGRAS_AUSENTES`) já prevista no GDScript. A separação de fronteiras impede que dois agentes sobrescrevam os mesmos arquivos; a Fronteira A produz documentação e a Fronteira B, código, sem builds ou gates.

## Condição/responsável
Fronteira A (13 artefatos SDD) e Fronteira B (`manifesto.rs`, `lib.rs`, `tests/manifesto.rs`) executam em paralelo, sem commit/gate/build/PR. `./verify.sh` permanece **[NV]** até saída observada; nenhuma afirmação de verde além dos 5 gates reportados em `21:24:08Z`. Factory `b1deab1` é fonte, não autoridade; nada transplantado.

## Handoff
Problema acima; abordagem = Opção 1; escopo = 13 artefatos na Fronteira A + 3 arquivos na Fronteira B; fora de escopo = segundo esquema, I/O na validação, dependências novas, factory, `docs/GDD*`, baselines, gates/CI, 194 paths untracked alheios, commits/builds/gates/PRs, janelas e credenciais; métricas = 6 campos e 5 códigos idênticos à âncora, ordem 011 preservada, 7 testes espelhados, R4 aprovado antes de R7 `1.0.0`, `[NV]` mantido onde não há saída.
