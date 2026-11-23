# Decision — Papéis, Níveis e Desbloqueio Q/W/E/R

- **Slug**: `papeis-niveis`
- **Decided**: 2026-11-23T15:08Z
- **R1**: `go` · **R3**: 0 clarificações · **R4 antes de R7**: `1.0.0-local` · **R6**: N/A (constituição v1.2.2)
- **Artifacts reviewed**: `intake.md` | `research.md` | `define.md` | `shape.md`

## Scorecard

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | strong | O GDD §10.1:515-524 descreve papéis materiais, teto 12 e desbloqueio Q1/W2/E3/R6, mas `Jogador` não tem `funcao`/`nivel`, `ACOES_PERMITIDAS` aceita tudo desde o tick 1 e o HUD não mostra identidade nem progresso; a regra existe apenas como linha de tabela `[HIPÓTESE]`. |
| Força da evidência | strong | GDD §10.1:504-524 lido com papéis e valores rotulados `[HIPÓTESE]`; §10.2–§10.5 lido para excluir as habilidades completas; `STATE.md` (15:08Z) fixa o 80/20 em 48/625, o empate com PVE/PNJs e os 10/625 da rede; código existente confirmado por leitura (`ACOES_PERMITIDAS`, `CodigoRecusa`, `ValidacaoComandos::validar`, `Jogador`). |
| Valor vs. inação | strong | **48/625**, empatado com PVE/PNJs (48/625) e à frente de rede (10/625); sem o recorte, a única regra de progressão escrita no GDD segue sem estado observável, recusa explícita ou exibição no cliente. |
| Viabilidade / appetite | strong | `small`: enum de cinco variantes, dois campos, duas constantes, +1/900 ticks no avanço determinístico, uma tabela de mapeamento, uma variante de recusa, hash e duas linhas de HUD; reaproveita seis ações já existentes e não toca em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, Lightyear, `async` ou sockets. |
| Alinhamento estratégico | strong | Preserva `TICKS_POR_NIVEL = 900` (30 s a 30 Hz), `NIVEL_MAXIMO = 12`, nível inicial 1 e Q1/W2/E3/R6 como `[HIPÓTESE]`/protótipo; respeita P2 (papéis materiais, não classes mágicas), a constituição v1.2.2 (R1 `go`, R4 antes de R7 `1.0.0-local`, R6 N/A) e ADR-004; novas habilidades W/E/R fora de escopo. |
| Postura de risco | strong | Riscos nomeados e mitigados por decisão: 899/900 e teto inclusivo, tabela única de mapeamento, `AcaoBloqueada` distinta de `AcaoDesconhecida`, hash posicional, sem novas teclas/campos; empate 48/625 desempatado pelo contrato e pelo lastro do §10.1; R6 N/A sem issue simulada. |

## 80/20

| Opção | Impacto × Confiança × Facilidade × Risco ÷ 625 | Resultado |
|---|---|---|
| Papéis + níveis + desbloqueio Q/W/E/R (escolhida) | 4×2×2×3 = 48/625 | **48/625** |
| PVE/PNJs | 4×2×2×3 = 48/625 | 48/625 |
| Rede (bloqueada por ADR-004 até decisão da SE) | 5×2×1×1 = 10/625 | 10/625 |

Critério: impacto × confiança × facilidade ÷ risco (quarto fator = risco invertido/segurança), escala 1–5, denominador **625**.

Papéis+níveis vence porque é a única opção com **regra de progressão escrita e fronteira pequena simultaneamente**: o GDD §10.1 fixou papéis materiais, teto 12 e Q1/W2/E3/R6, e o contrato da sessão fixou a conversão (900 ticks = 30 s a 30 Hz), o mapeamento das ações já existentes e a API, sem tocar em contrato, protocolo ou stack. Confiança 2 e facilidade 2 registram honestamente que os números são `[HIPÓTESE]` e que o estado de progressão não existe ainda no Núcleo; por isso o recorte usa ações consolidadas em vez de criar habilidades. O empate com PVE/PNJs (48/625) é resolvido pelo contrato e pelo lastro canônico do §10.1: PNJs dependem de IA, mapa e alvos ainda ausentes. A rede inverte a dependência entre determinismo e sincronização e permanece bloqueada por ADR-004.

## Veredito e justificativa

**Veredito: go** — Opção A (modelo mínimo de papéis e progressão por nível), scorecard 6/6 strong, R1 `go`, R3 = 0 clarificações, R4 antes de R7 (`1.0.0-local`), R6 N/A. O handoff abaixo entrega ao `/speckit.specify` o problema, a opção escolhida, o escopo, as métricas e os rastros FR-001..FR-006 / SC-001..SC-006 / T001..T012. Nenhum spec, plano, tarefa, código, gate, teste, formatação, commit, push ou PR foi produzido ou executado neste assessment.

