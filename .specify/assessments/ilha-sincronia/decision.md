# Decision — Desafio da Ilha: sincronia reduzida em 5 s

**Data:** 2026-11-20T03:10Z · **R1:** `go`

## Veredito
**go** — problema válido (impacto 5, evidência 5), opção recomendada = Opção 1 (Task 4 literal do plano).

## Gate de entrada
- Gate determinístico do alvo: 5/5 PASS em `2026-11-20T03:08:52Z` (`spec_dirs=9`, `tasks=222`, `cycles=0`); `pytest -q` N/A (exit 5) e `check_production_gate.py` N/A por Bloco 0.a.
- `.specify/feature.json` → `specs/010-ilha-sincronia`; branch `feature/010-ilha-sincronia`, HEAD `16e53d1`. Estado remoto nesta sessão é **[NV]**.
- Binário fixado: `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`; a versão só é conferida na fase de implementação **[NV]**.
- Contrato, servidor, validação e suítes anteriores já mesclados em `dev` (`.specify/memory/STATE.md:3-8`); CE-3 segue sem implementação.

## Restrições de execução
- Código literal da Task 4: cinco constantes, assinaturas de `sinalizar`/`aplicar_gas`/`reiniciar`, motivos, contagens de `PASS` e linha `DELTA_EVENTO_MS` inalterados; nenhuma reescrita "melhorada" e nenhuma alegação de 39/39 caminhos.
- Janela medida pelo tempo do servidor (`tick * PASSO_SEGUNDOS`); dano de 8 por amostra de 1,0 s e 24 em 3 s, nunca por quadro; regra de produção de 3 jogadores em `modulos/base/regras.cfg`, separada do exemplo de 2 jogadores.
- Comandos sempre com `--headless` e o binário absoluto; saídas coladas com data/hora na portaria de validação, sem afirmar execução não observada (**[NV]**). R4/R7 pertencem a fases posteriores e não são escritos nesta etapa.
- Sem tocar em `manifesto.json`, `Makefile`, `evidence.csv`, partidas 5v5, vitória da Ilha Central, elenco, factory, remote, baselines ou `converge.md`; sem branches, commits, pushes ou gates nesta etapa de escrita.
- Convergence/analyze é READ-ONLY: divergência corrigida na fonte e revalidada, nunca mascarada no artefato derivado.
