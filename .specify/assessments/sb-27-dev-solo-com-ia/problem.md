# Problem — Dev solo com IA

**Data**: 2026-11-21

O GDD §3.2 define como público engenheiros sem estúdio — exatamente o perfil que a IA pode ajudar a entrar. O problema é que o atalho sem compreensão cria dívida: um iniciante que delega tudo gera código que compila, mas que não consegue explicar nem depurar (Anthropic: 17 p.p. menos no quiz, maior dano em debugging). O caso Slotbound é inspiracional, mas n=1, com 6–12 meses, assets comprados e nenhum teste automatizado [NV]. [NV — nenhum gate executado]

## Critérios de sucesso
1. Iniciante pode contribuir usando IA como via de entrada, condicionado a checagem de compreensão: explica o módulo antes do merge.
2. Todo diff passa por `verify.sh` ALL STAGES OK; saída de LLM informativa, gate autoritativo (I/VI).
3. Docs de decisão append-only (constituição III) registram o que foi decidido e por quê.
4. Nenhuma dependência de provedor no caminho crítico (IV); ferramentas opcionais e open-source.
5. Decisões de produto/design humanas (V); nenhuma meta "100K em 3 dias" (ver sb-28).

## Custo de não agir
Afastar o público-alvo do GDD §3.2 se a barreira permanecer; ou, se a IA for adotada sem checagem, reproduzir a dívida de compreensão medida por Anthropic e a fragilidade de um projeto sem testes automatizados (E1/E2).
