# Concept — UI IA (sb-04)

**Data**: 2026-11-21

## Opções
1. **Manter HUD em `format!` + temas versionados (recomendada).** Continuar montando a UI a partir do estado determinístico em `atualizar_texto`; concentrar cores/strings em tabela de temas versionada e inspecionável. Custo zero, sem assets nem serviço externo, compatível com a isenção da itch.io para algoritmos autocontidos. Coerente com ADAPT.
2. **Resultado autoral revisado e atribuído (quando IA inspira layout).** Usar IA apenas fora do caminho crítico como referência de layout; versionar o artefato final revisado por humano, com inventário de proveniência (ferramenta, prompt/seed, licença, data). Aceitável, mas não necessário hoje.
3. **Geração de UI em runtime via modelo remoto.** Produzir elementos de interface dinamicamente durante o jogo. Dependência externa, saída irreproduzível, copyright incerto e disclosure obrigatório na Steam — sem ganho funcional sobre o `format!`. Rejeitada.

## Recomendação (ADAPT)
Opção 1, com a Opção 2 como saída controlada caso um layout inspirado por IA seja efetivamente desejado. Mantém a afirmação do vídeo como hipótese [NV] e preserva o verificável localmente: o adaptador de apresentação consome estado determinístico, e os gates continuam decidindo o que entra. Nenhuma IA em runtime, nenhum disclosure generativo para código/HUD determinístico.
