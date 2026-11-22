# ADR-003: Substituir Zitadel por IAMAdapter/IAMInterface

- **ID**: ADR-003
- **Status**: Accepted
- **Estado**: Aceito
- **Data**: 2026-11-20
- **Versão**: 1.0.0

## Contexto

O planejamento herdado do hub MDMV registrava o Zitadel como provedor de identidade central (OIDC) do ecossistema: a norma N13 recomenda FusionAuth/Cognito → Zitadel (`docs/AVALIACAO-STACK-MDMV.md:71`) e a norma N37 descreve "Zitadel como IdP OIDC; autenticação e e-mail atrás de traits/roteadores (domínio não chama SDK)" (`docs/AVALIACAO-STACK-MDMV.md:115`). A auditoria Aroeira chegou a propor um `IdentityProvider` em `libs/domain` com adaptador Zitadel em `libs/infra` (`docs/avaliacao-stack/02-aroeira.md:87`). Nada disso foi executado no alvo: o GDD mantém "Deploy/publicação, Postgres/Coolify remotos, Zitadel, `pgmq` e federação forte" como **[NV]** (`docs/GDD - Insurgência.md:1598`), e o estado atual repete Zitadel entre os remanescentes **[NV]** (`.specify/memory/STATE.md:9`). A afirmação de que "Zitadel era a identidade central" é, portanto, uma intenção documental herdada, não um serviço implantado — **[NV]** até confirmação ao vivo.

Três fatos tornam essa intenção inadequada ao alvo Insurgência:

1. O pilar **Soberania do Jogador** (princípio IV da constituição, `.specify/memory/constitution.md:13`) exige regras, servidores e conteúdo inspecionáveis, modificáveis e redistribuíveis sem autoridade central, com nenhum serviço externo como ponto único de falha. Uma autoridade de identidade remota é exatamente o tipo de dependência que o pilar rejeita no caminho crítico.
2. O produto verificado é **solo/single**: sem partidas 5v5, sem lobby, sem NAT traversal e sem escala de produção (`.specify/memory/STATE.md:9`). Não há usuários remotos, federação forte nem pairing obrigatório (`docs/AVALIACAO-STACK-MDMV.md:426`) que exijam um IAM remoto neste ciclo.
3. O princípio II da constituição (`.specify/memory/constitution.md:9`) exige que nenhum código de domínio chame diretamente serviço, SDK ou provedor concreto: toda integração passa por trait/interface e adaptador substituível. A intenção herdada de "adaptador Zitadel atrás de trait" acerta na forma (porta) mas vincula a porta a um provedor específico, contrariando a soberania quando esse provedor é a autoridade central.

A decisão R1 registrada em ADR-001 e ADR-002 (`docs/adr/001-excecao-linguagem-gdscript.md:12-14`; `docs/adr/002-licenca-mit-nucleo.md:12-14`) fixou Godot-first com GDScript e licença MIT-only, além do caráter documental do ciclo. Este ADR trata somente da fronteira de identidade; ele não reabre o motor, a licença nem o escopo solo/single.

## Decisão

Fica aprovada, em 2026-11-20, a **substituição do Zitadel como autoridade de identidade prevista pelo contrato `IAMInterface` mais um adaptador local determinístico**, conforme detalhado em `docs/architecture/iam-interface.md`.

- O domínio depende apenas do contrato `IAMInterface` (`autenticar`, `identidade`, `validar_permissao`, `revogar`). Nenhum SDK concreto, cliente OIDC, endpoint, token ou biblioteca de provedor é referenciado pelo domínio.
- O adaptador previsto para o modo solo é `IAMAdapterLocal`: identidade determinística, sem servidor, sem segredo e sem credencial persistida. Ele é **projeto neste ciclo e não é implementado aqui**; o contrato de retorno e suas regras estão especificados no documento de arquitetura.
- A decisão materializa o **princípio II (Adaptadores e portas)** da constituição (`.specify/memory/constitution.md:9`) e serve ao **princípio IV (Soberania do Jogador)** (`.specify/memory/constitution.md:13`). A porta é substituível: trocar de provedor significa escrever um novo adaptador, nunca editar o domínio.
- A decisão é documental e afirmativa: não depende de CI, merge, aprovação remota, deploy, credencial ou serviço externo. Nenhum código `.gd`, build, teste ou gate decorre deste registro.

## Alternativas consideradas

