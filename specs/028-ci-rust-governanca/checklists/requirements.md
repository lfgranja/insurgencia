# Checklist de Requisitos — 028-ci-rust-governanca

- [x] FR-001 job `rust` no CI executando `./verify.sh`
- [x] FR-002 toolchain lida de `rust-toolchain.toml`
- [x] FR-003 `cargo-nextest` instalado; fallback do script preservado
- [x] FR-004 AGENTS.md reescrito com realidade Rust e regra da janela
- [x] FR-005 cabeçalho verify.sh corrigido
- [x] FR-006 GDD:1627 com nota datada
- [x] FR-007 verify.sh + 5 gates PASS após mudança
- [x] FR-008 nenhum arquivo alheio tocado

## Revisão do checklist (obrigatória antes de tasks)
Todos os 8 itens são requisitos verificáveis por leitura/comando. Nenhum `[x]` afirma implementação — apenas que o requisito foi formulado e será verificado no converge. Nenhuma ambiguidade: toolchain, fallback e escopo de escrita estão definidos. Aprovado para tasks.
