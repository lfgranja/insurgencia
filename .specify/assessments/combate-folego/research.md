# Research — Combate e Fôlego

**Data**: 2026-11-23T07:35Z · Evidências lidas do GDD, do `STATE.md`, de assessments anteriores, da constituição e do contrato da sessão; **nada reexecutado**.

**Evidence confidence (overall)**: high para a decisão 80/20, a fronteira técnica e o rótulo `[HIPÓTESE]` dos números; low para qualquer pretensão de cânone em custo, dano, duração, regeneração ou alcance — todos são `[HIPÓTESE]`/`escolha de protótipo` e assim permanecem.

## Users & Demand

- Jogadores da Comuna (Guerreiro/Milícia) — o GDD §10.6:680 descreve "Ataque Forte" como golpe concentrado corpo a corpo e registra o nome como canônico do protótipo; hoje a ação existe em `ACOES_PERMITIDAS` sem efeito observável e não há Fôlego no `Jogador`. [source: `docs/GDD - Insurgência.md` §10.6:680, lido; contrato da sessão] (confidence: high)
- Jogadores em geral — precisam de um recurso físico previsível: o GDD §10.1:517 define Fôlego como reserva que se regenera por tempo, "não é mana (energia mágica) nem recurso sobrenatural"; sem estado no Núcleo, essa distinção não é verificável. [source: GDD §10.1:517, lido] (confidence: high)
- Mantenedores do Núcleo Rust e SWE/orquestrador — precisam de uma fatia auditável por `hash_estado`, com disciplina de tick e fronteira inalterada, para fechar a última frente de produto sem mudança de stack. [source: contrato da sessão; `STATE.md:348-351`, lidos] (confidence: high)
- Demanda observada é documental e decisória, não telemetria: não há dado de uso de combate coletado neste assessment. [source: contrato da sessão] (confidence: high)

## Prior Art

- [source: `.specify/memory/STATE.md:273,299,325,337`, lido] Combate/Fôlego aparece em quatro decisões 80/20 anteriores com I/C/F/R = 4/2/2/3 → 48/625, sempre como candidato rejeitado; o número é estável e não foi renegociado agora. (confidence: high)
- [source: `.specify/assessments/barricada-deterministica/research.md:24`, lido] A objeção recorrente é GDD §16.3:1509 — o MVP prova jogabilidade por desafios de ilha, "em vez de combate puro"; priorizar combate inverteria a intenção validada. O recorte 047 responde com escopo mínimo: um ataque, um recurso, um HUD — não um sistema de combate. (confidence: high)
- [source: `.specify/assessments/defesa-barricada/decision.md`, `.specify/assessments/ponte-provisoria/decision.md`, `.specify/assessments/suditos/decision.md`, lidos] Combate/Fôlego foi rejeitado em 44 (256/625 para defesa), 45 (108/625 para ponte) e 046 (48/625 para Súditos). Em 046 o desempate coube a Súditos por §9.2:464 ser regra de aceitação escrita; em 047 o contrato inverte o desempate e escolhe combate, mantendo o empate numérico. (confidence: high)
- [source: `.specify/assessments/suditos/{intake,shape,define,decision}.md`, lidos] Padrão reaproveitável: ação em `ACOES_PERMITIDAS`, campos no `Jogador`/`Mundo`, avanço determinístico por tick, `hash_estado` refletivo, `CodigoRecusa` específico, entrada just-pressed com `seq`/`request_id` monotônicos e HUD. (confidence: high)
- [source: `specs/047-combate-folego/spec.md`, lido no momento desta leitura] O diretório de especificação continha apenas o template inicial (marcadores `[FEATURE NAME]`, `[NEEDS CLARIFICATION]`): os rastros FR/SC/T aqui são antevisões do handoff. (confidence: high)

## Market & Context

- Alternativa atual: `atacar_forte` está em `ACOES_PERMITIDAS`, mas sem efeito; Fôlego, regeneração e combate não existem como estado. Implementar "combate puro" (IA, projéteis, colisão, balanceamento) exigiria sistemas ausentes e outro orçamento. [source: contrato da sessão; GDD §16.3:1505-1509, lidos] (confidence: high)
- Custo de inação: a última frente de produto sem mudança de stack fica sem recipiente observável; o próximo ciclo que precisar de "jogador cansado" ou "alvo atingido" não terá verdade determinística para consultar, e a distinção Fôlego ≠ mana (§10.1:517) continuará apenas declarada. [source: GDD §10.1:517; `STATE.md:351`; contrato da sessão] (confidence: high)

## Data & Constraints

