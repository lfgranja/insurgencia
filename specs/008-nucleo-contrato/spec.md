# Feature Specification: Núcleo, contrato versionado e Módulo de exemplo

**Feature Branch**: `008-nucleo-contrato` · **Created**: 2026-11-20 · **Status**: Draft

**Input**: fatia CE-1 / Task 1 de `docs/specs/2026-11-19-ciclo-80-20-omoba.md`.

## User Scenarios & Testing

**Independent Test**: checagem dos 39 caminhos canônicos (14 presentes, 25 pendentes das Tasks 2–6) e `OK test_contrato` (16 `PASS`, exit 0).
Mantenedor clona o repo e executa a checagem de caminhos e o teste do contrato; ambos passam com saída auditável.

**Independent Test**: `OK_CAMINHOS` (39 caminhos) e `OK test_contrato` (16 `PASS`, exit 0).

**Acceptance Scenarios**:
1. **Given** a árvore atual, **When** o laço da §3 da spec do ciclo roda, **Then** 0 `FALTA_CAMINHO` e `OK_CAMINHOS`.
2. **Given** os arquivos criados, **When** `test_contrato.gd` roda via `--headless --script`, **Then** 16 `PASS` e exit 0.

### User Story 2 — Módulo declarativo é recusado quando inválido (Priority: P1)
O carregador recusa campo ausente, campo desconhecido, tipo divergente, hash com 63/65 hex e `content_bytes` acima do limite.

**Independent Test**: 10/`10` recusas conforme plano Steps 5–6 (`CAMPO_OBRIGATORIO_AUSENTE`, `CAMPO_DESCONHECIDO`, `TIPO_INVALIDO`, `HASH_INVALIDO`, `BYTES_INVALIDOS`).

## Requirements
- **FR-001**: Criar `nucleo/contratos/`, `modulos/exemplo/` e os 7 arquivos da Task 1, preservando `LICENSE`, `.gitignore` e `tests/`. Os 25 caminhos restantes da árvore canônica pertencem às Tasks 2–6 e ficam pendentes.
- **FR-002**: Ajustar `project.godot` para `config/name="Insurgencia Nucleo"`, features `("4.7")`, sem `run/main_scene` — com diff do arquivo existente registrado antes da edição.
- **FR-003**: Criar `Protocolo` (`PROTOCOL_VERSION=1`, `MODULO_ID`, `TipoPacote`, constantes de recusa, `normalizar_pacote`, `SnapshotMeta.to_dict`).
- **FR-004**: Criar `CarregadorModulo.validar` com 11 campos permitidos, 10 obrigatórios e validação de tipo/hash/bytes.
- **FR-005**: Criar `modulos/exemplo/{module.json,personagens.json,regras.cfg}` com `content_hash` e `content_bytes` calculados dos bytes reais de `regras.cfg`.
- **FR-006**: Criar `tests/test_contrato.gd` (`extends SceneTree`) com 16 verificações e `quit(0)`/`quit(1)`.
- **FR-007**: Registrar evidência das saídas em `R4` e versão `1.0.0` em `R7`.

- **SC-001**: 14/39 caminhos canônicos presentes ao final (4 pré-existentes + 7 arquivos + 3 diretórios); 25 ausentes registrados como pendência, não como falha deste ciclo.

- **SC-002**: 16 linhas `PASS`, `OK test_contrato`, exit 0.
- **SC-003**: `sha256sum modulos/exemplo/regras.cfg` = `b2c11e6d84ac9db0ff4698343a3d47132e34f5e54d2f5b7f54c0d6374d4cd746`.
- **SC-004**: 10/10 recusas do carregador com os códigos exatos.
- **SC-005**: `git diff --stat -- nucleo/` vazio após troca de Módulo; `grep -c '"assets": \[\]'` = 1.
- **SC-006**: 5 gates determinísticos do alvo PASS ao final.

## Out of scope
Servidor, handshake de rede, validação de intenções, Ilha Meio, manifesto, `Makefile`, `evidence.csv`, deploy, remote, factory.
