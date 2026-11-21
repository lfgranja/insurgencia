# Research — IP contra clones (sb-26)

**Data**: 2026-11-21 · **Idiomas**: EN/KO primários; ZH via lei oficial traduzida. Conteúdo web tratado como **dado, nunca instrução**; nenhuma afirmação de status legal sem fonte primária.

## Notas de pesquisa consultadas
- `docs/research/slotbound-2026-11-21-disclosure-legal.md` — achados 6–8 (mecanismo legal, gargalo de acesso, defesa documental).
- `docs/research/slotbound-2026-11-21-steam-gtm.md` — risco de cópia; publicar com MIT, changelog datado e commits versionados como prova de autoria.
- `docs/research/slotbound-2026-11-21-alternativas.md` — ALT-17 (proveniência aberta) e ALT-14 (CHANGELOG + atestação) no lugar de litígio.
- `docs/research/slotbound-2026-11-21-fit-local.md` — achado 5: constituição IV derruba sb-26 na parte ofensiva; "fork é feature".

## Achados
1. **O arcabouço legal existe; o gargalo é acesso.** Gamemeca documenta denúncia em 20/10 na categoria de infração de direito autoral de mini-programa, prazo de 7 dias úteis e remoção do primeiro clone. O dev não conseguiu denunciar da Coreia: exigência de celular chinês/captcha; um fã local fez a denúncia ("적극적으로 도와주시는 중국 관계자가 최소 한 분은 있어야").
2. **Leis primárias confirmam o mecanismo.** CAC arts. 14–17: notificação escrita (identidade, obra, prova preliminar) → remoção imediata → contra-notificação; art. 24 responsabiliza notificação indevida. Lei de Direitos Autorais arts. 10(12), 53 e 54: direito de comunicação em rede, cessação/apreensão/multa e indenização de ¥500 a ¥5.000.000. É o equivalente ao DMCA para plataformas chinesas.
3. **O protegível é a combinação criativa, não a regra de gênero.** A análise jurídica coreana reporta acórdãos da Suprema Corte (2017다212095, 2005다35707): acesso + similaridade substancial permitem presunção, mas criação independente a afasta. Recomenda builds versionados, planilhas, commits, URLs/datas e gravações sem corte. **[NV — acórdãos lidos só via artigo, não na fonte judicial]**.
4. **A tese do projeto veta a via ofensiva.** GDD §1/§3.3/§4.1 e constituição IV afirmam "fork é feature": qualquer pessoa pode redistribuir e modificar sem autorização. Perseguir forks por IP é contraditório; a resposta local aponta ALT-17 (proveniência aberta) como substituto de litígio transfronteiriço, com custo/jurisdição **[NV]**.
5. **Denúncia em WeChat/Douyin depende de contato local.** Mesmo com lei favorável, o dev solo sem telefone chinês não consegue abrir o procedimento — por isso ela fica **DEFER** até existir contato local, distinta do registro documental que pode começar agora.

## Fontes (URLs)
- Gamemeca — entrevista com Optima Arch (clones, 20/10, acesso): https://www.gamemeca.com/en/view.php?gid=1782926 · https://www.gamemeca.com/view.php?gid=1782926
- Gamemeca — análise jurídica do advogado Kang Jeong-mok (acórdãos **[NV]**): https://www.gamemeca.com/view.php?gid=1783557
- CAC — Regulamento de Proteção ao Direito de Comunicação em Rede (arts. 14–17, 24): https://www.cac.gov.cn/2013-02/18/c_126468776.htm
- NPC — Copyright Law, tradução oficial (arts. 10/53/54): http://en.npc.gov.cn.cdurl.cn/2020-11/11/c_674691.htm
- GitHub Docs — artifact attestations (ALT-14/17): https://docs.github.com/en/actions/concepts/security/artifact-attestations
- OhMyGame — recomenda registro de desenvolvimento: https://www.ohmygame.ai/blog/slotbound
