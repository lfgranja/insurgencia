# Checklist de requisitos — 014-ilha-inferior

**Data:** 2026-11-20T09:38Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação nem execução.

- [x] Spec identifica FR-001..FR-007 e SC-001..SC-005 com critérios binários (5 Materiais a cada 3 s; esgotamento após 3 ciclos e recarga em 20 s; crédito inventado recusado; `PASS`/`OK` + exit 0; 5 gates PASS e `make check GODOT=$G` exit 0)
- [x] Cada FR rastreia para um caminho literal (`nucleo/desafios/ilha_inferior.gd`, `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `modulos/exemplo/mapa.json`, `tests/test_ilha_inferior.gd`) ou comando `--headless` verificável
- [x] Constantes e regras conferem literalmente com GDD §16.3 linha 1504 (3 s por ciclo, 5 Materiais, estoque de 3 ciclos, recarga em 20 s), §16.4 linhas 1529–1531 e hipótese H3 em 1677; pendência em 1592
- [x] R1 `go` materializado em `.specify/assessments/ilha-inferior/decision.md` com data, branch/HEAD (`feature/014-ilha-inferior`, `48ee860`), binário fixado e restrições
- [x] Impacto 5 e evidência 5 justificados em `.specify/assessments/ilha-inferior/problem.md`, com a pendência do GDD 1592 e a opção escolhida/rejeitadas em `concept.md`
- [x] Tempo do servidor (`float(tick) * PASSO_SEGUNDOS`) com tempo injetado no teste; nenhum relógio de cliente, sono ou aleatoriedade
- [x] Crédito, saldo e evento `ilha_inferior_coleta` só após validação no ponto único de aplicação; cliente nunca envia `materiais`/`quantidade`/`credito`, recusados com `CREDITO_FORJADO`
- [x] Preloads existentes de `IlhaSincronia`/`IlhaTopo` preservados; nenhuma instância de `Mundo.new()` com `:=` (`: Mundo` explícito) e nenhum `sha256_string` (`HashingContext` mantido)
- [x] Escopo explícito fora do ciclo declarado (5v5, Artesão/Guerreiro, vitória da Ilha Central, UDP real, `Makefile`, `evidence.csv`, deploy/remote, baselines, R4/R7)
- [x] Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos), de funções, de vitória, de UDP real nem de execução observada; itens não verificados marcados **[NV]**
- [x] Plano fixa `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64` e os comandos `--headless`/`make check GODOT=$G`/5 gates; tasks T001..T010 com marcos e convergência READ-ONLY
- [x] Nenhum artefato R4/R7, código, gate, teste ou comando executado nesta etapa de escrita; saídas só com data/hora na portaria própria
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado; fonte da verdade é o gate determinístico do alvo, e a convergência corrige a fonte, nunca o artefato derivado
