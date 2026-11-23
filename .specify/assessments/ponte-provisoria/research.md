# Research — Ponte provisória determinística

**Data**: 2026-11-23T06:25Z · Evidências coladas do contrato da sessão, do GDD, de STATE.md, da constituição e da ADR-004, sem reexecução.

## Evidência primária

- [source: `docs/GDD - Insurgência.md` §9.3:475, lido] Tabela de recursos: `Materiais` — origem "pontos neutros; Floresta (+4), Montanha (+6)", usos "barricada (20), **ponte provisória (15)**, sabotagem", renda "+2 por pulso por ponto". O custo 15 é canônico. (confidence: high)
- [source: `docs/GDD - Insurgência.md` §9.3:483, lido] Critério de aceitação de recursos: "Uma barricada custa exatamente 20 Materiais e **uma ponte provisória 15**; **sem saldo, a ação é recusada sem consumo parcial**." (confidence: high)
- [source: `docs/GDD - Insurgência.md` §8.4:407, lido] Delta-3 (Ilha da Barca): "Artesão constrói Ponte Provisória | Delta-2 | **atalho de 8 s entre ilhas por 45 s** | **ponte destrutível com 60 PV**." Fixam-se 60 PV e 45 s de duração; o atalho de 8 s depende de mapa/ilhas. (confidence: high)
- [source: `docs/GDD - Insurgência.md` §10.3:574, lido] "na Rota Inferior / Delta do Comércio, a mesma mecânica gera a Ponte Provisória usada em emboscadas. `[SUPOSIÇÃO]`" — o contexto de uso é suposição; custo, PV e duração permanecem canônicos. (confidence: high)
- [source: contrato da sessão, imutável] Conversão determinística a 30 ticks/s: **45 s = 1350 ticks** de duração. O **tempo de construção de 45 ticks** é escolha de protótipo, **não valor canônico**, e deve ser registrado como tal. (confidence: high)
- [source: contrato da sessão, imutável] API Rust prevista: ação `construir_ponte` em `ACOES_PERMITIDAS` da sim (sem mudança em `crates/contratos` ou `PROTOCOL_VERSION`); struct pública `Ponte { id, dono, posicao, pv, ticks_restantes }` em `BTreeMap<u64, Ponte>` em `Mundo`; constantes `CUSTO_PONTE_MATERIAIS`, `PV_PONTE`, `TICKS_DURACAO_PONTE`, `TICKS_CONSTRUCAO_PONTE`; validação `destino == posicao` e saldo; débito atômico; expiração determinística; `hash_estado`; HUD com contagem de Pontes e tecla `P`. (confidence: high)
- [source: contrato da sessão, imutável] Regra de recusa: sem saldo suficiente, a ação é recusada **sem consumo parcial**; o débito de 15 Materiais é atômico (todo ou nada). (confidence: high)
- [source: contrato da sessão, imutável] Regras Rust: `forbid(unsafe_code)`, `deny(clippy::pedantic)`, sem `unwrap`/`expect`/`#[allow]`, `max_width=100`, sem async/rede/Lightyear. (confidence: high)
- [source: `.specify/memory/STATE.md:317-328`, lido] Decisão 80/20 do ciclo 045 (2026-11-23T06:25Z): I/C/F/R = 3/3/3/4 → 108/625 para ponte provisória; 4/2/2/3 → 48/625 Súditos/Propaganda; 4/2/2/3 → 48/625 Combate/Fôlego; 5/2/1/1 → 10/625 rede; escolha `ponte-provisoria`, sem mudança de stack/dependência/protocolo; atalho de 8 s fora de escopo; R6 N/A. (confidence: high)
- [source: `.specify/memory/constitution.md:48-50,79-80,84`, lido] Constituição v1.2.2: R1 `go`, R4 antes de R7 (`1.0.0-local`), `analyze` READ-ONLY após `tasks`, `converge` append-only; R6 removida da sequência executável pela emenda de 23/11 — **R6 N/A**. (confidence: high)
- [source: `docs/adr/004-migracao-bevy-lightyear.md:15-24,53`, lido] ADR-004 Accepted (20/11) adota Bevy 0.20.1 + Lightyear 0.31, mas "nenhuma troca adicional de engine é permitida sem novo ADR"; Lightyear consta apenas em ADR/docs, ausente de `Cargo.toml`/`Cargo.lock`. Rede bloqueada até decisão da SE. (confidence: high)
- [source: `.specify/assessments/coleta-jogavel/decision.md`, lido] O padrão de ações do jogador já usa `ACOES_PERMITIDAS`, entrada de teclado no HUD e `hash_estado` estável sobre `BTreeMap`/`BTreeSet`; a ponte reusa esse caminho sem contrato novo. (confidence: high)

