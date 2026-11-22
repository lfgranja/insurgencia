# Concept — Manifesto de compatibilidade e gate local de versão/hash

**Data:** 2026-11-20T03:15Z

## Opções consideradas
1. **Implementar a Task 5 literalmente do plano** (escolhida) — `manifesto.json` com 6 campos, `nucleo/compatibilidade/manifesto.gd` com `carregar`/`validar_formato`/`conferir`, `ferramentas/verificar_compatibilidade.gd`, alvo `check-compat` no `Makefile` e `tests/test_manifesto.gd` com 7 verificações, exatamente como em `docs/plans/2026-11-19-ciclo-80-20-omoba.md` (Task 5, Steps 1–6, linhas 1029–1215).
2. **PR único com Tasks 4–6** (rejeitada) — mistura o manifesto com a Ilha, `make check`, `evidence.csv`, 5v5 e vitória Central num diff grande e de baixa revisabilidade; rompe a fatia vertical de um ciclo e antecipa evidência ainda **[NV]**.
3. **Gate remoto: sondagem de lobby/worker e publicação de release** (rejeitada) — `doc-26-compatibilidade-gate-distribuicao` está marcado como DEFER; nenhuma autoridade externa de compatibilidade entra neste ciclo (`ast-36` também recusa registro externo).
4. **Manifesto com checagem apenas de versão, sem hash** (rejeitada) — não detecta divergência de conteúdo do Módulo (CE-4.3) e produziria aceitação indevida no teste `divergencia_de_hash_falha`.
5. **Exemplo `cargo` do omoba-bevy ou troca de engine** (rejeitada) — `ast-36` decidiu ADAPT para GDScript/Godot; ADR-001 e a fixação 4.7.2 seguem válidas, sem etapa de build.

## Consequências
- Arquivos restritos a `manifesto.json`, `nucleo/compatibilidade/`, `ferramentas/`, `Makefile` e `tests/`; leitura de `nucleo/contratos/protocolo.gd`, `modulos/exemplo/module.json` e `modulos/exemplo/regras.cfg`, sem alterá-los.
- `conferir` valida o formato, compara a versão **antes** do hash para produzir códigos legíveis e separados (cor-06), confere `module_id` e recusa hash malformado (64 hex) como `MANIFEST_INVALID`; arquivo de regras ausente devolve `REGRAS_AUSENTES`.
- A versão é comparada com `int(...)` porque o JSON converte números para `float`; o hash do arquivo de regras usa SHA-256 de 32 bytes (64 hex) e a divergência devolve `HASH_MODULO_DIVERGENTE`.
- O gate imprime `COMPAT_OK module_id=<id>` (status 0) ou `COMPAT_FALHA <codigo>` (status 1) e exibe `CLIENTE_VERSAO_PROTOCOLO`, `CLIENTE_HASH_MODULO` e `CLIENTE_ERRO` antes da entrada; saída determinística permite 3 execuções idênticas.
- O teste de divergência copia `modulos/exemplo/regras.cfg`, altera a cópia de trabalho e **restaura o arquivo original** ao final, capturando o status 1 sem deixar sujeira.
- Evidência de execução permanece **[NV]** até comando com saída e data/hora; portarias de validação/release pertencem a fases posteriores, ausentes destes artefatos.
- Cada suíte tem contagem de `PASS`, linha `OK` e código de saída binários, o que permite Convergence READ-ONLY sem reinterpretar intenção.
