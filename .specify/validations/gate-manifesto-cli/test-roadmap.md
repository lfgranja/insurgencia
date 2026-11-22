# Roteiro de Testes — Gate CLI determinístico de manifesto de compatibilidade (ciclo 034)

**Status**: escrito antes do PR · **Execução**: pelo orquestrador · **Evidências aqui**: **[NV]** (nada executado neste ciclo)

## Escopo e rastreabilidade

| Item | Cobertura |
|---|---|
| FR-001 | passos 1 e 9 — assinatura, defaults, uso inválido |
| FR-002 | passos 2, 3 e 6 — leitura no adaptador, manifesto inválido, regras ausentes |
| FR-003 | passo 3 — SHA-256 real do arquivo de regras em hex minúsculo |
| FR-004 | passos 3–7 — `conferir`, `COMPAT_OK`, `COMPAT_FALHA <codigo>`, exit 0/1/2 |
| FR-005 | passo 10 — inspeção estática do arquivo novo |
| FR-006 / T014–T018 | passos 1–10 e plano de capturas abaixo |
| SC-001..SC-006 | passos 1–10 (8 casos funcionais + determinismo + inspeção) |

T-IDs relacionados: T014 (implementação), T015 (roteiro), T016 (converge), T017 (R4), T018 (R7).

## Pré-condições

1. Worktree `.specify/queue/worktrees/manifesto-cli/`, branch `034-gate-manifesto-cli`, base `7329c53`.
2. Toolchain fixada em `rust-toolchain.toml`; `sha2 = "=0.10.9"`, `thiserror = "=2.0.21"` e `insurgencia-contratos` já resolvidos em `Cargo.lock` — nenhum download de dependência nova.
3. Sem `cargo` concorrente de outros repos (evitar a contenção observada no ciclo 033). Opcional: `CARGO_TARGET_DIR` próprio.
4. Ambiente headless; nenhuma janela, engine, rede ou relógio envolvido.
5. `TMP=$(mktemp -d)` para todos os fixtures e capturas; nada é escrito dentro do worktree além do código/artefatos do ciclo.
6. Construir uma vez: `cargo build -p insurgencia-sim --example verificar_manifesto` e definir `BIN=target/debug/examples/verificar_manifesto` (ajuste se `CARGO_TARGET_DIR` estiver definido). A variável `$?` logo após cada chamada registra o exit code.

## Superfície CLI

```
verificar_manifesto <manifesto.json> <regras.cfg> [module_id] [versao_local]
```
- obrigatórios: caminho do manifesto (JSON textual) e caminho do arquivo de regras (bytes);
- opcionais: `module_id` (default `insurgencia.exemplo`) e `versao_local` (default `1`);
- stdout (sucesso): `COMPAT_OK module_id=<id> content_hash=<hash>`;
- stderr (recusa): `COMPAT_FALHA <codigo>`;
- stderr (uso/leitura): mensagem legível, sem prefixo `COMPAT_`;
- exit: `0` aceito · `1` recusa de contrato · `2` uso inválido ou falha de leitura.

## Passos numerados (ação → resultado esperado)

