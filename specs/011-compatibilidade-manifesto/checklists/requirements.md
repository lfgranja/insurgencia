# Checklist de requisitos — 011-compatibilidade-manifesto

**Data:** 2026-11-20T03:15Z

Revisão de qualidade de requisitos — `[x]` **não** significa implementação nem execução.

- [x] Spec identifica FR-001..FR-007 e SC-001..SC-004 com critérios binários (7 `PASS` + `OK` + exit 0; `COMPAT_OK module_id=insurgencia.exemplo` + exit 0; 3 execuções idênticas; `COMPAT_FALHA HASH_MODULO_DIVERGENTE` + exit 1 com arquivo restaurado; 5 gates)
- [x] Cada FR rastreia para um caminho literal (`manifesto.json`, `nucleo/compatibilidade/`, `ferramentas/`, `Makefile`, `tests/`) ou comando `--headless`/`make check-compat` verificável
- [x] R1 `go` materializado em `.specify/assessments/compatibilidade-manifesto/decision.md` com data, gate 5/5 e restrições; branch `feature/011-compatibilidade-manifesto`, HEAD `c2936db`
- [x] Opção escolhida = Task 5 literal; PR único com Tasks 4–6, gate remoto com lobby/worker/release e manifesto sem hash registrados como rejeitados
- [x] Impacto 5 e evidência 5 justificados em `.specify/assessments/compatibilidade-manifesto/problem.md`, com CE-4 sem implementação
- [x] Os 6 campos obrigatórios são literais e os 4 códigos de `Protocolo` (`VERSAO_PROTOCOLO_DIVERGENTE`, `HASH_MODULO_DIVERGENTE`, `MODULO_AUSENTE`, `MANIFEST_INVALID`) aparecem exatamente
- [x] Versão checada antes do hash (cor-06), comparação com `int(...)` pelo `float` do JSON e hash de 32 bytes (64 hex) com recusa de formato malformado
- [x] Exibição no cliente (`CLIENTE_VERSAO_PROTOCOLO`, `CLIENTE_HASH_MODULO`, `CLIENTE_ERRO`) e restauração de `modulos/exemplo/regras.cfg` após o teste de divergência explicitadas
- [x] Escopo explícito fora do ciclo declarado (doc-26 DEFER, `evidence.csv`, 5v5, Ilha, vitória Central, deploy/remote, CI, factory, baselines, R4/R7)
- [x] Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos) nem de execução observada; itens não verificados marcados **[NV]**
- [x] Convenção 80/20 respeitada: nenhum percentual de conclusão afirmado
- [x] Fonte da verdade = gate determinístico do alvo; saídas pertencem a R4 e versão/changelog/rollback a R7, sem artefatos escritos aqui; Convergence permanece READ-ONLY
