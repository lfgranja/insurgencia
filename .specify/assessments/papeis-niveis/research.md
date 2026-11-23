# Research — Papéis, Níveis e Desbloqueio Q/W/E/R

**Data**: 2026-11-23T15:08Z · Evidências lidas do GDD §10.1–§10.5, do `STATE.md`, de assessments anteriores, do código-fonte existente e do contrato da sessão; **nada reexecutado**.

**Evidence confidence (overall)**: high para a existência documental dos papéis, do teto 12 e da sequência Q1/W2/E3/R6 no GDD §10.1 e para o empate 48/625 no `STATE.md`; low para todos os números de progressão — `[HIPÓTESE]` por declaração explícita do GDD e pela escolha de 900 ticks (30 s) no contrato da sessão.

## Users & Demand

- Jogadores da Comuna — o GDD §10.1:504 define as funções como "papéis materiais na insurgência, não classes mágicas" (P2/Materialismo Histórico); hoje o Núcleo não registra qual papel o jogador exerce nem em que nível está, e todas as ações Q/W/E/R são indiscriminadamente aceitas. [source: `docs/GDD - Insurgência.md` §10.1:504-524, lido; `crates/sim/src/lib.rs:132-142,160-190`, lido] (confidence: high)
- Mantenedores do Núcleo Rust e SWE/orquestrador — precisam de progressão determinística por tick, refletida em `hash_estado` e recusada por `ValidacaoComandos::validar`, sem nova ação de protocolo, sem `crates/contratos` e sem stack de rede. [source: contrato da sessão, imutável] (confidence: high)
- Designer — precisa que os números do §10.1 continuem rotulados `[HIPÓTESE]`: teto 12 e desbloqueio Q1/W2/E3/R6 são chutes de design, e as habilidades completas de §10.2–§10.5 não devem ser implementadas por acidente nesta fatia. [source: GDD §10.1:515-524, §10.2–§10.5, lidos] (confidence: high)
- Demanda é documental, não telemetria: não há dado de uso coletado neste assessment; o que existe é a regra escrita no GDD e a decisão 80/20 registrada em `STATE.md` (15:08Z). [source: GDD §10.1; `.specify/memory/STATE.md`, lidos] (confidence: high)

## Prior Art

- [source: `.specify/memory/STATE.md`, lido] O candidato "Papéis W/E/R + níveis" apareceu a 48/625 em 049 (atrás de controle de ilha, 108/625), em 050 (atrás de Nós/vitória, 60/625) e em 051 (atrás de derrota por timeout, 192/625). Em 052 torna-se a frente de produto escolhida; a pontuação não muda, muda a prioridade. (confidence: high)
- [source: `.specify/assessments/funcoes/decision.md`, lido] O ciclo 015 já fixou funções de forma restrita ("Barricada do Artesão e Ataque Forte validados no servidor"), com números literais do GDD e validação no servidor. Aqui o aproveitamento é o inverso: reutilizam-se **ações já existentes** e acrescenta-se só o gatilho de nível, sem reabrir custo, dano, canalização ou recarga daquele ciclo. (confidence: high)
- [source: `crates/sim/src/lib.rs:132-142,1427-1450`, lido] `ACOES_PERMITIDAS` já contém `atacar_forte`, `construir_barreira`, `construir_ponte`, `propaganda`, `propaganda_no` e `atacar_no`; `CodigoRecusa` e `ValidacaoComandos::validar` já são o ponto de extensão natural para `AcaoBloqueada`. Nenhuma dessas ações precisa ser criada. (confidence: high)
- [source: `crates/sim/src/lib.rs:160-190`, lido] `Jogador` já carrega `posicao`, `vida`, `seq`, `request_id`, `materiais`, contadores de coleta/construção/ponte, `propaganda_alvo` e `no_alvo`, mas não `funcao` nem `nivel`; o avanço de ticks da simulação já é o local onde a progressão pode ser acoplada. (confidence: high)
- [source: `specs/052-papeis-niveis/spec.md`, lido] O diretório de especificação contém apenas o template inicial (marcadores `[FEATURE NAME]`, `[NEEDS CLARIFICATION]`): os rastros FR/SC/T deste assessment são antevisões do handoff, não requisitos já ratificados. (confidence: high)