## Condições (R1)

1. Regra congelada **`[HIPÓTESE]`/protótipo**: `TICKS_POR_NIVEL = 900` (30 s a 30 Hz) e `NIVEL_MAXIMO = 12`; `Jogador.nivel` começa em 1, ganha **+1 a cada 900 ticks** exatos e não ultrapassa 12. Sem ganho em 899 e sem avanço no teto.
2. Papéis e estado: `enum Funcao` com Artesão, Guerreiro, Intelectual, Camponês e Agitador (GDD §10.1, papéis materiais/P2); campos públicos `Jogador.funcao` e `Jogador.nivel`. O valor padrão de `funcao` é `[HIPÓTESE]` a confirmar na spec (candidato `Artesao`).
3. Mapeamento de desbloqueio **`[HIPÓTESE]`**, apenas sobre ações existentes: nível 1 → `atacar_forte` (Q); nível 2 → `construir_barreira` e `construir_ponte` (W); nível 3 → `propaganda` e `propaganda_no` (E); nível 6 → `atacar_no` (R). Nenhuma habilidade nova W/E/R entra nesta fatia.
4. Validação: `CodigoRecusa::AcaoBloqueada` é acrescentado sem remover os códigos existentes, e `ValidacaoComandos::validar` recusa ação conhecida cujo nível do jogador é insuficiente, preservando a ordem e a semântica das demais recusas.
5. Determinismo: `funcao` e `nivel` são refletidos em `hash_estado`, em posição estável e independente de formatação de texto.
6. Cliente: HUD com `Função: <nome>` e `Nível: N/12`, sem novas teclas, sem novos comandos e sem novos campos de protocolo.
7. Compatibilidade e estilo: nada em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, dependências ou `.specify/queue/concurrency.jsonl`, `STATE.md`, `AGENTS.md`; `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem `async`/rede/Lightyear.
8. Constituição v1.2.2 aplicada: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY depois de `tasks`, `converge` append-only; **R6 N/A** — fora da sequência executável, PARE se invocada, nunca issue simulada, nunca `gh` improvisado.

## Riscos registrados

- **Hipóteses do GDD §10.1**: teto 12, Q1/W2/E3/R6 e os 900 ticks (30 s) são chutes de design/protótipo, não cânone; rotular como tal em spec, código e HUD. Não reabrir o empate 48/625 com PVE/PNJs.
- **Confusão conceitual Q/W/E/R**: no GDD essas letras são habilidades completas por papel (§10.2–§10.5); aqui são apenas desbloqueio das ações existentes. Habilidades novas (Oficina de Campo, Análise Materialista, Plano Mestre etc.), Fôlego/alcance/recarga/dimensões e PVE/PNJs ficam fora de escopo.
- **Default de `funcao`**: o contrato não fixou o valor inicial; candidato `[HIPÓTESE]` `Artesao`, a validar na spec — não é uma decisão canônica tomada aqui.
- **Fronteira de tick e ordem de recusas**: risco invertido 3 (não 5) — 899/900, teto inclusivo, um único ganho por janela, `AcaoBloqueada` distinta de `AcaoDesconhecida` e hash posicional exigem verificação; permanecem **[NV]** até execução.
- **Dívida R6**: N/A por constituição v1.2.2, não é dívida de produto; nenhuma issue simulada, nenhum `gh` improvisado.
- **Verificação**: gates, build, testes, formatação, R4 e R7 (`1.0.0-local`) permanecem **[NV]** — nenhum foi executado por este agente.

## Handoff — `/speckit.specify`

- **Problem**: o GDD §10.1 descreve papéis materiais, nível 1–12 e desbloqueio Q1/W2/E3/R6, mas o Núcleo não registra `funcao`/`nivel`, aceita todas as ações desde o tick 1 e o HUD não exibe progressão; a regra permanece `[HIPÓTESE]` sem verdade observável.
- **Chosen approach**: Opção A — `enum Funcao`, `Jogador.funcao`/`Jogador.nivel`, `TICKS_POR_NIVEL = 900`, `NIVEL_MAXIMO = 12`, +1/900 ticks, mapeamento Q1/W2/E3/R6 sobre ações existentes, `CodigoRecusa::AcaoBloqueada` em `ValidacaoComandos::validar`, `hash_estado` refletivo e HUD `Função: <nome>` / `Nível: N/12`.
- **In scope**: enum e campos, constantes e progressão, tabela de desbloqueio, recusa, hash e HUD. **Out of scope**: habilidades completas W/E/R e números de Fôlego/alcance/recarga (`[HIPÓTESE]` §10.1–§10.5), PVE/PNJs/mapa, netcode, contratos/protocolo, R6 (N/A).
- **Success metrics**: SC-001..SC-006 em `define.md` (todas com baseline **[NV]** até verificação).
- **Carried-forward open questions**: default de `funcao` (`Artesao` `[HIPÓTESE]`), ganho global por jogador, ordem de `AcaoBloqueada` entre as recusas e posição dos campos no `hash_estado` — validar na especificação.

### Rastreabilidade FR / SC / T

| ID | Eco no assessment | Autoridade |
|---|---|---|
| FR-001 | `enum Funcao` (Artesão, Guerreiro, Intelectual, Camponês, Agitador) e `Jogador.funcao` | GDD §10.1:504; contrato da sessão |
| FR-002 | `Jogador.nivel`, inicial 1, teto `NIVEL_MAXIMO = 12` | GDD §10.1:523 `[HIPÓTESE]`; contrato |
| FR-003 | `TICKS_POR_NIVEL = 900` (30 s a 30 Hz) e +1 exato a cada 900 ticks | contrato da sessão; `define.md` |
| FR-004 | Desbloqueio Q1 `atacar_forte`, W2 `construir_barreira`/`construir_ponte`, E3 `propaganda`/`propaganda_no`, R6 `atacar_no` | GDD §10.1:522 `[HIPÓTESE]`; contrato |
| FR-005 | `CodigoRecusa::AcaoBloqueada` em `ValidacaoComandos::validar` e `funcao`/`nivel` em `hash_estado` | contrato; `shape.md` |
| FR-006 | HUD `Função: <nome>` / `Nível: N/12`, sem novas teclas; nenhum contrato/protocolo novo | contrato; `intake.md` |
| SC-001 | Nível inicial 1 e teto 12, inclusive após novos 900 ticks | `define.md` · Success Metrics 1 |
| SC-002 | 899 ticks não sobem; 900.º tick sobe +1 uma única vez | `define.md` · Success Metrics 2 |
| SC-003 | Nível 1 aceita Q e recusa W/E/R com `AcaoBloqueada` | `define.md` · Success Metrics 3 |
| SC-004 | Nível 2 libera W; nível 3 libera E; antes disso, recusa | `define.md` · Success Metrics 4 |
| SC-005 | R só a partir do nível 6; níveis 2–5 recusam | `define.md` · Success Metrics 5 |
| SC-006 | `hash_estado` posicional e HUD com função/nível; sem novas teclas | `define.md` · Success Metrics 6 |
| T001 | `enum Funcao` com cinco variantes materiais | `shape.md` · Opção A |
| T002 | Campos `Jogador.funcao` e `Jogador.nivel` | `shape.md` · Opção A |
| T003 | Constantes `TICKS_POR_NIVEL = 900` e `NIVEL_MAXIMO = 12` | `shape.md` · Opção A |
| T004 | Avanço de +1 por bloco de 900 ticks no tick determinístico | `shape.md` · Opção A |
| T005 | Teto inclusivo em 12, sem avanço adicional | `shape.md` · Assumptions |
| T006 | Tabela de mapeamento Q1/W2/E3/R6 → ações existentes | `shape.md` · Opção A |
| T007 | Variante `CodigoRecusa::AcaoBloqueada` | `shape.md` · Opção A |
| T008 | Recusa de ação bloqueada em `ValidacaoComandos::validar` | `shape.md` · Opção A |
| T009 | `hash_estado` cobrindo `funcao` e `nivel` em posição fixa | `shape.md` · Opção A |
| T010 | Cliente sem novas teclas; reaproveitamento dos comandos existentes | `shape.md` · Opção A |
| T011 | HUD `Função: <nome>` e `Nível: N/12` | `shape.md` · Opção A |
| T012 | Testes de 899/900, teto 12, desbloqueios Q/W/E/R, recusa e hash | `shape.md` · Assumptions |

Os nomes definitivos de T001..T012 serão os de `specs/052-papeis-niveis/tasks.md` (o diretório hoje contém apenas o template `spec.md`); a coluna "Eco" apenas rastreia a cobertura deste assessment. R6 é **N/A** por constituição v1.2.2 e não recebe tarefa. Nada aqui é especificação, plano ou implementação.

Veredito: go
