# Shape — Papéis, Níveis e Desbloqueio Q/W/E/R

- **Slug**: `papeis-niveis`
- **Created**: 2026-11-23T15:08Z
- **Recommended option**: Opção A — Modelo mínimo de papéis e progressão por nível

## Options

### Option A — Modelo mínimo de papéis e progressão por nível (recomendada)

- **Sketch**: o Núcleo passa a registrar, em cada `Jogador`, um papel material (`Funcao`: Artesão, Guerreiro, Intelectual, Camponês, Agitador) e um nível. O nível começa em 1 e sobe **+1 a cada 900 ticks (30 s a 30 Hz, `[HIPÓTESE]`)** até `NIVEL_MAXIMO = 12`, sem ganho antes de 900 e sem ultrapassar o teto. O desbloqueio **`[HIPÓTESE]`** incide sobre ações **já existentes**: nível 1 → `atacar_forte` (Q); nível 2 → `construir_barreira`/`construir_ponte` (W); nível 3 → `propaganda`/`propaganda_no` (E); nível 6 → `atacar_no` (R). `ValidacaoComandos::validar` recusa ação de nível superior com `CodigoRecusa::AcaoBloqueada`; `funcao` e `nivel` entram em `hash_estado`; o cliente mostra `Função: <nome>` e `Nível: N/12` sem novas teclas.
- **Appetite**: `small` (dias) — um enum, dois campos, duas constantes, um acúmulo por tick no avanço determinístico, uma tabela de mapeamento, uma variante de recusa, cobertura no hash e duas linhas de HUD; sem `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, rede ou dependência nova.
- **Trade-offs**: ganha a progressão do GDD §10.1 convertida em estado observável e em recusa auditável, reaproveitando ações já validadas nos ciclos 015/045/046/047/050. Sacrifica as habilidades completas Q/W/E/R de §10.2–§10.5 — justamente a parte `[HIPÓTESE]` mais custosa (Fôlego, áreas, canalização, NPCs) — e mantém os números do §10.1 rotulados como protótipo. Riscos: confiança 2 (tudo é hipótese de design), facilidade 2 (não há estado de progressão no Núcleo) e risco invertido 3 (fronteira de tick, ordem das recusas e ordem do hash exigem verificação); mitigados por constantes explícitas, teto inclusivo, tabela de mapeamento única e `hash_estado` posicional.
- **Rabbit holes**: implementar Oficina de Campo, Análise Materialista, Plano Mestre, Espalhar Rumores ou Sabotagem Estrutural; rebalancear Fôlego/alcance/dano das ações existentes; criar novas teclas ou campos de comando; trocar `ACOES_PERMITIDAS` por contrato de protocolo; renomear ações já consolidadas; admitir ganho de nível duplo no mesmo tick; transformar `funcao` em árvore de classes mágicas (contraria P2, §10.1:504).

### Option B — PVE/PNJs primeiro

- **Sketch**: investir a fatia em defensores/NPCs controlados pelo servidor, deixando papéis e níveis para depois.
- **Appetite**: `medium` (semanas) — puxa IA de defesa, alvos, mapa/ilhas e tarefas de ilha para dentro do recorte.
- **Trade-offs**: empata com A no scorecard (4×2×2×3 = 48/625), mas depende de sistemas ausentes (IA de PNJs, mapa, spawn) e não tem linha de progressão escrita equivalente à do §10.1; o contrato da sessão escolheu `papeis-niveis`.
- **Rabbit holes**: navegação de defensores, tabelas de aggro, spawn por ilha, recompensas por PVE — nenhum com valor canônico para este ciclo.

### Option C — Rede / Lightyear primeira

- **Sketch**: postergar a mecânica até transporte, predição e reconciliação cliente-servidor estarem prontos.
- **Appetite**: `large` (meses) — muda a natureza do trabalho de fatia de Núcleo para infraestrutura de rede.
- **Trade-offs**: impacto 5, mas facilidade 1 e risco invertido 1 → **10/625**; ADR-004 bloqueia a troca de stack até decisão da SE (Lightyear ausente de `Cargo.toml`/`Cargo.lock`). Sincronizaria papel e nível antes de o estado ser determinístico — dependência invertida.
- **Rabbit holes**: lobby/matchmaking (fora do MVP), predição de progressão, resolução de concorrência de comandos, troca de engine sem novo ADR.

## Recommendation

Escolher **Opção A**. Ela satisfaz as metas de `define.md` com a menor fronteira: impacto 4 (a progressão `[HIPÓTESE]` do §10.1 vira estado verificável e recusa observável), confiança 2 (todos os números do §10.1 e os 900 ticks são chutes de design rotulados), facilidade 2 (não existe `funcao`/`nivel` no Núcleo; as seis ações já existem e só precisam de gatilho) e risco invertido 3 (fronteira 899/900, teto 12, ordem de recusas e hash exigem verificação). Empate com B em 48/625 é desempatado pelo contrato da sessão e pelo lastro documental do §10.1; C inverte a dependência determinismo → sincronização e está bloqueada por ADR-004. 80/20: **48/625** contra 48/625 (B) e 10/625 (C).

A recomendação **não** propõe implementar as habilidades completas W/E/R: o recorte é o desbloqueio das ações existentes, não o ecossistema de habilidades em volta dele.

## Out of Scope (for the recommended option)

- `[HIPÓTESE]` GDD §10.2–§10.5: habilidades completas Q/W/E/R (Oficina de Campo, Armadilha Improvisada, Sabotagem Estrutural, Análise Materialista, Plano Mestre, Fuga pelas Sombras, Espalhar Rumores etc.); valores de Fôlego, alcance, recarga, custo e dimensões do §10.1–§10.5.
- PVE/PNJs, IA de defensores, mapa, ilhas, spawn, tarefas de ilha, vitória/derrota (frentes dos ciclos 049–051).
- Rebalanceamento de dano, custo, PV, Fôlego ou duração das ações existentes; novas teclas, novos comandos ou novos campos de protocolo.
- Mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`, `.specify/queue/concurrency.jsonl`, `STATE.md` ou `AGENTS.md`.
- R6 (N/A por constituição v1.2.2): nenhuma issue simulada, nenhum `gh` improvisado.

