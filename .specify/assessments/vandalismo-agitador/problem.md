# Problem — Ato de Vandalismo do Agitador sem recipiente no protótipo (BF-002c)

**Data:** 2026-11-25

## Problema

O GDD descreve a habilidade Q do Agitador com critérios de aceitação explícitos — 120 de dano, 1,5 s de silêncio, cancelamento de canalização em curso, 20 Fôlego, 2 m e 7 s de recarga, todos rotulados `[HIPÓTESE]` (`docs/GDD - Insurgência.md:604-612`) —, mas o protótipo determinístico não a implementa: `ACOES_PERMITIDAS` em `crates/sim/src/lib.rs:222-236` tem 14 tokens e não contém `vandalismo`; `fase_skills` (`:920-1010`) não tem ramo para a ação; `ValidacaoComandos::validar` (`:2463-2510`) não a aceitaria; o cliente Bevy usa Q para `atacar_forte` (`crates/cliente/src/main.rs:823-831`) e não oferece tecla, dica ou efeito de silêncio. Há, portanto, uma lacuna entre uma regra documentada e priorizada (BF-002c, 288/3125 — `.specify/memory/STATE.md:606-609`) e um comportamento observável no núcleo, na CLI e no cliente. Sem essa tradução, a função "infiltração, silêncio, confusão de NPCs" do §10.8 permanece apenas descritiva e qualquer demonstração de coordenação/controle dependeria de lógica no cliente, fora da autoridade determinística do `Mundo`.

## Por que é válido

- **Impacto 4** (escala 1–5) — o mais alto entre os candidatos restantes do ciclo 061; o efeito toca combate, controle de canalização e o papel do Agitador, mas não destrava topologia de ilhas nem partida real (por isso não é 5). [source: `.specify/memory/STATE.md:606-609`]
- **Confiança 4** (escala 1–5) — a lacuna é verificada por leitura direta de três fontes locais no âncora `origin/dev = 52138c3`: GDD §10.4/§10.8, `crates/sim/src/lib.rs:222-236,920-1010` e a decisão 80/20 em STATE.md:606-609. O desconto a 4 (e não 5) deve-se a três incertezas declaradas: (1) os números são `[HIPÓTESE]`, não cânone; (2) a representação exata de silêncio/recarga e sua entrada no `hash_estado` ainda não foi decidida; (3) nenhum comportamento foi executado ou medido nesta etapa. [source: fontes lidas; `[NV]`]
- **Fechamento 2 / esforço⁻¹ 3 / risco⁻¹ 3** — o gap é fechável com um recorte pequeno (um token, validação, um ramo de fase, uma tecla e hash), mas depende de superfície de Estado não validada e de atualização de baselines de hash; o risco é moderado, não alto. [source: `.specify/memory/STATE.md:606-609`]
- **Score 288/3125** (`4×4×2×3×3`) — superior a BF-015b (144/3125), com filtro eliminatório satisfeito: há comportamento observável e consumidor fora dos artefatos do ciclo (jogador Bevy e CLI). [source: `.specify/memory/STATE.md:606-609`]
- **Custo de não agir** — o próximo ciclo começaria com a mesma whitelist de 14 tokens e sem ramo de vandalismo; a reimplementação acabaria no cliente (sem determinismo, sem cobertura de hash e sem reprodutibilidade via CLI), ou seria adiada indefinidamente enquanto BF-015b e BF-002c seguem abertos. Corrigir agora reusa o padrão já entregue por `sinalizar_sincronia` no ciclo 060.

## Afetados & stakeholders

- **Usuários**: jogador do cliente Bevy na Função Agitadora (hoje sem Q de vandalismo, sem tecla e sem HUD de efeito); operador da CLI `insurgencia-servidor`, que precisa de um comando aceito e reproduzível por stdin.
- **Stakeholders**: SE — autoridade de governança que registrou a escolha e o DoD(0.9) em STATE.md:606-609; mantenedor único — responsável pelo núcleo determinístico, pela CLI e pelo cliente sob a constituição v1.2.2; backlog — BF-002c precisa passar a `fechado` sem abrir BF-015b.
- **Fora de afetação**: PR #76 (`dev→main`) e deploy, de outro dono e SWE-exclusivos; GDD, CI, stack, contrato de rede (`crates/contratos`, `PROTOCOL_VERSION`) e `Cargo.toml`.

## Metas

