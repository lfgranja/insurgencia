# Decision — Fechamento de advisories de ACR e rastreabilidade documental pós-034/041

- **Slug**: advisories-acr-rastreabilidade
- **Decided**: 2026-11-22T18:34:00Z
- **Veredito**: **go** · **R1**: `go`

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | 10 advisories ACR sem blockers e quatro divergências verificadas entre artefatos e HEAD; rastreabilidade e determinismo da saída do CLI em risco. |
| Força da evidência | strong | Leitura direta de `STATE.md`, `feature.json`, `.gitignore:5-6`, `evidence.md` e `verificar_manifesto.rs:135-136`; `./verify.sh` `ALL STAGES OK`/41 de 41 e 5 gates em `18:32:17Z`; PR #40 `OPEN` com `gates`/`rust` pass. |
| Valor vs. inação | strong | Fecha os PRs sem mudar contrato, restaura evidências reproduzíveis e torna a saída do gate canônica;Score 80/20 = 4×5×4×5 = 400/625 = 0,64, muito acima da paridade Godot (80/625). |
| Viabilidade / appetite | strong | Quatro correções pontuais (três documentais + uma linha de saída) e 14 artefatos SDD; nenhuma dependência, nenhum build, nenhum merge. |
| Alinhamento estratégico | strong | GDD §1/§14.2 e Constituição II preservados; ordem normativa com `analyze` (READ-ONLY) antes de `tasks`, R4 antes de R7 `1.0.0`, rollback documentado; Bloco 0.a respeitado. |
| Postura de risco | strong | Fronteira disjunta, rollback trivial, factory `b1deab1` somente fonte, 194 paths untracked e CI intactos; toda execução marcada **[NV]**; `main` não mesclado. |

## Scorecard
6/6 critérios **strong** → **go**. Nenhum critério `weak`; nenhum marcador `[NEEDS CLARIFICATION]` em aberto.

## Racional
Não há decisão de design aberta: a autoridade é `conferir` (011/033) e o advisory do CLI é apenas de **saída**. Manter a aceitação de `content_hash` maiúsculo (via `eq_ignore_ascii_case`) e imprimir o hash calculado em minúsculas resolve o determinismo sem alterar a ordem formato → versão → módulo → 64 hex → regras → hash. As três correções documentais são fatuais e não dependem de autorização externa — ao contrário de instalar Godot (80/625).

## 80/20 (registrado)
Escolhido: fechar advisories de ACR e deriva documental pós-034/041 = impacto 4 × confiança 5 × facilidade 4 × segurança 5 = **400/625 = 0,64**. Alternativa descartada: paridade Rust↔Godot = 5×4×2×2 = **80/625**; instalar Godot depende de autorização externa. `check_production_gate.py` da factory ausente → exit 2 = **N/A** (Bloco 0.a), não RED.

## Condição/responsável
Fronteira única deste agente: 5 artefatos de assess, 6 artefatos em `specs/035-advisories-acr-rastreabilidade/`, `validate.md`, `test-roadmap.md` e `release.md` — total 14 artefatos, nenhum código. Orquestrador/agente de código: aplicar as quatro correções na fonte, executar o roteiro, rodar gates/CI e manter PR #40 na fila SWE (sem merge em `main`); nada disso é feito aqui. `./verify.sh` e o roteiro permanecem **[NV]** até execução observada. Factory `b1deab1` é fonte, não autoridade; nada transplantado.

## Handoff
Problema acima; abordagem = Opção 1; escopo = 14 artefatos SDD, zero arquivos de código; fora de escopo = redefinir `conferir`/011, remover regra do `.gitignore`, instalar Godot, mesclar PR #40, factory, `docs/GDD*`, ledger, CI, baselines pré-existentes, 194 paths untracked alheios, janelas, credenciais, builds/gates/commits/push/PRs. Métricas: 10 advisories rastreados, 4 divergências corrigidas na fonte, caso `content_hash` maiúsculo → hash minúsculo canônico no roteiro, `analyze` antes de `tasks`, R4 antes de R7 `1.0.0`, `[NV]` preservado onde não houve execução.
