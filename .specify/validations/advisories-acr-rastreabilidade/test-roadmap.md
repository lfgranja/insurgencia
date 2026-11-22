# Roteiro de Testes — Fechamento de advisories de ACR e rastreabilidade (ciclo 035)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador/agente de código · **Evidências aqui**: **[NV]** (nada executado neste ciclo)

## Escopo e rastreabilidade

| Item | Cobertura |
|---|---|
| FR-001 | passo 8 — 10 advisories #39/#40/#41 rastreados, sem blockers, sem merge em `main` |
| FR-002 | passo 1 — `STATE.md` com 34/237/34/34/0/13 e âncora coerente |
| FR-003 | passo 2 — `evidence.md` com cinco execuções/capturas e converge `17/17` |
| FR-004 | passo 3 — afirmação de `.gitignore` corrigida em 034; regra preservada |
| FR-005 | passos 4, 5, 6, 7 — saída com hash minúsculo canônico, aceitação de maiúsculas, exit 0/1/2, inspeção estática |
| FR-006 | passo 9 — ordem normativa, `analyze` antes de `tasks`, R4 antes de R7, 14 artefatos |
| FR-007 | este documento — passos numerados, capturas, `automatizável?`, rollback |
| FR-008 | passo 10 — fronteira, sem build/gate/commit/PR, PR #40 `OPEN`, 194 paths untracked |
| SC-001..SC-005 | passos 1–10 |

T-IDs relacionados: T014 (roteiro), T018 (aplicação das correções na fonte e execução, **[NV]**).

## Pré-condições

1. Worktree `.specify/queue/worktrees/acr-trace-035/`, branch `035-advisories-acr-rastreabilidade`, base `f9d87e0`; `.specify/feature.json → specs/035`.
2. Toolchain fixada em `rust-toolchain.toml`; `sha2 = "=0.10.9"`, `thiserror = "=2.0.21"` e `insurgencia-contratos` já resolvidos em `Cargo.lock` — nenhuma dependência nova.
3. `conferir` mantém `eq_ignore_ascii_case` para `content_hash`; `hash_regras` disponível no adaptador. Nenhuma alteração em `crates/contratos`.
4. Ambiente headless; nenhuma janela, engine, rede ou relógio envolvido; sem `cargo` concorrente de outros repos.
5. `TMP=$(mktemp -d)` para fixtures e capturas; nada escrito dentro do worktree além dos artefatos do ciclo e das quatro correções na fonte.
6. Construir uma vez: `cargo build -q -p insurgencia-sim --example verificar_manifesto` e definir `BIN=target/debug/examples/verificar_manifesto`. A variável `$?` após cada chamada registra o exit code.

## Superfície CLI e documentos

```
verificar_manifesto <manifesto.json> <regras.cfg> [module_id] [versao_local]
```
- obrigatórios: caminho do manifesto (JSON textual) e do arquivo de regras (bytes);
- opcionais: `module_id` (default `insurgencia.exemplo`) e `versao_local` (default `1`);
- stdout (sucesso): `COMPAT_OK module_id=<id> content_hash=<hash-minúsculo-calculado>`;
- stderr (recusa): `COMPAT_FALHA <codigo>`; stderr (uso/leitura): mensagem legível sem prefixo `COMPAT_`;
- exit: `0` aceito · `1` recusa de contrato · `2` uso inválido ou falha de leitura.
- Documentos inspecionados: `.specify/memory/STATE.md`, `.specify/validations/gate-manifesto-cli/evidence.md`, artefatos de `specs/034-gate-manifesto-cli/`, `.gitignore`.

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | Após FR-002, ler `STATE.md` e comparar com `find`/`feature.json` | "34 specs, 237 assessments, 34 validations, 34 releases, 0 bugs, 13 research"; âncora do ciclo 034 coerente com `feature.json`; nenhuma contagem 33/236/33/33 ou `specs/033` | sim |
| 2 | Após FR-003, ler `evidence.md` passo 1 e parágrafo final | cinco capturas listadas (`uso0-*`, `uso1-*`, `uso1b-*`, `uso5-*`, `usoabc-*`) e converge `17/17` baselinado; sem "quatro execuções" nem "pendente" | sim |
| 3 | Após FR-004, buscar "intacto"/".gitignore" nos artefatos 034 e ler `.gitignore:1-6` | artefatos registram `/.specify/queue/` em `.gitignore:5-6`; a regra continua presente (não removida) | sim |
| 4 | **(`content_hash` maiúsculo — passo exigido)** `printf '%s\n' 'regras 035' > "$TMP/regras.cfg"`; `HASH=$(sha256sum "$TMP/regras.cfg" \| cut -d' ' -f1)` (minúsculo); gerar manifesto com os 6 campos e `content_hash` = `"${HASH^^}"` (maiúsculas); executar `$BIN` | `COMPAT_OK module_id=insurgencia.exemplo content_hash=$HASH` em stdout — hash **minúsculo calculado**, não as maiúsculas do manifesto; stderr vazia, exit **0** | sim |
| 5 | Gerar manifesto idêntico, porém com `content_hash=$HASH` (minúsculo); executar `$BIN` | mesma saída do passo 4 (hash minúsculo), exit **0** — nenhuma regressão no caso já canônico | sim |
| 6 | Variar um campo por execução: `protocol_version=2`, `module_id=outro.modulo`, regras inexistentes, `content_hash` = 64 zeros, JSON `{}` | `COMPAT_FALHA VERSAO_PROTOCOLO_DIVERGENTE` / `MODULO_AUSENTE` / `REGRAS_AUSENTES` / `HASH_MODULO_DIVERGENTE` / `MANIFEST_INVALID`, cada uma com exit **1** e ordem do contrato preservada | sim |
| 7 | Repetir o passo 4 três vezes, redirecionando stdout para `$TMP/capturas/aceito-1\|2\|3-stdout.txt` e registrando exit codes; `diff` 1↔2 e 1↔3 | `diff` vazio nas duas comparações, exits `0/0/0` — saída byte a byte idêntica inclusive com `content_hash` maiúsculo na entrada | sim |
| 8 | Conferir rastreabilidade dos comentários `7082836571` (3), `7082907820` (4), `7082916777` (3) em `research.md`/`converge.md`; checar status do PR #40 | 10 advisories rastreados, 0 blockers; PR #40 `OPEN`, nenhum merge em `main` | parcial (inspeção + consulta `gh`) |
| 9 | Inspecionar os 14 artefatos e a ordem: `analyze.md` antes de `tasks.md`, R4 em `validate.md` antes de R7 em `release.md` (`1.0.0` + changelog FR/T + rollback) | ordem normativa respeitada, `analyze` READ-ONLY, `converge` append-only, 14 artefatos presentes | sim |
| 10 | Após FR-005, inspecionar `verificar_manifesto.rs`: `grep -nE 'unwrap\|expect\|unsafe\|async'`, presença de `forbid(unsafe_code)`/`deny(clippy::pedantic)`, linhas >100 colunas e `git diff --name-only`; confirmar ausência de build/gate/commit e 194 paths untracked | único casamento proibido é a própria linha `#![forbid(unsafe_code)]`; 0 linhas >100; diff sem `crates/contratos`, `Cargo.toml`, `.gitignore` indevido; nenhum commit/push/PR; 194 paths untracked preservados | parcial (inspeção automatizada + revisão humana) |

