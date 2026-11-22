# Problem — Animações IA (sb-03)

**Data**: 2026-11-21

O intake afirma que as animações do Slotbound foram geradas por IA [NV — post original não verificado]. No Insurgência, o problema concreto é que o avatar do cliente é hoje **estático**: não existe sistema de animação, e a simulação já expõe estado determinístico (`avancar_tick`, `hash_estado`, `PASSO_SEGUNDOS = 1/30`) que poderia dirigir frames. Adotar animação gerada por IA como propõe o vídeo significaria introduzir modelo remoto em runtime (PixelLab/MCP), cuja saída não é reproduzível por seed e cuja titularidade é incerta — convertendo um sistema determinístico e auditável em dependência externa e opaca (constituições IV e VI).

## Sucesso observável
- Cada animação é derivada do estado da simulação ou do tempo (`hash_estado`/tick) por tabelas de **frames versionados**; mesma semente e mesmo tick produzem o mesmo frame.
- Nenhuma chamada a modelo/serviço de IA em runtime; IA, se usada, fica apenas na autoria offline de sprites, com prompt/seed/ToS registrados no inventário antes de versionar.
- Assets de animação passam pelo mesmo gate do código: `verify.sh` (check → clippy → nextest → fmt) + 5 gates, sem `.unwrap()`/`.expect()`, com `#![forbid(unsafe_code)]`/`#![deny(clippy::pedantic)]`.
- Regra `--minimizado` preservada (qualquer inspeção visual sem janela normal) e ADR-004 (Bevy+Lightyear) intacto.

## Custo de não agir
Sem frames derivados do estado, qualquer animação dependeria de arte externa irreproduzível: forks divergiriam, replay não seria auditável e a simulação determinística perderia sua principal vantagem. Tabelas versionadas são baratas; a alternativa remota é frágil e juridicamente incerta.

**[NV]** Quais animações são alegadas, se o post original foi verificado e a legalidade da saída IA não foram confirmados aqui; nenhum build/gate executado nesta escrita (constituição VI).
