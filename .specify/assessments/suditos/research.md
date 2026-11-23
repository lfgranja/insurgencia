# Research — Súditos e Propaganda

**Data**: 2026-11-23T06:58Z · Evidências lidas do GDD, do `STATE.md`, de assessments anteriores e do contrato da sessão; **nada reexecutado**.

**Evidence confidence (overall)**: high para a regra canônica de 4 s / 3 s; low para spawn, raio e funções — marcados `[HIPÓTESE]` e fora de escopo.

## Users & Demand

- Jogadores da Comuna (Intelectual) — o GDD §9.2:459 descreve a influência sobre Súditos como "avanço material e ideológico ... pelo trabalho e pela persuasão, não por exércitos automáticos"; hoje não há verdade observável no Núcleo sobre exposição, conversão ou reversão. [source: `docs/GDD - Insurgência.md` §9.2:457-459, lido] (confidence: high)
- Mantenedores do Núcleo Rust e SWE/orquestrador — precisam de uma fatia verificável por `hash_estado` sem depender de ilhas, mapa ou sistemas de spawn ainda hipotéticos. [source: contrato da sessão, imutável] (confidence: high)
- Demanda observada é documental, não telemetria: a regra aparece como critério de aceitação escrito (§9.2:463-464), não como uso medido; nenhum dado de uso foi coletado neste assessment. [source: GDD §9.2:463-464, lido] (confidence: high)

## Prior Art

- [source: `.specify/assessments/defesa-barricada/decision.md:14,25`, lido] Súditos/Propaganda foi candidata rejeitada em 44 (48/625): a conversão de Nós e a Resistência Ideológica eram `[HIPÓTESE]` (§6.2:275) e dependiam de Propaganda canalizada de 20 s. Aqui o recorte é menor — apenas a regra de exposição/reversão de §9.2:464. (confidence: high)
- [source: `.specify/assessments/ponte-provisoria/research.md:15,29`, lido] Em 45 Súditos/Propaganda voltou a 48/625 e foi rejeitada por "depender de sistema ausente e sem valor canônico de construção". A mudança de ciclo 046 é de **escopo**, não de valor: retira-se o sistema ausente e fica-se com o critério canônico. (confidence: high)
- [source: `.specify/assessments/barricada-deterministica/decision.md`, `.specify/assessments/defesa-barricada/decision.md`, `.specify/assessments/ponte-provisoria/decision.md`, lidos] Padrão já entregue e reaproveitável: ação em `ACOES_PERMITIDAS`, struct em `BTreeMap<u64, _>` no `Mundo`, avanço determinístico por tick, `hash_estado` e HUD com tecla dedicada. (confidence: high)
- [source: `specs/046-suditos/spec.md`, lido] O diretório de especificação contém apenas o template inicial (marcadores `[FEATURE NAME]`, `[NEEDS CLARIFICATION]`): os rastros FR/SC/T deste assessment são antevisões do handoff, não requisitos já ratificados. (confidence: high)

## Market & Context

- Alternativa atual: Súditos existem só como prosa de design; a condição de vitória e os Nós dependem de Propaganda de 20 s (GDD §7.1/§6.2, documental) — implementar isso inteiro exigiria mapa, ilhas e canalização, fora do orçamento desta fatia. [source: GDD §6.2:275, §9.2:461, lidos] (confidence: high)
- Custo de inação: a regra de aceitação §9.2:464 continua sem recipiente observável, e o próximo ciclo que precisar de "Súdito convertido" não terá estado determinístico para consultar. [source: GDD §9.2:463-464; contrato da sessão] (confidence: high)

## Data & Constraints

- Conversão de tempo do contrato: 30 ticks/s → **4 s = 120 ticks**, **3 s = 90 ticks**. É a mesma base usada em ciclos anteriores (1,5 s = 45 ticks; 10 s = 300 ticks). [source: contrato da sessão; STATE.md:290-360, lidos] (confidence: high)
- Fronteira técnica: nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`; a ação `propaganda` entra em `ACOES_PERMITIDAS` da sim e o comando carrega `BTreeSet<u64>` de ids expostos. [source: contrato da sessão, imutável] (confidence: high)
- Estilo obrigatório (ADR-004 / constituição): `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem `async`/rede/Lightyear. [source: `AGENTS.md`, lido; contrato da sessão] (confidence: high)
- R6 (`taskstoissues`) ratificada como intenção em v1.2.0, removida da sequência executável em v1.2.2: **N/A**; PARE e escale se invocada, nunca issue simulada. R4 vem antes de R7 (`1.0.0-local`). [source: `.specify/memory/constitution.md:23,50,79-84`, lido] (confidence: high)
- Scorecard ancorado no `STATE.md` (2026-11-23T06:58Z): Súditos 4/2/2/3 = 48/625; Combate/Fôlego 4/2/2/3 = 48/625; rede 5/2/1/1 = 10/625; escolha `suditos`; spawn/raio/função fora de escopo. [source: `.specify/memory/STATE.md`, lido] (confidence: high)

