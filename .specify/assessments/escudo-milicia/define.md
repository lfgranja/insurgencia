# Define — Escudo de Milícia (W do Guerreiro)

- **Slug**: `escudo-milicia`
- **Created**: 2026-11-23T22:00Z
- **Inputs used**: `intake.md` | `research.md` | contrato da sessão (decisão 80/20 e DoD em `.specify/memory/STATE.md`; GDD §10.6:687-692; `crates/sim/src/lib.rs`; `crates/cliente/src/main.rs`; constituição v1.2.2)
- **Scorecard único do ciclo**: `escudo-milicia` **960/3125** (4×4×5×3×4); mapa/rotas PVE 216/3125; rede 50/3125 (inelegível, ADR-004); rastreabilidade já fechada 0. Nenhuma pontuação paralela.

## Problem Statement

O GDD prevê o Escudo de Milícia (W do Guerreiro) como `[HIPÓTESE]`, mas o protótipo determinístico não o torna observável: `crates/sim/src/lib.rs` não tem ação `escudo`, constantes, campos de absorção nem rota única de dano ao jogador — o PvP debita `vida` diretamente em `aplicar_ataque` e o PNJ em `fase_ataque_pnjs` —, `hash_estado` não vê estado de escudo e a HUD do cliente não exibe absorção, embora a tecla `V` esteja livre. Sem esse recorte, o jogador não tem defesa verificável contra dano já implementado e o Núcleo não consegue auditar absorção nenhuma. O problema é de **observabilidade e autoridade do estado de defesa**, não de nova stack, regra canônica ou rede.

## Affected Users & Stakeholders

- **Users**: jogadores no cliente standalone — precisam ver `Escudo: N (ticks)` e perceber que dano de PNJ/PvP é absorvido por até 250 por 5 s, com recarga de 14 s e um alvo só; quem inspeciona a sim — precisa de um estado de escudo determinístico, serializado no hash, com expiração e recarga auditáveis.
- **Stakeholders**: mantenedores do Núcleo Rust — precisam de uma única rota `aplicar_dano_a_jogador` usada por PvP e PNJ, sem dupla subtração e com piso zero; mantenedores do cliente — precisam da tecla `V` e de uma linha de HUD sem conflito com `W`/`T`/`E`/`Q`; designer — precisa que 25/3/250/5 s/14 s sejam lidos como `[HIPÓTESE]`/`escolha de protótipo`, e que o +15% contínuo seja registrado como reserva documentada, não promessa; SWE/orquestrador — precisa de um recorte verificável por `./verify.sh` (141 testes), cinco gates, roteiro 10/10 e CI `gates`+`rust`, sem promoção `dev→main` nem deploy por este agente.

## Goals

