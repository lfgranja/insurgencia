# Roteiro de Teste Solo — Validação do MVP sem equipe

> **Nota datada (2026-11-20).** Este roteiro registra o estado verificado em 2026-11-20T10:58Z: MVP completo, 22 PRs mesclados, 39/39 caminhos canônicos, 131 `PASS` em 11 suítes, `make check` exit 0 e `make test-rede-e2e` exit 0 com dois processos `processoCliente` em 127.0.0.1:7780. É um registro descritivo, não uma promessa de execução remota. O que não foi executado continua **[NV]** (não verificado).

## Público

Desenvolvedor **solo** que precisa validar o MVP do Insurgência **sem equipe**, sem servidor dedicado e sem credenciais remotas. Tudo aqui roda em uma única máquina Linux, em modo headless (sem janela gráfica). O objetivo é reproduzir, de forma determinística, as mesmas evidências que sustentam o checklist 16.7 do GDD.

## Pré-requisitos

- **Godot 4.7.2 em caminho absoluto.** O binário desta máquina está fora do `PATH`: `/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64`. Defina `G` uma vez no início da sessão (o caminho absoluto evita ambiguidade com qualquer `godot` instalado):
  ```bash
  G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
  test -x "$G" && echo GODOT_OK
  ```
- **Bash** (`#!/usr/bin/env bash`) disponível; os scripts usam `set -euo pipefail`.
- **Porta UDP 7780 livre em 127.0.0.1** para o E2E de rede. O `test_servidor` isolado usa a porta padrão 7777 em processo (sem escuta de rede). Se outra instância estiver ocupando 7780, encerre-a antes de rodar o E2E (o script já faz `trap` de limpeza do PID do servidor que ele mesmo inicia).
- **Diretório do projeto** como cwd (onde estão `Makefile`, `nucleo/`, `tests/`).

> Opcional: `bash tests/roteiro_solo.sh` executa todas as etapas abaixo de uma vez e imprime `PASS`/`FAIL` por etapa, saindo 0 somente se tudo passar. O passo a passo a seguir explica cada comando para execução manual.

## O que dá para testar sozinho

Toda a lógica determinística do Núcleo roda em processo, com tempo injetado (`tick * PASSO_SEGUNDOS`, 30 Hz) — sem relógio do sistema, sono, aleatoriedade ou interface:

- **Contrato e caminhos canônicos** (CE-1): `test_contrato` — 16 `PASS`, 39/39 caminhos presentes.
- **Servidor e validação** (CE-2): `test_servidor` (6 `PASS`, `SERVIDOR_PRONTO`), `test_loopback` (8 `PASS`), `test_validacao` (9 `PASS`). Aceita só intenções normalizadas; recusa versão/hash divergentes.
- **Ilhas** (CE-3): `test_ilha_sincronia` (14 `PASS`), `test_loopback_ilha` (3 `PASS`), `test_ilha_topo` (16 `PASS`), `test_ilha_inferior` (16 `PASS`).
- **Funções e vitória**: `test_funcoes` (18 `PASS`) e `test_vitoria_central` (18 `PASS`) cobrem Artesão (20 Materiais/1,5 s/200 PV/12 s), Guerreiro (2×/1,2 s/6 s) e a condição de vitória com 3 ilhas, 4 s de canalização conjunta e decisão do servidor.
- **Manifesto e compatibilidade** (CE-4): `test_manifesto` (7 `PASS`) e `verificar_compatibilidade` (`COMPAT_OK module_id=insurgencia.exemplo`).
- **Rede real local** (CE-2.1): `make test-rede-e2e` sobe um servidor headless e conecta **dois processos UDP reais** em 127.0.0.1:7780.

## O que exige 2 jogadores e como é coberto por simulação headless

Duas regras do GDD são, por natureza, multiagente — mas **nenhuma exige duas pessoas** neste roteiro:

| Regra | Por que "exige 2 jogadores" | Cobertura sem equipe |
|---|---|---|
| **Ilha Meio** (`jogadores_necessarios=2`, 2 placas em 5 s) | A captura depende de dois `peer_id` distintos pressionando placas dentro da janela | `tests/test_ilha_sincronia.gd` (14 `PASS` + `OK test_ilha_sincronia`) e `tests/test_loopback_ilha.gd` (3 `PASS`, `DELTA_EVENTO_MS 0`) simulam os dois peers com tempo injetado. O E2E de rede complementa com 2 processos UDP reais (`join_aceito_1`, `join_aceito_2`). |
| **Vitória da Ilha Central** (Artesão + Guerreiro vivos e presentes, 4 s) | A canalização exige duas funções distintas identificadas por `peer_id`; Caudillo/Camponês não substituem | `tests/test_vitoria_central.gd` (18 `PASS`) fixa `peer_id` 2 = Artesão e 3 = Guerreiro, avança 3,99 s (sem vitória) e 4,0 s (vitória declarada pelo servidor), recusando funções ausentes/mortas. `tests/test_funcoes.gd` (18 `PASS`) valida as duas funções isoladamente. |

