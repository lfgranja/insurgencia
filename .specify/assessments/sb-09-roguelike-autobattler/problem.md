# Problem — Pressão para importar o loop de auto-battler solo sem coordenação nem servidor

**Data**: 2026-11-21

## Problema

O intake descreve o gênero de Slotbound como "roguelike auto-battler com mecânicas de caça-níquel e fusão" (https://youtu.be/Z4ayMTXnkoQ, `[UNVERIFIED — fetch skipped]`). A página oficial da Steam confirma o gênero e o modo **Singleplayer**, com "summon units and grow your army by absorbing others". Para **este** projeto, o problema não é "o gênero vende" (os números são outliers e autorrelato `[NV]`), mas que adotar o loop solo como núcleo **importaria a negação do Pilar IV (ação coletiva)** e o risco de "campeão solitário" (Constituição V / GDD §4.4, §5.1). Além disso, não existe no repo o pré-requisito que tornaria o modo compatível: não há servidor determinístico em produção, nem 5v5, e o cliente hoje nem chama `avancar_tick` (`Mundo` é espelho).

## Sucesso observável

- Nenhuma feature de auto-battler solo entra no Núcleo antes de existir servidor determinístico/headless e sessão 5v5 (pré-requisito explícito).
- Se o tema for retomado, a forma admitida é **auto-resolução determinística de ações coletivas já executadas** — um replay legível da coordenação —, com semente + `hash_estado()` auditáveis (`crates/sim/src/lib.rs`).
- A condição de vitória permanece a do GDD (converter/sustentar a Ilha Central por coordenação), nunca "soma de desempenhos independentes" (GDD §4.4).
- A regra da janela (`--minimizado`) e a ADR-004 (Bevy+Lightyear) seguem intactas.

## Custo de não agir

Adiar o tema tem custo **baixo e reversível**: nada é implementado agora, e o projeto preserva a coerência com a Constituição. O custo de *não adiar* — implementar o modo solo por pressão de mercado — é alto: reescrever a condição de vitória, contrariar IV/V, criar dependência de um backend de auto-battler (IV) e possivelmente travar os 34 testes/CI em uma direção descartada depois. O viés de sobrevivência (Balatro 5M, CloverPit 750k vs. Pip My Dice 8.744) mostra que "gênero quente" não é previsão para o Insurgência.

## Stakeholders e restrições

- **Afetados**: SE/mantenedor (autoridade sobre a direção de design); Núcleo/`insurgencia-sim` (determinismo existente); jogadores (servidos pela coordenação, não por carry).
- **Restrições**: constituição IV (soberania, sem serviço externo) e V (materialismo/ação coletiva, sem campeão solitário); GDD §2.6 (alea como contraste), §4.4 e §5.1; ADR-004 (Bevy+Lightyear fixo); regra da janela; CI verde autoriza merge, nunca auto-merge cego; saída de LLM informativa.
- **Fora de escopo**: implementar modo solo, auto-battler, servidor ou 5v5 nesta etapa; tocar crates/gates/testes/docs fora destes 4 artefatos.

## Perguntas abertas

- `[NV]` Quando existirá servidor determinístico/headless e sessão 5v5 no alvo (pré-requisito para retomar)?
- `[NV]` "Auto-resolução de coordenação" terá qual forma observável (replay de funções/materialização) sem virar "corrida de power-level"?
- `[NV]` A classificação de gênero é oficial da Steam (confirmada) — mas o "100K/3 dias" permanece autorrelato não verificado.
