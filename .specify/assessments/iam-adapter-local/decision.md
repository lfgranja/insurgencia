# Decision: IAMAdapterLocal em processo

- **Slug**: iam-adapter-local
- **Decided**: 2026-11-20
- **Verdict**: `go`
- **R1**: `go`
- **Artifacts reviewed**: concept.md | docs/adr/003-iam-adapter.md | docs/architecture/iam-interface.md | .specify/memory/STATE.md | .specify/memory/constitution.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Validade do problema | strong | A porta `IAMInterface` está aprovada desde ADR-003 (2026-11-20), mas `docs/architecture/iam-interface.md:76` registra `IAMAdapterLocal` como projeto sem código; sem adaptador, a identidade determinística do modo solo não tem provedor verificável. |
| Força da evidência | adequate | ADR-003 `Accepted` (`docs/adr/003-iam-adapter.md:5,23`), contrato de fronteira com assinaturas e códigos (`docs/architecture/iam-interface.md:24-39,56-70`) e diretriz viva da SE (`.specify/memory/STATE.md:5-9`) são fontes locais citadas; nenhum comportamento observado do adaptador é afirmado antes da implementação. |
| Valor vs. inação | strong | Entregar o adaptador materializa a porta no modo solo verificado e preserva a soberania do jogador; não entregar deixa a autorização da SE sem efeito e a porta sem implementação. |
| Viabilidade / apetite | adequate | Opção A, appetite small: dois arquivos `.gd` e uma suíte de testes, sem infraestrutura nem dependência remota; as convenções GDScript e os erros conhecidos do repo estão nomeados na spec. |
| Aderência estratégica | strong | Materializa P-II (adaptadores e portas, `.specify/memory/constitution.md:9`) e P-IV (soberania do jogador, `.specify/memory/constitution.md:13`), conforme ADR-003:27. |
| Postura de risco | adequate | Riscos nomeados e mitigados: autoridade central → rejeitada (Opção B); ciclo de vida/custo RSA → rejeitado (Opção C); recusa tratada como erro → `PERMISSAO_NEGADA` como caminho normal e teste explícito (FR-005/FR-007); remoto sem evidência → **[NV]**. |

R1 verificado: `concept.md` presente; quatro opções nomeadas, cada uma com desfecho explícito; recomendação (Opção A) nomeada; veredito `go` legítimo, sem alegação de execução nesta etapa.

## Verdict & Rationale

**go** para a Opção A. A autorização da SE remove a ressalva de "projeto" registrada em ADR-003 e no documento de arquitetura, e o contrato existente já fixa assinaturas e códigos, de modo que a implementação não exige nova decisão de desenho. A Opção B é preterida por reintroduzir autoridade central; a Opção C, por ciclo de vida de chave e custo RSA incompatíveis com identidade sem segredo; a Opção D, porque a SE autorizou o ciclo. Nenhum `unknown` é glossado: provedor real remoto, endpoints, federação e qualquer integração ao vivo permanecem **[NV]** até verificação própria.

## Aceite da SE (citado)

> "PRs #13–#22 aprovados retroativamente pela autorização da SE (2026-11-20T10:58Z)." — `.specify/memory/constitution.md:22`; `.specify/memory/STATE.md:5`

> "Identidade: Zitadel substituído por **IAMAdapter / IAMInterface** (ADR-003 + contrato em `docs/architecture/iam-interface.md`). Nenhum SDK concreto no domínio; adaptador local para solo; provedor real remoto **[NV]**." — `.specify/memory/STATE.md:7`

> "Escopo: **solo/single** confirmado pela SE — sem 5v5, sem lobby e sem NAT/Internet nesta fase." — `.specify/memory/STATE.md:9`

Este é o aceite que habilita o ciclo: o adaptador local deixa de ser apenas projeto e passa a escopo entregável, mantidas as restrições abaixo.

## Snapshot de gates (citado; não reexecutado nesta etapa)

`.specify/memory/STATE.md:11` registra em 2026-11-20T10:05:42Z: **5/5 gates PASS**, `make check` e `make test-rede-e2e` exit 0, 131 `PASS` em 11 suítes, 0 `FAIL`. Nenhum desses comandos foi executado durante esta decisão. A meta da feature é preservar 5 de 5 gates PASS e `make check` exit 0 na implementação, aferida em R4 — não afirmar verde remoto ou resultado novo sem saída colada e data/hora.

## If needs-clarification

Não aplicável: nenhuma pergunta ao usuário. As quatro ambiguidades — identidade determinística sem servidor, revogação como estado local, recusa de permissão como caminho normal e nenhuma credencial versionada — foram resolvidas por evidência local e registradas em `specs/018-iam-adapter-local/spec.md` / `## Clarifications` / `### Session 2026-11-20`.

## If go — Handoff to `/speckit.specify`

- **Problema**: a porta `IAMInterface` está aprovada (ADR-003), mas `IAMAdapterLocal` ainda não existe como código; a identidade determinística do modo solo não tem provedor verificável.
- **Abordagem escolhida**: Opção A — `IAMAdapterLocal` em processo, contrato exato de quatro métodos, sem servidor, sem segredo e sem credencial persistida; login não obrigatório.
- **Dentro/fora de escopo**: dentro — `nucleo/iam/iam_interface.gd`, `nucleo/iam/iam_adapter_local.gd`, `tests/test_iam_adapter_local.gd` e portarias R4/R7; fora — login obrigatório, servidor central, federação, 5v5, provedor real remoto **[NV]**, execução de gates/testes/`make`/Godot nesta etapa e stage/commit/push.
- **Métricas de sucesso**: 4 de 4 métodos e 9 de 9 códigos exatos; 100% dos testes com `PASS`/`OK` e exit 0, 0 `FAIL`; `make check` exit 0; 5 de 5 gates PASS; 0 credenciais versionadas; 0 servidor, federação, 5v5 ou login obrigatório.
- **Restrições**: contrato e códigos imutáveis; nenhum SDK concreto no domínio; nenhum segredo, token, chave privada ou credencial em arquivo, log ou mensagem; nenhum arquivo fora dos três previstos escrito nesta etapa; nenhum gate, teste, `make`, Godot, stage, commit ou push executado aqui.
- **Perguntas abertas**: nenhuma; quatro respostas em `## Clarifications`. Remoto e provedor real seguem **[NV]** até decisão e evidência próprias.