### Comandos de apoio (passo 4 — manifesto com `content_hash` maiúsculo)

```bash
TMP=$(mktemp -d); mkdir -p "$TMP/capturas"
printf '%s\n' 'regras 035' > "$TMP/regras.cfg"
HASH=$(sha256sum "$TMP/regras.cfg" | cut -d' ' -f1)
python3 -c 'import json,sys; d={"manifest_version":"1.0.0","protocol_version":1,"module_id":"insurgencia.exemplo","content_hash":sys.argv[2].upper(),"checked_at":"2026-11-22T00:00:00Z","tool_version":"4.7.2.stable.official"}; open(sys.argv[1],"w",encoding="utf-8").write(json.dumps(d,indent=2)+"\n")' "$TMP/manifesto-maiusculo.json" "$HASH"
cargo build -q -p insurgencia-sim --example verificar_manifesto
BIN=target/debug/examples/verificar_manifesto
"$BIN" "$TMP/manifesto-maiusculo.json" "$TMP/regras.cfg" > "$TMP/capturas/maiusculo-stdout.txt" 2> "$TMP/capturas/maiusculo-stderr.txt"; echo $? > "$TMP/capturas/maiusculo-exit.txt"
# esperado: stdout = "COMPAT_OK module_id=insurgencia.exemplo content_hash=$HASH" (minúsculo), exit 0
```

Para os demais casos, copiar `manifesto-maiusculo.json` e alterar somente o campo indicado, mantendo os outros cinco fixos.

## Plano de capturas (somente terminal/JSON — sem UI)

- Diretório temporário `$TMP/capturas/` (fora do worktree; removido no rollback).
- Por caso: `<caso>-stdout.txt`, `<caso>-stderr.txt`, `<caso>-exit.txt` — saídas cruas e código numérico.
- Fixtures: `regras.cfg`, `manifesto-maiusculo.json`, `manifesto-minusculo.json`, `manifesto-versao.json`, `manifesto-modulo.json`, `manifesto-hash.json`, `manifesto-vazio.json`.
- Determinismo: `aceito-1|2|3-stdout.txt`, `diff-12.txt`, `diff-13.txt` (ambos vazios).
- Cada captura permanece rotulada **[NV]** até o orquestrador executar o passo; nenhuma captura existe neste ciclo.

## Rollback do roteiro

`rm -rf "$TMP"` remove fixtures e capturas; nenhum arquivo do repositório é modificado pela **execução** do roteiro. Para desfazer as correções (T018), reverter `STATE.md`, `evidence.md`, a afirmação de `.gitignore` em 034 e a linha de saída do CLI ao estado anterior, e remover os 14 artefatos do ciclo. **Nunca executar neste ciclo — apenas documentar.** Não executar `git clean`, `git reset`, force-push, remover a regra `/.specify/queue/` do `.gitignore` nem mesclar PR #40. Se um passo falhar, registrar a saída obtida e corrigir a **fonte**, nunca a expectativa do teste.
