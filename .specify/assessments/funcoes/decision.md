# Decision — Funções: Barricada do Artesão e Ataque Forte validados no servidor

**Data:** 2026-11-20T09:41Z · **R1:** `go`

## Veredito
**go** — problema válido (impacto 5, evidência 5); opção escolhida = Opção 1 (regra pura em `nucleo/sim/funcoes.gd` com os números literais do GDD §16.3/§16.4, integração aditiva no servidor e suíte determinística).

## Gate de entrada
- Fonte: `docs/GDD - Insurgência.md:1505-1506` (20 Materiais, 1,5 s, 200 PV, 12 s; 1,2 s, 2× dano base, 6 s), `:1534-1535` (critérios de aceitação, inclusive a recusa do servidor na recarga), `:1563` (Task 3), `:1593` (pendência) e `:1678-1679` (H4/H5).
- Gates determinísticos do alvo: 5/5 PASS em 2026-11-20T03:21:31Z (`.specify/memory/STATE.md:5`; `spec_dirs=12`, `tasks=222`); reexecução neste ciclo e status remoto/CI permanecem **[NV]** até registro datado na portaria própria.
- `.specify/feature.json` → `specs/015-funcoes`; branch `feature/015-funcoes`, HEAD `7c19440`.
- Binário fixado: `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`; `--version` e saídas das suítes são **[NV]** nesta etapa de escrita.

## Restrições de execução
- Números literais do GDD e de `nucleo/sim/funcoes.gd`: 20 Materiais, 1,5 s e 200 PV e 12 s para a barreira; 1,2 s, `DANO_BASE=100` com `DANO_ATAQUE_FORTE=200` (2×) e 6 s de recarga para o ataque; nenhuma reescrita "melhorada" nem parâmetro negociado pelo cliente.
- Tempo medido pelo servidor (`float(tick) * PASSO_SEGUNDOS`, 30 Hz); o cliente envia apenas a intenção `construir_barreira`/`atacar`, nunca `materiais`/`dano`/`pv`/`recarga`; custo, dano, barreira e recarga existem somente após validação em `nucleo/sim/validacao.gd` e decisão em `nucleo/mundo.gd`.
- Preservar integralmente os preloads de `IlhaSincronia`, `IlhaTopo` e `PontoColeta` em `nucleo/mundo.gd`; nenhuma instância de `Mundo.new()` com `:=` (usar `: Mundo` explícito no teste); nenhum `sha256_string` (o `hash_estado()` existente já usa `HashingContext`).
- Comandos sempre `--headless` com o binário absoluto; `make check GODOT=$G` como porta de regressão, exit 0 exigido. Saídas coladas com data/hora apenas quando observadas; o não verificado segue **[NV]**.
- Fora de escopo: partidas 5v5, vitória da Ilha Central, conexão UDP real, custos de Fôlego/alcance/dimensões de produção, `Makefile`, `evidence.csv`, deploy, remote, factory, baselines, `converge.md` e portarias R4/R7. Nenhum artefato R4/R7 é escrito nesta etapa, e nenhuma alegação de vitória, 5v5 ou UDP real é feita.
- Sem branches, commits, pushes, gates, testes ou código nesta etapa de escrita; a implementação e a verificação pertencem à fase própria.
- Convergence/analyze é READ-ONLY: divergência detectada é corrigida na fonte (spec/plano/tasks) e revalidada, nunca mascarada no artefato derivado.
