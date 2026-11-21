# Validação (R4) — 027-lowpoly-real

**Data**: 2026-11-21T02:55Z · **Resultado**: PASS (técnico), inspeção visual **[NV]**.

|Verificação|Saída|
|---|---|
|`verify.sh`|ALL STAGES OK; 34/34 testes; fmt limpo|
|`cargo build -p insurgencia-cliente`|exit 0|
|Runtime `--minimizado`|GPU Vulkan RADV RAVEN, 0 B0004, 0 erros|
|Janela normal|aberta (PID 696504), `Creating new window` sem erro|
|5 gates do alvo|PASS (após baselines atualizados)|

## R4
Aprovado para R7: mudança é local ao cliente, sem alteração de contratos, protocolo ou simulação. `lib.rs` intacto.
