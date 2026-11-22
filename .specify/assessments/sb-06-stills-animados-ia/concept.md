# Concept — Como produzir material visual promocional sem stills sintéticos enganosos

**Data**: 2026-11-21 · **Opção recomendada**: Opção 1 — quadros reais do `sim` via captura determinística/rotulada; postar `DEFER`.

## Contexto

A suspeita de "stills animados por IA" em Slotbound é inferência de Garry Seto, não evidência primária `[NV]`. O problema real é de honestidade: um still sintético apresentado como jogo sugeriria recursos inexistentes no MVP (Constituição VI) e exigiria disclosure de marketing (Steam Content Survey §3). O repo já é headless por dentro (`insurgencia-sim` independe da engine; HUD 100% em `format!`), o que permite captura reproduzível sem janela. Veredito fixado: **ADAPT**.

## Opções consideradas

1. **Captura determinística de quadros reais do `sim`, com rótulo de origem — recomendada (ADAPT).**
   - **Sketch**: gerar quadros a partir de uma execução semeada do `insurgencia-sim` (ou, se necessário, do exemplo headless do Bevy com `primary_window: None`), registrar semente + commit no nome do artefato, codificar com FFmpeg `-bitexact`/`-map_metadata -1` e publicar `sha256`. Nenhum still IA apresentado como gameplay; se um still IA for usado, fica declarado como marketing e rotulado. Publicar em plataforma depende de conta/estreia → `DEFER`.
   - **Appetite**: small a medium — reaproveita o crate `sim` e o gate `verify.sh` (`--all-targets` já cobre `examples/`); custo em codificar/validar captura.
   - **Trade-offs**: ganha evidência auditável e coerência com I/IV/VI; sacrifica o "polimento" de uma imagem gerada. Risco: bit-exatidão entre versões do FFmpeg não medida `[NV]`.
   - **Rabbit holes**: gravar janela normal (proibido pela regra `--minimizado`); confundir captura com edição IA; perseguir o "look" de Slotbound em vez de mostrar o jogo real.
   - **Por que atende**: substitui a premissa frágil por evidência primária do próprio projeto — é o oposto de enganar sobre o estado do MVP.

2. **Gerar stills com IA e declará-los — rejeitada.**
   - **Sketch**: usar ferramenta de imagem/Animate para criar stills "estilo Slotbound", com rótulo na Steam e no vídeo.
   - **Appetite**: small, mas depende de ferramenta externa e de licença rastreável.
   - **Trade-offs**: entrega estética polida rapidamente; em troca mantém risco de o espectador ler o still como jogo real, cria dependência proprietária (IV) e passivo de disclosure.
   - **Rejeição**: a declaração mitiga o rótulo, mas não o risco de falsa evidência; a alternativa local produz prova melhor com menos passivo.

3. **Não produzir material visual algum — rejeitada como decisão.**
   - **Sketch**: evitar o tema por completo.
   - **Appetite**: zero.
   - **Trade-offs**: elimina risco de disclosure, mas também elimina a possibilidade de mostrar progresso do jogo.
   - **Rejeição**: opção "não fazer"; a captura determinística resolve o problema sem custo de engine.

## Recomendação

**Opção 1**, coerente com **ADAPT**: manter o valor (mostrar o jogo) e substituir a fonte não verificada (IA) por quadros reais, rotulados e rastreáveis. Se ainda assim um still IA for usado, a condição é declará-lo como marketing na Steam e rotulá-lo no vídeo. Postar fica `DEFER` até existir conta/estreia; produzir o quadro é imediato. Detalhes do pipeline (formato, encode, UTM) pertencem ao handoff, não a esta etapa.

## Fora de escopo

- Nesta etapa: qualquer captura, encode, ferramenta de IA, build, teste, gate, crate, janela, commit ou publicação; escrita fora dos 4 artefatos deste slug.
- No handoff: ferramenta de geração de imagem, serviço de nuvem, conteúdo realista enganoso, publicação sem conta, janela normal, alteração em crates/gates/engine (ADR-004 fixo).
- `[NV]`: vídeo original não inspecionado, captura/encode não executados, bit-exatidão não verificada.

## Suposições a validar

- O exemplo headless do Bevy (`headless_renderer.rs`) ou a saída direta do `sim` atende ao formato vertical desejado sem abrir janela.
- O `verify.sh` com `--all-targets` cobrirá um eventual `example` de captura sem novo job (nota fit-local, achado 2).
- A conta/estreia que destrava "postar" aparece em ciclo futuro; até lá, os quadros rotulados são arquivados localmente.
