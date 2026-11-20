# Convergence — 018-iam-adapter-local

**Data:** 2026-11-20T11:35Z · **Tipo:** append-only

## Phase 1: Convergence

- **Gap 1 — herança por `class_name`:** `iam_adapter_local.gd` usava `extends IAMInterface`, que não resolve em headless sem cache global. Corrigido na fonte para `extends "res://nucleo/iam/iam_interface.gd"`, preservando o contrato.
- **Gap 2 — anotações de tipo no teste:** o teste anotava `IAMAdapterLocal`/`IAMInterface` por `class_name`; trocado pelos constants de preload (`IAMAdapterLocalScript`/`IAMInterfaceLocal`).
- **Gap 3 — escape literal:** quatro linhas do teste continham `\t` literal em vez de TAB; corrigido.
- **Verificação:** 36 `PASS`, 0 `FAIL`, `OK test_iam_adapter_local`, exit 0; 131 `PASS` em regressão; `make check` exit 0. Saídas coladas no R4.

Nenhuma alteração de spec, plano, tasks, baseline, gate, ADR-003 ou contrato de arquitetura ocorreu nesta fase de convergência. Esta entrada é append-only; edições futuras devem acrescentar linhas, nunca reescrever ou renumerar as anteriores.
