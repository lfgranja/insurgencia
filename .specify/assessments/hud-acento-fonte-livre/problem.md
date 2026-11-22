# Problem — HUD com acento e fonte livre

**Data**: 2026-11-22T21:16Z

O HUD do cliente low-poly exibe `INSURGENCIA - low-poly` e `Limites: X +/-580 / Y +/-310` por causa do ciclo 029, que substituiu os glifos ausentes da fonte padrão do Bevy por ASCII seguro. A SE pediu explicitamente a grafia correta `INSURGÊNCIA — low-poly` e o separador `·`. Sem uma fonte com esses glifos, a correção textual reintroduz `▯` (tofu), corrompendo o nome do jogo e degradando a identidade do produto.

## Critérios de sucesso observáveis
1. As duas strings do HUD usam `Ê` (U+00CA), `—` (U+2014) e `·` (U+00B7), sem `▯`.
2. Fonte livre (Noto Sans Regular, OFL 1.1) embarcada, com proveniência e SHA-256 rastreáveis e licença integral no repositório.
3. `verify.sh` ALL STAGES OK (41/41 testes) e 5 gates PASS após a mudança **[NV]** — execução pertence ao orquestrador.
4. Captura em Xvfb isolada (sem janela no desktop da SE) confirma os três glifos e ausência de tofu **[NV]**.
5. Regra da janela preservada: agente nunca abre janela normal; verificação visual final com a SE.
6. Fronteira SDD respeitada: 14 artefatos, `analyze` antes de `tasks`, R4 antes de R7, nenhum build/gate/commit/push/PR executado por este agente.

## Custo de inação
Manter o ASCII contraria decisão explícita do SWE e deixa a identidade do produto corrompida (nome do jogo sem acento); aplicar só as strings sem fonte traria de volta o tofu visível do ciclo 029.

## Fora de escopo
Redefinir contratos de simulação/manifesto, alterar `Cargo.toml`/`Cargo.lock`, transplantar algo da factory `b1deab1` (somente fonte), mexer em `docs/GDD*`, baselines pré-existentes, ledger, CI, janelas no desktop, deploy, assinatura GPG, commits/push/PRs e reexecução de gates/build neste agente.
