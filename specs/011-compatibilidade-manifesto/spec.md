# Feature Specification: Manifesto de compatibilidade e gate local de versão/hash

**Feature Branch**: `011-compatibilidade-manifesto` · **Created**: 2026-11-20 · **Status**: Draft · **Data:** 2026-11-20T03:15Z

**Input**: Task 5 de `docs/plans/2026-11-19-ciclo-80-20-omoba.md` (linhas 1029–1215), literal.

## User Scenarios & Testing

### User Story 1 — Gate local aceita build compatível (P1)
Um build com `manifesto.json` íntegro (6 campos), `module_id` presente no Módulo de exemplo, versão de protocolo igual à local e SHA-256 de 32 bytes igual ao de `modulos/exemplo/regras.cfg` é aceito por `make check-compat`, que imprime `COMPAT_OK module_id=insurgencia.exemplo` e termina com status 0.

**Why this priority**: é a fatia mínima que prova CE-4.1 e destrava o uso do manifesto como contrato de compatibilidade do fork, sem serviço externo.

**Independent Test**: `godot --headless --path . --script res://ferramentas/verificar_compatibilidade.gd` → `COMPAT_OK module_id=insurgencia.exemplo`, exit 0.

### User Story 2 — Divergência falha com código legível e arquivo restaurado (P1)
Versão divergente, `module_id` ausente, manifesto ausente/incompleto, hash malformado ou divergente e arquivo de regras ausente são recusados com códigos distintos; a alteração do arquivo de regras gera `COMPAT_FALHA HASH_MODULO_DIVERGENTE` com status 1 e o arquivo é restaurado ao final do teste.

**Why this priority**: sem recusa local e sem códigos separados, a incompatibilidade só apareceria no handshake e a suíte deixaria resíduo no repositório.

**Independent Test**: copiar `modulos/exemplo/regras.cfg`, acrescentar uma linha, executar `make check-compat` capturando o status, restaurar o original e conferir status 1 com `HASH_MODULO_DIVERGENTE`.

### User Story 3 — Cliente exibe versão e hash antes de entrar (P2)
No caminho de falha, o executável do gate imprime `CLIENTE_VERSAO_PROTOCOLO`, `CLIENTE_HASH_MODULO` e `CLIENTE_ERRO`, com `MANIFEST_INVALID` exibido como erro; a saída é idêntica em 3 execuções consecutivas.

**Why this priority**: torna a decisão de compatibilidade auditável pelo usuário e verificável por comparação de logs (CE-4.5/CE-4.6).

**Independent Test**: executar a suíte 3 vezes redirecionando a saída e comparar com `diff -q`; nenhuma linha `FAIL` ou aceitação indevida.

### Edge Cases
- `manifesto.json` ausente ou JSON inválido → `carregar` devolve `{}` e `validar_formato` recusa com `MANIFEST_INVALID`.
- Campo obrigatório faltante (ex.: `checked_at`) → `MANIFEST_INVALID`; o teste usa uma cópia profunda do manifesto com o campo removido.
- `protocol_version` lido do JSON como `float` → comparação com `int(...)` contra `Protocolo.PROTOCOL_VERSION`.
- Hash com 63/65 caracteres hex → `MANIFEST_INVALID`; hash de 64 hex divergente do arquivo → `HASH_MODULO_DIVERGENTE`.
- `module_id` vazio ou diferente do Módulo → `MODULO_AUSENTE`; arquivo de regras inexistente → `REGRAS_AUSENTES`.
- Versão divergente é checada **antes** do hash, para produzir códigos legíveis e separados (cor-06).

## Requirements

