# Concept — Como materializar a migração autorizada para Rust/Bevy

**Data:** 2026-11-20

## Contexto

A ADR-004 autorizou a reescrita do Núcleo e do servidor para Rust 1.99.1 + Bevy 0.20.1 + Lightyear 0.31.0, com evidência prévia do ciclo 020 (77,1; Fase 0 passa) e aprovação formal da SE (`docs/adr/004-migracao-bevy-lightyear.md:11-17`). O alvo não tem nenhum código Rust (problema: impacto 5, confiança 4). A mudança é de engine/linguagem, não de regras: GDD, critérios §16.4, ADR-002 (licença MIT-only), ADR-003 (IAM) e o histórico GDScript permanecem. A governança `mdmv-rust-thin` impõe `unsafe forbid`, `clippy pedantic deny`, sem `unwrap`/`expect`, `Result` + `thiserror`/`anyhow`, `verify.sh` e dependências fixadas. A estrutura alvo declarada é workspace Cargo com `crates/` (Rust-only), módulos `contratos`, `dominio/sim`, `servidor`, `aplicacao`. Nenhuma opção abaixo executa build, gate, teste, formatação, stage/commit/push nem escreve fora dos cinco artefatos de assessment.

## Opções consideradas

1. **Workspace Rust com `crates/` — escolhida** — criar, em ciclo próprio e após spec/plan/tasks, um workspace Cargo Rust-only com `crates/` separando `contratos` (tipos/protocolo versionado), `dominio/sim` (regras de ilha e simulação), `servidor` (autoridade com Lightyear) e `aplicacao` (binários/headless via `ScheduleRunnerPlugin`, `default-features = false`). Versões fixadas (`bevy 0.20.1`, `lightyear 0.31.0`), MSRV compatível com o `rust-version: 1.97.1` do crate publicado registrado via `cargo info`, governança aplicada desde o primeiro arquivo e `./verify.sh` como gate. É a única opção que materializa integralmente a ADR-004 sem segunda linguagem no caminho crítico e com limites de módulo auditáveis; a reescrita dos 31 arquivos `.gd` vira trabalho rastreável por módulo, com o GDD como contrato de equivalência funcional.
2. **Wrapper FFI sobre GDScript (rejeitada)** — manter a lógica em GDScript e expor uma camada Rust por FFI (*interface de função estrangeira*) para "cumprir" a migração. Rejeitada porque contradiz a ADR-004, que determina **abandonar GDScript como linguagem de implementação** e adotar Rust como stack do Núcleo e do servidor (`:17`); além disso, FFI em código de produção colide com `unsafe_code = "forbid"` da governança e manteria duas linguagens e dois ciclos de teste, exatamente o risco que a ADR-004 descarta para o "serviço Rust separado mantendo Godot no cliente" (`:38`). Seria migração cosmética: o histórico GDScript continuaria sendo o código vivo, não registro.
3. **Manter Godot/GDScript como está (rejeitada por autorização)** — não migrar e seguir com Godot-first sob a ADR-001. Rejeitada de plano: a SE autorizou explicitamente a reescrita em 20/11/2026 e a ADR-004 (Accepted) **supera a ADR-001 na parte de linguagem/engine**, com evidência prévia e aprovação formal já satisfeitas (`:7,11,28`); a ADR-004 declara ainda que nenhuma troca adicional de engine é permitida sem novo ADR (`:53`). Manter o status quo exigiria reverter uma decisão aceita, contrariando a autorização vigente.

## Recomendação

**Opção 1.** É a única alinhada à ADR-004 e à governança: substitui a linguagem de implementação por Rust, preserva o GDD como contrato, separa responsabilidades em crates e torna o gate `verify.sh` aplicável de forma determinística desde o início. A Opção 2 é rejeitada por ser migração apenas nominal (GDScript continuaria vivo, com FFI e risco `unsafe`); a Opção 3, por violar uma autorização já emitida. Nenhuma opção autoriza build, benchmark, gate, teste, `make`, Godot, spec, plano, tasks, stage/commit/push ou escrita fora dos cinco artefatos de assessment nesta etapa.

## Consequências

- A escrita fica restrita a `.specify/assessments/migracao-bevy/{intake,research,problem,concept,decision}.md`; spec, plano, checklist e tasks pertencem ao handoff `/speckit.specify` em ciclo subsequente, sem pular portarias (constituição III).
- O histórico GDScript e os testes associados permanecem versionados; rollback da ADR-004 devolve a ADR-001 como decisão vigente sem alterar baseline de governança, gates ou trabalho de terceiros (ADR-004:42). Nenhum force-push, reescrita ou deleção de branch alheia.
- A divergência de MSRV (`1.97.1` no crate publicado vs. `1.100.0` no `main`) e a toolchain local `1.99.1` seguem como registro da ADR-004, não como compilação verificada **[NV]**; spikes A/B/C do ciclo 020 (benchmark pareado, reconexão/handshake, custo de migração) permanecem sem execução.
- Módulos `contratos`, `dominio/sim`, `servidor` e `aplicacao` são limites de responsabilidade, não crates já existentes; nenhum código, `Cargo.toml` ou dependência é materializado aqui.
- Norma externa intocada: GDD e critérios §16.4, ADR-002, ADR-003, baseline de converge, `Makefile`, `evidence.csv` e portarias R4/R7. Custo de reescrita (31 arquivos `.gd`), churn de API 0.x e equivalência funcional são riscos registrados, não afirmações de sucesso.