## Interpretação

A ponte provisória é o próximo recurso implementável sem renegociação de design porque **três números já estão fechados**: 15 Materiais (§9.3:475,483), 60 PV e 45 s (§8.4:407). O trabalho 80/20 não é inventar a mecânica, mas torná-la **estado determinístico observável**: uma ação `construir_ponte` que valida posição e saldo, debita 15 Materiais de uma vez, cria uma `Ponte` com 60 PV e 1350 ticks, e a remove por dano (`pv == 0`) ou por expiração determinística, com tudo refletido em `hash_estado` em ordem estável. Nenhuma decisão de design fica pendente: por isso a opção pontua 3 em confiança — o valor é canônico, mas o uso em emboscadas é `[SUPOSIÇÃO]` (§10.3:574) e o atalho de 8 s não entra neste recorte.

Consequência da duração: a ponte vive **1350 ticks** e o atalho de 8 s (240 ticks) é uma propriedade de mapa/ilhas, **fora de escopo e registrado como limitação** — o protótipo entrega o objeto destrutível e temporário, não a travessia. O tempo de construção de **45 ticks** é escolha de protótipo, não regra canônica: serve para dar uma janela de canalização determinística, mas não deve ser confundido com os 45 s de duração nem promovido a número do GDD. O fator risco/segurança é 4 (não 5) porque débito atômico, validação `destino == posicao`, ordem do `BTreeMap` no hash e expiração por tick exigem contrato explícito e verificação.

## Alternativas rejeitadas

1. **Rede (Lightyear/async/sockets)** — rejeitada (10/625): ADR-004 declarou a stack, mas Lightyear está ausente de `Cargo.toml`/`Cargo.lock` e "nenhuma troca adicional de engine é permitida sem novo ADR" (ADR-004:53); introduzir transporte exigiria decisão da SE. Impacto 5, mas confiança 2, facilidade 1 e risco invertido 1.
2. **Súditos/Propaganda** — rejeitada (48/625): conversão de Nós, Resistência Ideológica e Propaganda de 20 s são `[HIPÓTESE]` (GDD §6.2:275); depende de sistema ausente e sem valor canônico de construção.
3. **Combate/Fôlego** — rejeitada (48/625): Fôlego, alcance, recarga e dimensões são `[HIPÓTESE]` (GDD §10.3:574-579); mexer em combate puxa IA, dano e física para dentro do recorte, sem regra canônica de ponte.

## Riscos registrados (não reabertos)

- **Atalho de 8 s (§8.4:407)**: depende de mapa/ilhas; **fora de escopo**, registrado como limitação. Não modelar travessia, ilhas Delta-1/Delta-2 nem emboscadas neste ciclo.
- **Tempo de construção de 45 ticks**: escolha de protótipo, não valor canônico; registrar como tal e não reabrir nem promover a regra do GDD.
- **Risco invertido 4 (não 5)**: confiança e facilidade são medianas; débito atômico sem consumo parcial, validação `destino == posicao`, um tick de construção/expiração por ponte e ordem estável do `hash_estado` exigem contrato explícito; permanecem **[NV]** até execução.
- **Dívida R6**: R6 é **N/A** por constituição v1.2.2 — removida da sequência executável; se um fluxo tentar invocá-la, PARE e escale ao SWE, sem `gh` improvisado e sem issue simulada.
- **Verificação**: gates, testes, R4 e R7 permanecem **[NV]** — nenhum foi executado por este agente.

## Rastreabilidade

Fatos colados como observados, sem reexecução. Nenhum comando de gate, build, git remoto, GitHub ou emissão de issue foi executado por este agente. Rastros do ciclo: FR-001..FR-006 (constantes/ação/estado/validação e débito/expiração e hash/HUD), SC-001..SC-006 (critérios observáveis), T001..T012 (entregáveis); definição canônica em `specs/045-ponte-provisoria/`, scorecard em `.specify/memory/STATE.md:317-328`.