- Registrar a regra e todos os números como `[HIPÓTESE]`/`escolha de protótipo`: 25 Fôlego, 3 m, 250 de absorção, 150 ticks (5 s a 30 Hz), 420 ticks (14 s), ação `escudo` nível 2, um alvo, sem acumulação; `RESISTENCIA_CONTINUA_ESCUDO_PERC = 15` **reservada/documentada**, sem dano contínuo no Núcleo.
- Declarar `CUSTO_ESCUDO_FOLEGO = 25`, `ABSORCAO_ESCUDO = 250`, `TICKS_DURACAO_ESCUDO = 150`, `RECARGA_ESCUDO_TICKS = 420`, `ALCANCE_ESCUDO_M = 3.0` (e a constante reservada) com `///`, inteiros para ticks/Fôlego/absorção e `f64` apenas para distância, comparada por `hypot` sem tolerância.
- Acrescentar `escudo_pv`, `escudo_ticks`, `escudo_recarga_ticks`, `escudo_alvo` a `Jogador`, zerados na criação, sem `HashMap`, RNG, `Instant` ou estrutura paralela.
- Aceitar `escudo` em `ACOES_PERMITIDAS` com nível **2** explícito em `nivel_minimo_para_acao` e validação local ao sim (Fôlego ≥ 25, recarga 0, alvo self/aliado a ≤ 3 m, monotônicos só em comando aceito).
- Introduzir `aplicar_dano_a_jogador(&mut self, peer_id, dano)` como **rota única** e substituir as duas atribuições diretas em `aplicar_ataque` e `fase_ataque_pnjs`; a absorção consome `escudo_pv` da vítima e só o excedente toca `vida`, com piso zero e `combate_ticks` preservado.
- Aplicação válida **atribui** 250/150/420 ao alvo escolhido (nunca `+=`), expira exatamente ao completar 150 ticks, zera `escudo_pv`/`escudo_alvo` e só permite novo uso com recarga 0; um escudo cobre um alvo, nunca dois.
- Incluir os quatro campos em `serializar_jogadores`/`hash_estado`, em ordem de `peer_id` do `BTreeMap`.
- Cliente: `solicitar_escudo` com `just_pressed(KeyCode::KeyV)` escolhendo self/aliado dentro de 3 m, sem comando e sem avanço de monotônicos sem alvo; HUD com `Escudo: N (ticks)` (N = absorção restante), preservando as linhas e teclas atuais.
- Cinco testes determinísticos nomeados — `escudo_absorve_ate_250_dano`, `escudo_absorve_dano_de_pnj`, `escudo_recarga_420_ticks`, `escudo_expira_apos_150_ticks`, `hash_estado_cobre_escudo` — elevando 136 → **141**; estilo `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`.
- Rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012 e constituição v1.2.2: R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, **R6 N/A**.

## Non-Goals

