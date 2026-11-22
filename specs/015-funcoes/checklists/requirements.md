# Checklist de requisitos — 015-funcoes

**Data:** 2026-11-20T09:41Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação nem execução.

- [x] Spec identifica FR-001..FR-007 e SC-001..SC-005 com critérios binários (20 Materiais/1,5 s/200 PV/12 s; 1,2 s/2×/6 s com recusa no servidor; `PASS`/`OK` + exit 0; 5 gates PASS e `make check GODOT=$G` exit 0)
- [x] Cada FR rastreia para um caminho literal (`nucleo/sim/funcoes.gd`, `nucleo/mundo.gd`, `nucleo/sim/validacao.gd`, `tests/test_funcoes.gd`) ou comando `--headless` verificável
- [x] Constantes e regras conferem literalmente com GDD §16.3 linhas 1505–1506 (20 Materiais, 1,5 s, 200 PV, 12 s; 1,2 s, 2× dano base, 6 s) e §16.4 linhas 1534–1535, Task 3 em 1563, pendência em 1593 e hipóteses H4/H5 em 1678–1679
- [x] R1 `go` materializado em `.specify/assessments/funcoes/decision.md` com data, branch/HEAD (`feature/015-funcoes`, `7c19440`), binário fixado e restrições
- [x] Impacto 5 e evidência 5 justificados em `.specify/assessments/funcoes/problem.md`, com a pendência do GDD 1593 e a opção escolhida/rejeitadas em `concept.md`
- [x] Tempo do servidor (`float(tick) * PASSO_SEGUNDOS`) com tempo injetado no teste; nenhum relógio de cliente, sono ou aleatoriedade
- [x] Custo, barreira, dano e recarga só após validação no ponto único de aplicação; cliente nunca envia `materiais`/`dano`/`pv`/`recarga`, recusados com `MATERIAIS_FORJADOS`/`DANO_FORJADO`, e `atacar` em recarga é recusado no servidor
- [x] `DANO_BASE=100` e `DANO_ATAQUE_FORTE=200` declarados como literais, com `TOLERANCIA=0.000001` coerente entre regra, spec e teste
- [x] Preloads existentes de `IlhaSincronia`/`IlhaTopo`/`PontoColeta` preservados; nenhuma instância de `Mundo.new()` com `:=` (`: Mundo` explícito) e nenhum `sha256_string` (`HashingContext` mantido)
- [x] Escopo explícito fora do ciclo declarado (5v5, vitória da Ilha Central, UDP real, Fôlego/alcance/dimensões de produção, `Makefile`, `evidence.csv`, deploy/remote, baselines, R4/R7)
- [x] Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos), de vitória, de partidas 5v5, de UDP real nem de execução observada; itens não verificados marcados **[NV]**
- [x] Plano fixa `G=/home/luis/.local/bin/Godot_v4.7.2-stable_linux.x86_64` e os comandos `--headless`/`make check GODOT=$G`/5 gates; tasks T001..T010 com marcos e convergência READ-ONLY
- [x] Nenhum artefato R4/R7, código, gate, teste ou comando executado nesta etapa de escrita; saídas só com data/hora na portaria própria
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado; fonte da verdade é o gate determinístico do alvo, e a convergência corrige a fonte, nunca o artefato derivado
