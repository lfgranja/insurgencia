# Decision — Gate CLI determinístico de manifesto de compatibilidade

- **Slug**: gate-manifesto-cli
- **Decided**: 2026-11-22T16:54:17Z
- **Veredito**: **go** · **R1**: `go`

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | Contrato Rust correto e testado, porém sem superfície executável; cada chamador teria que recalcular hash e reinterpretar literais do handshake (GDD §14.2). |
| Força da evidência | strong | API lida diretamente em `manifesto.rs`/`lib.rs`; deps em `crates/sim/Cargo.toml`; `./verify.sh` `ALL STAGES OK`/41 de 41 em `2026-11-22T16:54:17Z`; 5 gates PASS; CI run `44952738240` `success` em `7329c53`. |
| Valor vs. inação | strong | Transforma o contrato em gate headless determinístico com saída e códigos estáveis, sem criar segunda fonte de verdade e sem novas dependências. |
| Viabilidade / appetite | strong | Um arquivo de código novo, adaptação direta de `demo_deterministica.rs`, roteiro de 8 casos; nenhuma mudança em biblioteca, `Cargo.toml` ou CI. |
| Alinhamento estratégico | strong | GDD §1 (soberania das regras) e §14.2 (versão/módulo/hash com recusa legível); Constituição II (I/O apenas no adaptador); SDD ordenado R1 → R4 → R7. |
| Postura de risco | strong | Fronteiras disjuntas e rollback trivial; factory `b1deab1` somente fonte; baselines, `docs/GDD*`, CI e 193 paths untracked alheios intactos; nenhum build/gate/commit e execução do roteiro marcada **[NV]**. |

## Scorecard
6/6 critérios **strong** → **go**. Nenhum critério `weak`; nenhum marcador `[NEEDS CLARIFICATION]` em aberto.

## Racional
Não há decisão de design aberta: a autoridade é `conferir` (011/033). O único ponto ambíguo — como devolver `REGRAS_AUSENTES` se a leitura do arquivo falha — resolve-se no adaptador: `NotFound` nas regras equivale a "arquivo de regras ausente" e é passado como `None`; qualquer outra falha de leitura é operacional (exit 2). A ordem do contrato é preservada porque a tradução acontece depois da chamada.

## Condição/responsável
Fronteira única deste agente: 5 artefatos de assess, 6 artefatos em `specs/034-gate-manifesto-cli/`, `validate.md`, `release.md`, `test-roadmap.md` e `crates/sim/examples/verificar_manifesto.rs`. Orquestrador: executar o roteiro, atualizar o baseline de converge (arquivo novo não baselined) e rodar gates/CI; nada disso é feito aqui. `./verify.sh` e o roteiro permanecem **[NV]** até execução observada. Factory `b1deab1` é fonte, não autoridade; nada transplantado.

## Handoff
Problema acima; abordagem = Opção 1; escopo = 14 artefatos + 1 arquivo de exemplo; fora de escopo = duplicar validação, I/O na biblioteca, dependências novas, `Cargo.toml`, factory, `docs/GDD*`, baselines, `.gitignore`, ledger, `STATE.md`, `PROGRESSO.md`, CI, 193 paths untracked alheios, janelas, credenciais, builds/gates/commits/PRs. Métricas: 8 casos do roteiro com exit codes e literais esperados, 3 execuções idênticas por `diff`, nenhuma dependência nova, R4 aprovado antes de R7 `1.0.0`, `[NV]` preservado onde não há execução.
