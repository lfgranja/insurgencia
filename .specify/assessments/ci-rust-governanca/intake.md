# Intake — CI Rust e governança alinhada

**Data**: 2026-11-21T04:24Z · **Slug**: `ci-rust-governanca` · **Origem**: orquestração autônoma (Blocos 0–5), auditada por scout.

## Ideia (registro fiel)
O CI do alvo (`.github/workflows/ci-gates.yml`) roda apenas os cinco gates Python de documentação; nenhum `cargo check/clippy/test/fmt`. Um PR com código Rust quebrado pode receber check verde e ser mesclado automaticamente pela norma da constituição. Além disso, `AGENTS.md` — autoridade que todo agente lê — afirma que não há código, build system ou `verify.sh`, e o cabeçalho de `verify.sh` diz "gate local do casv-rust". O GDD:1627 marca CI remoto como `[NV]`/DEFER embora o CI exista e tenha check SUCCESS documentado.

## Origem
- Bloco 0.a/0.b: gate autoritativo do alvo = `verify.sh` (4 estágios, 34 testes) + 5 gates no CI GitHub.
- Bloco 2/5: auditoria READ-ONLY (`AuditPendenciasNV`) com caminhos e trechos citados.
- Constituição, art. CI/CD: "Se o CI/CD não inspirar confiança, o próximo 80/20 **imediato** é reforçá-lo até inspirar confiança para merge autônomo."

## Fora de escopo desta ideia
Serviços externos, deploy, assinatura GPG, itens 5-6 do entregável, 5v5, rede. Nenhum trabalho da factory suja é transplantado.
