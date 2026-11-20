# Decision — Ilha Topo: três alavancas em ordem fixa

**Data:** 2026-11-20T09:28Z · **R1:** `go`

## Veredito
**go** — problema válido (impacto 5, evidência 5); opção escolhida = Opção 1 (replicar o padrão Ilha Meio com os números literais do GDD §16.3/§16.4).

## Gate de entrada
- Fonte: `docs/GDD - Insurgência.md:1502` (ordem fixa, 8 s, reinício em 2 s, limite 45 s) e `:1518-1521` (captura com evento, erro com aviso sem captura, falha no servidor); pendência registrada em `:1590`.
- Gates determinísticos do alvo: 5/5 PASS em 2026-11-20T03:21:31Z (`.specify/memory/STATE.md:5`; `spec_dirs=12`, `tasks=222`); status remoto/CI desta sessão é **[NV]**.
- `.specify/feature.json` → `specs/013-ilha-topo`; branch `feature/013-ilha-topo`, HEAD `2bda55c`.
- Binário fixado: `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`; `--version` e saídas das suítes são **[NV]** nesta etapa de escrita.

## Restrições de execução
- Números e ordem literais do GDD: `ORDEM=[1,2,3]`, 8 s por alavanca, reinício em 2 s, limite de 45 s; nenhuma reescrita "melhorada" nem parâmetro negociado pelo cliente.
- Tempo medido pelo servidor (`tick * PASSO_SEGUNDOS`); captura, `estado_persistido="ilha_topo_concluida"` e evento só após validação em `nucleo/sim/validacao.gd`; três ativações fora da janela falham no servidor.
- Comandos sempre `--headless` com o binário absoluto; `make check GODOT=$G` como porta de regressão, exit 0 exigido. Saídas coladas com data/hora apenas quando observadas; o não verificado segue **[NV]**.
- Fora de escopo: partidas 5v5, vitória da Ilha Central, Ilha Inferior, funções Artesão/Guerreiro, `Makefile`, `evidence.csv`, deploy, remote, factory, baselines, `converge.md` e portarias R4/R7. Nenhum artefato R4/R7 é escrito nesta etapa, e nenhuma alegação de 39/39 caminhos é feita.
- Sem branches, commits, pushes, gates, testes ou código nesta etapa de escrita; a implementação e a verificação pertencem à fase própria.
- Convergence/analyze é READ-ONLY: divergência detectada é corrigida na fonte (spec/plano/tasks) e revalidada, nunca mascarada no artefato derivado.
