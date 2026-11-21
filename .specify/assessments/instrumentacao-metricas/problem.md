# Problem — Itens 2 e 3 da SE: nenhuma instrumentação nem registro de partidas

**Data:** 2026-11-21T01:03Z

## Problema

A SE redefiniu a prioridade do MVP single-player e pediu, como itens 2 e 3 do entregável, "duas partidas consecutivas, uma com cliente modificado (GDD:1567)" e "instrumentação no Núcleo + 20 partidas com as métricas da §17" (`.specify/memory/STATE.md:5`). O GDD torna ambos normativos: o MVP só fecha quando todos os critérios passarem em duas partidas consecutivas, uma com cliente modificado (`:1567`), e "a instrumentação registra eventos no Núcleo, nunca apenas no cliente" (`:1607`), exigindo que o Núcleo grave capturas, tentativas de sincronia e divergências cliente-servidor antes da Fase 4 (`:1620`), com recalibração única após 20 partidas (`:1622`). Ocorre que **não existe nenhuma instrumentação nem registro de partidas no alvo**: `crates/contratos/src/lib.rs` define apenas o protocolo (`EVENTO_DESAFIO` com `ilha`, `evento`, `tick`), `crates/sim/src/lib.rs` tem `Mundo`/`ValidacaoComandos`/`hash_estado` sem emissão de eventos, e `crates/cliente/src/main.rs` mantém um espelho local (`Simulacao(Mundo)`) sem persistência. O release do ciclo 023 lista "duas partidas consecutivas, instrumentação/métricas" como **não incluído** (`.specify/releases/cliente-greybox/release.md`). Não há como demonstrar a partida modificada, calcular mediana/p95 ou afirmar qualquer meta da §17 sem primeiro criar a fonte de dados no Núcleo.

## Por que é válido

- **Impacto 5** — estrutural: os itens 2 e 3 são a própria prova da tese. Sem evento e sem registro, o fechamento de GDD:1567 é impossível de demonstrar, as seis métricas de GDD:1609-1616 permanecem apenas intenção escrita e a recalibração de GDD:1622 não tem amostra. Pior: a tentativa de preencher o vazio com saída de console do cliente produziria evidência inválida por norma (GDD:1607,1620) e intrinsecamente suspeita na partida modificada — o risco não é só ausência, é registrar a fonte errada. Nenhum dos itens 4–6 depende deste para existir, mas os itens 2 e 3 dependem inteiramente dele.
- **Confiança 4** — alta, não máxima: a lacuna foi confirmada por inspeção direta dos três crates nesta etapa, a norma é literal e datada no GDD (`:1567,1607,1619-1622`) e na interjeição da SE (STATE:3-8), e o release 023 declara o que ficou de fora. Não é 5 porque não há execução observada de partidas, nem definição operacional de "cliente modificado", nem decisão sobre quais métricas são calculáveis em single-player sem rede — incógnitas que pertencem à especificação, não a esta avaliação.
- **Custo de não agir** — a tese segue não falsificável; a SE recebe dois itens do entregável sem evidência; as metas **[HIPÓTESE]** nunca saem do papel; e a reconciliação entre a norma cliente-servidor do GDD e o escopo single-player fica adiada para depois de mais código, aumentando o churn que motivou a reorientação (STATE:7).

## Metas

1. **Eventos no Núcleo** — capturas, tentativas de sincronia e divergência cliente-servidor passam a ser emitidas no Núcleo com as âncoras já existentes (`match_id`, `epoch`, `tick`, `PASSO_SEGUNDOS`), sem criar novo tipo de pacote de rede e sem depender do cliente como fonte única.
2. **Registro por partida** — existe um registro persistente e local, consultável e determinístico, que associa cada evento à partida e permite comparar a execução normal com a do cliente modificado; nenhuma métrica entra no painel sem definição operacional e fonte declaradas (GDD:1619).
3. **Duas partidas consecutivas** — a partida normal e a com cliente modificado são executadas em sequência, com evidência carimbada que distingue as duas execuções e prova que os critérios de aceitação passaram em ambas (GDD:1567).
4. **Corredor de 20 partidas** — um corredor local executa 20 partidas instrumentadas e produz as métricas da §17 com mediana/p95 e taxas calculadas a partir do registro; as metas são revisadas uma única vez e continuam marcadas **[HIPÓTESE]** (GDD:1607,1622).
5. **Limites do escopo preservados** — execução local, single-player, sem rede externa e com a janela já minimizada do cliente (`crates/cliente/src/main.rs:53`); métricas que dependem de servidores comunitários ou forks publicados são explicitamente marcadas como não observáveis aqui, não preenchidas com estimativa.

A verificação pertence à portaria própria: nenhuma partida, build, teste, `verify.sh` ou gate é executado nesta etapa e toda saída permanece **[NV]** até registro datado em R4.

## Limites

Fora de escopo: instrumentação somente no cliente; telemetria remota, servidores comunitários, forks publicados, lobby, matchmaking (pareamento), 5v5, NAT/travessia de rede, UDP externo ou qualquer serviço de terceiros (STATE:6); itens 4 (exportação/`COPYRIGHT.txt`/commits assinados), 5 e 6 da interjeição; alteração de protocolo de rede, de `PROTOCOL_VERSION`, dos códigos de recusa ou do contrato de módulo; alteração de gate, baseline, `Makefile`, `evidence.csv`, lint ou portaria R4/R7; troca de engine, reabertura da ADR-001 ou remoção do histórico GDScript. Nenhuma alegação de partida executada, métrica medida, meta calibrada, janela inspecionada ou resultado de gate é feita aqui.
