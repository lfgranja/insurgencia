# Problem Definition: Baselines locais de governança

- **Slug**: baseline-governanca
- **Created**: 2026-11-19
- **Inputs used**: intake.md + research.md (fontes locais: STATE, PROGRESSO, três scripts de gate, 80/20, contratos de fase)

## Problem Statement

Quem mantém e audita o aparato SDD do alvo não consegue distinguir um gate que passa porque não há regressão de um gate que passa porque não há nada baselinado: os checks de sdd-completeness, converge-append-only e task-id-uniqueness operam sem baseline local e nenhum arquivo de baseline existe em `docs/governance/`. Isso importa agora porque a dívida 1 do STATE está aberta, a auditoria pós-ciclo-001 classifica os "verdes vazios" como uma falha de auditoria e os registros históricos de contagem já divergem entre si, sem uma âncora local que fixe o estado aceito.

## Affected Users & Stakeholders

- **Users**: mantenedores do repositório que executam os gates locais e precisam confiar que um PASS significa não-regressão — [source: `.specify/memory/STATE.md:36-46`]
- **Users**: auditores de governança que revisam R4/R7 e precisam de evidência de artefatos, não apenas de comandos — [source: `docs/governance/sdd-phase-contracts.md:24-27`]
- **Stakeholders**: autoridade de governança do hub, que decide o que conta como dívida legítima e sustenta a regra "factory é fonte, não autoridade" — [source: `docs/PROGRESSO.md:15-17`, `.specify/memory/STATE.md:50`]
- **Stakeholders**: agentes e revisores que consomem STATE/PROGRESSO como índice vivo do estado verificado — [source: `docs/PROGRESSO.md:1-11`]

## Goals

- Fazer com que a aprovação dos três gates de governança seja auditável a partir de um estado local verificável, com autoria e proveniência inequívocas.
- Fazer com que regressões — gaps novos em specs, reescrita/truncamento/ausência de `converge.md`, IDs `T###` duplicados — deixem de passar despercebidas.
- Fechar a dívida 1 do STATE sem importar chaves, formatos ou decisões da factory.
- Tornar explícito o critério pelo qual um estado atual é aceito como linha de base, em vez de implícito na ausência de arquivo.

## Non-Goals

- Copiar baselines, specs ou configurações da factory para o alvo.
- Alterar a lógica, os limiares ou os schemas dos scripts de gate existentes.
- Declarar como verdes CI, merge, publicação, proteção de branch ou qualquer estado remoto — permanecem `[NV]`.
- Materializar engine, código-fonte, telemetria, banco ou CI neste escopo.
- Reabrir ou reclassificar dívidas documentais já registradas (itens 2–6 do STATE) como se estivessem resolvidas.
- Produzir a especificação técnica, a arquitetura ou as tarefas deste ciclo — isto é problema, não solução.

## Success Metrics

- Arquivos de baseline locais presentes em `docs/governance/` para os três gates (baseline: 0 de 3; listagem local de `docs/governance/`).
- Contadores `baselined_features`, `baselined_files` e `baselined_specs` das saídas dos gates deixarem de ser simultaneamente zero **ou**, se o estado atual estiver limpo, presença explícita do arquivo com schema local e proveniência registrada — sem inventar entradas (baseline: `0/0/0` em `.specify/memory/STATE.md:40-43`).
- Nenhuma chave de spec da factory aparecer nos baselines do alvo (baseline: risco registrado em `.specify/memory/STATE.md:50`).
- Um cenário de regressão controlado — gap artificial, `converge.md` reescrito, `T###` duplicado — ser recusado por pelo menos um gate antes de R4 (baseline: não medido; sem evidência de detecção no estado atual).
- Saída executada dos três gates colada em R4 com data/hora (baseline: dívida aberta; `.specify/memory/STATE.md:36-46` registra saídas, mas não há baseline materializado).

## Cost of Inaction

A dívida 1 permanece aberta e os gates continuam emitindo PASS sem lastro auditável: qualquer leitor de STATE/PROGRESSO pode interpretar "verde" como cobertura implementada quando ela não existe. À medida que novas specs e arquivos `converge.md` aparecem, não há linha de base local que distinga acréscimo legítimo de reescrita, apagamento ou duplicação de IDs — a detecção de regressão depende da memória de quem rodou o gate por último. O custo não é a ausência de arquivo em si, mas a perda de uma afirmação verificável de integridade no único aparato determinístico que o alvo possui hoje.

## Open Questions

- [NEEDS CLARIFICATION: qual é o estado exato do alvo no ato da materialização, dada a divergência histórica entre `spec_dirs=0`, `1` e `2` em `docs/PROGRESSO.md`?]
- [NEEDS CLARIFICATION: um baseline com mapas vazios, derivado de estado limpo, satisfaz a dívida 1 ou é necessária ao menos uma entrada por gate?]
- [NEEDS CLARIFICATION: que critério separa gap legítimo (a registrar como aceito) de defeito que deve ser corrigido antes de virar baseline?]
