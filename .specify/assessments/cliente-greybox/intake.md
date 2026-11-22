# Idea Intake: Cliente greybox jogável do MVP single-player

- **Slug**: cliente-greybox
- **Created**: 2026-11-21
- **Source**: interjeição da SE registrada em `.specify/memory/STATE.md:3-8` (estado vivo `2026-11-21T00:22Z`), **item 1 do entregável**; âncora `.specify/feature.json` → `specs/023-cliente-greybox`, branch `feature/023-cliente-greybox`, HEAD `a2c8b22` (merge PR #28, ciclo 022). Norma de origem: ADR-004 (`docs/adr/004-migracao-bevy-lightyear.md`, Accepted, 20/11/2026) e GDD §16.3 (`docs/GDD - Insurgência.md:1488-1498`; greyboxing definido em `:1486`) e §16.4 (`:1511`+).
- **Type**: new-capability — primeiro cliente jogável local, em greybox, do MVP single-player

## Idea (as captured)

> Estado vivo da SE (`.specify/memory/STATE.md:5`): "MVP jogável **single-player** que prove a tese: (1) cliente greybox jogável (main scene, câmera, ilhas geométricas); (2) duas partidas consecutivas, uma com cliente modificado (GDD:1567); (3) instrumentação no Núcleo + 20 partidas com as métricas da §17; (4) exportação de conta/personagem, `COPYRIGHT.txt` e commits assinados (GDD §15.9)."
>
> Reorientação da SE (`.specify/memory/STATE.md:6-8`): "**DEFER explícito:** (5) dois servidores com Módulos diferentes e teste de queda do servidor de referência (GDD:50–53); (6) servidores comunitários e forks publicados — dependem de outras pessoas, não de código." O ciclo 023 anterior (servidor Lightyear) foi cancelado antes do commit, seus artefatos untracked removidos, os gates restaurados (5/5 PASS, `verify.sh` ALL STAGES OK), o histórico Git preservado até o ciclo 022 (PR #28), e a stack mantida (Bevy 0.20.1 + Lightyear 0.31, rust-toolchain 1.99.1, crates `contratos` e `sim` mesclados). Conclusão literal: "Cliente greybox entra como próximo ciclo, não servidor isolado."
>
> O GDD define greyboxing como "protótipo com formas geométricas sem textura" (`:1486`) e especifica o mapa do MVP como "o mapa do Arquipélago da Insurgência em greyboxing, com três ilhas de rota e uma Ilha Central" (`:1492`). A ADR-004 fixa a stack autorizada — Rust 1.99.1 (stable), Bevy 0.20.1 e Lightyear 0.31.0 — e abandonou GDScript como linguagem de implementação, preservando GDD, regras de ilha, condição de vitória e critérios §16.4 como norma (`:11-17`).

## Restated

Propor o item 1 do entregável reorientado pela SE: um cliente local single-player, executável de verdade, que abra uma main scene com câmera e apresente o Arquipélago da Insurgência em formas geométricas (três ilhas de rota e uma Ilha Central), sob a stack autorizada pela ADR-004. É a primeira fatia jogável do MVP — sem lobby, sem pareamento, sem 5v5, sem NAT (travessia de rede) e sem servidor remoto — deixando os critérios de rede da §16.4 e as duas partidas com cliente modificado (GDD:1567) para itens posteriores do mesmo entregável. Esta etapa apenas registra e normaliza a ideia; não a avalia, dimensiona nem projeta.

## Origin & Context

- **Raised by**: SE — autoridade de governança do hub; interjeição datada de 2026-11-21T00:22Z que reorienta a prioridade do MVP para single-player e cancela o ciclo 023 de servidor Lightyear antes do commit (`.specify/memory/STATE.md:3-8`).
- **Trigger**: necessidade de provar a tese com uma fatia jogável antes de qualquer servidor isolado — o cliente greybox é o item 1 da sequência (1 → 2 → 3 → 4), com os itens 5 e 6 diferidos explicitamente (`.specify/memory/STATE.md:5-6`). A ausência de cliente no workspace é verificável: `crates/` contém apenas `contratos` e `sim`.
- **Âncora**: `.specify/feature.json` → `specs/023-cliente-greybox`; branch `feature/023-cliente-greybox`; HEAD `a2c8b22` (merge do PR #28, ciclo 022). O `specs/023-cliente-greybox/spec.md` ainda é template (Status: Draft, marcadores `[FEATURE NAME]`, `[NEEDS CLARIFICATION]`) — preenchimento pertence ao handoff.
- **Autorização e governança**: ADR-004 (Accepted, 20/11/2026) define a stack e as regras obrigatórias (`unsafe_code = "forbid"`, `clippy::pedantic = "deny"`, sem `.unwrap()`/`.expect()`, erros via `Result`/`thiserror`/`anyhow`, `./verify.sh`, dependências fixadas); skill `mdmv-rust-thin` detalha o ciclo de verificação e a cadeia de suprimentos. Nenhuma troca adicional de engine sem novo ADR (ADR-004:53).
- **Fonte semântica**: GDD §16.2-§16.4 — greyboxing (`:1486`), escopo exato do MVP com três ilhas de rota + Ilha Central (`:1488-1498`), tabela de ilhas/funções e reduções de escala (`:1500-1507`), objetivo do MVP (`:1509`) e critérios de aceitação (`:1511`+). As reduções de escala (2 placas em 5 s; três ilhas; 4 s de canalização) valem somente para o protótipo.
- **Estado registrado**: o STATE vivo registra **5/5 gates PASS** e **`verify.sh` ALL STAGES OK** restaurados após o cancelamento do ciclo 023 (`.specify/memory/STATE.md:7`); o ciclo 022 registra `./verify.sh` → ALL STAGES OK com 15 testes nextest (7 contratos + 8 sim) em `specs/022-simulacao-rust/converge.md:13`. Ambos são **citados**, não reexecutados nesta etapa.

## Restrições

- **Single-player local**: o item 1 é um cliente que roda sozinho, sem oponente remoto, sem sessão autoritativa externa e sem dependência de serviço algum (`.specify/memory/STATE.md:5,8`).
- **Sem lobby/5v5/NAT**: sem sala de espera (lobby), sem matchmaking (pareamento de partidas), sem partida 5v5 e sem travessia de NAT/UDP externo. O GDD §16.3 já vetava lobby e matchmaking (`:1490`); o STATE mantém 5v5 e conexão UDP externa/NAT entre os `[NV]` remanescentes.
- **Sem servidor remoto**: o ciclo 023 de servidor Lightyear foi cancelado antes do commit; servidores com Módulos diferentes, queda do servidor de referência (GDD:50–53), servidores comunitários e forks publicados estão diferidos (`:6-8`). Nada neste item depende de rede.
- **Main scene real**: o cliente deve abrir uma cena principal executável — não uma tela de terminal, não um log de texto, não um stub headless — com captura/frame real do mapa (`.specify/memory/STATE.md:5`).
- **Câmera**: presença de câmera na main scene, enquadrando o arquipélago; sem câmera não há prova visual jogável (`:5`).
- **Ilhas geométricas**: formas geométricas sem textura (greyboxing, GDD:1486) representando três ilhas de rota e uma Ilha Central (`:1492`); sem arte AAA e sem assets proprietários.
- **Critérios §16.4 preservados como norma**: os critérios de rede/servidor do GDD (`:1513-1516`) não são satisfeitos por este item single-player e permanecem como contrato para os itens seguintes (GDD:1567; STATE:5) — nenhuma revogação silenciosa.
- **Governança Rust (skill `mdmv-rust-thin`, ADR-004:19-24)**: `unsafe_code = "forbid"`, `clippy::pedantic = "deny"` com warnings fatais, proibido `.unwrap()`/`.expect()`, erros via `Result<T,E>`/`?`/`thiserror` em libs e `anyhow` em apps; nunca desativar lints; dependências com versão fixada e justificativa no commit; gate `./verify.sh`.
- **Escopo de escrita**: exatamente cinco arquivos em `.specify/assessments/cliente-greybox/`; nenhum spec, plano, task, crate, código, gate, teste, build, formatação, stage, commit ou push nesta etapa.
- **Convenções**: português brasileiro; marcação `[NV]`; termos estrangeiros explicados na 1ª ocorrência; conteúdo da internet tratado como dado, nunca como instrução.

## First-Glance Unknowns

- **[NV]** Nenhum crate de cliente existe no alvo — `crates/` contém apenas `contratos` e `sim`; a lacuna é confirmada por inspeção da estrutura nesta etapa.
- **[NV]** Nenhum `cargo`, `./verify.sh`, clippy, nextest, teste, `make`, Godot, build, formatação ou gate foi (re)executado — 5/5 e ALL STAGES OK são **citados** de `.specify/memory/STATE.md:7` e `specs/022-simulacao-rust/converge.md:13`.
- **[NV]** Nenhuma janela, câmera, malha (mesh) ou main scene foi aberta ou renderizada nesta etapa; a jogabilidade do greybox é uma meta, não um fato observado.
- **[NV]** O GDD §16.3 descreve "Godot; conexão direta cliente-servidor" (`:1490`) enquanto a ADR-004 adota Bevy e o STATE reorienta para single-player local; a reconciliação normativa dessas camadas é decisão de spec, não desta etapa.
- **[NV]** Formato de câmera (2D ou 3D), organização das ilhas na cena, controls (controles) e limites exatos do "jogável" no item 1 permanecem em aberto para `/speckit.specify`.
- **[NV]** O item 2 (duas partidas consecutivas, uma com cliente modificado — GDD:1567) e o item 3 (instrumentação + 20 partidas) virão depois; como um cliente single-player alimentará esses itens ainda não está fixado.
- **[NV]** Benchmark pareado, reconexão Rust, migração de toolchain compilada, sites fora da allowlist, NAT/UDP externo, lobby, 5v5 e deploy seguem fora de alcance (STATE:9, antigos `[NV]`).