| # | Ação | Resultado esperado | Automatizável? |
|:--|:--|:--|:--|
| 1 | Invocar `$BIN` sem argumentos; com 1 argumento; com 5 argumentos; e com `versao_local=abc` | quatro execuções com mensagem de uso/erro na stderr, nenhuma linha `COMPAT_*`, exit **2** | sim |
| 2 | Criar `$TMP/manifesto-vazio.json` com `{}` e `$TMP/manifesto-ruido.json` com `nao-e-json`; invocar `$BIN` com cada um e `$TMP/regras.cfg` válido | `COMPAT_FALHA MANIFEST_INVALID` na stderr, exit **1**, stdout vazio | sim |
| 3 | `printf '%s\n' 'regras de teste 034' > "$TMP/regras.cfg"`; `HASH=$(sha256sum "$TMP/regras.cfg" | cut -d' ' -f1)`; gerar `$TMP/manifesto-aceito.json` com os 6 campos, `content_hash=$HASH`, `protocol_version=1`, `module_id=insurgencia.exemplo`; invocar `$BIN "$TMP/manifesto-aceito.json" "$TMP/regras.cfg"` | `COMPAT_OK module_id=insurgencia.exemplo content_hash=$HASH` em stdout, stderr vazia, exit **0**; o hash impresso é o SHA-256 real do arquivo temporário | sim |
| 4 | Gerar `$TMP/manifesto-versao.json` idêntico ao aceito, porém com `protocol_version=2`; invocar com as regras válidas | `COMPAT_FALHA VERSAO_PROTOCOLO_DIVERGENTE`, exit **1** (versão antes de módulo/hash) | sim |
| 5 | Gerar `$TMP/manifesto-modulo.json` idêntico ao aceito, porém com `module_id=outro.modulo`; invocar com as regras válidas | `COMPAT_FALHA MODULO_AUSENTE`, exit **1** | sim |
| 6 | Invocar `$BIN "$TMP/manifesto-aceito.json" "$TMP/regras-inexistentes.cfg"` | `COMPAT_FALHA REGRAS_AUSENTES`, exit **1** (`NotFound` → `None` → literal do contrato) | sim |
| 7 | Gerar `$TMP/manifesto-hash.json` idêntico ao aceito, porém com `content_hash` = 64 zeros; invocar com as regras válidas | `COMPAT_FALHA HASH_MODULO_DIVERGENTE`, exit **1** | sim |
| 8 | Repetir o passo 3 três vezes, redirecionando stdout para `$TMP/capturas/aceito-1-stdout.txt`, `-2-`, `-3-` e registrando os três exit codes; executar `diff` entre o arquivo 1 e os outros dois | `diff` vazio nas duas comparações e exit codes `0/0/0` — saída byte a byte idêntica | sim |
| 9 | Invocar o caso aceito com `module_id=insurgencia.exemplo` e `versao_local=1` explícitos; invocar com um caminho de manifesto inexistente | explícitos: igual ao passo 3, exit **0**; caminho inexistente: mensagem `erro ao ler manifesto`, exit **2** | sim |
| 10 | Inspecionar `crates/sim/examples/verificar_manifesto.rs`: busca por `unwrap`, `expect`, `unsafe` e `async` (com `grep -nE` usando alternância), presença de `forbid(unsafe_code)`/`deny(clippy::pedantic)`, verificação de linhas acima de 100 colunas e `git diff --name-only` | nenhuma ocorrência proibida; nenhuma linha >100 colunas; diff limitado ao exemplo e aos artefatos do ciclo — nenhum `Cargo.toml`, biblioteca ou `manifesto.json` alterado | parcial (inspeção automatizada + revisão humana da legibilidade) |

### Comandos de apoio (geração do manifesto aceito)

```bash
TMP=$(mktemp -d); mkdir -p "$TMP/capturas"
printf '%s\n' 'regras de teste 034' > "$TMP/regras.cfg"
HASH=$(sha256sum "$TMP/regras.cfg" | cut -d' ' -f1)
python3 -c 'import json,sys; d={"manifest_version":"1.0.0","protocol_version":1,"module_id":"insurgencia.exemplo","content_hash":sys.argv[2],"checked_at":"2026-11-22T00:00:00Z","tool_version":"4.7.2.stable.official"}; open(sys.argv[1],"w",encoding="utf-8").write(json.dumps(d,indent=2)+"\n")' "$TMP/manifesto-aceito.json" "$HASH"
cargo build -p insurgencia-sim --example verificar_manifesto
BIN=target/debug/examples/verificar_manifesto
"$BIN" "$TMP/manifesto-aceito.json" "$TMP/regras.cfg" > "$TMP/capturas/aceito-stdout.txt" 2> "$TMP/capturas/aceito-stderr.txt"; echo $? > "$TMP/capturas/aceito-exit.txt"
```

Para os demais casos, copiar `manifesto-aceito.json` e alterar somente o campo indicado no passo, com o mesmo `python3` (ou `sed` controlado), mantendo os outros cinco campos fixos.

## Plano de capturas (somente terminal/JSON — sem UI)

- Diretório temporário `$TMP/capturas/` (fora do worktree; removido no rollback).
- Por caso: `<caso>-stdout.txt`, `<caso>-stderr.txt`, `<caso>-exit.txt` — saídas cruas de terminal e código numérico.
- Fixtures JSON: `manifesto-aceito.json`, `manifesto-versao.json`, `manifesto-modulo.json`, `manifesto-hash.json`, `manifesto-vazio.json`, `manifesto-ruido.json`; regras: `regras.cfg`.
- Determinismo: `aceito-1|2|3-stdout.txt` e o par de saídas `diff-12.txt`, `diff-13.txt` (ambos vazios).
- Cada captura deve ser rotulada com `[NV]` até o orquestrador executar o passo; nenhuma captura existe neste ciclo.

## Rollback do roteiro

`rm -rf "$TMP"` remove fixtures e capturas; nenhum arquivo do repositório é modificado pela execução do roteiro. Não executar `git clean`, `git reset`, force-push nem remover artefatos do ciclo. Se um passo falhar, registrar a saída obtida e corrigir a **fonte** (código ou spec), nunca a expectativa do teste.
