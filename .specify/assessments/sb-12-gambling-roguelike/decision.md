# Decision — Tendência gambling-roguelike (sb-12)

- **Slug**: sb-12-gambling-roguelike
- **Decided**: 2026-11-21
- **Veredito**: AVOID

| Critério | Avaliação | Justificativa |
|---|---|---|
| Validade do problema | weak | "Tendência" é percepção de mercado, não dor do projeto; GDD §2.6 já trata o tema como contraste. |
| Força da evidência | adequate | Vendas reais e cobertura ZH, mas autorrelato, viés de sobrevivência e vídeo de origem não assistido [NV]. |
| Valor vs. inação | strong | Evitar a corrida de moda poupa ciclos de design e preserva identidade; inação aqui é a ação correta. |
| Viabilidade/Appetite | strong | Nenhum código envolvido: basta não adotar o rótulo nem a estética. |
| Alinhamento estratégico | strong | AVOID protege materialismo/ação coletiva (V/IV) e a precedência I→III→II→IV do GDD. |
| Postura de risco | strong | O risco é entrar na moda; a aversão explícita elimina o risco de "carry"/cassino. |

## Racional
O gênero existe e vende (Balatro >5M, CloverPit 750k), mas é um mercado de *outliers*, com cópia rápida e retenção dependente de converter sorte em habilidade — não do giro em si. Insurgência já tem `alea` como anti-exemplo no GDD §2.6 e uma tese de coordenação material; perseguir o rótulo "gambling-roguelike" importaria, junto, a negação da ação coletiva que a Riot documenta em TFT e o risco de campeão solitário via fusão (sb-11). AVOID não nega os dados: mantém-nos como contexto de mercado e barra a conversão em requisito de produto. A alternativa saudável já está decidida em sb-10/sb-14: transformador determinístico e RNG semeado, subordinados à clareza causal.

## Condição/responsável
SE mantém sb-12 como dado de mercado em `docs/research/` e não o promove a spec. Reabertura só com novo ADR que demonstre benefício mensurável à coordenação material — não com citação de vendas de terceiros. Sem Handoff: não há feature a entregar.
