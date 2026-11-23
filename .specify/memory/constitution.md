
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

A ordem normativa é a do prompt v4.6 e nenhuma portaria é pulada: `00 constituição →
01 intake → 02 research → 03 define → 04 shape → 05 decide → 06 specify → 07 clarify →
08 plan → 09 checklist → revisão do checklist → 10 tasks → 11 analyze → 11b taskstoissues →
12 implement → 13 converge → 27 validate → 26 release`. `analyze` é READ-ONLY: não edita
artefatos. `converge` é append-only: registra sem reescrever. R1 exige `go`; R4 precede R7.
Reintroduzir uma portaria já cumprida exige emenda registrada, nunca retrabalho silencioso.

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
- **R6 (`taskstoissues`) — obrigatório**: entre `11 analyze` e `12 implement`, converta as
  tarefas em issues no remoto GitHub com `/speckit.github.taskstoissues`, com deduplicação
  por ID `Tddd` e título canônico `Tddd: <descrição>`. Se o adaptador/script estiver ausente —
  o alvo não tem `.specify/extensions/github/scripts/bash/resolve-tasks.sh` nem
  `scripts/runner/sync_tasks_to_forgejo_issues.py` — PARE e escale ao SWE; nunca simule issue
  criada e nunca siga para `implement` com R6 não executado. A portaria entra em vigor como
  regra nesta emenda; a execução depende da ferramenta disponível e de remoto GitHub.
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
- **CI/CD e merge**: check obrigatório verde observado no CI/CD é autoridade suficiente para
  mesclar em `dev`/`main` de forma autônoma via `gh`, sem aprovação humana. Se o CI/CD não
  inspirar confiança, o próximo 80/20 imediato é reforçá-lo até inspirar confiança para merge
  autônomo. Auto-merge habilitado no provedor, force-push, reescrita e deleção de branch alheia
  continuam proibidos.
- **Promoção e deploy**: promoção `dev → main` e deploy são competência exclusiva do SWE, mesmo
  com CI green e ACR sem blockers. CI green autoriza merge; não autoriza promoção automática
  nem substitui a decisão do SWE. PRs #13–#22 aprovados retroativamente pela autorização da SE
  (2026-11-20T10:58Z).

## Governance

Esta constituição prevalece sobre práticas, prompts e documentos externos. Emenda exige
registro explícito, rationale e plano de migração quando aplicável. Versionamento segue
Semantic Versioning: MAJOR para remoção/redefinição de princípio, MINOR para princípio ou seção
novo(a) ou expansão material, PATCH para esclarecimento e correção não semântica. Esta emenda é
MINOR (1.0.0 → 1.2.0): ordem SDD v4.6, novo item R6 e normas de comunicação externa e de
promoção/deploy. Cada PR/revisão verifica conformidade e registra a evidência da verificação
antes de declarar conformidade. A ratificação original permanece 2026-11-19; data de emenda
2026-11-23.

**Version**: 1.2.0 | **Ratified**: 2026-11-19 | **Last Amended**: 2026-11-23