Ou seja: a "segunda pessoa" é substituída por um **segundo `peer_id` determinístico** nos testes, e pelo **segundo processo cliente** no E2E de rede. Não há lobby, matchmaking 5v5 nem NAT neste escopo.

## Passo a passo (comandos copiáveis)

1. **Defina o caminho absoluto do Godot e confirme o executável:**
   ```bash
   G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64
   test -x "$G" && echo GODOT_OK
   ```

2. **Rode os 5 gates determinísticos (contrato, servidor, validação, ilha, manifesto) e o gate de compatibilidade** — todos via `make check`:
   ```bash
   make check GODOT=$G
   ```
   Confira a saída: `OK test_contrato`, `OK test_servidor`, `OK test_validacao`, `OK test_ilha_sincronia`, `OK test_manifesto` e `COMPAT_OK module_id=insurgencia.exemplo`; nenhuma linha `FAIL`/`FALHAS`/`COMPAT_FALHA`; código de saída `0`.

3. **Rode o E2E de rede com dois processos cliente:**
   ```bash
   make test-rede-e2e GODOT=$G
   ```
   Confira `SERVIDOR_PRONTO`, `PASS servidor_pronto`, `PASS join_aceito_1`, `PASS comando_enviado_1`, `PASS cliente_1`, `PASS join_aceito_2`, `PASS comando_enviado_2`, `PASS cliente_2` e `OK test_e2e_rede`. O script encerra o servidor ao final (`trap`).

4. **Verifique a evidência auditável em `docs/progress/evidence.csv`:**
   ```bash
   column -s, -t docs/progress/evidence.csv 2>/dev/null || cat docs/progress/evidence.csv
   ```
   Esperado: 9 colunas (`id,criterio,verdict,source,command,result,confidence,unverified,recorded_at`), linhas `EV-20261119-001`..`EV-20261119-007` com `verdict=PASS`/`confidence=high`, `EV-20261120-008` (CE-2.1-REDE) `PASS` e `EV-20261120-009` (CE-2.1-NAT) `NAO_VERIFICADO` com `unverified` preenchido. A coluna `unverified` só deve estar vazia nas linhas `PASS`.

