# Checklist de requisitos — 016-vitoria-central

**Data:** 2026-11-20T09:48Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação nem execução.

- [x] Spec identifica FR-001..FR-007 e SC-001..SC-005 com critérios binários (3 ilhas, 4,0 s, 3,99 s vs 4,0 s, progresso zero na interrupção, vitória declarada pelo servidor; `PASS`/`OK test_vitoria_central` + exit 0; 5 gates PASS e `make check GODOT=$G` exit 0)
- [x] Cada FR rastreia para um caminho literal (`nucleo/desafios/ilha_central.gd`, `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `tests/test_vitoria_central.gd`) ou comando `--headless` verificável
- [x] Constantes e regras conferem literalmente com GDD §16.3 linha 1507 (três ilhas de rota, 4 s de canalização conjunta, Artesão e Guerreiro vivos e presentes) e §16.4 linhas 1537–1541 (pré-requisito, decisão do servidor, não substituição, zeração do progresso), Fase 4 em 1563 e pendência em 1593
- [x] R1 `go` materializado em `.specify/assessments/vitoria-central/decision.md` com data, branch/HEAD (`feature/016-vitoria-central`, `ca9421f`), binário fixado e restrições
- [x] Impacto 5 e evidência 5 justificados em `.specify/assessments/vitoria-central/problem.md`, com a pendência do GDD 1593 e a opção escolhida/rejeitadas em `concept.md`
- [x] Tempo do servidor (`float(tick) * PASSO_SEGUNDOS`, 30 Hz) com tempo injetado no teste; nenhum relógio de cliente, sono ou aleatoriedade
- [x] Vitória, ilhas capturadas, função, vida e presença nunca são aceitas prontas do cliente; o Núcleo conta as ilhas, verifica a dupla e declara o resultado com `declarada_por="servidor"`
- [x] Apenas Artesão e Guerreiro vivos e presentes, identificados por `peer_id`, sustentam a canalização; Caudillo/Camponês não substituem quem morre ou sai
- [x] Interrupção zera `progresso` e exige novos 4,0 s contínuos; 2,5 s + interrupção + 3,0 s não vencem, enquanto 3,0 s + 4,0 s vencem em 7,0 s
- [x] Evento `vitoria_ilha_central` e `estado_persistido` alimentam a tela de resultado; nenhuma cena de UI, vitória efetiva ou execução de interface é afirmada nesta etapa
- [x] Preloads existentes de `IlhaSincronia`/`IlhaTopo`/`PontoColeta`/`Funcoes` preservados; nenhuma instância de `Mundo.new()` com `:=` (`: Mundo` explícito) e nenhum `sha256_string` (`HashingContext` mantido)
- [x] Escopo explícito fora do ciclo declarado (5v5, UDP real, regra de produção com seis ilhas/Plano Mestre/20 s, Fôlego/alcance/dimensões de produção, cena de interface, `Makefile`, `evidence.csv`, deploy/remote, baselines, R4/R7)
- [x] Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos), de vitória executada, de partidas 5v5, de UDP real ou de escala de produção; itens não verificados marcados **[NV]**
- [x] Plano fixa `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64` e os comandos `--headless`/`make check GODOT=$G`/5 gates; tasks T001..T010 com marcos e convergência READ-ONLY
- [x] Nenhum artefato R4/R7, código, gate, teste ou comando executado nesta etapa de escrita; saídas só com data/hora na portaria própria
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado; fonte da verdade é o gate determinístico do alvo, e a convergência corrige a fonte, nunca o artefato derivado
