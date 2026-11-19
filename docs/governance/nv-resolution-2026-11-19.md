# Contrato de resolução de [NV] — 2026-11-19T22:46Z

## Evidência ao vivo (comandos executados nesta sessão)

- PR #1 `MERGED` 22:45:33Z, commit `5f18526`; PR #2 `MERGED` 22:45:53Z, commit `741f4f1`; PR #3 `MERGED` 22:46:15Z, commit `3e0a213`; `mergedBy=lfgranja`; `gh pr list --state open` = `[]`; `dev` = `3e0a213`.
- `statusCheckRollup=[]` nos três PRs: nenhum check de CI configurado/reportado (verificado ao vivo, não "[NV]").
- `gh api repos/lfgranja/insurgencia/branches/dev/protection` → HTTP 403 "Upgrade to GitHub Pro or make this repository public": proteção **não consultável por limitação de plano**; registrar como limitação conhecida, não como verde.
- Usuário autorizou explicitamente os merges e pediu: "Efetuar o necessário para resolver todos os [NV]".

## Decisões canônicas

1. **Aprovação dos ADRs:** a autorização explícita do usuário + `mergedBy=lfgranja` nos PRs #2/#3 constituem registro humano de aprovação. Atualizar `docs/adr/001-*.md` e `002-*.md`: `Estado: Aprovado`, com entrada de changelog `### [1.0.1] — 2026-11-19` explicando a origem da aprovação (sem alterar o corpo das decisões). Onde houver `aprovação registrada em plataforma remota [NV]`, substituir por `aprovada pelo maintainer lfgranja (autorização explícita + merges dos PRs #2/#3 em 2026-11-19); nenhum review APPROVED emitido na plataforma, e nenhum check exigido`.
2. **CI:** substituir `execução de integração contínua [NV]` por `nenhum check de CI reportado (statusCheckRollup=[] nos PRs #1–#3, verificado ao vivo)`.
3. **Estado vivo:** atualizar `.specify/memory/STATE.md` com os fatos acima; NÃO reescrever histórico datado em `docs/PROGRESSO.md` — acrescentar seção append-only; em `docs/governance/80-20-ranking-2026-11-19.md`, acrescentar nota de superação dos snapshots `OPEN`.
4. **Pesquisa/avaliação:** os 11 marcadores de classe A podem virar citações locais (`docs/avaliacao-engines/pesquisa/licencas_lockin.md:24`, `bevy.md:8,11,23,26`). Não alterar os 92 de classe B (execução externa) nem os 157 de classe C (salvaguarda metodológica).
5. **Bloqueios que permanecem [NV]:** publicação/release/deploy externo (nenhuma evidência); proteção de branch (API 403 por plano). Registrar como limitação conhecida, não como resolvido.

## Fronteiras

- Nenhum arquivo de `converge` pode ser editado/renumerado.
- Nenhum histórico datado pode ser reescrito; apenas notas de superação/append.
- Nenhum [NV] metodológico legítimo pode ser apagado.
