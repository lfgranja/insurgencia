# Research — Escudo de Milícia (W do Guerreiro)

**Data**: 2026-11-23T22:00Z · **Slug**: `escudo-milicia` · Evidências lidas do contrato da sessão, da decisão 80/20, do GDD, da constituição v1.2.2 e do checkout; **nenhum comando de gate, build, teste, git remoto ou GitHub foi executado**. Evidência confidence overall: **high** para regra, constantes, rotas e fronteira; **[NV]** para efeito renderizado e verde de gates.

## Scorecard único do ciclo (repetido, sem pontuação paralela)

| Candidato | I × C × Fch × E⁻¹ × R⁻¹ ÷ 3125 | Resultado |
|:--|:--|:--|
| `escudo-milicia` (BF-002a) | 4×4×5×3×4 = 960/3125 | **960/3125** |
| Mapa/rotas de PVE (BF-003) | 4×3×3×2×3 = 216/3125 | 216/3125 |
| Rede real (BF-001) | 5×2×5×1×1 = 50/3125 — inelegível (ADR-004) | 50/3125 |
| Rastreabilidade 053/054 (BF-008) | 2×5×0×5×5 = 0 — já fechada | 0 |

## Usuários e demanda

- [source: `.specify/memory/STATE.md`, decisão `2026-11-23T22:00Z`, lida] Consumidor 1: **jogador humano no cliente standalone**, que passa a ver `Escudo: N` e a sobreviver a dano de PNJ/PvP de forma observável; consumidor 2: **o próprio Núcleo determinístico**, que passa a ter rota única de dano ao jogador com absorção. São consumidores fora dos artefatos do ciclo — passam o filtro eliminatório. (confidence: high)
- [source: `docs/GDD - Insurgência.md` §10.6:690-692, lido] Critérios de aceitação escritos no GDD: "O escudo absorve até 250 de dano e expira após 5 s"; "Enquanto ativo, reduz em 15% o dano contínuo de gás lacrimogêneo"; "O escudo pode ser aplicado a um aliado ou ao próprio Guerreiro, nunca a dois alvos ao mesmo tempo". Demanda documentada, não observada em partida: o gás e o dano contínuo permanecem fora do corte. (confidence: high para o texto; **[NV]** para comportamento jogado)

## Prior art

- [source: `.specify/memory/STATE.md`, decisão 80/20 das 22:00Z, lida] BF-002a é a fativa `a` de um conjunto de 22 habilidades Q/W/E/R (BF-002b..v); Q (`atacar_forte`) e E da Intelectual (`analise_materialista`) já foram entregues em ciclos anteriores. O escudo é a primeira **W** implementada; o padrão de ciclos anteriores — constante `[HIPÓTESE]`, campo em `Jogador`, ação validada no sim, hash, HUD e tecla — é reaproveitado, não reinventado. (confidence: high)
- [source: `specs/054-analise-materialista/tasks.md` e `specs/053-pnj-defensores/tasks.md`, lidos] Ciclos 053/054 entregaram 128→136 testes com o mesmo desenho: `BTreeMap`, `ValidacaoComandos`, relógio por tick em `avancar_tick`, segmento estável em `serializar_jogadores` e linha de HUD. O escudo segue esse precedente; nenhuma biblioteca, stack ou protocolo novo. (confidence: high)
- [source: `crates/sim/src/lib.rs:568-584,841,992`, lido] Já existem `aplicar_dano_a_pnj` (com piso zero e bônus de análise), o desconto de vida no PvP e o dano de PNJ. O escudo **reaproveita** esse vocabulário; a novidade é a rota única `aplicar_dano_a_jogador`, não um novo sistema de combate. (confidence: high)
- [source: `.specify/assessments/alvo-treino/{research,decision}.md`, lidos] A limitação do cliente standalone (um único `PEER_ID = 1`) já foi registrada e contornada com scaffolding não-canônico; o escudo não reabre essa decisão e não depende de segundo jogador para ser observável (a absorção incide no próprio jogador). (confidence: high)

