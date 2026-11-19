# Constitution — Insurgência (alvo)

**Version:** 1.0.0 | **Ratified:** 2026-11-19 | **Fonte do aparato:** factory HEAD `b1deab1` (registrada em `SOURCE_FACTORY.md`)

## Princípios

I. **Determinismo primeiro.** Todo trabalho computável por script, gate ou template é feito por script, gate ou template. Saída de LLM é informativa; nunca roteia estado nem finaliza validação.

II. **Adaptadores e portas.** Nenhum código de domínio chama diretamente serviço, SDK ou provedor concreto (persistência, identidade, rede, telemetria, objetos). Toda integração passa por trait/interface e adaptador substituível.

III. **Fluxo SDD ordenado.** intake → research → define → shape → decide → specify → clarify → plan → checklist → analyze → tasks → implement → converge → validate → release. Nenhuma portaria é pulada. `converge.md` é append-only. R1 exige `go`, R4 precede R7.

IV. **Soberania do jogador (pilar do produto).** Regras, servidores e conteúdo devem ser inspecionáveis, modificáveis e redistribuíveis sem autoridade central. Nenhuma dependência proprietária no caminho crítico; nenhum serviço externo como ponto único de falha.

V. **Materialismo e ação coletiva.** Sem magia, sem "campeão" solitário: habilidades são ações materiais e a vitória exige coordenação entre funções.

VI. **Verificação honesta.** Nada é afirmado como verde sem comando executado e saída colada. Status remoto, CI, merge e proteções são `[NV]` até confirmação ao vivo. Diferenças entre documento e código são registradas, nunca escondidas.

## Normas de trabalho

- Commits convencionais em português brasileiro; staging explícito; nunca `git add -A` ou `git add .`.
- Branch `feature/<NNN>-<slug>`, PR para `dev`/`main` via `gh`; sem merge, auto-merge ou force-push automáticos.
- Identidade da feature vem de `.specify/feature.json` / `check-prerequisites.sh --json`, nunca do nome do branch.
- Nenhum segredo, token ou credencial em arquivos, logs ou mensagens.
- Em caso de divergência entre constituição, AGENTS.md e documento normativo externo, prevalece o registro de governança do alvo (Bloco 0.a).