- Promover qualquer número a cânone ou alterar o GDD; implementar a execução de 0,3 s, o +15% de resistência a dano contínuo, gás lacrimogêneo, proteção ao Artesão na Sabotagem ou aos três entrantes — a constante é reservada/documentada e o Núcleo não tem dano contínuo.
- Canalização, magia, RNG, `Instant`, relógio wall-clock, tolerância de distância, efeitos visuais 3D do escudo ou geometria de mira nova.
- Mapa/rotas de PVE (216/3125), ilhas, spawn por ilha, redes, Lightyear, sockets, async, lobby, matchmaking, predição/reconciliação ou qualquer troca de stack (ADR-004 permanece autorização da SE).
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`/`Cargo.lock`, CI, baseline, `STATE.md`, `AGENTS.md`, GDD, specs, validações ou releases neste assessment; fronteira de escrita restrita aos 5 artefatos de assess.
- Executar gates, build, testes, formatadores, `./verify.sh`, commit, push ou PR; adotar R6, simular issue ou usar `gh` improvisado; promover `dev→main` ou disparar deploy (SWE-exclusivos).
- Captura visual real prometida onde não houver surface automatizável — efeito renderizado fica **[NV]**; a prova é estrutural (código + binário com `--minimizado` + leitura da HUD).

## Success Metrics

- **SC-001** — Com escudo ativo, dano de até **250** é absorvido e `vida` não cai; acima de 250, o excedente debita `vida` com piso zero. Teste `escudo_absorve_ate_250_dano`. (baseline: sem escudo, dano integral — **[NV]** até verificação)
- **SC-002** — Dano de PNJ em `fase_ataque_pnjs` passa por `aplicar_dano_a_jogador` e é absorvido pelo escudo do alvo no mesmo tick; PvP e PNJ usam a **mesma** rota. Teste `escudo_absorve_dano_de_pnj`. (baseline: duas rotas diretas — **[NV]**)
- **SC-003** — Recarga de **420 ticks**: novo `escudo` antes disso é recusado sem efeito e sem avanço de monotônicos; ao completar 420 a recarga zera. Teste `escudo_recarga_420_ticks`. (baseline: recarga inexistente — **[NV]**)
- **SC-004** — Escudo expira exatamente ao completar **150 ticks**: `escudo_pv`/`escudo_alvo` zeram e a absorção cessa; reaplicação atribui (nunca soma) e cobre um alvo, nunca dois. Teste `escudo_expira_apos_150_ticks`. (baseline: duração inexistente — **[NV]**)
- **SC-005** — `hash_estado()` muda quando qualquer um dos quatro campos muda (`escudo_pv`, `escudo_ticks`, `escudo_recarga_ticks`, `escudo_alvo`) e dois mundos idênticos mantêm o mesmo hash tick a tick. Teste `hash_estado_cobre_escudo`. (baseline: campos ausentes — **[NV]**)
- **SC-006** — Cliente e fronteira: tecla `V` gera `acao: "escudo"` com alvo self/aliado a ≤ 3 m, HUD mostra `Escudo: N (ticks)`, `./verify.sh` espera **141 testes** (136 preservados + 5), cinco gates locais e roteiro 10/10 em `.specify/validations/escudo-milicia/`; nenhuma mudança em contratos/protocolo/Cargo/stack/rede/async e estilo Rust respeitado. (baseline: sem KeyV, sem linha `Escudo:`, 136 testes — **[NV]** até verificação)

## Cost of Inaction

O W do Guerreiro permanece só no papel: o jogador não tem defesa observável contra o dano de PNJ/PvP já implementado e a HUD continua muda quanto à absorção. O Núcleo mantém duas rotas diretas de dano ao jogador, sem ponto único onde auditar absorção, piso zero ou `combate_ticks`; qualquer habilidade defensiva futura teria de intervir duas vezes e poderia divergir entre PvP e PNJ. O custo é a perda da próxima fatia jogável de combate sem mudança de stack — exatamente a categoria de trabalho que os ciclos 047-054 tornaram barata e verificável —, além de deixar BF-002a aberto enquanto as alternativas reais (mapa e rede) exigem autorização ou regras ainda hipotéticas.

## Open Questions

- R3 = **0 clarificações**: nenhuma pergunta de produto bloqueia a especificação; o contrato fixa constantes, ticks, alvo único, não-acumulação, tecla e formato da HUD.
- **[NV]** Nomes definitivos de FR-001..FR-006 / SC-001..SC-006 / T001..T012 são os de `specs/055-escudo-milicia/spec.md` e `tasks.md` (em construção); o rastro abaixo registra o eco deste assessment.
- **[NV]** A escolha do alvo no cliente (self vs. aliado a 3 m) precisa de regra determinística de seleção no `solicitar_escudo`; o contrato fixa o alcance, não o critério de desempate entre aliados — validar na spec, sem abrir clarificação de produto aqui.
- **[NV]** Conformidade de R4 (antes de R7 `1.0.0-local`) depende de gates e roteiro não executados neste assessment.
- **[NV]** Captura visual da HUD com `--minimizado` e verificação ao vivo de CI/merge não são prometidas.

## Rastreabilidade (escopo do problema)

- FR-001 → regra e números `[HIPÓTESE]`, sem alterar o GDD; FR-002 → constantes e quatro campos em `Jogador`; FR-003 → ação `escudo` nível 2 e validação local ao sim; FR-004 → `aplicar_dano_a_jogador`, aplicação/expiração/recarga/não-acumulação; FR-005 → hash cobrindo os quatro campos; FR-006 → cliente (tecla `V` + `Escudo: N (ticks)`), cinco testes, estilo Rust e governança.
- SC-001..SC-006 → Success Metrics acima (SC-001..SC-005 com teste nomeado; SC-006 com fronteira, cliente e verificação); T001..T012 → entregáveis em `specs/055-escudo-milicia/tasks.md`, fora desta fronteira de escrita.
- R1 `go`, R3 = 0, R4 antes de R7 `1.0.0-local`, **R6 N/A** por constituição v1.2.2. Todo número citado é `[HIPÓTESE]`/`escolha de protótipo`; nada aqui é especificação, plano ou implementação. Nada executado.
