# ADR-004 — Migrar o Núcleo de Godot/GDScript para Bevy 0.20.1 + Lightyear 0.31

- **ID**: ADR-004
- **Data**: 2026-11-20
- **Status**: Accepted
- **Versão**: 1.0.0
- **Supersede**: ADR-001 (`docs/adr/001-excecao-linguagem-gdscript.md`) na parte de linguagem/engine; o restante da ADR-001 permanece como registro histórico

## Contexto

A SE autorizou explicitamente, em 2026-11-20, a reescrita do projeto para **Bevy 0.20.1 + Lightyear 0.31**. A ADR-001 permitia reavaliação com evidência prévia + aprovação formal: ambas estão presentes. A evidência é o ciclo 020 (`docs/AVALIACAO-ENGINES-RUST.md`): 10 opções, 10 critérios, 32 linhas de evidência primária; Bevy+Lightyear marcou 77,1 e passou na Fase 0, enquanto Godot marcou 82,6. O GDD §16.2 e a ADR-001 mantinham Godot-first como vinculante; esta ADR é a aprovação formal que altera essa decisão, sem modificar o GDD.

Fato técnico verificado em 2026-11-20 via `cargo info bevy@0.20.1`: o **crate publicado** declara `rust-version: 1.97.1`, não `1.100.0` (valor lido no `main` do repositório e registrado na pesquisa). A toolchain local `stable 1.99.1` atende ao crate publicado. A divergência é registrada.

## Decisão

Adotar **Rust 1.99.1 (stable), Bevy 0.20.1 e Lightyear 0.31.0** como stack do Núcleo e do servidor, abandonando GDScript como linguagem de implementação. Os artefatos GDD (pilares, regras de ilha, condição de vitória, critérios §16.4) permanecem normativos; apenas a implementação米uda de engine/linguagem.

Regras obrigatórias da governança MDMV (skill `mdmv-rust-thin`):
- `unsafe_code = "forbid"` em todo código de produção;
- `clippy::pedantic = "deny"`, warnings fatais;
- proibido `.unwrap()`/`.expect()`; erros via `Result<T,E>`, `?`, `thiserror` em libs e `anyhow` em apps;
- `./verify.sh` (check → clippy → nextest → fmt) é o gate determinístico Rust;
- dependências com versão fixada e justificativa no commit; nunca substituição automática de pacote.

## Consequências

- A ordem de prioridade da ADR-001 para troca de engine foi satisfeita; esta ADR a encerra com efeito a partir de 2026-11-20.
- O aprendizagem de Godot e os testes GDScript viram registro histórico versionado; não são apagados do histórico do Git.
- O servo headless passa a depender de `bevy` com `default-features=false` + `ScheduleRunnerPlugin` e de `lightyear` para rede autoritativa.
- Custo assumido: reescrita de 31 arquivos `.gd`, toolchain Rust e churn de API ~3 em 3 meses do Bevy 0.x.

## Alternativas consideradas

- **Manter Godot-first** — rejeitada pela autorização explícita da SE e pela evidência do ciclo 020.
- **Bevy puro sem Lightyear** — rejeitada: C2=4\* (falta rede autoritativa), trava inviolável.
- **Fyrox** — rejeitada: C2=4\*, sem crate de rede no workspace.
- **Serviço Rust separado mantendo Godot no cliente** — rejeitada: introduz segunda linguagem sem necessidade após a autorização de migração总.

## Rollback

Reverter esta ADR e os commits subsequentes de migração devolve a ADR-001 como decisão vigente. O histórico Git preserva o código GDScript até o ponto da migração. Nenhuma baseline de governança, gate ou trabalho de terceiros é alterado pelo rollback.

## Rastreabilidade

| Referência | Uso |
|---|---|
| Ciclo 020 / `docs/AVALIACAO-ENGINES-RUST.md` | evidência prévia (77,1; Fase 0 passa) |
| `cargo info bevy@0.20.1` (20/11/2026) | MSRV real do crate publicado |
| ADR-001 §Critérios de reavaliação | autoridade para esta mudança |
| skill `mdmv-rust-thin` | regras de implementação Rust |

Estado: migração autorizada; nenhuma troca adicional de engine é permitida sem novo ADR.
