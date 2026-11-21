# Concept — Como tratar narração em material promocional

**Data**: 2026-11-21 · **Opção recomendada**: Opção 1 — devlog escrito + narração sintética local e rotulada (somente se houver ferramenta rastreável); publicar fica `DEFER`.

## Contexto

O intake parte de uma suspeita (narração IA em shorts de Slotbound) que a nota `ai-promo` classifica como **inferência de Garry Seto**, não evidência primária. O problema real para o Insurgência é produzir comunicação honesta sem depender de serviço externo e sem transformar uma inferência alheia em strategy. As regras 2026 de YouTube (isenta gameplay/voz própria clonada; exige rótulo para conteúdo realista enganoso) e Steam (declaração de IA inclusive em marketing) são fatores de borda, não motivos para adotar a técnica. Fora do código dos 5 crates; nenhuma engine nova (ADR-004 fixa Bevy+Lightyear); regra da janela (`--minimizado`) preservada. Veredito fixado: **ADAPT**.

## Opções consideradas

1. **Devlog escrito/versionado como núcleo + voz sintética local e rotulada apenas como complemento — recomendada (ADAPT).**
   - **Sketch**: o conteúdo informativo vive em texto sob `git` (auditável, reproduzível, sem janela); se alguma narração for desejada, usar **gerador local/rastreável**, com **rótulo explícito** no vídeo e **declaração de marketing** na Steam. Nunca usar a voz como "prova" do sucesso de Slotbound. Publicar em plataforma depende de conta/estreia → `DEFER` até lá; produzir o artefato rotulado é imediato.
   - **Appetite**: small — documentação e, no máximo, uma ferramenta local já licenciada; zero código nos crates.
   - **Trade-offs**: ganha coerência com a tese de verificação honesta (Constituição VI) e soberania (IV); sacrifica o "polimento" de voz profissional. Risco residual: rótulo em Shorts pode reduzir alcance (documentação do YouTube diz que não afeta recomendação; o controle de feed do TikTok é variável não quantificada `[NV]`).
   - **Rabbit holes**: perseguir o "som viral" de Garry (ele mesmo diz que o TikTok não viralizou); importar TTS em nuvem; gravar janela normal; tratar autorrelato de vendas como evidência.
   - **Por que atende**: adapta o *valor* (comunicar o jogo) sem importar a *premissa não verificada* (IA como causa do sucesso), respeitando I/IV/VI.

2. **Adotar TTS em nuvem/voz clonada como receita de crescimento — rejeitada.**
   - **Sketch**: contratar/gerar narração sintética online para todos os shorts, imitando o que se atribui a Slotbound.
   - **Appetite**: small a medium, mas recorrente (custo por vídeo + chave de API).
   - **Trade-offs**: parece "profissional" e escalável; em troca cria dependência proprietária, passivo de disclosure e uma premissa falsificada (a origem IA da voz é `[NV]`).
   - **Rejeição**: contraria IV (serviço externo) e VI (usar inferência como fato); o próprio Garry atribui o sucesso ao básico (demo/Next Fest), não à voz.

3. **Não produzir nenhum material promocional — rejeitada como decisão.**
   - **Sketch**: evitar o tema por completo.
   - **Appetite**: zero.
   - **Trade-offs**: elimina risco de rótulo, mas também elimina a comunicação do projeto — e não resolve a dúvida sobre disclosure quando o projeto eventualmente publicar.
   - **Rejeição**: é a opção "não fazer"; o assessment pede uma postura explícita, e a nota `ai-promo` mostra que devlogs escritos são baratos e auditáveis.

## Recomendação

**Opção 1**, coerente com o veredito **ADAPT**: manter o que é transportável (comunicação clara, versionada) e reescrever o que é frágil (premissa de IA como receita) sob as regras locais. Narração sintética, se usada, é **local, rastreável e rotulada**; publicar fica `DEFER` até existir conta. Detalhes de ferramenta, checklist de disclosure e canal pertencem ao handoff `/speckit.specify` — não a esta etapa.

## Fora de escopo

- Nesta etapa: qualquer pipeline de vídeo, TTS, captura, encode, janela, build, teste, gate, crate, commit ou publicação; escrita fora dos 4 artefatos deste slug.
- No handoff: serviços de nuvem, voz clonada de terceiros, conteúdo realista enganoso, uso da inferência de Slotbound como evidência, postagem antes de existir conta/estreia (`DEFER`), abertura de janela normal.
- `[NV]`: nenhum áudio/vídeo original inspecionado, nenhuma ferramenta TTS avaliada, nenhuma eficácia de short-form medida para o Insurgência.

## Suposições a validar

- Existe (ou pode ser licenciada) uma ferramenta TTS local com licença rastreável antes de qualquer uso — senão a Opção 1 segue apenas com devlog escrito.
- A checklist de disclosure (Steam/YouTube/TikTok) é documentada em coordenação com `sb-24`/`DisclosureLegalEvidence`.
- A conta/estreia que destrava "postar" aparece em ciclo futuro; até lá, o clipe rotulado é produzido e arquivado localmente.
