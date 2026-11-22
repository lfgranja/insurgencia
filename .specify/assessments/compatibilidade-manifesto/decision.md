# Decision — Manifesto de compatibilidade e gate local de versão/hash

**Data:** 2026-11-20T03:15Z · **R1:** `go`

## Veredito
**go** — problema válido (impacto 5, evidência 5), opção recomendada = Opção 1 (Task 5 literal do plano).

## Gate de entrada
- Gate determinístico do alvo: 5/5 PASS em `2026-11-20T03:08:52Z` (`spec_dirs=9`, `tasks=222`, `cycles=0`); `pytest -q` N/A (exit 5) e `check_production_gate.py` N/A por Bloco 0.a.
- `.specify/feature.json` → `specs/011-compatibilidade-manifesto`; branch `feature/011-compatibilidade-manifesto`, HEAD `c2936db`. Estado remoto nesta sessão é **[NV]**.
- Binário fixado: `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`; a versão só é conferida na fase de implementação **[NV]**. O manifesto regista `tool_version = 4.7.2.stable.official`.
- `Protocolo` (4 códigos de recusa, `PROTOCOL_VERSION = 1`), `modulos/exemplo/module.json` e `modulos/exemplo/regras.cfg` (hash `b2c11e6d…d746`) já mesclados em `dev` (`.specify/memory/STATE.md:3-8`); CE-4 segue sem implementação. `doc-26` é DEFER (sem lobby/worker e sem release).

## Restrições de execução
- Código literal da Task 5: 6 campos obrigatórios, assinaturas de `carregar`/`validar_formato`/`conferir`, ordem versão→módulo→hash, códigos exatos, 7 verificações e saídas `COMPAT_OK`/`COMPAT_FALHA` inalteradas; nenhuma reescrita "melhorada" e nenhuma alegação de 39/39 caminhos.
- Hash de 32 bytes (64 hex) calculado do arquivo de regras com a primitiva disponível no Godot 4.7.2 (o ciclo 009 registou em R4 que `sha256_string()` não existe e exigiu `HashingContext`); o contrato de 64 hex e o código `HASH_MODULO_DIVERGENTE` não mudam.
- Comandos sempre com `--headless` e o binário absoluto; saídas coladas com data/hora na portaria de validação, sem afirmar execução não observada (**[NV]**). R4/R7 pertencem a fases posteriores e não são escritos nesta etapa.
- Teste de divergência deve capturar o status 1 e restaurar `modulos/exemplo/regras.cfg` ao final; as 3 execuções precisam produzir saída idêntica (CE-4.5).
- Sem tocar em `evidence.csv`, Ilha, 5v5, vitória Central, elenco, factory, remote, baselines, gates/CI ou `converge.md`; sem branches, commits, pushes ou execução de Godot/gates nesta etapa de escrita.
- Convergence/analyze é READ-ONLY: divergência corrigida na fonte e revalidada, nunca mascarada no artefato derivado.