## Assumptions to Validate

- O relógio corrente da simulação é 30 ticks/s (`PASSO_SEGUNDOS = 1/30`), logo 30 s = **900 ticks** — `[HIPÓTESE]` de protótipo fixada no contrato da sessão; confirmar no `/speckit.specify`.
- `NIVEL_MAXIMO = 12` é inclusivo: o nível 12 não avança, mesmo após novos blocos de 900 ticks.
- O valor inicial de `Jogador.funcao` é `Artesao` (**[HIPÓTESE]** de default, candidato do contrato/§7.1:306); confirmar na spec, sem alçar a escolha a cânone.
- O ganho de +1 é global por jogador e ocorre uma vez por janela de 900 ticks, independentemente da função ou da ação executada.
- A tabela de mapeamento Q1/W2/E3/R6 incide simultaneamente sobre as duas ações de W (`construir_barreira`, `construir_ponte`) e as duas de E (`propaganda`, `propaganda_no`).
- `CodigoRecusa::AcaoBloqueada` é retornado quando a ação é conhecida mas o nível é insuficiente, sem conflito com `AcaoDesconhecida` e sem alterar a ordem dos demais códigos.
- `funcao` e `nivel` entram em `hash_estado` em posição fixa e não dependem de formatação de texto; o HUD usa apenas dados já presentes no cliente.
- Estilo obrigatório: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem `async`/rede/Lightyear.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; B e C são as alternativas de `research.md` e não recebem rastro de implementação. T001..T012 pertencem ao handoff em `decision.md`. Todos os valores do GDD §10.1 e os 900 ticks são registrados como `[HIPÓTESE]`/protótipo, não como pendências de design; R6 é **N/A** por constituição v1.2.2.