## Contexto e custo da inação

- [source: `.specify/memory/STATE.md`, BF-002a e BF-002b..v, lido] Sem o corte, as 21 habilidades restantes permanecem apenas no GDD; o W do Guerreiro continua sem efeito observável e o dano ao jogador segue em duas rotas diretas, sem absorção auditável. A alternativa de maior impacto (rede, 5) está bloqueada por ADR-004; mapa/rotas PVE dependem de regras de mapa ainda `[HIPÓTESE]`. (confidence: high)
- [source: `.specify/assessments/alvo-treino/decision.md`, lido] A cadeia jogável (ataque, alvo, Fôlego, HUD) já foi fechada nos ciclos 047-054; o escudo é a próxima fatia de combate defensor sem mudança de stack. Inação deixaria o jogador sem contrapartida defensiva verificável. (confidence: medium — inferência de sequência de ciclos, não medição de uso)

## Dados e restrições

- [source: `docs/GDD - Insurgência.md` §10.6:687-692, lido] Números `[HIPÓTESE]`: 25 Fôlego, 0,3 s, 3 m, 250/5 s, recarga 14 s, +15% contínuo. O corte registra 25/3/250/5/14 como escolhas de protótipo; a execução de 0,3 s e o +15% contra gás ficam de fora (o Núcleo não tem dano contínuo). (confidence: high)
- [source: `crates/sim/src/lib.rs:15,101,110,137,143,154,166-178,190-196,248-...`, lido] `PASSO_SEGUNDOS = 1/30` → 5 s = **150 ticks**, 14 s = **420 ticks**; `CUSTO_ATAQUE_FOLEGO = 30`, `DANO_ATAQUE`, `BONUS_DANO_ANALISE_PERC` mostram o padrão de constantes rotuladas; `ACOES_PERMITIDAS` e `nivel_minimo_para_acao` vivem no sim; `Jogador: Clone + Copy + Debug` comporta quatro campos `i64`/`Option<u64>` sem estrutura paralela. (confidence: high)
- [source: `crates/sim/src/lib.rs:568-584,841,961-995,1614-1650,1742-1785`, lido] Duas rotas diretas de dano ao jogador: `vitima.vida` em `aplicar_ataque` e `jogador.vida` em `fase_ataque_pnjs`. `serializar_jogadores` já inclui `analise_recarga_ticks`; `hash_estado` usa SHA-256 sobre segmentos estáveis. A absorção deve ocorrer na **vítima**, consumir `escudo_pv` primeiro e aplicar só o excedente em `vida`, com piso zero. (confidence: high)
- [source: `.specify/memory/STATE.md`, DoD do ciclo 055, lida] Comandos que provam: `./verify.sh` esperado **141 testes**, cinco gates locais, roteiro 10/10 em `.specify/validations/escudo-milicia/`, CI `gates`+`rust` no head do PR. Nenhum foi executado neste assessment. (confidence: high para o contrato; **[NV]** para o resultado)
- [source: `.specify/memory/constitution.md:23-26,48-50,78-79`, lida] Constituição v1.2.2: R1 `go`; R4 antes de R7 `1.0.0-local`; `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A/bloqueado** (sem adaptador), PARE se invocada, nunca issue simulada, nunca `gh` improvisado; promoção `dev→main` e deploy são SWE-exclusivos. (confidence: high)
- [source: contrato da sessão, imutável] Rust `#![forbid(unsafe_code)]`, `#![deny(clippy::pedantic)]`, sem `unwrap`/`expect`/`#[allow]`, `max_width = 100`; sem contratos, protocolo, Cargo, stack, rede ou async. (confidence: high)

## Evidências contrárias à ideia

