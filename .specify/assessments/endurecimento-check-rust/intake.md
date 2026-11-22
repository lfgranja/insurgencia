# Idea Intake: Endurecimento do check `rust` na proteção de `dev`

- **Slug**: endurecimento-check-rust
- **Created**: 2026-11-21T20:26:00Z
- **Source**: auditoria de confiança da proteção da branch `dev` no repositório `lfgranja/insurgencia`; ciclo SDD 032 no branch `032-endurecimento-check-rust` (base `ce2d1f0` = `origin/dev`)
- **Type**: improvement (governança de branch / documental)

## Idea (as captured)

> "Exigir o check `rust` na proteção da branch `dev`, fechando o risco de confiança identificado: hoje só `gates` é obrigatório, então um merge pode acontecer com a CI Rust falha. Autorização explícita do SE nesta sessão para (a) mesclar para `dev` sempre que CI verde e (b) endurecer a proteção de `dev` com o check `rust`."

## Restated
Registrar, com evidências datadas e coladas, o merge do PR #36 para `dev` e a mutação autorizada da proteção de `dev` — de `contexts=["gates"]` para `contexts=["gates","rust"]` — preservando o SDD ordenado (R1 → R4 → R7), `converge.md` append-only e a fronteira restrita de 12 artefatos. Deploy/publicação e assinatura GPG continuam [NV]/DEFER.

## Origin & Context

- **Raised by**: SE (auditoria de confiança da proteção de `dev`) · **Trigger**: evidências datadas coletadas em 2026-11-21 (`gh` ao vivo, `./verify.sh`, 5 gates Python, `gh pr checks`).

## First-Glance Unknowns

- [RESOLVIDO: autorização — SE autorizou explicitamente o merge e o endurecimento da proteção de `dev` nesta sessão.]
- [RESOLVIDO: autoridade de gate — `./verify.sh` + 5 gates Python (locais) e `gh pr checks` (CI); factory é fonte, não autoridade.]
- [RESOLVIDO: escopo de escrita — somente os 12 artefatos da fronteira; nenhum código, gate, commit, janela do cliente ou credencial; 194 paths untracked alheios intactos.]
