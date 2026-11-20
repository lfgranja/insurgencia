# Concept: IAMAdapterLocal em processo

- **Slug**: iam-adapter-local
- **Created**: 2026-11-20
- **Recommended option**: Opção A — adaptador local em processo (`IAMAdapterLocal`)

## Contexto

`docs/adr/003-iam-adapter.md` (Accepted, 2026-11-20) substitui Zitadel por `IAMInterface` mais adaptador local determinístico; o contrato está em `docs/architecture/iam-interface.md`. A SE autorizou este ciclo em 2026-11-20T10:58Z: "Identidade: Zitadel substituído por **IAMAdapter / IAMInterface** (ADR-003 + contrato em `docs/architecture/iam-interface.md`). Nenhum SDK concreto no domínio; adaptador local para solo; provedor real remoto **[NV]**" (`.specify/memory/STATE.md:7`) e "Escopo: **solo/single** confirmado pela SE — sem 5v5, sem lobby e sem NAT/Internet nesta fase" (`.specify/memory/STATE.md:9`). A ressalva "projeto, não implementado" de ADR-003 e da arquitetura descreve o ciclo anterior; a autorização da SE torna a implementação escopo entregável neste ciclo. Nenhum login obrigatório, servidor central, federação ou partida 5v5 é introduzido.

## Options

Quatro opções, da menor à maior superfície de autoridade; a recomendação é nomeada ao final.

### Option A — Adaptador local em processo (`IAMAdapterLocal`) — escolhida

- **Sketch**: classe GDScript em `nucleo/iam/iam_adapter_local.gd` que implementa a porta `IAMInterface` (`nucleo/iam/iam_interface.gd`) com os quatro métodos de contrato exatos. Identidade determinística por regras locais fixas sobre `(identidade, acao, recurso)`; sessões e revogações vivem somente em estado local do objeto. Nenhum segredo, token, endpoint, SDK, servidor ou credencial persistida; login não é obrigatório. Códigos de retorno imutáveis: `AUTENTICADO`, `CREDENCIAIS_INVALIDAS`, `CREDENCIAIS_AUSENTES`, `SESSAO_VALIDA`, `SESSAO_INVALIDA`, `PERMISSAO_CONCEDIDA`, `PERMISSAO_NEGADA`, `REVOGADA`, `NAO_REVOGADA`.
- **Appetite**: small — dois arquivos `.gd` e uma suíte em `tests/test_iam_adapter_local.gd`; sem infraestrutura, sem serviço, sem dependência remota.
- **Trade-offs**: ganha soberania do jogador (P-II/P-IV), determinismo e testabilidade sem rede; sacrifica a capacidade de autenticar usuários remotos, que não é requisito do modo solo verificado. Risco: confundir recusa de permissão com erro; mitigado ao tratá-la como caminho normal e testá-la explicitamente.

### Option B — Serviço próprio de identidade (autoridade central) — rejeitada

- **Sketch**: serviço de identidade dedicado (Zitadel/OIDC ou equivalente), com o domínio chamando cliente remoto atrás da porta.
- **Rejeição por autoridade central**: reintroduz ponto único de falha e dependência proprietária no caminho crítico, contrariando o princípio IV (soberania do jogador, `.specify/memory/constitution.md:13`) e o escopo solo/single confirmado pela SE (`.specify/memory/STATE.md:9`). ADR-003 já rejeitou Zitadel como IdP central (`docs/adr/003-iam-adapter.md:32`). Instância, endpoint e credencial permanecem **[NV]**.

### Option C — Identidade por chave do jogador — rejeitada

- **Sketch**: vincular a identidade a um par de chaves assimétricas do jogador, com assinatura e verificação por RSA.
- **Rejeição por ciclo de vida de chave e custo RSA**: geração, rotação, expiração e revogação de chaves criam um ciclo de vida de segredo e estado persistido que a Opção A elimina; o custo computacional e a superfície de chave privada são desproporcionais a uma identidade determinística local, sem segredo (`docs/architecture/iam-interface.md:74,84`). Nenhuma chave privada é versionada.

### Option D — Nenhuma implementação neste ciclo — rejeitada

- **Sketch**: manter `IAMAdapterLocal` apenas como projeto documental, como registrado em ADR-003 e na arquitetura.
- **Rejeição porque a SE autorizou o ciclo**: a diretriz viva de 2026-11-20T10:58Z autoriza explicitamente o adaptador local para solo e o escopo do ciclo (`.specify/memory/STATE.md:5,7,9`); não entregar a implementação deixaria a porta sem provedor verificável e contrariaria a autorização. A ressalva "PROJETO — NÃO IMPLEMENTADO NESTE CICLO" (`docs/architecture/iam-interface.md:76`) refere-se ao ciclo anterior, não a este.

## Recommendation

Opção A. É a única que implementa a porta preservando o contrato exato e serve ao modo solo/single sem introduzir autoridade central, segredo ou credencial. As opções B e C são rejeitadas por autoridade central e por ciclo de vida/custo de chave, respectivamente; a D é rejeitada porque a SE autorizou este ciclo. Nenhuma opção inclui login obrigatório, servidor, federação forte ou 5v5; provedor real remoto permanece **[NV]** e entra, se necessário, como novo adaptador atrás da mesma porta.

## Out of Scope (for the recommended option)

- Login obrigatório, conta corporativa, lobby, matchmaking e partidas 5v5.
- Servidor central, federação forte, NAT/Internet, endpoints, SDKs e tokens de provedor; provedor real remoto **[NV]**.
- Segredos, chaves privadas e qualquer credencial em arquivo versionado, log ou mensagem.
- Execução de gates, testes, `make` ou Godot nesta etapa de conceito/decisão/especificação; nenhum stage, commit ou push; nenhum arquivo fora dos três previstos.

## Assumptions to Validate

- A porta `IAMInterface` e seus nomes e códigos são imutáveis (`docs/architecture/iam-interface.md:42,70`); o adaptador não os renomeia.
- O modo verificado permanece solo/single (`.specify/memory/STATE.md:9`); qualquer requisito remoto exige decisão própria e continua **[NV]**.
- Os testes da suíte cobrem explicitamente `PERMISSAO_NEGADA`/`false` e `SESSAO_INVALIDA`, além dos caminhos de sucesso (`docs/architecture/iam-interface.md:85`).
- Convenções GDScript do alvo: TAB, tipos explícitos, `HashingContext` (não `sha256_string`), `Time.get_ticks_msec` (não `OS.get_ticks_msec`) e variáveis locais que não colidem com `for i`.
