# Problem — Coleta determinística de materiais

**Data**: 2026-11-23T01:54Z

O GDD §7.1 define a Fase 1 do pulso de 30 s com precisão canônica — 6 s em ponto neutro rendem exatamente +3 Suprimentos e +2 Materiais, e sair antes zera o acúmulo parcial — mas o Núcleo determinístico em Rust ainda não a executa: `crates/sim/src/lib.rs` decora `PASSO_SEGUNDOS=1/30` e já aceita `coletar_materiais` em `ACOES_PERMITIDAS`, porém `Jogador` não possui estoques nem contador de coleta, `fase_skills` só processa `mover`, a validação não exige destino para coleta e `hash_estado` não reflete nenhum desses valores. Sem esse incremento, a recompensa canônica não existe no sim autoritativo, não há rastro determinístico do acúmulo e o comportamento de saída antecipada não é verificável. A fila documental foi esgotada no ciclo 038; este é o menor incremento de produto com valor canônico (180/625). **Este ciclo entrega somente os artefatos SDD**; a mudança de código é escopo posterior do SWE/orquestrador, **[NV]**.

## Critérios de sucesso observáveis
1. Os 14 artefatos registram a regra canônica: 6 s = 180 ticks a 1/30 s em ponto neutro → +3 Suprimentos e +2 Materiais, com acúmulo parcial zerado na saída antecipada (GDD §7.1).
2. O contrato técnico auditado aparece rastreado: `TICKS_COLETA=180`, campos `materiais`/`suprimentos`/`coleta_ticks`/`coleta_ponto` em `Jogador`, recusa `DestinoInvalido` para coleta sem destino ou com destino ≠ posição, crédito atômico em `fase_skills` e `hash_estado` incluindo os novos campos.
3. `Comando.destino` é registrado como ID do ponto neutro **sem mudança de protocolo**: nenhuma alteração em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml`.
4. Constituição v1.2.2 do alvo aplicada na cadeia: ordem `... 09 checklist → revisão → 10 tasks → 11 analyze (READ-ONLY) → 12 implement → 13 converge → 27 validate → 26 release`, R1 `go`, R4 antes de R7 `1.0.0`, `converge` append-only; **R6 fora do fluxo** por decisão ratificada, PARE se invocada, nenhuma issue simulada.
5. `test-roadmap.md` escrito **antes do PR**, com 10 passos numerados e os casos: 180 ticks, saída antes, recusa sem destino, hash divergente, ausência de consumo parcial e determinismo.
6. Nenhum build/gate/formatador/teste/commit/push/PR executado; fronteira exata de 14 artefatos respeitada; `.specify/memory/STATE.md`, `converge.md` alheio e alterações preexistentes fora da fronteira intactos.

## Custo de inação
Sem a coleta determinística, o pulso do GDD permanece apenas documentado: o sim não produz Suprimentos/Materiais, a saída antecipada não tem comportamento definido, o hash não distingue estoques distintos e o determinismo entre pares não pode ser verificado. Adiar para depois de novas mecânicas (barricada, dano) aumentaria o raio de mudança sobre o mesmo `fase_skills`/`hash_estado`.

## Fora de escopo
Alteração de código em `crates/sim/src/lib.rs` neste agente; mudanças em `crates/contratos`, `PROTOCOL_VERSION`, `Cargo.toml`; barricada, dano/emboscada, GUI, rede, GDScript; criação de entidade "ponto neutro" no protocolo; execução de gates/build/testes/formatador/commit/push/PR/merge/deploy; adoção de R6, `taskstoissues`/`resolve-tasks.sh` ou simulação de issues; edição de `converge.md` existente, `.specify/memory/STATE.md` (memória local do SWE, não versionada) ou da alteração preexistente em `.specify/feature.json`. Tudo o que depende de execução externa permanece **[NV]**.
