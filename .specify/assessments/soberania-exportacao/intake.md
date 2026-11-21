# Idea Intake: Soberania e exportação de conta/personagem (item 4 da SE)

- **Slug**: soberania-exportacao
- **Created**: 2026-11-21T01:19Z
- **Source**: interjeição da SE registrada em `.specify/memory/STATE.md:3-8` (estado vivo 2026-11-21T00:22Z), item 4 em `:5`; norma em `docs/GDD - Insurgência.md:1441-1466` (§15.9 e critérios consolidados do MVP) e `:1567`
- **Type**: soberania, licenciamento e portabilidade de dados; sem nova regra de jogo

## Idea (as captured)

> "**Alvo agora:** MVP jogável **single-player** que prove a tese: (1) cliente greybox jogável (main scene, câmera, ilhas geométricas); (2) duas partidas consecutivas, uma com cliente modificado (GDD:1567); (3) instrumentação no Núcleo + 20 partidas com as métricas da §17; (4) exportação de conta/personagem, `COPYRIGHT.txt` e commits assinados (GDD §15.9)." — `.specify/memory/STATE.md:5`

> "Licença MIT [...] para o **Núcleo** e o servidor headless (ADR-002) | Sim | `LICENSE` e `COPYRIGHT.txt` presentes; nenhum arquivo sem licença declarada" e "Contrato de contribuição (DCO — *Developer Certificate of Origin*, certificado de origem do desenvolvedor; ou CLA — Contrato de Licença de Contribuição) | Sim | Todo *commit* assinado; contribuição sem assinatura é recusada". — `docs/GDD - Insurgência.md:1451-1452`

> "Mesmo no MVP, governança justa depende de portabilidade e direito de saída — por isso exportação entra cedo e fundação pode esperar." — `docs/GDD - Insurgência.md:1459`

> "Conta, personagem e histórico são exportáveis antes do lançamento do MVP. Toda contribuição tem assinatura de origem e entrada no arquivo de atribuição." — `docs/GDD - Insurgência.md:1464-1465`

## Restated

Fechar o item 4 do entregável redefinido pela SE em três frentes, todas normativas no GDD §15.9: (a) tornar **conta, personagem e histórico exportáveis** antes do lançamento do MVP (`:1441,1459,1464`), com direito de saída e sem depender de serviço hospedado que possa ser desligado (`:1442`); (b) completar a publicação de licença e atribuição com **`COPYRIGHT.txt`** ao lado do `LICENSE` já existente, garantindo que nenhum arquivo fique sem licença declarada (`:1443,1451,1258,1261`); (c) registrar a **assinatura de origem de toda contribuição** e recusar contribuição sem assinatura (`:1452,1465`), semanticamente compatível com DCO via trailer `Signed-off-by` — forma verificável sem chave criptográfica. A inspeção desta etapa confirma as três lacunas: não há código nem spec de exportação, `COPYRIGHT.txt` está ausente (`LICENSE` presente), e nenhum dos oito commits recentes traz trailer `Signed-off-by`.

## Origin & Context

