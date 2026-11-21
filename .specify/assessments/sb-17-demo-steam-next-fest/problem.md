# Problem Definition: Demo Steam + Next Fest

- **Slug**: sb-17-demo-steam-next-fest
- **Created**: 2026-11-21
- **Inputs used**: intake.md + research.md (steam-gtm, alternativas, fit-local)

## Problem Statement

A proposta assume que "demo na Steam + Next Fest" impulsionaram streamers e visibilidade algorítmica e trata isso como uma jogada disponível. Não é: a Insurgência não tem Steam page, App ID, build de loja nem demo versionada, e a participação no Next Fest é **uma vez por título**, com build review de 3–5 semanas e registro até 10/01/2027 para a edição de fevereiro de 2027. Inscrever-se agora gastaria a janela única antes de existir artefato validado, e o mecanismo atribuído ao evento é `[NV]`/contradizido pelos próprios números do caso (100k wishlists antes de outubro). O problema concreto é de **sequenciamento e evidência**, não de vontade de publicar.

## Affected Users & Stakeholders

- **Users**: jogadores que encontrariam a demo na loja — hoje não podem, inexistindo página e App ID — [source: fit-local matriz sb-17]
- **Users**: engenheiros sem estúdio (GDD §3.2), que não devem pagar US$100 nem depender de SDK para ver a tese — [source: alternativas ALT-11; constituição IV]
- **Stakeholders**: SE, responsável por qualquer conta/pagamento Steam e pela marca; decisão irreversível de janela única — [source: steam-gtm Steamworks Next Fest]
- **Stakeholders**: autoridade do gate, que não pode declarar "pronto para loja" sem `verify.sh` executado — [source: constituição VI]

## Goals

- Impedir o gasto prematuro da janela única de Next Fest.
- Estabelecer pré-requisitos explícitos e observáveis para reavaliar a proposta.
- Separar o que é canal externo (Steam, festival) do que é ativo soberano (sim headless, release versionada, gate verde).
- Revisitar a decisão em data determinística, não por pressão de narrativa de terceiros.

## Non-Goals

- Criar Steam page, conta de publisher ou App ID neste ciclo.
- Pagar US$100, preparar build review ou produzir assets de loja.
- Perseguir wishlists como meta, nem assumir que elas movem visibilidade algorítmica.
- Declarar que o Next Fest causou o sucesso do caso estudado — efeito `[NV]`.
- Bloquear sb-16 (demo versionada soberana), que é pré-requisito e segue em frente.

## Success Metrics

- Nenhuma inscrição em Next Fest antes de: release versionada publicada (sb-16), `verify.sh` verde executado e janela com ≥5 semanas de folga (baseline: zero pré-requisitos atendidos).
- Reavaliação agendada antes de 10/01/2027 (Feb/2027) ou antes da edição de jun/2027 (baseline: sem data).
- Critérios de go registrados em ata: build versionada, store page pública, jogo não lançado antes do fim do evento, capitação de imprensa própria (baseline: inexistentes).
- Nenhum serviço externo no caminho crítico do gate local (baseline: satisfeito; manter).

## Cost of Inaction

Adiar não é custo zero em termos de calendário — a janela de fevereiro de 2027 tem registro até 10/01 e a de junho é a seguinte —, mas o custo de **não adiar** é maior: queimar a participação única com build imaturo, pagar taxa e SDK sem tração medida, e absorver uma narrativa de "algoritmo" que a documentação da Valve desmente. Enquanto sb-16 entrega a demo verificável, o diferimento preserva a opção de entrar no festival com artefato que já passou no gate.

## Open Questions

- [NEEDS CLARIFICATION: qual edição (Feb/2027 ou jun/2027) é alvo se e quando os pré-requisitos existirem?]
- [NEEDS CLARIFICATION: quem detém a conta Steam e autoriza os US$100 por app, caso a decisão mude?]
- [NEEDS CLARIFICATION: que evidência local substituirá "visibilidade algorítmica" como critério de sucesso do festival?]
