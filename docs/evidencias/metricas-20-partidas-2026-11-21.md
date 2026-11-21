# Relatório — 20 partidas instrumentadas (GDD §17)

**Data:** 2026-11-21T01:12Z · **Fonte:** `crates/metricas` (`insurgencia-metricas`), `correr_partidas(20, false)` e `correr_partidas(2, true)` — funções puras e determinísticas, sem relógio, rede, arquivo ou aleatoriedade. **Comando:** `./verify.sh` → `ALL STAGES OK`, 28 testes (8 do corredor de métricas).

## Amostra

| Item | Valor |
|:--|:--|
| Partidas na amostra | 20 consecutivas |
| Instrumentação | Núcleo (`EventoNucleo`: captura, tentativa de sincronia, divergência cliente-servidor, início, vitória) |
| Cliente | canônico nas 19 primeiras; modificado na última (GDD:1567) |
| Segunda execução | 2 partidas consecutivas, a segunda com cliente modificado — 3 divergências registradas no Núcleo |

## Métricas (GDD:1609–1616)

| Métrica | Valor observado | Meta [HIPÓTESE] | Resultado |
|:--|:--|:--|:--|
| M1 Tempo até primeira captura | mediana 78 s; p95 118 s | ≤ 90 s mediana; p95 ≤ 150 s | dentro da meta |
| M2 Taxa de conclusão da sincronia | 71% | 60%–90% | dentro da meta |
| M3 RTT e percepção | mediano 69 ms; p95 95 ms; ≥ 90% "sem atraso" | ≤ 80 ms local; p95 ≤ 250 ms; ≥ 90% | dentro da meta |
| M4 Servidores ativos | 5 (na amostra simulada) | ≥ 5 em 3 meses / ≥ 20 em 12 meses | limiar inferior atingido na amostra; **não** é evidência de servidores reais |
| M5 Diversidade de módulos/forks | 8 módulos; 2 forks | ≥ 3 em 3 meses; ≥ 10 em 12 meses; ≥ 2 forks | módulos/forks simulados, não publicados — **[NV]** como prova comunitária |
| Taxa de partidas concluídas | 75% | ≥ 50% | dentro da meta |

## Ressalvas obrigatórias

- Os números vêm de **simulação determinística no Núcleo**, não de partidas jogadas por humanos nem de servidores reais. GDD:1607 exige medição no Núcleo — atendido; GDD:1622 pede recalibração única após 20 partidas — as metas permanecem **[HIPÓTESE]**, não recalibradas.
- M4 e M5 são contagens **da amostra simulada**; itens 5 e 6 do entregável da SE (servidores com Módulos diferentes e forks publicados) estão **DEFER** e dependem de outras pessoas.
- "Duas partidas consecutivas, uma com cliente modificado" (GDD:1567) foi exercitada no corredor determinístico com 3 divergências registradas; não houve partida em janela gráfica com cliente humano modificado **[NV]**.
- Nenhuma afirmação remota, CI ou deploy é feita neste relatório.
