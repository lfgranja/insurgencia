# Intake — Armadilha Improvisada (E do Artesão)

**Data**: 2026-11-24T12:15Z · **Slug**: `armadilha-improvisada` · **Sessão**: `20261124T121311Z-armadilha-improvisada`, `session_title=insurgencia - armadilha improvisada`.

## Ideia (como capturada)

> Capturar, sem executar nada neste assessment, a fatia 80/20 do ciclo 057: tornar **observável no cliente standalone e verificável no Núcleo determinístico** a **Armadilha Improvisada** (E do Artesão), prevista em `docs/GDD - Insurgência.md` §10.3:588-593 como `[HIPÓTESE]`: 25 Fôlego + 5 Materiais, 1 s de arme, 3 m de alcance, raio 1 m, imobilização de 2 s, duração de 60 s, máximo de três ativas e aliados que não a ativam. A decisão 80/20 das 12:15Z recorta: ação `armadilha` nível 3, `Armadilha { id, dono, posicao, ticks_armar, ticks_restantes }` em `Mundo.armadilhas: BTreeMap<u64, Armadilha>` com `proxima_armadilha_id`, `Jogador.equipe: u8`, `armadilha_recarga_ticks` e `imobilizado_ticks`, validação na ordem destino/alcance → Fôlego → Materiais → recarga com `CodigoRecusa::ArmadilhaEmRecarga`, fase `avancar_armadilhas()` entre `avancar_escudos()` e `fase_skills()`, disparo em inimigo a `<= 1 m` com `equipe != dono`, remoção da armadilha, imobilização por 60 ticks e expiração em 1800 ticks; `mover` ignorado enquanto imobilizado, hash cobrindo os campos novos e as armadilhas em ordem de `id`, tecla `E` com despacho por `Funcao::Artesao`, bloqueio local de `mover_jogador`/`sincronizar_sim` e HUD `Armadilhas: N`. Cinco testes nomeados elevam o inventário de **146 para 151**.

Fonte colada (lida, não reexecutada): `.specify/memory/STATE.md`, entradas `2026-11-24T12:15Z — auditoria, backlog e decisão 80/20 do ciclo 057` e `DoD (0.9)` do mesmo ciclo. Nenhuma URL foi buscada; nenhuma fonte externa foi usada. Todo número citado é `[HIPÓTESE]`/`escolha de protótipo` do GDD e da decisão, nunca cânone. Saídas de gates, build, testes, roteiro e CI permanecem **[NV]**.

## Restated

Propõe-se acrescentar ao protótipo determinístico uma arma de controle de área que não causa dano: o Artesão arma um artefato na própria posição, que após 30 ticks imobiliza o primeiro inimigo que entra em 1 m por 60 ticks e expira em 1800 ticks, respeitando imunidade de aliados por `equipe` e limite de três por jogador. O ganho esperado é duplo: o jogador do cliente passa a ver `Armadilhas: N` e a sofrer/observar imobilização de forma auditável, e o Núcleo passa a ter estado de armadilha e imobilização determinísticos, serializados no hash, sem tocar contrato, protocolo, stack ou rede.

## Contexto do ciclo (observado no contrato da sessão; não reexecutado)

