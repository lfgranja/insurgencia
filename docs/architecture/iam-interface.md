# Interface de identidade — IAMInterface

- **Data**: 2026-11-20
- **Estado**: projeto aprovado (ADR-003, `docs/adr/003-iam-adapter.md`)
- **Escopo**: contrato e regras de fronteira; nenhuma implementação neste ciclo
- **Regime**: documento — nenhum código `.gd`, teste, gate ou serviço decorre aqui

## Finalidade

Este documento fixa a porta de identidade do Núcleo. O domínio depende somente do contrato abaixo; qualquer provedor — local ou remoto — entra como adaptador substituível. A decisão e suas alternativas estão registradas em `docs/adr/003-iam-adapter.md`. O princípio II da constituição exige adaptadores e portas (`.specify/memory/constitution.md:9`) e o princípio IV exige soberania do jogador, sem autoridade central (`.specify/memory/constitution.md:13`).

O modo verificado é solo/single (`.specify/memory/STATE.md:9`): a identidade prevista é local, determinística e sem servidor. Zitadel, provedores OIDC remotos e credenciais reais permanecem **[NV]** e fora do alvo.

## Contrato (GDScript-like, ilustrativo)

O bloco abaixo descreve a forma do contrato. É especificação em linguagem semelhante a GDScript, **não** um arquivo `.gd` implementado neste ciclo.

```gdscript
class_name IAMInterface
extends RefCounted

# Autentica um pedido a partir de credenciais.
# Retorna Dictionary com o contrato de retorno definido abaixo.
func autenticar(credenciais: Dictionary) -> Dictionary:
    pass  # contrato — implementado por adaptador

# Resolve a identidade associada a uma sessao.
# Retorna Dictionary com o contrato de retorno definido abaixo.
func identidade(sessao: String) -> Dictionary:
    pass  # contrato — implementado por adaptador

# Autoriza uma acao sobre um recurso para uma identidade.
# Retorna true somente quando a permissao e concedida.
func validar_permissao(identidade: Dictionary, acao: String, recurso: String) -> bool:
    pass  # contrato — implementado por adaptador

# Revoga uma sessao. Retorna true somente se a sessao existia e foi revogada.
func revogar(sessao: String) -> bool:
    pass  # contrato — implementado por adaptador
```

Notação: `Dictionary` é o valor de retorno estruturado; `bool` é usado apenas onde a decisão é binária e inequívoca (`validar_permissao`, `revogar`). Nenhum método recebe ou devolve segredo, token de provedor, endpoint ou SDK concreto. Os nomes de parâmetros são parte do contrato e não devem ser renomeados por adaptadores.

## Contrato de retorno — códigos legíveis

Todo `Dictionary` retornado por `autenticar` e `identidade` deve conter, no mínimo, `sucesso`, `codigo` e `mensagem`. Campos adicionais só são acrescentados pelo adaptador quando o código os exige. Códigos são strings estáveis, legíveis e em português; o domínio compara códigos, nunca textos de mensagem.

| Campo | Tipo | Obrigatório | Descrição |
|---|---|---|---|
| `sucesso` | `bool` | sim | `true` somente quando a operação atingiu o efeito esperado. |
| `codigo` | `String` | sim | Código legível da tabela abaixo; imutável por adaptador. |
| `mensagem` | `String` | sim | Descrição humana, diagnóstica; nunca usada como condição de ramificação. |
| `sessao` | `String` | quando aplicável | Identificador opaco da sessão; ausente em falha de autenticação. |
| `identidade` | `Dictionary` | quando aplicável | Identidade resolvida; ausente quando não há sessão válida. |

Códigos de retorno:

