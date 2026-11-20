# Idea Intake: Substituir marcadores `[NV]` de pesquisa por fatos verificados ao vivo

- **Slug**: verifica-nv-externa
- **Created**: 2026-11-19
- **Source**: pedido do mantenedor (texto fornecido nesta sessão), com evidência já coletada em `specs/004-verifica-nv-externa/research-evidence/godot-marca.md`, `stride-fyrox.md`, `lightyear-bevy.md` e decisão de governança em `docs/governance/nv-resolution-2026-11-19.md`
- **Type**: improvement

## Idea (as captured)

> "Complete assignment thoroughly: Feature 004 — verifica-nv-externa. Execute EM ORDEM: intake → research → define → shape → decide(go) → specify → clarify → plan → checklist → tasks. Objeto: substituir marcadores `[NV]` de pesquisa por fatos verificados ao vivo, COM RESSALVAS, nos documentos integrados: `docs/AVALIACAO-COMPARATIVA-ENGINES.md`, `docs/avaliacao-engines/**`, `docs/AVALIACAO-STACK-MDMV.md`, `docs/avaliacao-stack/**`. Escopo: marca/licença Godot; Stride #1666/manutenção; Fyrox/Urho3D/Dviglo/Defold; Lightyear↔Bevy/book/cadência. Regras: 0 afirmação de benchmark/execução de engine; preservar `[NV]` de classe C; cada substituição com título+URL+data e 2 fontes; docs de snapshot histórico ganham nota de superação, não reescrita. FRs numerados, SCs mensuráveis (ex.: N marcadores resolvidos, 0 fatos sem 2 fontes), checklist `[ ]`, tasks T### sem portarias posteriores. Aceitação: todos os artefatos no diretório, sem seção vazia, sem TBD."

## Restated

Substituir, nos documentos de avaliação já integrados do hub, os rótulos de "não verificado" que possuem evidência primária coletada ao vivo em 19/11/2026, preservando as ressalvas de escopo de cada fato e sem transformar evidência documental em alegação de execução ou benchmark de engine. A mudança é de precisão e rastreabilidade documental: cada fato passa a citar título, URL e data com pelo menos duas fontes, enquanto marcadores metodológicos legítimos permanecem intactos.

## Origin & Context

- **Raised by**: mantenedor `lfgranja`, pedido explícito nesta sessão (2026-11-19).
- **Trigger**: resolução de `[NV]` de 19/11/2026 (`docs/governance/nv-resolution-2026-11-19.md`), que autorizou a conversão dos 11 marcadores de classe A em citações locais e determinou a preservação dos 92 de classe B e 157 de classe C; três arquivos de evidência ao vivo foram produzidos em sequência e ainda não foram aplicados aos documentos integrados.
- **Contexto de localização**: os artefatos de avaliação desta feature ficam colocalizados em `specs/004-verifica-nv-externa/` (o diretório da spec), conforme o escopo desta atribuição. Nenhum arquivo fora desse diretório é escrito por esta feature de planejamento.

## First-Glance Unknowns

- Resolvido: existem 12 fatos resolvíveis distintos (NV-A01–NV-A12), inventariados em `research.md` (§Síntese dos 12 fatos) e mapeados para arquivo/linha de destino no inventário de FR-013; as três ambiguidades restantes foram resolvidas em `spec.md` / `## Clarifications`.
- Resolvido: snapshot histórico = `docs/avaliacao-engines/pesquisa/**` (corpo datado preservado, nota de superação append-only); documento integrado = `docs/AVALIACAO-COMPARATIVA-ENGINES.md`, `docs/avaliacao-engines/` exceto `pesquisa/`, `docs/AVALIACAO-STACK-MDMV.md` e `docs/avaliacao-stack/**` (substituição no corpo), conforme `docs/governance/nv-resolution-2026-11-19.md:20-22`.
- Resolvido: a afirmação "sem correção na 4.4" da issue Stride #1666 é redigida como "issue permanece `open` após 4.4.3 e nenhum fix foi registrado na issue ou nas notas de release", com ressalva explícita de que não houve build/import empírico — nunca como falha observada (`research-evidence/stride-fyrox.md:7-13`).
- Resolvido: "4 versões por ciclo do Bevy" é observação do ciclo 0.19 (0.28.0, 0.29.0, 0.30.0, 0.30.1), não lei geral; a tabela upstream conta séries menores (3), a UI de Releases marca 0.30.0 como "Latest" embora o crates.io já tenha 0.31.0 (`research-evidence/lightyear-bevy.md:29-43`).
- Resolvido: a substituição é estritamente factual/citativa — FR-009 e SC-007 proíbem recalcular ou reescrever notas, vetores C1–C10, índices ponderados, scores ou recomendações; a divergência de versão do Fyrox é correção de fato, não de nota.