1. **Aceitar o comando** — adicionar `vandalismo` como 15º token de `ACOES_PERMITIDAS`, com exigência de destino, e validar alcance de contato (2 m), Fôlego (20) e recarga (210 ticks), preservando `seq`/`request_id` monotônicos e `ultima_recusa` sticky nas recusas.
2. **Aplicar o efeito no núcleo** — ramo em `fase_skills` que cause 120 de dano e 45 ticks de silêncio no alvo, cancele canalização em andamento e não duplique crédito por tick, respeitando a ordem das fases.
3. **Cobrir o novo estado no hash** — incluir silêncio (e eventual recarga) no `hash_estado` em ordem estável, com baselines/golden hashes atualizados no mesmo conjunto de mudanças.
4. **Entregar consumidor e prova** — tecla protótipo `H` no cliente (Q já é `atacar_forte`), dica na HUD, mais testes em `sim`, `servidor` e `cliente` e `./verify.sh`, fechando BF-002c.
5. **Preservar a fronteira** — nenhuma regra canônica inventada, nenhuma promoção de `[HIPÓTESE]`, nenhuma mudança de GDD/CI/stack/contrato de rede, R6 N/A, R4 antes de R7.

## Limites

Fora de escopo nesta etapa: escrita de qualquer token, validação, ramo, tecla, HUD, spec, plano ou task; execução de `cargo`, `./verify.sh`, clippy, nextest, testes, build ou formatação; stage, commit, push, merge, deploy ou promoção `dev→main`; issue simulada ou `gh` improvisado (R6 N/A). No ciclo de implementação: nenhuma topologia canônica de ilhas, nenhum sistema de papéis/funções no núcleo, nenhum modelo de itens (o GDD menciona itens disponíveis durante o silêncio, mas o protótipo não os tem), nenhuma rede/lobby/5v5 e nenhuma alteração de GDD, contrato de rede, `Cargo.toml` ou CI. Convenções: pt-BR, `[NV]`, termos estrangeiros explicados na 1ª ocorrência, sem alegação de execução ou validação ao vivo.

## Critérios de sucesso

- Comando `vandalismo` aceito por `ValidacaoComandos::validar` e por `fase_skills`, com destino obrigatório e nível compatível (baseline: 14 tokens e zero ramos; meta: 15 tokens, um ramo, nenhuma recusa indevida).
- Alvo com 120 de dano e `silenciado_ticks = 45` no tick de acerto; canalização em curso cancelada no mesmo tick (baseline: **[NV]**; meta: verificável em teste determinístico de `sim`).
- Recusa fora de alcance, sem Fôlego ou em recarga preserva estado e monotônicos: `seq`, `request_id`, `ultima_recusa` e contadores do alvo inalterados (baseline: **[NV]**; meta: caso de borda coberto em `sim` e CLI).
- `hash_estado` reflete o novo estado: dois estados que diferem só no silêncio produzem hashes diferentes; golden hashes atualizados (baseline: 12 campos; meta: cobertura do novo campo sem quebrar determinismo).
- Consumidores entregues: tecla `H` no cliente com dica na HUD e comando reproduzível via CLI `insurgencia-servidor` (baseline: Q = `atacar_forte`, sem H; meta: H aciona `vandalismo` sem colisão com `B,C,P,G,Q,R,F,T,E,Y,V` ou `Espaço`).
- Prova e backlog: `cargo test --locked -p insurgencia-sim --test simulacao`, `cargo test --locked -p insurgencia-servidor --test cli`, `cargo test --locked -p insurgencia-cliente --lib`, `cargo check --locked -p insurgencia-cliente --all-targets` e `./verify.sh` verdes; **BF-002c `fechado`**, BF-015b aberto (baseline: **[NV]**; meta: verde em ciclo próprio).

## Perguntas abertas

- **[NV]** Qual a representação exata do silêncio no `Jogador` (contador, decremento em qual fase, interação com `imobilizado_ticks`) e como ele entra no `hash_estado` em ordem estável?
- **[NV]** Quais contadores de canalização zerar no acerto e em que ordem frente a `fase_finish`; o que acontece se o dano for letal no mesmo tick?
- **[NV]** A recarga de 210 ticks é por agressor, por habilidade ou por par agressor-alvo, e onde o contador é decrementado?
- **[NV]** A tecla `H` e o texto da dica na HUD são protótipos: confirmar não colisão com atalhos existentes e definir o formato da dica na spec.
- **[NV]** Como tratar a cláusula do GDD "itens e ataques básicos continuam disponíveis durante o silêncio" se o protótipo não modela itens — registrar como ressalva, sem inventar superfície.
