# Concept: Demo Steam + Next Fest

- **Slug**: sb-17-demo-steam-next-fest
- **Created**: 2026-11-21
- **Recommended option**: Option A — Preparar sem inscrever e reavaliar por janela

## Options

### Option A — Preparar sem inscrever e reavaliar por janela

- **Sketch**: manter o trabalho local em sb-16 (demo headless determinística, tag, checksum, changelog, gate verde). Registrar os pré-requisitos do Next Fest — store page pública, demo jogável, build review de 3–5 semanas, jogo não lançado antes do fim do evento — e uma data de reavaliação: até 10/01/2027 para Feb/2027 (22/02–01/03) ou antes de jun/2027. Nenhuma conta, pagamento ou inscrição agora. O "impulsionamento por algoritmo" é substituído por critérios observáveis (execuções da demo, downloads da release, cobertura com comando reproduzível anexado).
- **Appetite**: none neste ciclo (dia zero); preparação de documentação e do exemplo de sb-16 já conta como crédito.
- **Trade-offs**: preserva a janela única e evita US$100 + SDK no caminho crítico; adia alcance potencial de uma plataforma que hoje não existe para o projeto. Como o efeito do festival no caso é `[NV]`, o custo de esperar é pequeno comparado ao de gastar a chance cedo.
- **Rabbit holes**: tratar "preparar" como autorização para criar Steam page; marcar a reavaliação como aprovação implícita; usar os 100k do caso como previsão.

### Option B — Inscrever-se na edição Feb/2027 imediatamente

- **Sketch**: abrir conta de publisher, pagar US$100 por app, criar store page e submeter build para a janela de fevereiro de 2027.
- **Appetite**: large — semanas de build review, assets de loja, traduções e política de disclosure.
- **Trade-offs**: entra na maior vitrine disponível, mas consome a participação única sem demo versionada validada, introduz SDK proprietário e repete a tese de visibilidade contrariada pela Valve; 7.240 demos em out/2026 mostram diluição extrema.
- **Rabbit holes**: confundir compatibilidade MIT com Steamworks pronta para uso; achar que "registro até 10/01" significa "registre já sem build".

### Option C — Abandonar Steam como canal permanentemente

- **Sketch**: declarar que a Insurgência nunca usará lojas proprietárias e concentrar distribuição em GitHub Releases/itch.io.
- **Appetite**: nenhum.
- **Trade-offs**: máxima coerência com IV, mas decide sobre um canal externo que nem existe hoje e remove opção futura sem evidência nova; a constituição veta dependência proprietary, não a existência de canais adicionais.
- **Rabbit holes**: transformar preferência de arquitetura em dogmática de distribuição.

## Recommendation

Option A, compatível com o veredito DEFER. A decisão não nega o canal; ela condiciona a entrada a pré-requisitos que o próprio caso mostra terem vindo **antes** do festival (demo e audiência própria). O marco temporal é concreto (10/01/2027 para Feb/2027; jun/2027 como alternativa) e o gatilho de reavaliação é sb-16 publicado com gate verde — não uma meta de wishlists importada.

## Out of Scope (for the recommended option)

- Conta Steam, US$100, App ID, store page, build review ou assets de loja.
- Inscrição em qualquer edição do Next Fest neste ciclo.
- Uso de 100k wishlists/3 dias do caso como previsão ou meta.
- Alterações em `verify.sh`, CI, simulação ou constituição.
- Decisão definitiva de nunca usar Steam (pertence a uma futura ADR, se surgir).

## Assumptions to Validate

- A janela Feb/2027 (registro até 10/01/2027, build de Press Preview até 18/01) permanecerá conforme a doc viva da Valve — reconfirmar na reavaliação.
- sb-16 entregará demo versionada antes dessa data; se não entregar, o diferimento se estende automaticamente para jun/2027.
- [NEEDS CLARIFICATION: edição alvo e responsável pela conta/pagamento Steam, se a decisão for revertida.]
- [NEEDS CLARIFICATION: métrica local que substituirá "visibilidade algorítmica" como critério de go.]

## Nota de fronteira

Steam, se algum dia adotado, entra como **canal adicional**, nunca como sistema de registro de progresso, partida ou identidade. O modo headless determinístico (`--minimizado` / exemplo em `crates/sim/examples/`) permanece o equivalente funcional da demo e a fonte de verdade auditável.
