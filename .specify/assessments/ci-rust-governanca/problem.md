# Problem — CI Rust e governança alinhada

**Data**: 2026-11-21T04:25Z

## Declaração
O check obrigatório do CI do alvo é verde para qualquer PR cuja documentação SDD esteja íntegra, mesmo que o código Rust não compile ou os 34 testes falhem. Isso invalida a premissa da constituição de que "check verde autoriza merge autônomo" e expõe o projeto a regressões silenciosas no cliente low-poly, no Núcleo e na simulação. Em paralelo, a autoridade documental (`AGENTS.md`) descreve um projeto que não existe mais (sem código, sem gate Rust), induzindo agentes a ignorar o `verify.sh` e a stack ADR-004.

## Por que é real e urgente
- Evidência executada: `verify.sh` ALL STAGES OK localmente; CI `ci-gates.yml` não chama `verify.sh` nem `cargo` — a diferença é verificável por leitura direta.
- A constituição torna a confiança no CI condição do merge autônomo; hoje essa confiança é infundada.
- Os 33 PRs mesclados incluem código Rust; nenhum check de código rodou no CI até agora **[NV]}** (histórico de checks antigos não reauditado nesta sessão).

## Impacto
| Dimensão | Sem correção | Com correção |
|---|---|---|
| Regressão de código | Check verde + merge automático | Gate Rust bloqueia PR |
| Agentes futuros | Leem AGENTS.md obsoleto e desconsideram gate | Autoridade alinhada ao código |
| Auditoria | GDD marca CI como `[NV]`/DEFER | Documento reflete realidade |

## Critérios de sucesso
1. CI roda os 4 estágios do `verify.sh` (ou equivalentes) em PR/push para `dev`/`main`.
2. `AGENTS.md` descreve a realidade Rust (5 crates, 34 testes, `verify.sh`, regra da janela).
3. `verify.sh:2` não menciona `casv-rust`.
4. GDD:1627 deixou de afirmar que CI remoto está DEFER/`[NV]`.
5. Nenhum gate determinístico regrediu (5/5 SDD + `verify.sh`).

## Fora de escopo
Mudança de provider,CLI de PR, deploy, credenciais, itens 5-6, assinatura GPG, trabalho da factory.
