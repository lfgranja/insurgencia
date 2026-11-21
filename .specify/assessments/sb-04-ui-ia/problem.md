# Problem — UI IA (sb-04)

**Data**: 2026-11-21

O intake afirma que a interface do Slotbound teria sido gerada por IA [NV — não verificado]. No Insurgência, o problema concreto é que **não há problema a resolver** na forma proposta: o HUD já é textual e determinístico (`atualizar_texto` em `main.rs:547-561` monta posição/ilha/limites inteiramente com `format!`, sem assets). Introduzir UI gerada por IA — especialmente em runtime — adicionaria dependência externa (constituição IV), titularidade incerta (USCO/D.C. Circuit) e obrigação de disclosure (Steam/itch.io) sem qualquer ganho funcional. O risco real é perder a simplicidade auditável de um HUD que hoje é puro estado da simulação formatado.

## Sucesso observável
- HUD permanece em `format!` sobre estado determinístico, com temas versionados (cores/strings em tabela versionada) quando necessário; nenhuma geração de UI em runtime.
- Se IA inspirar um layout, apenas o resultado autoral revisado, atribuído e inventariado entra no repositório — nunca a saída bruta do modelo.
- Mudanças no HUD passam por `verify.sh` (check → clippy → nextest → fmt) + 5 gates, sem `.unwrap()`/`.expect()`, com `#![forbid(unsafe_code)]`/`#![deny(clippy::pedantic)]`.
- Regra `--minimizado` preservada para qualquer inspeção visual; ADR-004 (Bevy+Lightyear) intacto; nenhum disclosure generativo necessário para algoritmo autocontido.

## Custo de não agir
Trocar um HUD auditável por UI opaca gerada por modelo tornaria a apresentação irreproduzível, criaria dependência de fornecedor e exporia o projeto a risco de disclosure e copyright — tudo para resolver um problema que o `format!` local já resolveu. A inação (manter o HUD atual) tem custo zero; o custo está em adotar a proposta sem adaptação.

**[NV]** Quais elementos seriam gerados por IA, se a afirmação consta no post original e a redação viva da loja não foram verificados aqui; nenhum build/gate executado nesta escrita (constituição VI).
