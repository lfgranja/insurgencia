# Shape — Servidor headless determinístico

- **Slug**: `servidor-headless`
- **Created**: 2026-11-23T04:30Z
- **Recommended option**: Opção A — Servidor headless determinístico

## Options

### Option A — Servidor headless determinístico (recomendada)

- **Sketch**: um novo crate `crates/servidor` produz o binário `insurgencia-servidor`, que depende só de `insurgencia-sim`. Ele lê `--ticks N`, lê a stdin linha a linha no formato `peer_id acao seq request_id [x y]`, valida a ação e a presença/ausência de coordenadas, agenda os comandos em ordem antes do tick 0 e imprime `<tick>\t<hash sha256>` para cada tick de `0` a `N-1`, usando `Mundo::hash_estado()`. Entrada malformada ou ação desconhecida gera erro determinístico e `ExitCode` 2, sem pânico. Testes unitários cobrem o parser; teste de integração compara duas execuções, conta as linhas e rejeita entrada inválida.
- **Appetite**: `small` (dias) — um crate com um punhado de módulos (argumentos, parser, execução, erro) e três suítes de teste; sem contrato, protocolo, dependência externa ou infraestrutura de rede novos.
- **Trade-offs**: ganha o recibo determinístico mais barato e auditável — saída textual comparável por `diff`, sem janela e sem plataforma — e cria a pré-condição para qualquer netcode futuro. Sacrifica apenas a alternativa de "primeiro fazer rede" e trabalho de apresentação. Risco principal é a disciplina de interface (parser sem pânico, erro determinístico, ordem de agendamento e fonte única do hash), coberta por contrato explícito e por testes.
- **Rabbit holes**: transformar o binário em servidor de rede com Lightyear ou sockets; adicionar `tokio`/`async` "para ler a stdin"; reimplementar `hash_estado` no crate para "formatar" a saída; criar um framework genérico de comandos; tocar em `crates/contratos`, `PROTOCOL_VERSION` ou `Cargo.toml` além da inclusão do membro; reabrir ações ou valores canônicos do `sim`.

### Option B — Só fechar a dívida documental

- **Sketch**: entregar apenas a correção/consolidação documental pendente (por exemplo, alinhar notas e documentação do recibo headless), sem criar o binário.
- **Appetite**: `xs` (horas) — edição de documentos, risco mínimo (fator 5).
- **Trade-offs**: ganha exatidão documental e é necessária, mas tem impacto 2: não produz evidência executável, não permite `diff` entre execuções e não destrava a verificação do Núcleo. É uma condição includa na Opção A, não uma entrega que fecha o ciclo. É a alternativa mais próxima em pontuação (250/625 contra 256/625), o que torna o argumento de impacto — não o número isolado — o critério de escolha.
- **Rabbit holes**: transformar a correção em auditoria documental ampla; reescrever ADRs ou o GDD para descrever um servidor que ainda não existe.

### Option C — Combate / Fôlego

- **Sketch**: investir a fatia em combate (Ataque Forte, tiros, dano, Fôlego), deixando o servidor headless para depois.
- **Appetite**: `medium` (semanas).
- **Trade-offs**: ganha feedback de ação imediato; sacrifica a tese do MVP e o recibo verificável. Confiança 2 e facilidade 2, com risco moderado (fator 3). GDD §16.3:1509 é explícito: provar jogabilidade baseada em desafios de ilha, "em vez de combate puro".
- **Rabbit holes**: balanceamento de armas, recarga e IA — nenhum com valor canônico para este ciclo, e nenhum deles produz determinismo observável sem a fronteira de saída.

### Option D — Rede Lightyear (netcode)

- **Sketch**: implementar primeiro transporte, predição e reconciliação cliente-servidor, adiando o harness determinístico.
- **Appetite**: `large` (meses) — muda a natureza do trabalho de uma fatia de ferramenta de Núcleo para infraestrutura de rede.
- **Trade-offs**: impacto nominal 5, mas confiança 2, facilidade 1 e risco máximo (fator 1): sincronizaria um estado sem meio auditável de provar determinismo. GDD §16.3:1490 pede Núcleo isolado, e o contrato deste ciclo veta sockets, `async` e Lightyear.
- **Rabbit holes**: lobby/matchmaking (fora do MVP), NAT, predição de comandos e resolução de concorrência — todos dependentes de uma verdade determinística que ainda não foi materializada.

## Recommendation

Escolher **Opção A**. É a única que satisfaz simultaneamente as metas de `define.md`: impacto 4 (recibo executável e comparável do determinismo), confiança 4 (contrato fecha crate, formato, saída, erro e testes; integração ainda não executada), facilidade 4 (crate único, uma dependência, sem infraestrutura) e risco 4 (sem rede, sem `async`, sem dependência nova). B é necessária mas de impacto 2; C contradiz a tese do MVP; D inverte a dependência determinismo → sincronização e é explicitamente vetada pelo contrato. Scorecard único do ciclo: **256/625** contra 250/625 (B), 48/625 (C) e 10/625 (D). A margem de 6 pontos sobre B é deliberada: o contrato prefere capacidade verificável nova a correção documental isolada.

## Out of Scope (for the recommended option)

- Netcode, Lightyear, sockets, `async`, transporte, lobby, matchmaking, predição e reconciliação.
- Novas dependências externas, mudanças em `crates/contratos`, `PROTOCOL_VERSION` ou reimplementação do hash fora de `Mundo::hash_estado()`.
- Janela, Bevy, renderização, plataforma de distribuição e captura de vídeo.
- Reabertura de ações/valores canônicos do `sim`; R6 (N/A por constituição v1.2.2), issues simuladas e `gh` improvisado; gates, build, testes e formatadores neste assessment.

## Assumptions to Validate

- `members = ["crates/*"]` em `Cargo.toml:5` já inclui `crates/servidor`; confirmar no `/speckit.specify` se o contrato exige também uma entrada explícita e se `Cargo.lock` deve ser registrado como artefato do ciclo.
- `Comando.destino: Option<(f64, f64)>` comporta o par `x y` da stdin sem extensão de contrato; a proibição de coordenadas para as outras cinco ações conhecidas vive no parser do servidor.
- `agendar_comando` preserva a ordem da stdin (`push_back`) e `fase_prepare` consome os pendentes no primeiro tick; a especificação deve fixar o instante de impressão do hash no tick 0 para garantir exatamente N linhas.
- O binário precisa garantir existência do `peer_id` antes de agendar, sob pena de `CodigoRecusa::PeerNaoAutorizado` em `validar_comando` — definir o uso de `adicionar_jogador` na especificação.
- `hash_estado()` é a única fonte da coluna `<hash sha256>`; qualquer formatação adicional deve ser só o separador `\t` exigido pelo contrato.
- Erros de parser devem ser convertidos em `Result`/`ExitCode` com código 2 e mensagem determinística, sem `unwrap`/`expect`.

## Rastreabilidade

Opção A ↔ FR-001..FR-006 e SC-001..SC-006; alternativas B/C/D são as mesmas rejeitadas em `research.md` e não recebem rastro de implementação. T001..T012 pertencem ao handoff em `decision.md`, com definição canônica em `specs/042-servidor-headless/`. R6 = N/A (constituição v1.2.2).