| Código | Método | Sentido |
|---|---|---|
| `AUTENTICADO` | `autenticar` | Credenciais aceitas; `sessao` e `identidade` presentes. |
| `CREDENCIAIS_INVALIDAS` | `autenticar` | Credenciais recusadas; nenhuma sessão emitida. |
| `CREDENCIAIS_AUSENTES` | `autenticar` | Dicionário de credenciais vazio ou incompleto. |
| `SESSAO_VALIDA` | `identidade` | Sessão resolvida; `identidade` presente. |
| `SESSAO_INVALIDA` | `identidade` | Sessão desconhecida, expirada ou revogada. |
| `PERMISSAO_CONCEDIDA` | `validar_permissao` | equivalente a `true` — não é string de retorno, apenas o sentido do `bool`. |
| `PERMISSAO_NEGADA` | `validar_permissao` | equivalente a `false` — deve ser tratada como caminho normal, não como erro. |
| `REVOGADA` | `revogar` | A sessão existia e foi revogada (`true`). |
| `NAO_REVOGADA` | `revogar` | A sessão não existia ou já estava revogada (`false`). |

Regras de códigos: um código nunca é renomeado para acomodar um provedor; falha de autenticação não distingue "usuário inexistente" de "segredo incorreto" na mensagem, evitando vazamento; `PERMISSAO_NEGADA` é um resultado esperado e deve ser testado (ver `## Regras`).

## Adaptador local previsto — IAMAdapterLocal

`IAMAdapterLocal` é o adaptador previsto para o modo solo: identidade determinística, sem servidor, sem segredo e sem credencial persistida. Ele implementa `IAMInterface` e resolve `validar_permissao` por regras locais fixas sobre `(identidade, acao, recurso)`, com recusa explícita quando a regra não concede a ação.

**PROJETO — NÃO IMPLEMENTADO NESTE CICLO.** Nenhum arquivo `.gd` foi criado, nenhuma classe `IAMAdapterLocal` existe no repositório e nenhum teste foi executado. Sua implementação, incluindo a tabela concreta de permissões e os testes de recusa, fica para ciclo próprio. Qualquer afirmação de comportamento observado do adaptador é **[NV]** até essa implementação.

Um provedor real, se surgir, entra como **novo adaptador** atrás da mesma `IAMInterface`: o domínio não é editado, o contrato de retorno é preservado e nenhuma credencial é versionada.

## Regras

1. **Domínio nunca chama SDK concreto.** Nenhum módulo de domínio referencia cliente OIDC, Zitadel, OAuth, endpoint, token ou biblioteca de provedor; toda chamada passa por `IAMInterface`.
2. **Troca de provedor = novo adaptador.** Substituir a origem da identidade significa escrever um adaptador que implemente o contrato; nenhuma regra de gameplay, assinatura de método ou código de retorno muda por causa da troca.
3. **Nenhuma credencial em arquivo versionado.** Segredos, tokens e credenciais reais não entram em arquivos, logs, mensagens nem fixtures versionadas; o adaptador local não possui segredo.
4. **Testes futuros cobrem recusa de permissão.** A suíte vindoura deve incluir explicitamente o caminho negativo de `validar_permissao` (`PERMISSAO_NEGADA` / `false`) e de sessão inválida, além dos casos de sucesso — não apenas autorização concedida.

## Fora de escopo

Sem login obrigatório, conta corporativa, lobby, matchmaking, partidas 5v5, NAT traversal, federação forte, Postgres/Coolify remotos ou `pgmq`. Nada aqui autoriza integração remota, deploy, CI, gate ou execução de Godot neste ciclo. Zitadel como autoridade central foi rejeitado no ADR-003 e qualquer provedor real permanece **[NV]**.

## Rastreabilidade

| Referência | Tipo | Descrição | Origem local |
|---|---|---|---|
| `ADR-003` | Decisão | Substituir Zitadel por `IAMInterface` + adaptador local; Accepted em 2026-11-20. | `docs/adr/003-iam-adapter.md` |
| `P-II` | Princípio | Adaptadores e portas; domínio sem SDK concreto. | `.specify/memory/constitution.md:9` |
| `P-IV` | Princípio | Soberania do jogador; sem autoridade central. | `.specify/memory/constitution.md:13` |
| `N37` | Norma herdada | Identidade atrás de trait/roteador, domínio sem SDK. | `docs/AVALIACAO-STACK-MDMV.md:115` |
| `SOLO` | Escopo verificado | Sem 5v5, lobby, NAT ou produção; remoto e Zitadel **[NV]**. | `.specify/memory/STATE.md:9` |