## Evidence Against the Idea

- O próprio GDD cerca a regra de hipóteses: §9.2:461 marca spawn a cada 30 s, raio de 6 m, limite de 3 Súditos, conservação de função e permanência de 60 s como `[HIPÓTESE]`. Sem esses sistemas, o Súdito convertido não tem origem, destino nem função observável — daí confiança **2**, facilidade **2** e risco invertido **3** no scorecard, e a decisão de injetar ids por `adicionar_sudito(id, posicao)` em vez de spawn automático. [source: GDD §9.2:461; contrato da sessão] (confidence: high)
- Empate com Combate/Fôlego (48/625) significa que o número sozinho não justifica a escolha: o desempate vem do contrato da sessão e do fato de §9.2:464 ser critério de aceitação escrito, enquanto Fôlego/alcance/recarga/dimensões seguem `[HIPÓTESE]` em §10.3:574-579. [source: STATE.md:346-356; GDD §10.3, lidos] (confidence: high)
- A rede tem impacto 5, mas está bloqueada por ADR-004 enquanto Lightyear não entra em `Cargo.toml`/`Cargo.lock` e a SE não decide troca de engine; "sincronizar" um estado que ainda não é determinístico inverte a dependência. [source: `.specify/assessments/ponte-provisoria/research.md`, lido; ADR-004] (confidence: high)
- Nenhum teste, gate ou execução foi realizado neste assessment: toda conformidade futura é **[NV]**. [source: contrato da sessão] (confidence: high)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: o relógio corrente do Núcleo é 30 ticks/s? A conversão 4 s = 120 / 3 s = 90 depende dele — validar no `/speckit.specify`]
- [NEEDS CLARIFICATION: `propaganda` em `ACOES_PERMITIDAS` e o comando com `BTreeSet<u64>` comportam a exposição sem estender `crates/contratos` / `PROTOCOL_VERSION`?]
- [NEEDS CLARIFICATION: `adicionar_sudito` e `aplicar_dano_a_sudito` são a única via de injeção/dano neste recorte, sem IA, projéteis ou colisão?]
- Fora de escopo por decisão, não lacuna: spawn por pulso, raio de 6 m, funções trabalhador/propagandista/miliciano, 60 s de permanência, renda do trabalhador, 3 Nós e Resistência Ideológica — todos `[HIPÓTESE]` (§9.2:461; §6.2:275).

## Sources

- `docs/GDD - Insurgência.md` §9.2:457-467 (arquivo local, lido; sem URL).
- `.specify/memory/STATE.md` (decisão 80/20 do ciclo 046, 2026-11-23T06:58Z; arquivo local, lido).
- `.specify/memory/constitution.md:23,50,79-84` e `AGENTS.md` (arquivos locais, lidos).
- `.specify/assessments/defesa-barricada/decision.md`, `.specify/assessments/ponte-provisoria/{research,decision}.md`, `.specify/assessments/barricada-deterministica/decision.md` (arquivos locais, lidos).
- `specs/046-suditos/spec.md` (template inicial, lido).
- Contrato da sessão `20261123T065800Z-suditos` (imutável; fonte dos valores de conversão e da API prevista).
- Nenhuma URL foi buscada; nenhum comando de gate, build, git remoto, GitHub ou emissão de issue foi executado por este agente.

## Rastreabilidade

Fatos colados como observados, sem reexecução. Rastros do ciclo: FR-001..FR-006 (constantes/estado/API/ação/hash/HUD), SC-001..SC-006 (critérios observáveis), T001..T012 (entregáveis); a definição canônica viverá em `specs/046-suditos/`, hoje ainda em template. Tudo o que depende de execução (gates, testes, R4, R7) permanece **[NV]**.