- Worktree isolado `.specify/queue/worktrees/armadilha-057/`, branch `feature/057-armadilha-improvisada`, base `origin/dev = 661890f`; sessão `20261124T121311Z-armadilha-improvisada`. Alvo de especificação `specs/057-armadilha-improvisada/`; rastros FR-001..FR-006, SC-001..SC-006, T001..T012. Assess `.specify/assessments/armadilha-improvisada/`, validação `.specify/validations/armadilha-improvisada/`, release `.specify/releases/armadilha-improvisada/`.
- Âncora GDD §10.3:588-593 (E do Artesão, `[HIPÓTESE]`): 25 Fôlego e 5 Materiais, execução 1 s, alcance 3 m, raio 1 m, imobilização 2 s, duração 60 s, máximo 3, aliados imunes, recarga 14 s. A conversão para ticks adota `PASSO_SEGUNDOS = 1/30`: arme **30 ticks**, imobilização **60 ticks**, duração **1800 ticks** e recarga **420 ticks** (escolha de protótipo derivada dos 14 s do GDD). Os critérios de aceitação do GDD falam em "sem dano mágico", "aliados e Súditos aliados atravessam sem ativar" e "expira após 60 s mesmo sem ativação" — os três primeiros entram no corte; Súditos aliados são cobertos pelo predicado de `equipe`, não por um novo subsistema.
- Estado verificado no checkout no momento da leitura: `crates/sim/src/lib.rs:190-202` tem 13 ações em `ACOES_PERMITIDAS`, sem `armadilha`; `nivel_minimo_para_acao` (`sim:214-220`) já mapea Q1/W2/E3/R6; `Jogador` (`sim:281-...`) não tem `equipe`, `armadilha_recarga_ticks` nem `imobilizado_ticks`, e `Mundo` (`sim:462-...`) não tem `armadilhas`/`proxima_armadilha_id`; `avancar_tick` (`sim:768-790`) chama `avancar_analises_materialistas()` e `avancar_escudos()` antes de `fase_skills()`, sem fase de armadilhas; `hash_estado` (`sim:1903-1949`) serializa jogadores e estruturas sem os campos novos; `CodigoRecusa` (`sim:1953-1991`) não tem `ArmadilhaEmRecarga`. No cliente, `E` agenda `analise_materialista` (`cliente:923-955`), `texto_hud` (`cliente:1123-1221`) tem 99 linhas e não exibe `Armadilhas:`; `mover_jogador` (`cliente:612`) e `sincronizar_sim` (`cliente:1042`) não consultam imobilização.
- Constituição v1.2.2: R1 `go`; R3 = 0 clarificações; R4 antes de R7 `1.0.0-local`; `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A/bloqueado** (BF-007, sem `resolve-tasks.sh`/adaptador). Promoção `dev→main` e deploy são SWE-exclusivos.
- Baseline: **146 testes** determinísticos ancorados em `661890f`; o corte prevê **5 novos** nomeados (`armadilha_custa_25_folego_e_5_materiais`, `armadilha_recusa_destino_alem_de_3_m`, `armadilha_imobiliza_inimigo_por_60_ticks_e_poupa_aliados`, `armadilha_expira_apos_1800_ticks_e_limite_de_tres`, `hash_reflete_armadilha_e_imobilizacao`) → **151**.

## Scorecard único do ciclo

| Candidato | I × C × Fch × E⁻¹ × R⁻¹ ÷ 3125 | Resultado | Consumidor |
|:--|:--|:--|:--|
| `armadilha-improvisada` (BF-002b) | 5×4×4×3×3 = 720/3125 | **720/3125** | jogador do cliente Bevy (Artesão) |
| Mapa/rotas de PVE (BF-003) | 5×3×3×2×3 = 270/3125 | 270/3125 | jogador/servidor |
| HUD republica `hash_estado` (BF-014) | 2×5×1×5×5 = 250/3125 | 250/3125 | jogador (diagnóstico) |
| Cliente agenda `mover` (BF-012) | 4×3×3×2×2 = 144/3125 | 144/3125 | jogador (correção de plumbing) |
| `contratos` consumido por binários (BF-013) | 5×2×5×1×2 = 100/3125 | 100/3125 | protocolo/servidor |
| Rede real (BF-001) | 5×2×5×1×1 = 50/3125 — **inelegível** (autorização ADR-004) | 50/3125 | SWE decide |

Critério normativo: `impacto × confiança × fechamento × esforço⁻¹ × risco⁻¹`, escala 1–5, denominador **5⁵ = 3125**. Filtro eliminatório: comportamento observável + consumidor fora dos artefatos do ciclo. Impacto **5** (controle de área jogável com âncora escrita no GDD e efeito visível), confiança **4** (números e rotas lidos, todos rotulados `[HIPÓTESE]`), fechamento **4** (jogador do Artesão e testes da sim nomeados), esforço⁻¹ **3** (sim + cliente + 5 testes, sem stack nova), risco⁻¹ **3** (nova fase de tick, imobilização, cota e hash por verificar; fronteira sem contrato/protocolo/Cargo/rede). Reescalonamento registrado na decisão: BF-014 cai de 1125 para 250 porque exibir hash não muda o jogo. Este é o **único** scorecard do assessment: `research.md`, `shape.md` e `decision.md` repetem os mesmos valores, sem pontuação paralela.

## Incógnitas de primeira vista

- **Nenhuma clarificação bloqueante (R3 = 0)**: o contrato fixa constantes, conversão em ticks, ordem de validação, fase do ciclo, imunidade por `equipe`, tecla, bloqueio local e formato da HUD. Não há `[NEEDS CLARIFICATION]` de produto a responder neste assessment.
- Incógnitas operacionais, não de produto, ficam **[NV]** até verificação: efeito renderizado da HUD com `--minimizado` e `texto_hud` abaixo de 100 linhas; nomes definitivos de T001..T012 em `specs/057-armadilha-improvisada/tasks.md`; conformidade de R4 e R7 com gates não executados aqui.

## Restrição de execução

Este agente escreve **somente** os 5 artefatos em `.specify/assessments/armadilha-improvisada/`. Não executa gates, build, testes, formatadores, `./verify.sh`, comandos de CI, git remoto ou GitHub; não faz commit, push ou PR; não toca `crates/sim`, `crates/cliente`, `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `STATE.md`, `AGENTS.md`, GDD, specs, validações, releases, `.specify/queue/concurrency.jsonl` nem qualquer código-fonte. Tudo o que depende de execução permanece **[NV]**. **R6 não é invocada** — N/A por constituição v1.2.2 (BF-007): PARE se invocada, nunca issue simulada, nunca `gh` improvisado. Promoção `dev→main` e deploy são SWE-exclusivos; este ciclo termina no escalonamento, não em merge em `main`.
