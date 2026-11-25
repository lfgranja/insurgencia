# Research — Coleta Eficiente (BF-002k)

**Data**: 2026-11-25 · Fonte primária lida nesta sessão; nada reexecutado. Âncora `origin/dev = d3be560`.

## Evidência primária
- [source: `docs/GDD - Insurgência.md:646`, lido] **Q — Coleta Eficiente**: passiva "+25% de Ouro e +25% de experiência em pontos neutros"; ativa "aumenta a velocidade de coleta"; **custo 15 Fôlego**, execução 0,2 s, alcance "ponto de coleta a 2 m", **recarga 10 s**; **efeito "ativa dobra a taxa de coleta por 5 s"**. Critérios de aceitação: "+25% de Ouro e experiência", "dobra a taxa de coleta por 5 s e não se acumula com outra ativação", "nenhum bônus incide sobre pontos controlados pelo time inimigo". Bloco inteiro marcado **`[HIPÓTESE]`** — os números são protótipo, não cânone.
- [source: `crates/sim/src/lib.rs`, lido] `TICKS_COLETA: i64 = 180` (linha 18; `PASSO_SEGUNDOS = 1/30`); `Jogador.coleta_ticks` (linha 354) e `folego`/`folego_ticks` (373–375), `FOLEGO_MAXIMO = 100` (106). `ACOES_PERMITIDAS` (233–249) tem **15 tokens** — `mover`, `sinalizar_sincronia`, `sinalizar_alavanca`, `coletar_materiais`, `construir_barreira`, `atacar_forte`, `construir_ponte`, `propaganda`, `propaganda_no`, `atacar_no`, `atacar_pnj`, `analise_materialista`, `escudo`, `armadilha`, `vandalismo` — e **não contém `coleta_eficiente`**: hoje não há sequer token para a ação.
- [source: `crates/sim/src/lib.rs`, lido] `aplicar_coleta` (1559–1582) exige `destino` igual à posição do jogador, preserva o acúmulo no mesmo ponto, incrementa `coleta_ticks` no máximo uma vez por tick e chama `concluir_coleta` ao atingir 180; `avancar_coletas_automaticas` (1585–1608) faz o mesmo sem comando e zera o acúmulo se o jogador sai do ponto; `concluir_coleta` (1611–1619) credita atomicamente +3 suprimentos e +2 materiais e limpa `coleta_ticks`/`coleta_ponto`. **É aqui que o buff de 150 ticks deve alterar o limiar de 180 para 90 ticks**, sem tocar no relógio (`avancar_tick` não possui clock próprio).
- [source: `crates/sim/src/lib.rs`, lido] `serializar_jogadores` (2141–2156) já inclui `coleta_ticks`, `coleta_ponto`, `folego`, `folego_ticks` e as recargas; `hash_estado` (2348) é SHA-256 canônico sobre os campos serializados. Buff e recarga novos precisam entrar nessa serialização, senão estados diferentes colidem no hash.
- [source: `.specify/memory/STATE.md:625-635`, lido] nota `2026-11-25T06:40:58Z`: decisão 80/20 do ciclo 062, **BF-002k 1024/3125 (4×4×4×4×4)**; escolha de novo token `coleta_eficiente` (whitelist com 16 tokens), com destino, nível 1, custo 15 Fôlego, buff 150 ticks (90 vs 180), recarga 300 ticks, sem acumulação; passiva de Ouro/XP fora de escopo; tecla `K` no cliente, HUD derivada do espelho; R1 `go` registrado; R6 fora da sequência executável; PR #76 SWE-exclusivo.
- [source: git/worktree] HEAD `d3be560`, branch `feature/062-coleta-eficiente`, `.specify/feature.json → specs/062-coleta-eficiente`. Estado remoto vivo e CI **[NV]**.

## Interpretação
A lacuna não é de regra documentada: o GDD já dá os números e o sim já tem a máquina de coleta. Falta a **aresta entre os dois** — token whitelistado, validação com destino/Fôlego/buff/recarga, mutação determinística do limiar de coleta e reflexo no hash, além de um consumidor observável (CLI e cliente). Como os números são `[HIPÓTESE]`, a implementação é protótipo: ela não inventa cânone e não altera GDD/ADR/protocolo. O risco está concentrado no estado autoritativo (hash, IDs, acumulação), não em transporte.

## Evidence Against
- **Passiva Ouro/XP sem primitiva.** `GDD:646` pede +25% de Ouro e +25% de experiência, mas `Jogador` em `crates/sim/src/lib.rs` não possui campo de Ouro ou XP (há `materiais`, `suprimentos`, `pontos_autogestao`, `folego`). Implementar a passiva exigiria criar uma economia nova e uma decisão canônica; por isso ela fica **fora de escopo** e o ciclo entrega apenas a parte ativa.
- **Risco no hash.** `serializar_jogadores`/`hash_estado` são a base do determinismo entre pares. Inserir buff e recarga sem serializá-los faria estados distintos produzirem o mesmo SHA-256; mudar o limiar para 90 ticks altera também o tick de crédito e, portanto, hashes intermediários. Golden hashes e testes precisam ser atualizados **juntos**, e qualquer uso de relógio, aleatoriedade ou acumulação quebra a equivalência sim/CLI/cliente.
- **Sem partida real.** `crates/contratos` é declarativo (sem transporte); Lightyear aparece apenas em `docs/adr/004-migracao-bevy-lightyear.md` e está ausente de manifests e `Cargo.lock`. Não há rede, lobby, matchmaking ou 5v5 executável: o "efeito" só pode ser provado em sim determinístico + CLI + cliente local. Impacto em partida real ou captura visual da HUD permanece **[NV]**.

## Alternativas rejeitadas
1. Implementar também a passiva Ouro/XP — rejeitada: sem primitiva no núcleo e fora do DoD do ciclo (ver *Evidence Against*).
2. Apenas whitelistar o token sem efeito — rejeitada: nenhum comportamento observável e nenhum consumidor; não fecha o DoD (ver `concept.md`, Opção B).
3. Definir topologia/ordem de ilhas ou partida 5v5 para "contextualizar" a habilidade — rejeitada: decisão canônica ausente em `GDD:376,381,1502` (BF-015b/BF-003b inelegíveis).
4. Usar tempo de parede, `rand` ou `HashMap` no buff — rejeitada: quebra determinismo e as convenções do repositório.
5. Ativar R6/criar issues — rejeitada: R6 ratificada mas fora da sequência executável; PARE se invocada, nunca simular issue.

## Rastreabilidade
Fontes: `docs/GDD - Insurgência.md:646`, `crates/sim/src/lib.rs`, `.specify/memory/STATE.md:625-635`, git/worktree; todas lidas, nenhuma reexecutada. Gates, testes, build, commit, push, PR, merge e deploy não executados por este agente; resultados de execução permanecem **[NV]**. Nenhum arquivo além dos cinco artefatos deste assessment foi tocado.