- O +15% de resistência a dano contínuo e a proteção ao Artesão/Sabotagem do GDD **não** entram no corte: o Núcleo não tem dano contínuo. Quem esperar o critério completo de §10.6:691 verá apenas a constante reservada/documentada. [source: GDD §10.6:687-692 + STATE 22:00Z] (confidence: high)
- A rota única `aplicar_dano_a_jogador` toca **todo** o dano ao jogador (PvP e PNJ): uma regressão nela afeta duas fontes. É o maior risco técnico do corte e exige os cinco testes nomeados, especialmente as fronteiras 250/150/420 e o hash. [source: `crates/sim/src/lib.rs:841,992`] (confidence: high)
- O efeito renderizado na HUD não tem surface automatizável de captura com janela minimizada: a prova é estrutural e o visual fica **[NV]**. Não prometer screenshot real. [source: padrão dos ciclos 047-054, `alvo-treino/decision.md`] (confidence: medium)
- Rede (50/3125) e mapa/rotas PVE (216/3125) são frequentemente citadas como "maiores" que o escudo em impacto bruto; o scorecard não as ignora — elas são rejeitadas por confiança/facilidade/risco e, no caso da rede, por autorização ADR-004. (confidence: high)

## Lacunas e perguntas em aberto

- R3 = **0 clarificações**: não há pergunta de produto bloqueante; o contrato resolve constantes, alvo único, não-acumulação, tecla `V` e formato `Escudo: N (ticks)`.
- **[NV]** Nomes definitivos de FR/SC/T em `specs/055-escudo-milicia/spec.md` e `tasks.md` (em construção); o rastro abaixo é o eco deste assessment, não a redação canônica.
- **[NV]** Verificação ao vivo: gates, `./verify.sh` (141), roteiro 10/10, CI e R4/R7 — nenhum executado aqui.
- **[NV]** Idem para renderização da HUD e para a ausência de divergência de hash entre mundos idênticos após a inclusão dos quatro campos.

## Rastreabilidade

- FR-001 (regra/números `[HIPÓTESE]`, sem alterar GDD) ↔ GDD §10.6:687-692 + decisão 22:00Z; FR-002 (constantes + quatro campos em `Jogador`) ↔ `sim:15,101,248-...`; FR-003 (ação `escudo` nível 2, custo 25, alcance 3 m, validação) ↔ `sim:166-178,190-196`; FR-004 (`aplicar_dano_a_jogador`, aplicação, 150/420 ticks, não-acumulação) ↔ `sim:841,992,699-712`; FR-005 (hash cobre os campos) ↔ `sim:1614-1650,1742-1785`; FR-006 (cliente, testes, estilo, governança) ↔ contrato da sessão + constituição v1.2.2.
- SC-001..SC-006 em `define.md`; T001..T012 em `decision.md` (nomes finais em `specs/055-escudo-milicia/tasks.md`). R1 `go`, R4 antes de R7 `1.0.0-local`, **R6 N/A**. Nada executado; todo resultado de verificação permanece **[NV]**.

## Fontes

Todas as fontes são artefatos locais do worktree lidos em modo somente-leitura; nenhuma URL foi buscada e nenhuma página externa foi citada (logo, não há sanitização de URL a registrar).

- `.specify/memory/STATE.md` (decisão 80/20 e DoD, 2026-11-23T22:00Z) — host: local, policy: allowlisted (artefato do repo).
- `docs/GDD - Insurgência.md` §10.6:687-692 — host: local, policy: allowlisted.
- `.specify/memory/constitution.md` v1.2.2 — host: local, policy: allowlisted.
- `crates/sim/src/lib.rs` e `crates/cliente/src/main.rs` no checkout `3f3b01c` — host: local, policy: allowlisted.
- `specs/053-pnj-defensores/tasks.md`, `specs/054-analise-materialista/tasks.md`, `.specify/assessments/alvo-treino/{research,decision}.md` — host: local, policy: allowlisted.
