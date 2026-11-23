
# Insurgência Constitution
<!-- Fonte do aparato: factory HEAD `b1deab1` (registrada em `SOURCE_FACTORY.md`). -->

## Core Principles

### I. Determinismo primeiro

Todo trabalho computável por script, gate ou template é feito por script, gate ou template.
Saída de LLM é informativa; nunca roteia estado nem finaliza validação.

### II. Adaptadores e portas

Nenhum código de domínio chama diretamente serviço, SDK ou provedor concreto (persistência,
identidade, rede, telemetria, objetos). Toda integração passa por trait/interface e adaptador
substituível.

### III. Fluxo SDD ordenado

A sequência executável vigente é `00 constituição → 01 intake → 02 research → 03 define →
04 shape → 05 decide → 06 specify → 07 clarify → 08 plan → 09 checklist → revisão do
checklist → 10 tasks → 11 analyze → 12 implement → 13 converge → 27 validate → 26 release`.
Nenhuma portaria **desta sequência** é pulada. A portaria R6 (`taskstoissues`) foi ratificada
como intenção, mas **não integra a sequência executável** enquanto seu adaptador não existe;
quando existir e for ratificada por emenda, será inserida entre `11 analyze` e `12 implement`.
`analyze` é READ-ONLY; `converge` é append-only. R1 exige `go`; R4 precede R7. Reintroduzir uma
portaria já cumprida exige emenda registrada, nunca retrabalho silencioso.

### IV. Soberania do jogador

Regras, servidores e conteúdo devem ser inspecionáveis, modificáveis e redistribuíveis sem
autoridade central. Nenhuma dependência proprietária no caminho crítico; nenhum serviço
externo como ponto único de falha.

### V. Materialismo e ação coletiva

Sem magia, sem "campeão" solitário: habilidades são ações materiais e a vitória exige
coordenação entre funções.

### VI. Verificação honesta

Nada é afirmado como verde sem comando executado e saída colada. Status remoto, CI, merge e
proteções são `[NV]` (não verificado) até confirmação ao vivo. Diferenças entre documento e
código são registradas, nunca escondidas.

## Portarias de governança e comunicação

- **R1 (`go`)**: nenhum ciclo avança sem decisão `go` explícita e registrada.
- **R4 antes de R7**: R4 precede R7 na sequência; inversão exige emenda registrada.
- **R6 (`taskstoissues`) — ratificada, não executável neste alvo**: a intenção foi ratificada em v1.2.0, mas R6 **não pertence à sequência executável** enquanto `.specify/extensions/github/scripts/bash/resolve-tasks.sh` e o adaptador de issues não existirem. Se um ciclo tentar invocá-la sem a ferramenta, **PARE e escale ao SWE**; nunca simule issue criada, nunca use `gh` improvisado e nunca alegue execução. A inclusão de R6 no fluxo exige emenda ratificada após a ferramenta existir.
- **Comunicação externa**: PRs, comentários, escalonamentos e relatórios são didáticos e
  acionáveis. Toda afirmação de estado traz fonte observável; quando houver ação, inclua
  URL explícita. Diferencie caminho versionado no repositório de URL clicável: cite o caminho
  entre backticks e forneça a URL completa quando o leitor precisar agir. Nunca inclua
  credenciais, tokens ou segredos em comunicação externa.
- **Identidade da feature**: vem de `.specify/feature.json` / `check-prerequisites.sh --json`,
  nunca do nome do branch.
- **Segredos**: nenhum segredo, token ou credencial em arquivos, logs ou mensagens.
- **Precedência (Bloco 0.a)**: em divergência entre constituição, AGENTS.md e documento
  normativo externo — inclusive prompt de sessão — prevalece o registro de governança do alvo.

## Fluxo de trabalho, proteção e release

- Commits convencionais em português brasileiro; staging explícito; nunca `git add -A` ou
  `git add .`.
- **CI/CD e merge em `dev`**: check obrigatório verde observado no CI/CD é autoridade suficiente para mesclar em `dev` de forma autônoma via `gh`, sem aprovação humana. Se o CI/CD não inspirar confiança, o próximo 80/20 imediato é reforçá-lo até inspirar confiança para merge autônomo em `dev`. Auto-merge habilitado no provedor, force-push, reescrita e deleção de branch alheia continuam proibidos.
- **Promoção e deploy**: promoção `dev → main` e deploy são competência exclusiva do SWE, mesmo
  com CI green e ACR sem blockers. CI green autoriza merge; não autoriza promoção automática
  nem substitui a decisão do SWE. PRs #13–#22 aprovados retroativamente pela autorização da SE
  (2026-11-20T10:58Z).

## Governance

Esta constituição prevalece sobre práticas, prompts e documentos externos. Emenda exige
registro explícito, rationale e plano de migração quando aplicável. Versionamento segue
Semantic Versioning: MAJOR para remoção/redefinição de princípio, MINOR para princípio ou seção
novo(a) ou expansão material, PATCH para esclarecimento e correção não semântica. A emenda
anterior foi MINOR (1.0.0 → 1.2.0): ordem SDD v4.6, item R6 e normas de comunicação e de
promoção/deploy. Seguiram-se PATCHes: 1.2.1 limitou merge autônomo a `dev`; 1.2.2 remove R6 da
sequência executável até o adaptador existir, eliminando a contradição com o Princípio III.
Cada PR/revisão verifica conformidade e registra a evidência antes de declarar conformidade.
A ratificação original permanece 2026-11-19; data desta emenda 2026-11-23.

**Version**: 1.2.2 | **Ratified**: 2026-11-19 | **Last Amended**: 2026-11-23
