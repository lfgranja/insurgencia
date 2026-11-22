# Idea Intake: CI de gates determinísticos e proteção de dev

- **Slug**: ci-protecao-dev
- **Created**: 2026-11-20
- **Source**: pedido do mantenedor nesta sessão (texto fornecido); evidência de visibilidade `visibility=PUBLIC` em 2026-11-20T00:26:47Z; contexto em `.specify/memory/STATE.md`, `AGENTS.md`, `docs/governance/sdd-phase-contracts.md` e `scripts/ci/*.py`
- **Type**: new-capability

## Idea (as captured)

> "Objeto (autorizado pelo usuário): criar workflow GitHub Actions executando os 5 gates determinísticos do alvo em PR/push para dev/main; tornar o repo público (já feito, evidência 2026-11-20T00:26:47Z, visibility=PUBLIC); conduzir proteção de dev com required status check após observar o nome real do check em execução ao vivo. NÃO incluir publicação/deploy (não autorizado). NÃO importar `.forgejo/**` da factory. Execute EM ORDEM: intake → research → define → shape → decide(go) → specify → clarify → plan → checklist → tasks. FRs testáveis, SCs mensuráveis (check observado ao vivo, gates PASS no CI, proteção aplicada e verificada via API), ressalvas [NV] até execução."

> Aceitação solicitada: dez artefatos, nenhuma seção vazia, nenhum marcador pendente e veredito R1 `go`.

## Restated

Propor que as mudanças destinadas a `dev` e `main` passem a ser verificadas automaticamente pelos cinco gates determinísticos já existentes no alvo, e que a proteção da branch `dev` passe a exigir o status check real — identificado somente depois de uma execução ao vivo — como condição de merge. A mudança é de automação e governança de verificação; não inclui publicação, deploy nem importação do aparato Forgejo da factory.

## Origin & Context

- **Raised by**: mantenedor `lfgranja`, pedido explícito nesta sessão (2026-11-20).
- **Trigger**: item 3 do "Próximo 80/20 (vivo)" em `.specify/memory/STATE.md` — gate CI remoto sem check reportado (`statusCheckRollup=[]` nos PRs #1–#6) — combinado com a remoção do bloqueio de plano pela publicação do repositório (evidência 2026-11-20T00:26:47Z, `visibility=PUBLIC`), que torna a consulta e a configuração de proteção de branch possíveis.
- **Contexto de localização**: os artefatos de assess e de entrega desta feature ficam colocalizados em `specs/005-ci-protecao-dev/`, conforme o escopo de escrita da atribuição. Nenhum arquivo fora desse diretório é escrito nesta etapa de planejamento.
- **Limites de autoridade**: publicação/deploy não foram autorizados; o gate Forgejo da factory é fonte, não autoridade (Bloco 0.a, `AGENTS.md`); nenhuma transição remote é afirmada antes de comando executado com saída colada e data/hora.

## First-Glance Unknowns

- Resolvido (dado do pedido): repositório público (`visibility=PUBLIC`, 2026-11-20T00:26:47Z); cinco gates determinísticos nomeados em `AGENTS.md`; ordem obrigatória intake→research→define→shape→decide→specify→clarify→plan→checklist→tasks; R1 `go`; dez artefatos finais; sem publicação/deploy; sem importação de `.forgejo/**`.
- Qual é o nome exato do status check que o GitHub reporta na primeira execução ao vivo do workflow? (depende de execução; não se presume o nome a partir do YAML.)
- Os cinco gates terminam com `PASS` no ambiente do GitHub com a spec 005 presente no repositório? (depende de execução; a lacuna de diretórios derivados pode afetar o gate de completude.)
- A mutação de proteção da branch `dev` é permitida e verificável via API com o repositório público, e qual é a autoridade que a autoriza? (depende de autorização e verificação ao vivo.)
- Como a ausência de `ci-protecao-dev` em `.specify/assessments/`, `.specify/validations/` e `.specify/releases/` afeta o resultado do `check_sdd_completeness.py` no CI? (depende de leitura do gate e das fases R4/R7.)
