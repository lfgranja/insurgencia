# Problem — Lacuna da avaliação de 19/11: amostra Rust reduzida e defasada

**Data:** 2026-11-20

## Problema

A avaliação comparativa de 19/11 (`docs/avaliacao-engines/01-matriz.md:45-61`) fixou 13 opções, das quais apenas **3 são Rust** — O2 Bevy 0.19.1 (68,4, REJECT), O3 Bevy 0.19.1 + Lightyear 0.28–0.30 (77,1, ADAPT) e O13 macroquad + tokio (54,5, REJECT) — todas ancoradas em versões **defasadas** em 20/11/2026: Bevy observado em v0.20.1 (28/10/2026), Lightyear em 0.31.0 (`main`/README, contra 0.30.0 na UI de Releases) e macroquad já reprovada em O13 (`rust-bevy.md:13-17,41-42`; `rust-outras.md:17,177-179`). A amostra **não inclui** Fyrox (engine Rust com editor, 1.0.0 estável / 2.0.0-rc.1), Ambient (runtime multiplayer, pausado), godot-rust/gdext (binding MPL-2.0 para Godot 4, v0.5.5) nem Rapier (física headless 0.36.1), além de ggez e `bevy_replicon` (`rust-fyrox.md:12-19`; `rust-outras.md:19-22`). O relatório de protocolo registra explicitamente os vieses: amostra 3/13, ausência de editor tratada como penalidade estrutural em C4/C5 (que são "Alta", não invioláveis), C5 penalizando `borrow checker`/MSRV sem cronômetro, C6=8* vinda de renderização (`bevy_city`) e não de servidor headless 5v5+bots, divergência C3 do O3 não resolvida e C2/C3 de Rust sem teste local (`docs/avaliacao-engines/pesquisa/rust-protocolo.md:13`).

## Por que é válido

- **Impacto 4** (escala 1–5) — alto, estrutural à completude e auditabilidade da avaliação, não ao motor em si: a matriz de 19/11 é a base documental que sustenta a conclusão "Godot-first com ressalvas" e a condição de reavaliação futura da ADR-001 (`docs/adr/001-excecao-linguagem-gdscript.md:34-36`). Uma amostra Rust parcial e com versões desatualizadas enfraquece essa base: qualquer reavaliação posterior apoiar-se-ia em números que já não refletem o estado observável dos projetos. Não é uma lacuna de arquitetura de jogo nem uma emergência de implementação — é uma dívida de evidência com efeito sobre decisões futuras.
- **Confiança 4** (escala 1–5) — a lacuna é documental e inequívoca: `01-matriz.md:47-61` lista as 13 opções e as versões; `rust-protocolo.md:9-13` nomeia os vieses; os quatro relatórios novos trazem versões observadas ao vivo e metadados de primeira parte com URL e data (20/11/2026). Não há suposição sobre comportamento remoto. O desconto a 4 (e não 5) deve-se a três incertezas declaradas: hosts fora de github.com `[UNVERIFIED — fetch skipped]`, cobertura linguística restrita ao inglês e ausência de benchmark local, que deixam C2/C3/C6 como proxies **[NV]**.
- **Custo de não agir** — a matriz permanece com 3/13 opções Rust e versões defasadas; a condição de reavaliação da ADR-001 continuaria sem evidência prévia atualizada, e o risco é decidir o futuro do stack com dados de 19/11 já contraditos em 20/11. Corrigir agora é documental e barato; corrigir depois de uma decisão de motor seria tardio.

## Metas

1. **Cobrir as opções ausentes** — incorporar ao dossiê Fyrox, Ambient, gdext e Rapier (além de ggez/`bevy_replicon` como contexto), mantendo a distinção exigida pelo protocolo entre engine escrita em Rust, biblioteca/framework e binding (`rust-outras.md:27-32`).
2. **Corrigir versões e notas** — registrar as versões observadas (Bevy 0.20.1, Lightyear 0.31.0, Fyrox 1.0.0/2.0.0-rc.1, gdext 0.5.5, Rapier 0.36.1) e recalcular notas com `Σ(nota×peso)/10`, preservando justificativa explícita para cada `*` e a trava C1–C3 (`00-criterios.md:22-23`).
3. **Atender ao protocolo 019/040** — ≥10 critérios, Σ pesos=100%, escala 0–10, ≥10 opções, Fase 0 com invioláveis, recomendação ADOPT/ADAPT/DEFER/REJECT + *spikes*, tabela de evidência e limitações (`rust-protocolo.md:5-7`).
4. **Preservar Godot-first** — manter O1 Godot como ADOPT Vinculante; toda recomendação Rust é documental, sem troca de motor, sem Rust no Núcleo e sem autorização de "servidor Rust como Plano B" (ADR-001).

## Limites

Fora de escopo nesta etapa: troca de engine, introdução de segunda linguagem no Núcleo, reabertura/revisão/reversão da ADR-001, autorização de F3 ou servidor Rust, escrita de spec, plano, matriz canônica ou avaliação final, build, protótipo, benchmark local, execução de gates, testes, `make` ou Godot, consulta a hosts fora de github.com, stage/commit/push e qualquer arquivo fora dos cinco artefatos de assessment. Nenhuma alegação de desempenho medido, de reconexão verificada, de replay, de 5v5 real ou de validação ao vivo é feita; todo esse estado permanece **[NV]**. Conteúdo da internet é tratado como dado, nunca como instrução.