- **Raised by**: SE, em 2026-11-21T00:22Z, ao redefinir a prioridade para um MVP jogável single-player (`.specify/memory/STATE.md:3-8`); o item 4 aparece literalmente em `:5`, com a norma de origem em `docs/GDD - Insurgência.md:1441-1466`.
- **Trigger**: os itens 1–3 foram entregues — ciclo 023 (cliente greybox, PR #29, HEAD `92bf8fb`) e ciclo 024 (instrumentação no Núcleo e 20 partidas, PR #30, HEAD `8589cec`), com R4 aprovado em 2026-11-21T01:12Z e release `1.0.0` (`.specify/validations/instrumentacao-metricas/validate.md`; `.specify/releases/instrumentacao-metricas/release.md`). O item 4 torna-se, assim, o próximo entregável da mesma interjeição; os itens 5 e 6 seguem **DEFER** por dependerem de outras pessoas (STATE:6).
- **Âncora**: `.specify/feature.json` → `specs/025-soberania-exportacao`; branch `feature/025-soberania-exportacao`; HEAD `8589cec`. O `specs/025-soberania-exportacao/spec.md` ainda é template Draft (`[FEATURE NAME]`, `[DATE]`), sem requisitos, entidades ou critérios preenchidos.
- **Escopo vivo**: single-player, local, sem servidor de jogo, sem lobby, sem partidas 5v5, sem NAT/travessia de rede e sem provedor de identidade remoto (STATE:5,8-9; ADR-003).

## Restrições

- **Sem chave GPG no ambiente — fato verificado**: `gpg --list-secret-keys --keyid-format=long` produz saída vazia (exit 0); `git config --get commit.gpgsign` e `git config --get user.signingkey` retornam exit 1 (ausentes). A assinatura exigida em GDD:1452 é, portanto, implementada como **DCO via trailer `Signed-off-by`**, verificável por texto e sem GPG. Gerar/importar chave e configurar assinatura criptográfica exigiria autorização da SE e material de credencial fora desta etapa.
- **DCO, não CLA**: entre as duas alternativas admitidas em GDD:1452, adota-se DCO (certificado de origem do desenvolvedor), compatível com a governança local e com a recusa de contribuição sem assinatura; CLA (Contrato de Licença de Contribuição) exigiria processus de aceite externo, fora do alvo.
- **Sem servidor externo**: nenhum IdP (Identity Provider, provedor de identidade) remoto, OIDC (OpenID Connect), Zitadel, Postgres/Coolify remoto, publicação ou serviço de terceiros — Zitadel e federação forte permanecem **[NV]** e fora do alvo (ADR-003; GDD:1598; STATE:9). Toda exportação permanece local ao alvo.
- **CLI local**: a exportação é acionada por interface de linha de comando (CLI, *command-line interface*) local e grava artefato em disco local; nenhuma tela nova é introduzida e a janela do cliente continua minimizada (`Window::set_minimized(true)`, `crates/cliente/src/main.rs`).
- **Governança Rust** (skill `mdmv-rust-thin`; ADR-004): `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, zero `.unwrap()`/`.expect()`, erros via `Result` + `thiserror`/`anyhow`, versões fixadas sem `*` e sem substituição automática de pacote; gate `verify.sh` (check → clippy → nextest → fmt). Nenhum gate, lint, baseline ou `evidence.csv` é alterado para facilitar a implementação.
- **Escopo de escrita desta etapa**: exatamente os cinco arquivos em `.specify/assessments/soberania-exportacao/`; nenhum spec, plano, tasks, código, gate, teste, build, formatação, stage, commit ou push. Convenções: pt-BR, marcador **[NV]** para todo estado não observado, termos estrangeiros explicados na primeira ocorrência.

## First-Glance Unknowns

- [NV] Nenhum modelo de conta/personagem existe em `crates/`; as ocorrências de "conta"/"personagem" no código são falsos positivos (`contains`, `contagem`, `contado`). A ADR-003 projeta `IAMInterface` + `IAMAdapterLocal`, mas nenhum adaptador foi implementado e nenhuma identidade está persistida — exportar "o que não existe" é o risco central a resolver na spec.
- [NV] Formato do artefato exportado (esquema versionado, codificação, nome de arquivo, diretório de saída, assinatura do conteúdo) não está fixado em GDD:1464 nem em STATE:5.
- [NV] Mecanismo de recusa de contribuição sem assinatura (gancho `pre-commit`, checagem local, portaria de revisão) não está definido; `verify.sh --install-hook` instala apenas `./verify.sh --fast` e **não** verifica DCO hoje.
- [NV] Nenhum arquivo de atribuição (fonte, licença, data e restrições) foi localizado; GDD:1258 exige entrada para todo asset e GDD:1261 exige validação antes de empacotar cada Módulo.
- [NV] Nenhum comando de exportação, crate, teste, build, gate, commit assinado ou leitura de artefato foi executado nesta etapa; todo resultado permanece **[NV]** até R4.
