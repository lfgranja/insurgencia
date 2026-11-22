# Concept: Demo no itch.io

- **Slug**: sb-16-demo-itch-io
- **Created**: 2026-11-21
- **Recommended option**: Option A — Demo headless versionada com espelho itch.io

## Options

### Option A — Demo headless versionada com espelho itch.io

- **Sketch**: criar `crates/sim/examples/demo_deterministica.rs` usando apenas `insurgencia_sim`: `Mundo::new()` → `adicionar_jogador(1)` → `agendar_comando(…)` → laço de `avancar_tick()` com `PASSO_SEGUNDOS`, imprimindo `hash_estado()` a cada tick com seed fixa. Nenhuma janela, nenhum `DefaultPlugins`. O exemplo entra no `verify.sh` por `--all-targets`. Publicar uma tag semver com o binário/fonte, checksum e notas curtas em CHANGELOG/GitHub Releases; espelhar os mesmos arquivos no itch.io (comissão 0%) como canal de descoberta, nunca como fonte de verdade. O comando exato e a seed acompanham o download.
- **Appetite**: small (dias) — um exemplo Rust, uma tag, duas páginas de texto; sem serviço novo no caminho crítico.
- **Trade-offs**: entrega proveniência verificável e reprodução independente; o espelho itch.io depende de hosting externo (opcional, removível) e o efeito de descoberta com 0% não foi medido `[NV]`. Exige congelar seed e formato de saída para não quebrar a comparação entre versões.
- **Rabbit holes**: transformar o espelho em canal exclusivo; perseguir "visual bonito" com Bevy e reintroduzir janela/SDK; publicar sem checksum ou sem saída executada do gate; inflar a demo com mecânicas instáveis do GDD.

### Option B — Somente GitHub Releases, sem itch.io

- **Sketch**: mesma demo determinística e mesma tag, sem espelho de descoberta.
- **Appetite**: small, menor que A.
- **Trade-offs**: zero dependência externa e rastreabilidade máxima; perde o canal que precedeu a Steam no caso estudado e o devlog nativo. Não satisfaz totalmente o nome do slug, embora satisfaça a soberania.
- **Rabbit holes**: confundir "publicar release" com "publicar demo jogável" sem instruções de execução.

### Option C — Demo com cliente Bevy e publicação em loja

- **Sketch**: empacotar o cliente com `DefaultPlugins`, publicar em Steam/itch.io com SDK e almejar visibilidade algorítmica.
- **Appetite**: large — exige App ID, US$100, build review e janela de loja.
- **Trade-offs**: alcance maior em teoria, mas quebra a constituição IV (dependência proprietária no caminho crítico), depende de pré-requisitos ausentes e repete a tese de visibilidade que a documentação da Valve contradiz.
- **Rabbit holes**: achar que MIT compatível com Steamworks equivale a ter infraestrutura de publicação; gastar a janela única de Next Fest antes de ter release estável (ver sb-17).

## Recommendation

Option A. É a única que transforma o ativo já existente — a sim headless determinística — em artefato público verificável sem introduzir serviço externo no caminho crítico. Casa com o veredito ADOPT: o que se adota é o padrão "demo versionada + gate verde + espelho opcional", e não o canal itch.io como dependência. O ponto de atenção é o `[NV]` sobre descoberta com comissão 0%: o espelho entra como experimento de canal, medido por downloads, não como promessa de alcance.

## Out of Scope (for the recommended option)

- Steam page, App ID, Steamworks SDK ou build review.
- Janela gráfica (inclusive minimizada) no exemplo da demo.
- Alterações na simulação, nos gates ou na constituição.
- Metas de wishlists, números do caso estudado ou métricas de terceiros como sucesso.
- Publicação sem tag, checksum, notas e saída executada de `verify.sh`.

## Assumptions to Validate

- `Mundo::new()` + `agendar_comando()` com seed fixa produz saída estável entre máquinas — validar com duas execuções antes da tag.
- `verify.sh --all-targets` continuará cobrindo `crates/sim/examples/` sem novo job (verificado em fit-local; reconfirmar no ato da publicação).
- [NEEDS CLARIFICATION: seed e formato de `hash_estado()` por tick congelados como contrato público da demo.]
- [NEEDS CLARIFICATION: operador da conta itch.io e política de espelhamento (GitHub como fonte, itch.io como cópia).]