- **Manter Zitadel como IdP OIDC central**: rejeitada por centralização. Mesmo atrás de trait, um provedor remoto como autoridade de identidade introduz dependência proprietária e ponto único de falha no caminho crítico, contrariando o princípio IV; a instância segue **[NV]** (`.specify/memory/STATE.md:9`).
- **Nenhuma interface de identidade (domínio sem IAM)**: rejeitada por falta de portabilidade. Sem uma porta explícita, cada futuro provedor ou modo de jogo exigiria editar o domínio, violando o princípio II e deixando a soberania sem mecanismo verificável.
- **OAuth direto no domínio**: rejeitada por acoplamento. Integrar fluxo OAuth/OIDC diretamente no código de jogo amarra o domínio a SDK, endpoints e formatos de token de um provedor, impedindo a substituição sem reescrita e reintroduzindo autoridade externa.
- **Implementar `IAMAdapterLocal` neste ciclo**: rejeitada por escopo. O ciclo é documental (ADR-001/ADR-002); a implementação fica marcada como projeto, com testes futuros de recusa de permissão, sem stubs nem código `.gd` agora.

## Consequências

- A porta de identidade passa a ser substituível por contrato: provedor real, quando existir, entra como novo adaptador atrás de `IAMInterface`, sem alterar regras de gameplay.
- **Nada muda no gameplay**: o modo solo/single verificado permanece o mesmo; não há login obrigatório, conta corporativa, lobby, partidas 5v5 nem federação introduzidos por esta decisão.
- O adaptador local previsto é determinístico e sem segredo; nenhuma credencial real é lida, copiada ou versionada. Testes futuros devem cobrir explicitamente a recusa de permissão (caminho negativo), não apenas o sucesso.
- **Provedor real permanece [NV]**: nenhuma instância, endpoint, credencial ou integração de Zitadel (ou qualquer IdP) foi verificada, executada ou adotada neste registro.
- Nenhum código, compilação, teste, gate, pipeline, serviço externo ou emenda da norma externa decorre deste ADR.

## Rollback

Se a decisão for revertida, o rollback é estritamente documental neste ciclo: remover `docs/adr/003-iam-adapter.md` e `docs/architecture/iam-interface.md` do conjunto versionado e registrar a reversão em novo ADR datado, com autoridade competente — nunca por edição silenciosa destes arquivos. Como nenhum adaptador, serviço, deploy ou credencial foi implementado, não há código, estado remoto ou segredo a desfazer. A reversão não reabre Godot-first, a licença MIT-only nem o escopo solo/single, e não reativa Zitadel automaticamente: eventual adoção de provedor real exige decisão própria. Critérios de reavaliação: evidência prévia de necessidade (por exemplo, modo multijogador ou federação) e aprovação formal antes de qualquer efeito.

## Rastreabilidade

| Referência | Tipo | Descrição | Origem local |
|---|---|---|---|
| `P-II` | Princípio | Adaptadores e portas: domínio nunca chama SDK ou provedor concreto. | `.specify/memory/constitution.md:9` |
| `P-IV` | Princípio | Soberania do jogador: sem autoridade central nem serviço externo no caminho crítico. | `.specify/memory/constitution.md:13` |
| `N13` | Norma herdada | FusionAuth/Cognito → Zitadel como provedor OIDC. | `docs/AVALIACAO-STACK-MDMV.md:71` |
| `N37` | Norma herdada | Zitadel como IdP OIDC; autenticação e e-mail atrás de traits/roteadores. | `docs/AVALIACAO-STACK-MDMV.md:115` |
| `ADR-03 (Aroeira)` | Proposta não executada | Trait `IdentityProvider` com adaptador Zitadel — substituída por esta decisão. | `docs/avaliacao-stack/02-aroeira.md:87` |
| `ZIT-NV` | Escopo [NV] | Zitadel, Postgres/Coolify remotos e federação forte fora do alvo. | `docs/GDD - Insurgência.md:1598`; `.specify/memory/STATE.md:9` |
| `SOLO` | Escopo verificado | Sem 5v5, lobby, NAT ou escala de produção. | `.specify/memory/STATE.md:9`; `docs/AVALIACAO-STACK-MDMV.md:426` |
| `ADR-001/002` | Decisões prévias | Godot-first/GDScript excepcionado e licença MIT-only; ciclo documental. | `docs/adr/001-excecao-linguagem-gdscript.md:12-14`; `docs/adr/002-licenca-mit-nucleo.md:12-14` |

As referências acima são as aplicáveis a este registro. Tudo o que depende de serviço remoto, provedor real ou credencial segue **[NV]** até verificação ao vivo.

## Histórico de versões

### [1.0.0] — 2026-11-20

- Materialização inicial do registro: substituição de Zitadel por `IAMInterface` + adaptador local determinístico; status `Accepted`.
- Nenhum código `.gd`, teste, gate, build, serviço ou credencial produzido neste ciclo.
