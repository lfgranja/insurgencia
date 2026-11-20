# Decision — Ilha Inferior: ponto de coleta com estoque e recarga

**Data:** 2026-11-20T09:38Z · **R1:** `go`

## Veredito
**go** — problema válido (impacto 5, evidência 5); opção escolhida = Opção 1 (replicar o padrão Ilha Topo com os números literais do GDD §16.3/§16.4).

## Gate de entrada
- Fonte: `docs/GDD - Insurgência.md:1504` (ciclo de 3 s, 5 Materiais, estoque de 3 ciclos, recarga em 20 s), `:1529-1531` (crédito do servidor, esgotamento, recusa de material forjado), `:1592` (pendência) e `:1677` (hipótese H3).
- Gates determinísticos do alvo: 5/5 PASS em 2026-11-20T03:21:31Z (`.specify/memory/STATE.md:5`; `spec_dirs=12`, `tasks=222`); reexecução neste ciclo e status remoto/CI permanecem **[NV]** até registro datado na portaria própria.
- `.specify/feature.json` → `specs/014-ilha-inferior`; branch `feature/014-ilha-inferior`, HEAD `48ee860`.
- Binário fixado: `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`; `--version` e saídas das suítes são **[NV]** nesta etapa de escrita.

## Restrições de execução
- Números literais do GDD: 3 s por ciclo, 5 Materiais por ciclo, 3 ciclos de estoque (15 Materiais) e 20 s de recarga; nenhuma reescrita "melhorada" nem parâmetro negociado pelo cliente.
- Tempo medido pelo servidor (`float(tick) * PASSO_SEGUNDOS`); o cliente envia apenas a intenção `coletar`, nunca quantidade; crédito, eventos e saldo existem somente após validação em `nucleo/sim/validacao.gd`.
- Preservar integralmente os preloads de `IlhaSincronia` e `IlhaTopo` em `nucleo/mundo.gd`; nenhuma instância de `Mundo.new()`/`_mundo_com_topo()` com `:=` (usar `: Mundo` explícito); nenhum `sha256_string` (o `hash_estado()` existente já usa `HashingContext`).
- Comandos sempre `--headless` com o binário absoluto; `make check GODOT=$G` como porta de regressão, exit 0 exigido. Saídas coladas com data/hora apenas quando observadas; o não verificado segue **[NV]**.
- Fora de escopo: partidas 5v5, funções Artesão/Guerreiro, vitória da Ilha Central, conexão UDP real, `Makefile`, `evidence.csv`, deploy, remote, factory, baselines, `converge.md` e portarias R4/R7. Nenhum artefato R4/R7 é escrito nesta etapa, e nenhuma alegação de funções, vitória, UDP real ou 39/39 caminhos é feita.
- Sem branches, commits, pushes, gates, testes ou código nesta etapa de escrita; a implementação e a verificação pertencem à fase própria.
- Convergence/analyze é READ-ONLY: divergência detectada é corrigida na fonte (spec/plano/tasks) e revalidada, nunca mascarada no artefato derivado.