- Base de tempo: 30 ticks/s. **90 ticks = 3 s** (escolha de protótipo); **10/s → +1 a cada 3 ticks** e **5/s → +1 a cada 6 ticks** (`[HIPÓTESE]` §10.1:521), sem arredondamento na conversão. [source: contrato da sessão; GDD §10.1:521, lidos] (confidence: high)
- Fronteira técnica: nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`; o efeito vive em `crates/sim/src/lib.rs`, a ação já existe em `ACOES_PERMITIDAS`. [source: contrato da sessão, imutável] (confidence: high)
- Estilo obrigatório (ADR-004 / constituição): `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem `async`/rede/Lightyear. [source: `AGENTS.md`, lido; contrato da sessão] (confidence: high)
- R6 (`taskstoissues`) ratificada como intenção em v1.2.0 e removida da sequência executável em v1.2.2: **N/A**; PARE e escale se invocada, nunca issue simulada, nunca `gh` improvisado. R4 vem antes de R7 (`1.0.0-local`). [source: `.specify/memory/constitution.md:23,26,49-50,79-84`, lido] (confidence: high)
- Scorecard ancorado em `STATE.md:342-351` (07:35Z): Combate/Fôlego 4/2/2/3 = **48/625**; rede 5/2/1/1 = 10/625; escolha `combate-folego`; valores de Fôlego rotulados `[HIPÓTESE]` e registrados como escolhas de protótipo; dano e alcance também hipotéticos; R6 N/A. [source: `.specify/memory/STATE.md`, lido] (confidence: high)

## Evidence Against the Idea

- GDD §16.3:1509 é explícito quanto ao MVP: jogabilidade baseada em desafios de ilha, "em vez de combate puro". O contrato 047 escolhe combate de qualquer forma; o assessment registra a objeção e limita o recorte a `atacar_forte` + Fôlego + HUD. É por isso que confiança e facilidade são **2**, não 3 ou 4. [source: GDD §16.3:1509; `STATE.md:346-351`, lidos] (confidence: high)
- Todo número é hipótese: §10.1:519-523 marca 100, 10/s, 5/s e 2 m como `[HIPÓTESE]`; §10.6:680 marca os valores de "Ataque Forte" como `[HIPÓTESE]`; custo 30, dano 10 e 90 ticks são escolhas de protótipo do contrato. Nenhum pode ser promovido a cânone pela spec. [source: GDD §10.1, §10.6; contrato da sessão] (confidence: high)
- A rede tem impacto 5, mas está bloqueada por ADR-004 enquanto Lightyear não entra em `Cargo.toml`/`Cargo.lock`; "sincronizar" um combate que ainda não é determinístico inverte a dependência. [source: `.specify/assessments/ponte-provisoria/research.md`, lido; ADR-004] (confidence: high)
- Nenhum teste, gate ou execução foi realizado neste assessment: toda conformidade futura é **[NV]**. [source: contrato da sessão] (confidence: high)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: a validação de alvo usa "no ponto/alcance" de 2 m (`[HIPÓTESE]` §10.1:523) — como o ponto do alvo é obtido sem geometria de projétil ou colisão? Validar no `/speckit.specify`.]
- [NEEDS CLARIFICATION: `CodigoRecusa::FolegoInsuficiente` e `AlvoInvalido` "se necessário" — confirmar quais variantes existem hoje e se a recusa é sem consumo parcial.]
- [NEEDS CLARIFICATION: `atacar_forte` e o `BTreeSet<u64>` de atacantes cabem em `ACOES_PERMITIDAS` sem estender `crates/contratos` / `PROTOCOL_VERSION`?]
- [NEEDS CLARIFICATION: o contador de regeneração `folego_ticks` e o relógio `combate_ticks` coexistem em `fase_finish` sem duplo crédito no tick do ataque?]
- Fora de escopo por decisão, não lacuna: IA de Súditos, projéteis, colisão, armas, channeling de 0,4 s / recarga de 6 s (§10.6, `[HIPÓTESE]`), combate em equipe, mapa/ilhas e rede.

## Sources

- `docs/GDD - Insurgência.md` §10.1:517-523, §10.6:680, §16.3:1505-1509 (arquivo local, lido; sem URL).
- `.specify/memory/STATE.md:273,299,325,337,342-351` (decisão 80/20 do ciclo 047, 07:35Z; arquivo local, lido).
- `.specify/memory/constitution.md:23,26,49-50,79-84` e `AGENTS.md` (arquivos locais, lidos).
- `.specify/assessments/barricada-deterministica/research.md`, `.specify/assessments/defesa-barricada/decision.md`, `.specify/assessments/ponte-provisoria/{research,decision}.md`, `.specify/assessments/suditos/*.md` (arquivos locais, lidos).
- `specs/047-combate-folego/spec.md` (template inicial no momento da leitura).
- Contrato da sessão `20261123T073500Z-combate-folego` (imutável; fonte de custo 30, dano 10, 90 ticks, API prevista e rótulos de protótipo).
- Nenhuma URL foi buscada; nenhum comando de gate, build, git remoto, GitHub ou emissão de issue foi executado por este agente.

## Rastreabilidade

Fatos colados como observados, sem reexecução. Rastros do ciclo: FR-001..FR-006 (constantes, estado, efeito/validação, dano/débito, regeneração, recusa/cliente/HUD), SC-001..SC-006 (critérios observáveis), T001..T012 (entregáveis); a definição canônica viverá em `specs/047-combate-folego/`, hoje ainda em template. Todos os valores de combate citados são `[HIPÓTESE]`/`escolha de protótipo`. **R6 é N/A** por constituição v1.2.2 e não entra neste rastro. Tudo o que depende de execução (gates, testes, R4, R7) permanece **[NV]**.
