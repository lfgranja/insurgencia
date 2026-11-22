# Idea Intake: Instrumentação no Núcleo e corredor de 20 partidas (itens 2 e 3 da SE)

- **Slug**: instrumentacao-metricas
- **Created**: 2026-11-21T01:03Z
- **Source**: interjeição da SE registrada em `.specify/memory/STATE.md:3-8` (estado vivo 2026-11-21T00:22Z), itens 2 e 3 em `:5`; norma em `docs/GDD - Insurgência.md:1567,1605-1622`
- **Type**: instrumentação e validação de tese; sem nova regra de jogo

## Idea (as captured)

> "**Alvo agora:** MVP jogável **single-player** que prove a tese: (1) cliente greybox jogável (main scene, câmera, ilhas geométricas); (2) duas partidas consecutivas, uma com cliente modificado (GDD:1567); (3) instrumentação no Núcleo + 20 partidas com as métricas da §17; (4) exportação de conta/personagem, `COPYRIGHT.txt` e commits assinados (GDD §15.9)." — `.specify/memory/STATE.md:5`

> "O MVP está fechado quando todos os critérios de aceitação acima passarem em duas partidas consecutivas, uma delas com cliente modificado." — `docs/GDD - Insurgência.md:1567`

> "A instrumentação registra eventos no Núcleo, nunca apenas no cliente. Toda meta abaixo é [HIPÓTESE] e deve ser recalibrada após 20 partidas instrumentadas. [HIPÓTESE — amostra mínima para separar sorte de padrão.]" — `docs/GDD - Insurgência.md:1607`

## Restated

Fechar os itens 2 e 3 do entregável redefinido pela SE: (a) executar **duas partidas consecutivas, uma com cliente modificado**, como condição literal de fechamento do MVP (GDD:1567); (b) **instrumentar o Núcleo** — nunca apenas o cliente — com capturas, tentativas de sincronia e divergências cliente-servidor (GDD:1607,1620), e conduzir **20 partidas** para calibrar uma única vez as seis métricas da §17 (GDD:1605-1622), mantendo as metas marcadas como **[HIPÓTESE]**. Hoje não existe nenhuma instrumentação nem registro de partidas: `crates/sim` modela o relógio lógico, a validação e o `hash_estado`, e `crates/cliente` mantém um espelho local da simulação, mas nenhum dos dois emite ou persiste eventos, e nenhum corredor de partidas existe.

## Origin & Context

- **Raised by**: SE, em 2026-11-21T00:22Z, ao redefinir a prioridade para um MVP jogável single-player (`.specify/memory/STATE.md:3-8`). Os itens 2 e 3 aparecem literalmente em `:5`, com as normas de origem em GDD:1567 e GDD:1605-1622.
- **Trigger**: o item 1 (cliente greybox) já foi entregue — HEAD `92bf8fb` (merge do PR #29, ciclo 023), com R4 aprovado às 2026-11-21T01:01Z; a janela passou a iniciar minimizada (`crates/cliente/src/main.rs:53`; `.specify/releases/cliente-greybox/release.md:10`) e o release registra explicitamente "Servidor/rede, duas partidas consecutivas, instrumentação/métricas" como **não incluído** (`release.md`: "Não incluído"). A inspeção visual do greybox permanece **[NV]** (`.specify/validations/cliente-greybox/validate.md`, Ressalva).
- **DEFER explícito**: itens 5 (dois servidores com Módulos diferentes e teste de queda do servidor de referência, GDD:50–53) e 6 (servidores comunitários e forks publicados) dependem de outras pessoas, não de código (`.specify/memory/STATE.md:6`); o item 4 (exportação/`COPYRIGHT.txt`/commits assinados) vem depois dos itens 2 e 3 (`:5`).
- **Âncora**: `.specify/feature.json` → `specs/024-instrumentacao-metricas`; branch `feature/024-instrumentacao-metricas`; HEAD `92bf8fb`. O `specs/024-instrumentacao-metricas/spec.md` ainda é template Draft, sem requisitos, entidades ou critérios preenchidos.
- **Escopo vivo**: single-player, local, sem servidor de jogo, sem lobby, sem partidas 5v5 e sem NAT/travessia de rede (`.specify/memory/STATE.md:5,8`; `specs/023-cliente-greybox/spec.md:117-121`).

## Restrições

- **Sem rede externa**: nenhum servidor remoto, UDP externo, `lightyear`, lobby, matchmaking (pareamento), 5v5, publicação ou serviço de terceiros; todo registro permanece local ao alvo. As métricas que dependem de servidores comunitários e módulos/forks publicados (GDD:1614-1615) pertencem ao DEFER dos itens 5 e 6 (STATE:6) e não podem ser medida aqui sem inventar dado.
- **Janela minimizada**: o cliente já inicia com `Window::set_minimized(true)` (`crates/cliente/src/main.rs:53`; `validate.md`/`release.md` do ciclo 023); o corredor de partidas não deve abrir janelas visíveis, trocar esse padrão nem depender de captura de tela — a inspeção visual do greybox é **[NV]** por falha de captura no ambiente Wayland/KDE.
- **Single-player**: um jogador local, teclado e estado de sessão local; nenhum par remoto, nenhuma autoridade distribuída e nenhum cliente externo nesta etapa (STATE:5,8).
- **Fonte única no Núcleo**: GDD:1607 exige eventos registrados no Núcleo "nunca apenas no cliente"; GDD:1620 exige que o Núcleo grave capturas, tentativas de sincronia e divergências cliente-servidor antes da Fase 4. O cliente modificado de GDD:1567 é justamente a fonte não confiável — não pode ser o único produtor do registro.
- **Corredor e calibragem**: 20 partidas instrumentadas (GDD:1607), metas revisadas **uma única vez** e ainda marcadas **[HIPÓTESE]** (GDD:1622), com definição operacional e fonte de dados declaradas para toda métrica que entrar no painel (GDD:1619).
- **Governança Rust** (skill `mdmv-rust-thin`; ADR-004): `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, zero `.unwrap()`/`.expect()`, erros via `Result` + `thiserror`/`anyhow`, versões fixadas sem `*` e sem substituição automática de pacote; gate `verify.sh` (check → clippy → nextest → fmt). Nenhum gate, lint, baseline ou `evidence.csv` é alterado para facilitar a implementação.
- **Escopo de escrita desta etapa**: exatamente os cinco arquivos em `.specify/assessments/instrumentacao-metricas/`; nenhum spec, plano, tasks, código, gate, teste, build, formatação, stage, commit ou push. Convenções: pt-BR, marcador **[NV]** para todo estado não observado, termos estrangeiros explicados na primeira ocorrência.

## First-Glance Unknowns

- [NV] Nenhuma instrumentação ou registro de partida existe hoje; formato de evento/esquema de registro e ponto de emissão no Núcleo são decisão de especificação, não fato observado.
- [NV] Definição operacional de "cliente modificado" sem rede (binário com *flag*? parâmetro de execução? código alterado?) não está fixada em STATE:5 nem no GDD:1567.
- [NV] Nenhuma partida foi executada nesta etapa; as 20 partidas e as duas consecutivas não têm saída, data/hora ou evidência colada — todo resultado permanece **[NV]** até R4.
- [NV] Das seis métricas da §17, quais são realmente calculáveis em single-player local (tempo até captura, sincronia, latência local, conclusão) e quais dependem do DEFER dos itens 5/6 (servidores ativos, módulos/forks) — reconciliação a registrar na spec.
- [NV] O `specs/024-instrumentacao-metricas/spec.md` é template Draft; nomes de tipos, fronteira de crate e API pública não são evidência atual.