## Market & Context

- Alternativa atual: nenhuma progressão — o jogador tem acesso imediato a todas as ações em `ACOES_PERMITIDAS`, e o HUD não mostra papel nem nível. Implementar o §10.2–§10.5 completo exigiria Fôlego, habilidades, alvos, áreas, canalização e NPCs, fora do orçamento desta fatia. [source: GDD §10.1-§10.5; código lido] (confidence: high)
- Custo de inação: o desbloqueio Q1/W2/E3/R6 e o teto 12 permanecem apenas uma linha de tabela do GDD; qualquer ciclo futuro que dependa de "jogador de nível N" ou de "função material" precisaria reconstruir o estado e a validação desde zero, e a progressão curta pensada para partidas de 20–30 min (§10.1:523) continuaria sem verificação. [source: GDD §10.1:515-524; contrato da sessão] (confidence: high)

## Data & Constraints

- Contrato de tempo: **30 ticks/s** (PASSO_SEGUNDOS = 1/30, observado em `crates/sim/src/lib.rs`), logo **30 s = 900 ticks**; constantes `TICKS_POR_NIVEL = 900` e `NIVEL_MAXIMO = 12`, nível inicial 1 e +1 a cada 900 ticks até o teto. Os 30 s são escolha de protótipo **`[HIPÓTESE]`**/contrato da sessão; o GDD não fixa o ritmo de subida de nível. [source: contrato da sessão; `crates/sim/src/lib.rs:8`, lido] (confidence: high)
- Mapeamento de desbloqueio **`[HIPÓTESE]`** (GDD §10.1:522, contra 052 da sessão): Q/nível 1 → `atacar_forte`; W/nível 2 → `construir_barreira` e `construir_ponte`; E/nível 3 → `propaganda` e `propaganda_no`; R/nível 6 → `atacar_no`. As novas habilidades W/E/R do GDD (Oficina de Campo, Análise Materialista, Plano Mestre etc.) ficam fora de escopo. [source: GDD §10.1; contrato da sessão] (confidence: high)
- Enum `Funcao` com Artesão, Guerreiro, Intelectual, Camponês e Agitador — os cinco papéis materiais do GDD §10.1/§7.1; o valor padrão de `Jogador.funcao` não foi fixado pelo contrato e é tratado como **`[HIPÓTESE]`** a validar na especificação (candidato: `Artesao`, por ser o papel construtor citado em §7.1:306 e no ciclo 015). [source: GDD §10.1:504, §7.1:303-307; contrato da sessão] (confidence: low para o default; high para a lista)
- Fronteira técnica: `ValidacaoComandos::validar` passa a rejeitar ação bloqueada com `CodigoRecusa::AcaoBloqueada`; `funcao` e `nivel` entram em `hash_estado`; cliente mostra `Função: <nome>` e `Nível: N/12` sem novas teclas — nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, nem novos campos de comando. [source: contrato da sessão, imutável] (confidence: high)
- Estilo obrigatório (ADR-004 / constituição): `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem `async`/rede/Lightyear. [source: `AGENTS.md`, lido; contrato da sessão] (confidence: high)
- R6 (`taskstoissues`) ratificada como intenção em v1.2.0 e removida da sequência executável em v1.2.2: **N/A**; PARE se invocada, nunca issue simulada nem `gh` improvisado. R4 vem antes de R7 (`1.0.0-local`). [source: `.specify/memory/constitution.md`, lido; contrato da sessão] (confidence: high)
- Scorecard ancorado em `STATE.md` (2026-11-23T15:08Z): papéis+níveis 4/2/2/3 = 48/625; PVE/PNJs 4/2/2/3 = 48/625; rede 5/2/1/1 = 10/625; escolha `papeis-niveis`. Critério impacto × confiança × facilidade ÷ risco, denominador **625**. [source: `.specify/memory/STATE.md`, lido] (confidence: high)

## Evidence Against the Idea

- Todos os números da progressão são `[HIPÓTESE]`: o GDD não define 30 s por nível, nem ritmo de ganho, e marca o teto 12 e Q1/W2/E3/R6 como "chutes de design" (§10.1:515-524). Daí confiança **2** e a obrigação de rotular os valores como protótipo — apresentá-los como cânone seria evidência inventada. [source: GDD §10.1:515-524] (confidence: high)
- O mapeamento é uma simplificação deliberada: Q/W/E/R no GDD são habilidades completas e diferentes por papel (§10.2–§10.5), enquanto aqui Q/W/E/R apenas desbloqueiam ações já existentes. Risco de confusão conceitual é real; a spec deve dizer explicitamente que novas habilidades ficam fora de escopo. [source: GDD §10.2–§10.5; contrato da sessão] (confidence: high)
- Empate com PVE/PNJs (48/625) significa que o número sozinho não justifica a escolha: o desempate vem do contrato da sessão e do fato de o §10.1 conter uma regra escrita de desbloqueio, enquanto PVE/PNJs dependem de IA de defensores, mapa e alvos ainda ausentes. [source: `.specify/memory/STATE.md:decisão 052`, lido] (confidence: high)
- A rede tem impacto 5, mas está bloqueada por ADR-004 enquanto Lightyear não entra em `Cargo.toml`/`Cargo.lock`; sincronizar papéis e níveis antes de o estado ser determinístico inverte a dependência. [source: ADR-004; decisões 049-051 em `STATE.md`, lidos] (confidence: high)
- Nenhum teste, gate, build ou execução foi realizado neste assessment: toda conformidade futura é **[NV]**. [source: contrato da sessão] (confidence: high)

## Gaps & Open Questions

- [NEEDS CLARIFICATION: qual é o valor padrão de `Jogador.funcao` para jogadores já existentes? Candidato `[HIPÓTESE]`: `Artesao` — validar no `/speckit.specify`]
- [NEEDS CLARIFICATION: o ganho de +1 a cada 900 ticks corre para todos os jogadores, independentemente de papel, e o teto 12 é inclusivo? Validar no `/speckit.specify`; o contrato fixa 1 → 12, +1/900]
- [NEEDS CLARIFICATION: `CodigoRecusa::AcaoBloqueada` entra antes ou depois das recusas já existentes em `ValidacaoComandos::validar`? A ordem das recusas é observável e deve ser congelada na spec]
- [NEEDS CLARIFICATION: `funcao` e `nivel` entram em `hash_estado` na ordem dos campos de `Jogador` ou em bloco explícito? Confirmar a estabilidade da ordem no `/speckit.specify`]
- Fora de escopo por decisão, não lacuna: habilidades completas W/E/R do GDD §10.2–§10.5, Fôlego/alcance/recarga/dimensões (todos `[HIPÓTESE]`), IA de PNJs, mapa/ilhas, netcode e R6 (N/A).

## Sources

- `docs/GDD - Insurgência.md` §10:504-524 e §10.2–§10.5 (arquivo local, lido; sem URL).
- `.specify/memory/STATE.md` (decisões 049-052; arquivo local, lido).
- `.specify/memory/constitution.md` e `AGENTS.md` (arquivos locais, lidos).
- `.specify/assessments/funcoes/decision.md` (arquivo local, lido).
- `crates/sim/src/lib.rs:8,132-142,160-190,1427-1450` e `crates/cliente/src/main.rs` (arquivos locais, lidos).
- `specs/052-papeis-niveis/spec.md` (template inicial, lido).
- Contrato da sessão `20261123T150800Z-papeis-niveis` (imutável; fonte das constantes 900/12, do mapeamento Q1/W2/E3/R6 e da API prevista).
- Nenhuma URL foi buscada; nenhum comando de gate, build, teste, formato, git remoto, GitHub ou emissão de issue foi executado por este agente.

## Rastreabilidade

Fatos colados como observados, sem reexecução. Rastros do ciclo: FR-001..FR-006 (enum/campos/constantes e progressão/mapeamento de ações/validação e hash/HUD), SC-001..SC-006 (critérios observáveis), T001..T012 (entregáveis); a definição canônica viverá em `specs/052-papeis-niveis/`, hoje ainda em template. Todos os valores do GDD §10.1 e os 900 ticks são **`[HIPÓTESE]`**/protótipo. R6 é **N/A** por constituição v1.2.2 e não entra neste rastro. Tudo o que depende de execução (gates, testes, R4, R7) permanece **[NV]**.
