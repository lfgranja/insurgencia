# Intake — Papéis, Níveis e Desbloqueio Q/W/E/R

**Data**: 2026-11-23T15:08Z · **Slug**: `papeis-niveis` · **Sessão**: `session_id=20261123T150800Z-papeis-niveis`, `session_title=insurgencia - papeis e niveis`.

## Ideia

Capturar, sem tocar em código, a fatia 80/20 do ciclo 052: um **modelo mínimo de papéis materiais e progressão por nível** no Núcleo Rust, com o desbloqueio das **ações já existentes** segundo o GDD §10.1: a tecla Q (`atacar_forte`) no nível 1, W (`construir_barreira`/`construir_ponte`) no nível 2, E (`propaganda`/`propaganda_no`) no nível 3 e R (`atacar_no`) no nível 6. O GDD §10.1 fixa também o teto de nível **12** e o desbloqueio Q1/W2/E3/R6, todos explicitamente rotulados **`[HIPÓTESE]`** — são escolhas de protótipo aceitas como valores de implementação, não dogmas canônicos a renegociar neste assessment.

Implementação mínima prevista no contrato (sem contratos, protocolo, Cargo, stack ou rede novos): enum `Funcao` com os cinco papéis materiais do GDD §10.1 (Artesão, Guerreiro, Intelectual, Camponês, Agitador); campos `Jogador.funcao` e `Jogador.nivel`; constantes `TICKS_POR_NIVEL = 900` (**30 s a 30 Hz**) e `NIVEL_MAXIMO = 12`; nível inicial **1** e ganho de **+1 a cada 900 ticks**, limitado ao teto; `ValidacaoComandos::validar` rejeita ação bloqueada com `CodigoRecusa::AcaoBloqueada`; cliente exibe no HUD `Função: <nome>` e `Nível: N/12`, **sem novas teclas**.

**Motivação**: hoje o Núcleo não sabe qual papel material o jogador exerce nem há quanto tempo ele progride, de modo que Q/W/E/R estão indiscriminadamente disponíveis em `ACOES_PERMITIDAS`; o recorte torna a progressão do GDD observável e verificável por estado determinístico e por recusa de comando, reaproveitando ações já validadas — sem criar as habilidades W/E/R do §10.2–§10.5 (Oficina de Campo, Análise Materialista, Plano Mestre etc.), que ficam fora de escopo.

## Contexto do ciclo (observado no contrato da sessão e em `STATE.md`; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/papeis-052/`, branch `052-papeis-niveis`, base `4f3966e`; alvo de especificação `specs/052-papeis-niveis` (o diretório contém apenas o template inicial de `spec.md`).
- Rastros do ciclo: FR-001..FR-006, SC-001..SC-006, T001..T012.
- R1 `go`; R3 = 0 clarificações; R4 antes de R7 (`1.0.0-local`); **R6 N/A** por constituição v1.2.2 — fora da sequência executável até existir adaptador de issues; PARE se invocada, nunca issue simulada.
- Constituição v1.2.2 e GDD como âncora de valores; nenhuma troca de stack, engine, dependência ou protocolo (`ADR-004`).

## 80/20 (decisão de escopo)

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Papéis + níveis + desbloqueio Q/W/E/R (escolhida) | 4×2×2×3 = 48/625 | **48/625** |
| PVE/PNJs | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (quarto fator = risco invertido/segurança), escala 1–5, denominador **625**. Confiança **2** porque os números do §10.1 são `[HIPÓTESE]`; facilidade **2** porque a progressão e o mapeamento ainda não existem no Núcleo; risco invertido **3** porque fronteira de tick, validação e hash exigem verificação.

Escolhe-se `papeis-niveis`: o contrato da sessão a nomeia e os rastros FR/SC/T já previstos cobrem o modelo mínimo sem tocar em contrato, protocolo ou stack. Há empate numérico com PVE/PNJs (ambos 48/625); o desempate é do contrato e do lastro documental — o §10.1 traz valores escritos para papéis, teto e desbloqueio, enquanto PVE/PNJs dependem de IA de defensores, mapa e alvos ainda não implementados. A rede, embora de impacto 5, segue bloqueada por ADR-004 (Lightyear ausente de `Cargo.toml`/`Cargo.lock`). O mesmo candidato "Papéis W/E/R + níveis" apareceu a 48/625 nos ciclos 049, 050 e 051 e foi adiado em favor de ilhas, Nós e derrota; a mudança agora é de prioridade, não de pontuação.

## Evidências de entrada (coladas; não reexecutadas)

- GDD §10.1:515-524 — tabela de convenções: "Desbloqueio | Q no nível 1, W no 2, E no 3, R no 6 ... `[HIPÓTESE]`" e "Teto de nível | 12 ... `[HIPÓTESE]`"; o §10.1:504-508 define funções como "papéis materiais na insurgência, não classes mágicas" (P2 — Materialismo Histórico).
- GDD §10.1:506 — os cinco papéis materiais usados no jogo base: Artesão, Guerreiro, Intelectual, Camponês, Agitador (aparecem também em §7.1:303-307, §8.2:386-409 e §10.2–§10.5). Todos os valores de progressão do §10.1 são `[HIPÓTESE]`.
- GDD §10.2–§10.5 — as habilidades completas Q/W/E/R de cada papel (Oficina de Campo, Análise Materialista, Plano Mestre, Fuga pelas Sombras etc.) permanecem `[HIPÓTESE]` e são explicitamente **fora de escopo**: aqui só o desbloqueio das ações já existentes no Núcleo.
- `.specify/memory/STATE.md` — decisão 80/20 do ciclo 052 (2026-11-23T15:08Z): Papéis + níveis 4/2/2/3 → **48/625**; PVE/PNJs 4/2/2/3 → 48/625; rede 5/2/1/1 → 10/625; escolha `papeis-niveis`; implementação mínima com campo `funcao`, nível por pulso, desbloqueio de ações existentes e HUD; novas habilidades W/E/R ficam para ciclo posterior; R6 N/A.
- Código observado (leitura documental, nada executado): `crates/sim/src/lib.rs` já expõe `ACOES_PERMITIDAS` com `atacar_forte`, `construir_barreira`, `construir_ponte`, `propaganda`, `propaganda_no` e `atacar_no`, o enum `CodigoRecusa` e `ValidacaoComandos::validar`; `Jogador` ainda não tem `funcao` nem `nivel`; o cliente (`crates/cliente/src/main.rs`) já solicita as ações sem HUD de papel/nível.
- Contrato da sessão `20261123T150800Z-papeis-niveis` (imutável): `Funcao`, `Jogador.funcao`, `Jogador.nivel`, `TICKS_POR_NIVEL = 900`, `NIVEL_MAXIMO = 12`, progressão +1/900 ticks, mapeamento Q1/W2/E3/R6, `CodigoRecusa::AcaoBloqueada`, HUD `Função: <nome>` / `Nível: N/12`.

## Restrição de execução

Agente escreve **somente** os 5 artefatos em `.specify/assessments/papeis-niveis/`. Não executa gates, build, testes, formatadores, git remoto, GitHub ou emissão de issue; não faz commit/push/PR; não toca `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml` nem código-fonte. Tudo o que depende de execução permanece **[NV]**.