5. **Conferir o checklist 16.7 do GDD** (`docs/GDD - Insurgência.md`, §16.7, linhas ~1569–1600): todos os itens de governança, fatia 80/20 (CE-1..CE-5) e critérios de aceitação (§16.4) devem estar `[x]`, com os números de `PASS` e os marcadores `COMPAT_OK`/`SERVIDOR_PRONTO`/E2E conferindo com o observado nas etapas 2–4. Os itens de Fase 5 e deploy/remoto permanecem `[ ]`/**[NV]**.

6. **Registre o resultado datado** (ver §"Como registrar resultado datado" abaixo). Se preferir automação, rode `bash tests/roteiro_solo.sh` do passo 1 ao 5 e guarde a saída.

## Saída esperada (resumo)

| Etapa | Trecho da saída | Interpretação |
|---|---|---|
| `make check` — gate contrato | `OK test_contrato` | 16 `PASS`, contrato íntegro |
| `make check` — servidor | `OK test_servidor` (+ `SERVIDOR_PRONTO porta=7777` em processo) | 6 `PASS`, join/validação/recusa OK |
| `make check` — validação | `OK test_validacao` | 9 `PASS`, só intenções aceitas |
| `make check` — ilha | `OK test_ilha_sincronia` | 14 `PASS`, Ilha Meio com 2 peers simulados |
| `make check` — manifesto | `OK test_manifesto` | 7 `PASS`, manifesto íntegro |
| `make check` — compatibilidade | `COMPAT_OK module_id=insurgencia.exemplo` | hash do manifesto confere |
| `make test-rede-e2e` — subida | `SERVIDOR_PRONTO` / `PASS servidor_pronto` | servidor headless aceitou a porta 7780 |
| `make test-rede-e2e` — cliente 1 | `PASS join_aceito_1` / `PASS comando_enviado_1` / `PASS cliente_1` | 1º peer UDP real entrou e enviou comando |
| `make test-rede-e2e` — cliente 2 | `PASS join_aceito_2` / `PASS comando_enviado_2` / `PASS cliente_2` | 2º peer UDP real entrou e enviou comando |
| `make test-rede-e2e` — fim | `OK test_e2e_rede` | E2E completo, exit 0 |

Resumo: **5 `OK` + `COMPAT_OK` + `SERVIDOR_PRONTO` + `join_aceito_1`/`join_aceito_2` + `OK test_e2e_rede`**.

## Interpretando `FAIL` e timeout

- **`FAIL <rotulo>` ou `FALHAS n`** em qualquer gate: o teste determinístico encontrou divergência de regra (ex.: Ilha Meio fora da janela de 5 s, vitória antes de 4,0 s, hash de manifesto diferente). O script sai `1`. Investigue o `rotulo` no `.gd` correspondente antes de rodar o E2E.
- **`COMPAT_FALHA`**: hash/manifesto divergente. Não rode o E2E até corrigir; o gate de compatibilidade é autoritativo.
- **`FAIL servidor_pronto`** no E2E: o `grep` não achou `SERVIDOR_PRONTO` no log em até ~5 s. O script imprime `/tmp/servidor-e2e.log` — leia ali o erro do Godot (binário inexistente, porta 7780 ocupada, erro de parse).
- **`FAIL cliente_1`/`cliente_2`**: o `timeout 10s` matou o cliente ou ele retornou não-zero — normalmente servidor caiu, porta bloqueada por firewall local ou `join` recusado (versão/hash divergentes). Confira `SERVIDOR_PRONTO` e a livreza da porta.
- **timeout sem mensagem**: use `timeout 15s` no servidor e `timeout 10s` nos clientes como referência (valores de `tests/run_e2e_rede.sh`). Tempo esgotado é `FAIL`, nunca `PASS` — não reinterprete ausência de saída como sucesso.

## Limpeza (trap e portas)

- `tests/run_e2e_rede.sh` instala `trap limpeza EXIT`, `trap 'exit 130' INT` e `trap 'exit 143' TERM`; ao sair, mata e aguarda o PID do servidor que iniciou. Se o processo foi interrompido na mão, confirme:
  ```bash
  pgrep -af 'main_server.gd|test_cliente_rede.gd' || echo SEM_PROCESSOS_E2E
  ```
- Se sobrar algo escutando 7780, encerre apenas o PID encontrado (não use `killall godot`, para não afetar outro trabalho). O log do E2E fica em `/tmp/servidor-e2e.log` e pode ser removido após a inspeção.
- `tests/roteiro_solo.sh` não altera arquivos do projeto, não usa `git` e não instala nada; imprime apenas resumo em stdout/stderr.

## Limites honestos

- **Sem UI gráfica:** toda a validação é headless (`--headless --script`). HUD, câmera e input de tela não são exercitados — a tela apenas exibiria o resultado que o Núcleo já persistiu.
- **Sem 5v5 / lobby / NAT:** não há partidas de 10 jogadores, matchmaking ou travessia de NAT. O E2E cobre **2 peers locais em 127.0.0.1:7780**; a entrada de clientes externos é **[NV]** (`EV-20261120-009`).
- **Sem Internet / credenciais remotas:** Postgres/Coolify remotos, Zitadel, `pgmq`, deploy/publicação e federação forte estão fora do escopo e permanecem **[NV]** / `[ ]` no checklist 16.7. Nenhum comando deste roteiro toca ambiente remoto.
- **Registro não é alegação remota:** `[x]` no checklist significa comando executado localmente com saída colada; não afirma execução em CI ao vivo sem confirmação.

## Como registrar resultado datado

1. Crie/atualize um registro em `docs/progress/` com título contendo a data, no padrão `2026-11-20-validacao-solo.md`, referenciando este roteiro, `docs/progress/evidence.csv` e o §16.7 do GDD.
2. Cole a saída observada das etapas 2–5 (os `OK`, `COMPAT_OK`, `SERVIDOR_PRONTO`, `join_aceito_1/2`, `OK test_e2e_rede`) com o timestamp ISO-8601 da execução, por exemplo `2026-11-20T11:00Z`.
3. Se houver nova evidência, acrescente linha em `evidence.csv` com `id` datado (ex.: `EV-20261120-010`), `verdict=PASS`, `source`, `command`, `result`, `confidence=high`, `unverified` vazio e `recorded_at` preenchido. Se algo não foi verificado, use `verdict=NAO_VERIFICADO` com `unverified` descrevendo o limite e **deixe `result`/`recorded_at` vazios** — nunca transforme `[NV]` em `PASS` por inferência.
4. Atualize a nota datada no topo deste arquivo se o estado verificado mudar, mantendo a convenção de não alegar o que não foi executado.
