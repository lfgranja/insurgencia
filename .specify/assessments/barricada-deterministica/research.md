# Research — Barricada determinística de materiais

**Data**: 2026-11-23T02:43Z · Evidências coladas do contrato da sessão e do GDD, sem reexecução.

## Evidência primária

- [source: `docs/GDD - Insurgência.md` §9.3:475,483, lido] Materiais vêm de pontos neutros e financiam "barricada (20), ponte provisória (15), sabotagem". Critério de aceitação: "Uma barricada custa exatamente 20 Materiais e uma ponte provisória 15; sem saldo, a ação é recusada sem consumo parcial". (confidence: high)
- [source: `docs/GDD - Insurgência.md` §10.3:574, lido] "Q — Construir Barricada": custo 20 Materiais, "1,5 s de canalização", "200 pontos de vida", "12 s de duração"; o texto declara "Materiais, canalização, pontos de vida e duração são canônicos" e marca `[HIPÓTESE]` apenas Fôlego, alcance, recarga e dimensões. (confidence: high)
- [source: `docs/GDD - Insurgência.md` §10.3:577-579, lido] Critérios: parede interdita corredor de 3 m por 12 s e é destruída com 200 de dano; projéteis inimigos colidem; "no máximo duas barricadas ativas; a terceira remove a mais antiga". (confidence: high)
- [source: contrato da sessão, imutável] Conversão determinística a 30 ticks/s: 1,5 s = **45 ticks**, 12 s = **360 ticks**. API Rust prevista: `CUSTO_BARREIRA_MATERIAIS`, `TICKS_CONSTRUCAO_BARREIRA`, `PV_BARREIRA`, `TICKS_DURACAO_BARREIRA`, `MAX_BARREIRAS_POR_JOGADOR`; `Jogador.construcao_ticks`/`construcao_ponto`; `Mundo.barreiras`/`proxima_barreira_id`; `Barreira { id, dono, posicao, pv, ticks_restantes }`; `CodigoRecusa::SaldoInsuficiente`; `Comando.destino` como ponto de construção. (confidence: high)
- [source: contrato da sessão, imutável] Validação e fase: `construir_barreira` exige `destino == posicao` via `ponto_igual` e `materiais >= 20`; `fase_skills` revalida saldo ao completar, avança no máximo 1 canalização por jogador por tick (`BTreeSet<u64>` local), zera canalização se `mover` mudar posição, debita 20 atomicamente, insere `Barreira` e remove a mais antiga ao exceder 2; `hash_estado` inclui os novos campos e todas as barreiras em ordem de `BTreeMap`. (confidence: high)
- [source: `docs/GDD - Insurgência.md` §16.3:1505 e §20.2/H4:1678, lidos] O mesmo conjunto numérico reaparece como `[HIPÓTESE]` na linha do Artesão do MVP e na lista H4. (confidence: high) — divergência de **rótulo**, não de valor.
- [source: `.specify/memory/constitution.md` via `.specify/assessments/estado-pos-v122/research.md:9`, lido] Constituição v1.2.2: R6 ratificada como intenção, fora da sequência executável até existir `resolve-tasks.sh`/adaptador; PARE se invocada, nunca simular issue. (confidence: high)
- [source: `.specify/assessments/estado-escalonamento-45/decision.md:14`, lido] `construir_barreira` já apareceu como alternativa rejeitada (60/625) em ciclo de escopo documental; hoje é o objeto dedicado do ciclo 040. (confidence: high)

## Interpretação

A barricada é o número mais bem ancorado do GDD: aparece em §9.3 e §10.3 com rótulo explícito de "canônica", e a conversão para ticks é determinística. Isso dá confiança máxima à implementação sem exigir decisão de design: o que fica de fora (Fôlego, alcance, recarga, dimensões) está marcado como `[HIPÓTESE]` pelo próprio GDD. A contribuição 80/20 está em transformar esses valores em comportamento observável — recusa sem consumo parcial, uma canalização por jogador por tick, cancelamento por movimento, débito atômico e hash determinístico — em vez de construir netcode ou combate primeiro. Nenhum campo novo em `crates/contratos`, nenhuma mudança de `PROTOCOL_VERSION`: o recorte é autocontido no Núcleo e no cliente.

## Alternativas rejeitadas

1. **Servidor + Lightyear** — rejeitada (10/625): sincronizar pressupõe estado determinístico para sincronizar; sem a barricada no Núcleo não há fatia verificável nem métrica observável. GDD §16.3:1490 define conexão direta e Núcleo isolado, mas não inverte a ordem de implementação.
2. **Ponte provisória** — rejeitada (108/625): GDD §10.3:574 marca a ponte como `[SUPOSIÇÃO]` e §9.3:475 lhe atribui 15 Materiais; é variação do mesmo caminho, sem valores canônicos próprios e sem rastro SC dedicado.
3. **Combate/Fôlego** — rejeitada (48/625): GDD §16.3:1509 estabelece que o MVP prova jogabilidade por desafios de ilha, "em vez de combate puro"; priorizar combate inverte a intenção validada pelo projeto.

## Riscos registrados (não reabertos)

- **Conflito canônico §16.3/H4**: §9.3/§10.3 rotulam os valores como canônicos; §16.3:1505 e H4:1678 os repetem como `[HIPÓTESE]`. Decisão: prevalecem §9.3/§10.3; o rótulo divergente é dívida documental registrada, sem renegociação de valores e sem reabertura da decisão de design.
- **Dívida R6**: a constituição v1.2.2 mantém R6 como intenção, mas fora da sequência executável até existir adaptador de issues; nenhum `gh` improvisado, nenhuma issue simulada, PARE se invocada. A dívida é de automação, não de produto.

## Rastreabilidade

Fatos colados como observados, sem reexecução. Nenhum comando de gate, build, git remoto, GitHub ou emissão de issue foi executado por este agente. Tudo o que depende de execução (gates, testes, R4, R7) permanece **[NV]**. Rastros do ciclo: FR-001..FR-006 (contrato/estado/comportamento/hash/entrada), SC-001..SC-006 (critérios observáveis), T001..T014 (entregáveis); a definição canônica vive em `specs/040-barricada-deterministica/`.