- **FR-001**: `manifesto.json` contém 6 campos, todos obrigatórios: `manifest_version` (`"1.0.0"`), `protocol_version` (`1`), `module_id` (`"insurgencia.exemplo"`), `content_hash` (64 hex, `b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746`), `checked_at` (`"2026-11-19T00:00:00Z"`) e `tool_version` (`"4.7.2.stable.official"`).
- **FR-002**: `nucleo/compatibilidade/manifesto.gd` (`extends RefCounted`, `class_name ManifestoCompatibilidade`) declara `const Protocolo = preload("res://nucleo/contratos/protocolo.gd")` e `CAMPOS` com os 6 nomes; `carregar(caminho: String) -> Dictionary` abre com `FileAccess.open(..., FileAccess.READ)`, devolve `{}` quando o arquivo não abre ou quando o JSON não é `Dictionary`; `validar_formato(manifesto: Dictionary) -> Dictionary` exige todos os campos e devolve `{"ok": true, "codigo": "OK"}` ou `{"ok": false, "codigo": Protocolo.REJEICAO_MANIFESTO}`.
- **FR-003**: `conferir(manifesto: Dictionary, modulo: Dictionary, caminho_regras: String, versao_local: int) -> Dictionary` devolve sempre `{"ok": bool, "codigo": String}` e aplica, nesta ordem: formato inválido → `REJEICAO_MANIFESTO`; `int(manifesto["protocol_version"]) != versao_local` → `REJEICAO_VERSAO`; `module_id` diferente do Módulo → `REJEICAO_MODULO`; `content_hash` com comprimento diferente de 64 → `REJEICAO_MANIFESTO`; arquivo de regras ausente → `REGRAS_AUSENTES`; SHA-256 de 32 bytes do arquivo diferente do manifesto → `REJEICAO_HASH`; caso contrário `{"ok": true, "codigo": "OK"}`. Os códigos literais vêm de `Protocolo`: `VERSAO_PROTOCOLO_DIVERGENTE`, `HASH_MODULO_DIVERGENTE`, `MODULO_AUSENTE` e `MANIFEST_INVALID`.
- **FR-004**: `ferramentas/verificar_compatibilidade.gd` (`extends SceneTree`) carrega `res://manifesto.json`, `res://modulos/exemplo/module.json` e `res://modulos/exemplo/regras.cfg`, chama `conferir` com `Protocolo.PROTOCOL_VERSION` e imprime `COMPAT_OK module_id=<id>` + `quit(0)` no sucesso, ou `COMPAT_FALHA <codigo>` + `quit(1)` na falha; emite também `CLIENTE_VERSAO_PROTOCOLO`, `CLIENTE_HASH_MODULO` e `CLIENTE_ERRO` antes de entrar (CE-4.4/CE-4.6).
- **FR-005**: `Makefile` expõe o alvo `check-compat` (`.PHONY`) que invoca `$(GODOT) --headless --path . --script res://ferramentas/verificar_compatibilidade.gd`, com `GODOT ?= godot`; nenhuma sondagem de lobby/worker e nenhuma publicação de release (doc-26 DEFER).
- **FR-006**: `tests/test_manifesto.gd` (`extends SceneTree`) executa 7 verificações determinísticas — `manifesto_original_aceito`, `manifesto_ausente_recusado`, `manifesto_incompleto_recusado`, `divergencia_de_versao_falha`, `divergencia_de_hash_falha`, `modulo_ausente_falha` e `arquivo_de_regras_ausente_falha` — imprimindo `PASS <rotulo>`, `OK test_manifesto` e `quit(0)` com zero falhas, ou `FALHAS <n>` e `quit(1)` caso contrário; nenhuma linha `FAIL`.
- **FR-007**: Registrar as saídas executadas (com data/hora e marcação **[NV]** para o não observado) na portaria de validação R4; versão, changelog com FR/T e rollback em R7. Nenhum artefato R4/R7 é escrito nesta especificação.

## Success Criteria

- **SC-001**: `<G> --headless --path . --script res://tests/test_manifesto.gd` → sete linhas `PASS ...`, `OK test_manifesto`, nenhuma linha `FAIL` e código de saída `0`.
- **SC-002**: `make check-compat` → `COMPAT_OK module_id=insurgencia.exemplo` e código de saída `0`.
- **SC-003**: Três execuções de `test_manifesto.gd` redirecionadas para logs distintos produzem saída idêntica (`diff -q` sem diferenças), com os mesmos 4 códigos e 0 aceitações indevidas (CE-4.5).
- **SC-004**: Divergência de hash em `modulos/exemplo/regras.cfg` gera `COMPAT_FALHA HASH_MODULO_DIVERGENTE` e `CLIENTE_ERRO HASH_MODULO_DIVERGENTE`, com falha verificada (o script Godot encerra com `1`; GNU `make` reporta `2` para a receita que falhou — comportamento observado em 2026-11-20T03:18Z), e o arquivo restaurado ao final; 5 gates determinísticos do alvo PASS ao final.

## Out of scope
Sondagem de lobby/worker e publicação de release (`doc-26` DEFER), `evidence.csv`, partidas 5v5, Ilha Meio, vitória da Ilha Central, elenco Artesão/Guerreiro, deploy, remote, CI remoto, factory, alteração de baselines, gates ou configuração de CI e artefatos R4/R7. Nenhuma alegação de cobertura integral da árvore canônica (39/39 caminhos) é feita por este ciclo; o que não foi executado permanece **[NV]**.
