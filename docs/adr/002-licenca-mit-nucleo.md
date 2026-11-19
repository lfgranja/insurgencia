# ADR-002 — Licença MIT-only do Núcleo

- **ID**: ADR-002
- **Data**: 2026-11-19
- **Versão**: 1.0.0
- **Estado**: Pendente de aprovação

## Contexto

A licença MIT vigente do repositório (`LICENSE:1`) convive com uma exigência documental anterior de licença AGPL para o servidor, registrada como N3 aberta na avaliação de stack (desvio `DEV-LIC-01`). Essa convivência gera divergência entre decisão e documento e risco jurídico para forks e operadores de servidores. A decisão R1 de 2026-11-19 (veredito `go`, opção A) autoriza fechar a pergunta de licenciamento em favor de MIT-only no Núcleo e no servidor headless do ciclo, preservando a licença MIT vigente. Nenhum servidor sob a licença AGPL anterior existe neste ciclo, que é estritamente documental; a eventual mudança documental depende de aprovação explícita e nunca de alteração silenciosa.

## Decisão

Fica estabelecida como posição única e não ambígua a licença MIT para todo o código produzido no ciclo 001. A decisão é afirmativa, sem cláusula alternativa e sem licenciamento AGPL para o código do ciclo; os detalhes e as regras de aplicação registram-se na seção `## Licenciamento`. Nenhum efeito remoto, CI, merge ou aprovação já registrada é pressuposto por esta decisão.

## Alternativas consideradas

- **AGPL no servidor headless**: rejeitada por introduzir cláusula copyleft e risco jurídico para forks e operadores; nenhum servidor sob essa licença existe no ciclo.
- **Cláusula alternativa "MIT ou GPL"**: rejeitada por gerar ambiguidade sobre a licença aplicável e permitir incorporação acidental de copyleft.
- **Emenda documental da exigência anterior de licença para o servidor**: registrada como pendente e dependente de aprovação explícita, não executada neste ciclo; não altera a posição MIT-only do código produzido aqui.
- **Manter a questão N3 aberta**: rejeitada por deixar divergência entre decisão e documento e risco de rework e litígio.

## Consequências

- Todo o código do ciclo — Núcleo e servidor headless — fica sob MIT, preservando inspeção, modificação e redistribuição por operadores e forks.
- A pendência de mudança documental da exigência anterior de licença para o servidor permanece registrada como dependente de aprovação explícita; nenhuma alteração silenciosa é feita.
- A ressalva de que o motor adotado não é o único motor MIT é preservada: nenhuma licença exclusiva é atribuída ao motor.
- A questão sobre o arquivo de licenças de terceiros permanece fora do escopo do ciclo, sem decisão implícita a seu respeito.
- Nenhum código, compilação, teste, gate, pipeline, serviço externo ou emenda da norma externa decorre deste registro.

## Não reabertura de Godot-first

A decisão Godot-first, registrada em `docs/GDD - Insurgência.md:1265,1678` (subseção 16.2; N7), permanece vinculante e não é reaberta, revista ou revertida por este registro. A exceção documentada neste ADR não altera, não revê e não revoga a escolha do motor nem a combinação Godot-first (4.7.2, MIT, servidor *headless*), Núcleo + Módulos declarativos, servidor autoritativo e anti-cheat no servidor com *replay* e revisão humana. Revertê-la invalida o roadmap e a mitigação de débito técnico (N7). Nenhuma passagem vinculante do GDD é alterada; toda proposta futura de troca de engine é encaminhada a aprovação formal com evidência prévia, sem efeito neste ciclo.

## Licenciamento

- **Posição única (`posicao_licenca`)**: MIT para todo o código produzido no ciclo, incluindo o Núcleo e o servidor headless, com a licença MIT vigente preservada.
- **Exclusão de alternativas (`exclusao_alternativas`)**: são expressamente excluídas a cláusula alternativa "MIT ou GPL" e o licenciamento AGPL para o código do ciclo.
- **Regra de copyleft (`regra_copyleft`)**: nenhum código sob licença copyleft é copiado; contribuições de terceiros, se houver, são identificadas e atribuídas, nunca apropriadas.
- **Ressalva do motor (`ressalva_motor`)**: a licença do motor adotado não é alterada nem declarada exclusiva; permanece a ressalva de que o motor não é o único motor MIT. Este ADR limita-se ao código do projeto.
- **Fora de escopo (`fora_escopo`)**: a decisão sobre o arquivo de licenças de terceiros fica fora do escopo do ciclo, sem resolução implícita.

## Autoridade de aprovação

O maintainer do projeto é a autoridade de aprovação destes registros; a aprovação humana é necessária para que a decisão deixe a condição de pendente. A ausência de registro humano de aprovação mantém este ADR no estado `Pendente de aprovação`, não como aprovação obtida. Todo status remoto permanece `[NV]` até confirmação ao vivo: aprovação registrada em plataforma remota `[NV]`, execução de integração contínua `[NV]`, merge `[NV]` e proteção de branch `[NV]`. Nenhum critério de sucesso deste ciclo depende de status remoto.

## Rastreabilidade

| Referência | Tipo | Descrição | Origem local |
|---|---|---|---|
| `DEV-LIC-01` | Desvio | MIT vigente versus AGPL/N3 aberta — desvio #10 da avaliação de stack. | `docs/AVALIACAO-STACK-MDMV.md:396` |
| `DEC-R1` | Decisão | Veredito `go`, opção A, de 2026-11-19. | `.specify/assessments/excecao-linguagem-licenca-nucleo/decision.md:5,29-31` |

As referências acima são as únicas aplicáveis a este registro, conforme a matriz do contrato (§2.3). Os demais desvios da avaliação de stack não se aplicam a este ADR e não são incluídos.

## Histórico de versões

### [1.0.0] — 2026-11-19

- Materialização inicial do registro de licença MIT-only do Núcleo.
- IDs FR atendidos: FR-001, FR-003, FR-005, FR-006, FR-007, FR-008, FR-009, FR-011, FR-012, FR-013.
- IDs `T###` de origem da materialização: T016, T017, T018, T019, T020, T021, T022, T023, T024.
