# Checklist de requisitos — 013-ilha-topo

**Data:** 2026-11-20T09:28Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação nem execução.

- [x] Spec identifica FR-001..FR-008 e SC-001..SC-005 com critérios binários (captura em até 45 s com evento; reinício em 2 s sem captura; três ativações fora da janela falhas no servidor; `PASS`/`OK` + exit 0; 5 gates PASS e `make check GODOT=$G` exit 0)
- [x] Cada FR rastreia para um caminho literal (`nucleo/desafios/ilha_topo.gd`, `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `modulos/exemplo/mapa.json`, `tests/test_ilha_topo.gd`) ou comando `--headless` verificável
- [x] Constantes e regras conferem literalmente com GDD §16.3 linha 1502 (`ORDEM` fixa de 3 alavancas, 8 s por alavanca, erro reinicia em 2 s, limite de 45 s) e §16.4 linhas 1518–1521
- [x] R1 `go` materializado em `.specify/assessments/ilha-topo/decision.md` com data, branch/HEAD, binário fixado, gates 5/5 e restrições
- [x] Impacto 5 e evidência 5 justificados em `.specify/assessments/ilha-topo/problem.md`, com a pendência do GDD 1590 e a opção escolhida/rejeitadas em `concept.md`
- [x] Tempo do servidor (`tick * PASSO_SEGUNDOS`) com tempo injetado no teste; nenhum relógio de cliente, sono ou aleatoriedade
- [x] Captura, `estado_persistido="ilha_topo_concluida"` e evento só após validação no ponto único de aplicação; cliente não envia estado pronto
- [x] Escopo explícito fora do ciclo declarado (5v5, vitória da Ilha Central, Ilha Inferior, Artesão/Guerreiro, `Makefile`, `evidence.csv`, deploy/remote, baselines, R4/R7)
- [x] Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos) nem de execução observada; itens não verificados marcados **[NV]**
- [x] Plano fixa `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64` e os comandos `--headless`/`make check GODOT=$G`/5 gates; tasks T001..T010 com marcos e convergência READ-ONLY
- [x] Nenhum artefato R4/R7, código, gate, teste ou comando executado nesta etapa de escrita; saídas só com data/hora na portaria própria
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado; fonte da verdade é o gate determinístico do alvo, e a convergência corrige a fonte, nunca o artefato derivado
