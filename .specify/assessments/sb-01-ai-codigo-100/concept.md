# Concept — Código 100% IA

**Data**: 2026-11-21

## Opções
1. **"IA propõe, gate + humano dispõem" (recomendada)**: autoria permitida e mista; toda saída de LLM é informativa e passa por `verify.sh` + revisor humano que explica o módulo. Ferramentas opcionais e open-source (Aider Apache-2.0 / OpenCode MIT); nenhum SDK de provedor no caminho crítico (IV). Coerente com ADAPT.
2. Adotar "100% IA" como meta: rejeitada — autodeclarada e infalsificável [NV], conflita com I/VI e ignora que o próprio autor descreve delegação verificada, não geração autônoma.
3. Proibir IA no projeto: rejeitada — descarta ganhos reais em tarefas bem especificadas (GitHub 55%, DORA amplificador) e é inimiga da praticidade do fluxo SDD.

## Decisão (ADAPT)
Opção 1. A fronteira é política e técnica: IA sugere código, o compilador/gates decidem o que entra e o humano responde pelo módulo. Impacto 3, confiança 3, facilidade 2, risco 2 → 9 (alinhado à matriz de encaixe local). Mantém ADR-004 (Bevy+Lightyear), a regra `--minimizado` e a proibição de `.unwrap()`/`.expect()`.
