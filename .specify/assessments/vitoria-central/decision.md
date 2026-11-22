# Decision — Vitória da Ilha Central: três ilhas de rota e canalização conjunta de 4 s

**Data:** 2026-11-20T09:48Z · **R1:** `go`

## Veredito
**go** — problema válido (impacto 5, evidência 5); opção escolhida = Opção 1 (regra pura `IlhaCentral` com os números literais do GDD §16.3/§16.4, integração aditiva no servidor e suíte determinística).

## Gate de entrada
- Fonte: `docs/GDD - Insurgência.md:1507` (redução de escala: três ilhas de rota, 4 s de canalização conjunta, Artesão e Guerreiro vivos e presentes), `:1537-1541` (critérios de aceitação, inclusive decisão do servidor, não substituição e zeração do progresso), `:1563` (Fase 4) e `:1593` (pendência 1538–1541).
- Gates determinísticos do alvo: 5/5 PASS em 2026-11-20T03:21:31Z (`.specify/memory/STATE.md:5`; `spec_dirs=12`, `tasks=222`); reexecução neste ciclo, execução de Godot e status remoto/CI permanecem **[NV]** até registro datado na portaria própria.
- `.specify/feature.json` → `specs/016-vitoria-central`; branch `feature/016-vitoria-central`, HEAD `ca9421f`.
- Binário fixado: `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`; `--version` e saídas das suítes são **[NV]** nesta etapa de escrita.

## Restrições de execução
- Números literais do GDD: `ILHAS_NECESSARIAS=3`, `CANALIZACAO_SEGUNDOS=4,0`; 3,99 s não vence, 4,0 s vence; interrupção zera `progresso`; `EVENTO_VITORIA="vitoria_ilha_central"`; `declarada_por="servidor"`. Nenhum parâmetro negociado pelo cliente e nenhuma reescrita "melhorada" da regra.
- Apenas Artesão e Guerreiro vivos e presentes, identificados por `peer_id`, podem iniciar e sustentar a canalização; outros jogadores não substituem quem morre ou sai. A tela de resultado consome evento/estado do servidor e nunca decide a vitória.
- Tempo medido pelo servidor (`float(tick) * PASSO_SEGUNDOS`, 30 Hz); o cliente envia apenas intenções, nunca `vitoria`, ilhas capturadas, função, vida ou presença prontas como verdade. Validação e decisão permanecem no Núcleo.
- Preservar integralmente os preloads de `IlhaSincronia`, `IlhaTopo`, `PontoColeta` e `Funcoes` em `nucleo/mundo.gd`; nenhuma instância de `Mundo.new()` com `:=` (usar `: Mundo` explícito no teste); nenhum `sha256_string` (o `hash_estado()` existente já usa `HashingContext`).
- Comandos sempre `--headless` com o binário absoluto; `make check GODOT=$G` como porta de regressão, exit 0 exigido. Saídas coladas com data/hora apenas quando observadas; o não verificado segue **[NV]**.
- Fora de escopo: partidas 5v5, conexão UDP real, regra de produção (seis ilhas/Plano Mestre/20 s de Propaganda), custos de Fôlego/alcance/dimensões de produção, cena de interface, `Makefile`, `evidence.csv`, deploy, remote, factory, baselines, `converge.md` e portarias R4/R7. Nenhum artefato R4/R7 é escrito nesta etapa e nenhuma alegação de vitória, 5v5, UDP real ou escala de produção é feita.
- Sem branches, commits, pushes, gates, testes ou código nesta etapa de escrita; a implementação e a verificação pertencem à fase própria.
- Convergence/analyze é READ-ONLY: divergência detectada é corrigida na fonte (spec/plano/tasks) e revalidada, nunca mascarada no artefato derivado.
