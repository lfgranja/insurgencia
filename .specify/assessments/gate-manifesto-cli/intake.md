# Idea Intake: Gate CLI determinístico de manifesto de compatibilidade

- **Slug**: gate-manifesto-cli
- **Created**: 2026-11-22T16:54:17Z
- **Source**: ciclo SDD 034 no branch `034-gate-manifesto-cli` (base `7329c53`); âncora = `specs/011-compatibilidade-manifesto/spec.md` FR-001..FR-006, ciclo 033 (`crates/contratos/src/manifesto.rs`) e `docs/GDD - Insurgência.md` §14.2
- **Type**: improvement (adaptador CLI fino sobre contrato já existente)

## Idea (as captured)

> "Tornar o contrato de manifesto de compatibilidade verificável por linha de comando, de forma headless e determinística: um exemplo em `crates/sim/examples/` que lê `manifesto.json` e um arquivo de regras, calcula o SHA-256 real das regras, chama `conferir` do contrato e devolve `COMPAT_OK`/`COMPAT_FALHA` com códigos de saída estáveis — sem nova dependência e sem I/O dentro da biblioteca."

## Restated

Acrescentar um único arquivo executável de exemplo (`crates/sim/examples/verificar_manifesto.rs`) que atua como **adaptador de fronteira** (Constituição II): toda I/O (argumentos, leitura de arquivos, impressão, códigos de saída) fica no exemplo; a decisão permanece em `insurgencia_contratos::manifesto::conferir`, que não abre arquivos nem calcula hash. O contrato de 011/033 não é redefinido: mesmos 6 campos, mesmos 5 códigos literais e mesma ordem formato → versão → módulo → 64 hex → regras → hash.

## Origin & Context

- **Raised by**: SE (continuidade dos ciclos 011 e 033) · **Trigger**: o contrato Rust existe e é testado, mas não há superfície CLI para usá-lo em gates, CI ou verificação manual de um manifesto gerado a partir de um arquivo de regras real.
- **Estratégia**: GDD §14.2 (handshake troca versão do protocolo, identificador do módulo e hash SHA-256; incompatibilidade recusa com erro legível) e GDD §1 (soberania das regras). O CLI é apenas a porta de entrada; a autoridade continua sendo o contrato.
- **Evidência de partida**: `./verify.sh` em `2026-11-22T16:54:17Z` → `ALL STAGES OK`, `41 tests run: 41 passed`, `VERIFY_EXIT=0`; 5 gates Python PASS (SDD `33/0`, DAG `296/0`, task-id `0 duplicatas`, converge `16/16`, hub-docs `0 erros`); CI remota no head `7329c53`, run `44952738240`, `conclusion=success` (contextos `gates` + `rust`).

## First-Glance Unknowns

- [RESOLVIDO: superfície — assinatura fixa `verificar_manifesto <manifesto.json> <regras.cfg> [module_id] [versao_local]`, com os valores padrão de `MODULO_ID` e `PROTOCOL_VERSION` do crate de contratos.]
- [RESOLVIDO: dependências — `crates/sim/Cargo.toml` já tem `insurgencia-contratos`, `sha2 = "=0.10.9"` e `thiserror = "=2.0.21"`; nenhuma dependência nova e nenhum `Cargo.toml` alterado.]
- [RESOLVIDO: I/O — permanece fora da biblioteca; o CLI lê arquivos e repassa o hash como `Option<&str>` a `conferir`.]
- [RESOLVIDO: saída — sucesso `COMPAT_OK module_id=<id> content_hash=<hash>` (stdout, exit 0); recusa `COMPAT_FALHA <codigo>` (stderr, exit 1); uso inválido/falha de leitura (stderr, exit 2).]
- [RESOLVIDO: `REGRAS_AUSENTES` — arquivo de regras inexistente é mapeado pelo adaptador a `hash_regras = None`, devolvendo o código oficial via `conferir`; demais falhas de leitura são exit 2.]
- [NV: execução do roteiro de testes pelo orquestrador — nenhum build, teste, gate, commit, push ou PR executado neste ciclo; nenhum verde afirmado além das evidências datadas acima.]
