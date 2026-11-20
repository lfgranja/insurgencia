# Evidência externa — Stride #1666, manutenção Stride, Fyrox e Urho3D/Dviglo

Data de verificação: 2026-11-19. Fontes primárias consultadas ao vivo (GitHub REST API, repositórios oficiais e sites dos projetos). Conteúdo web tratado como dado não-confiável: nenhuma instrução encontrada foi executada. `[NV]` = não verificado; abaixo, os marcadores indicados são resolvidos ou reduzidos ao resíduo explícito.

## 1. Stride issue #1666 — GLB com texturas embarcadas

**Veredito:** `[NV]` **resolvido no nível documental**: a issue permanece **ABERTA** após toda a linha 4.4; não há correção registrada na issue, PR vinculado ou menção de correção nas notas de release 4.4.x.

- Estado ao vivo: `state: "open"`, `closed_at: null`, `state_reason: null`, sem assignee/milestone; criada em 2023-05-24, última atualização em 2026-10-31; 2 comentários, 1 reação `+1`. Título real: **"glb import"** (versão 4.1, Windows, labels `bug`, `area-GameStudio`, `area-Asset`). Fonte: "Issue 1666 — glb import", GitHub API, consultada em 2026-11-19, https://api.github.com/repos/stride3d/stride/issues/1666 (HTML: https://github.com/stride3d/stride/issues/1666).
- Causa-raiz documentada na própria issue: `MeshConverter.ExtractTextureDependencies` chama `assimp.GetMaterialTexture`, que devolve `*0`, `*1`… para texturas embutidas no `.glb`; `UPath.Decode` rejeita `*` (`ArgumentException: Invalid character [*] found in path [I:\*0]`). Workaround registrado: converter para glTF não-embarcado, OBJ ou FBX. Comentários de 2023-10-06 e 2023-10-25, sem resposta de mantenedor. Fontes: "Issue 1666 comments", GitHub API, 2026-11-19, https://api.github.com/repos/stride3d/stride/issues/1666/comments; "Issue 1666 timeline", GitHub API, 2026-11-19, https://api.github.com/repos/stride3d/stride/issues/1666/timeline.
- Última atividade é triagem, não correção: em 2026-10-31 o mantenedor `xen2` apenas acrescentou os labels `area-Asset` e `area-GameStudio`. Fonte: timeline citada acima.
- As notas oficiais da 4.4 (0, 4.4.1, 4.4.2, 4.4.3) não mencionam GLB, texturas embarcadas ou a issue #1666. A 4.4.2 (2026-11-13) registra apenas que "model import uses Stride's own Assimp build, which fixes bone offsets" — mudança de build do Assimp com escopo declarado em offsets de ossos, sem alegação de corrigir texturas incorporadas. Fontes: "Stride 4.4 release notes", stride-docs (atualizado com 4.4.3 em 2026-11-17), https://raw.githubusercontent.com/stride3d/stride-docs/master/en/release-notes/4.4.md e https://doc.stride3d.net/en/release-notes/4.4.html; release "Stride 4.4.2", GitHub, publicada em 2026-11-13, https://github.com/stride3d/stride/releases/tag/releases/4.4.2.
- **Ressalva de autoridade:** "sem correção" é inferido de estado aberto + ausência de registro de fix, não de teste empírico do importador na 4.4.3. Não foi executado build/import; essa limitação deve ser mantida se o texto reivindicar falha funcional observada.

## 2. Manutenção e comunidade Stride

**Veredito:** projeto ativo, não arquivado; manutenção contínua em novembro de 2026.

- Repositório `stride3d/stride`: `archived: false`, `pushed_at: 2026-11-19T14:35:53Z`, 7.955 estrelas, 1.188 forks, 693 issues abertas, 182 watchers, licença MIT, C#. Fonte: "Repository: stride3d/stride", GitHub API, consultada em 2026-11-19, https://api.github.com/repos/stride3d/stride.
- Cadência de releases: 4.4.0 em 2026-10-09; 4.4.1 em 2026-10-17; 4.4.2 em 2026-11-13; 4.4.3 em 2026-11-17; Launcher 6.0.9 em 2026-11-18 e CLI 1.0.7 em 2026-11-19. Fontes: release notes oficiais citadas; "Releases · stride3d/stride", GitHub API, consultada em 2026-11-19, https://api.github.com/repos/stride3d/stride/releases?per_page=4.
- Direção declarada: porte do editor para Avalonia em curso, CI com matriz completa em todas as plataformas/APIs e chamada por desenvolvedores. Fontes: "Announcing Stride 4.4", site oficial, 2026-10-09, https://www.stride3d.net/blog/announcing-stride-4-4/; release notes 4.4 citadas.

## 3. Fyrox — licença, manutenção, comunidade

**Veredito:** MIT confirmado por duas fontes primárias independentes; ativo.

- Repositório `FyroxEngine/Fyrox`: licença `MIT` (`spdx_id: MIT`), `archived: false`, `pushed_at: 2026-11-19T18:06:57Z`, 9.685 estrelas, 467 forks, 60 issues abertas, 102 watchers, Rust. Fonte: "Repository: FyroxEngine/Fyrox", GitHub API, consultada em 2026-11-19, https://api.github.com/repos/FyroxEngine/Fyrox.
- Texto integral da licença (MIT, "Copyright (c) 2019-present Dmitry Stepanov and Fyrox Engine contributors") no próprio repositório. Fonte: "Fyrox/LICENSE.md", GitHub, branch master, consultado em 2026-11-19, https://raw.githubusercontent.com/FyroxEngine/Fyrox/master/LICENSE.md.
- Atividade de release: v1.0.0 publicada em 2026-03-29 (RC1 em 2025-10-21, RC2 em 2026-02-09); repositório com ~8.600 commits. Fontes: "Releases · FyroxEngine/Fyrox", GitHub API, consultada em 2026-11-19, https://api.github.com/repos/FyroxEngine/Fyrox/releases?per_page=5; "GitHub - FyroxEngine/Fyrox", GitHub, consultado em 2026-11-19, https://github.com/FyroxEngine/Fyrox.
- Site oficial descreve o projeto como "feature-rich, production-ready, open-source game engine written in Rust". Fonte: "Fyrox - A feature-rich game engine built in Rust", https://fyrox.rs/, consultado em 2026-11-19.

## 4. Urho3D e Dviglo — licença e estado de manutenção

**Veredito:** Urho3D é MIT porém **arquivado**; Dviglo é **MIT** (o `[NV]` sobre sua licença fica resolvido), mas com atividade recente muito baixa.

- Urho3D: licença `MIT` no repositório, `archived: true`, `pushed_at: 2023-01-25T17:29:02Z`, 4.778 estrelas, 1.018 forks, 72 issues abertas, 2 watchers. Fontes: "Repository: urho3d/urho3d", GitHub API, consultada em 2026-11-19, https://api.github.com/repos/urho3d/Urho3D; "Urho3D - About", site oficial, https://urho3d.io/about.html, consultado em 2026-11-19 ("released under the MIT license"); texto da licença em https://raw.githubusercontent.com/urho3d/urho3d/master/licenses/urho3d/LICENSE. O repositório foi arquivado pelo owner em 2023-01-25 (aviso exibido na página do repositório).
- Dviglo: README oficial afirma "Dviglo распространяется под лицензией MIT" ("Dviglo é distribuído sob licença MIT") e aponta `licenses/dviglo/LICENSE`, cujo texto é o MIT integral com "Copyright (c) the Dviglo project". **A API do GitHub reporta `license: null` apenas porque não há arquivo de licença reconhecido na raiz** — não significa ausência de licença. Fontes: README em https://raw.githubusercontent.com/dviglo/dviglo/main/README.md; licença em https://raw.githubusercontent.com/dviglo/dviglo/main/licenses/dviglo/LICENSE; "Repository: dviglo/dviglo", GitHub API, consultada em 2026-11-19, https://api.github.com/repos/dviglo/dviglo. O fork é explicitamente declarado no README como voltado à comunidade russofalante a partir do Urho3D.
- Atividade Dviglo: `archived: false`, último push em 2024-03-09, 54 estrelas, 11 forks, 0 issues abertas, 4 watchers. Mesma fonte: GitHub API citada. Triangulação de manutenção: Urho3D arquivado desde 2023 e Dviglo sem pushes desde 2024 — contraste com Stride e Fyrox com pushes em 2026-11-19 (APIs citadas).

## 5. `[NV]` "conta Defold" — resolvido

**Veredito:** não é necessária conta Defold para baixar, instalar ou usar o editor/motor; a documentação oficial descreve fluxo inteiramente local.

- A página de download declara nos metadados oficiais: "start building games without extra setup, **registration**, or runtime fees" e oferece links diretos para os binários das releases do GitHub; o corpo afirma "There's nothing to set up or configure... Simply download the editor". Fonte: "Download Defold", Defold Foundation, consultado em 2026-11-19, https://defold.com/download/.
- O manual de instalação descreve apenas baixar, descompactar e executar (`./Defold` no Linux; `Defold.exe` no Windows), sem login, conta ou ativação em nenhuma etapa. Fonte: "Installing Defold", documentação oficial, consultado em 2026-11-19, https://defold.com/manuals/install/.
- O FAQ corrobora: editor com funcionalidade completa é gratuito, sem custos ocultos, taxas ou royalties; telemetria é anônima. Fonte: "Defold engine and editor FAQ", Defold Foundation, consultado em 2026-11-19, https://defold.com/faq/faq/.
- Contexto de comunidade (para a mesma linha da pesquisa): `defold/defold` tem 6.469 estrelas, 470 forks, 886 issues abertas, push em 2026-11-19, `license.spdx_id: NOASSERTION` (Defold License 1.0, não-OSI). Fontes: "Repository: defold/defold", GitHub API, consultada em 2026-11-19, https://api.github.com/repos/defold/defold; "The Defold License", site oficial, https://defold.com/license/, consultado em 2026-11-19.
- **Limite:** contas opcionais em serviços periféricos (fórum, Asset Portal, build servers) não foram auditadas; a afirmação resolvida cobre apenas download/instalação/uso do motor e editor, que é o alcance do `[NV]` original.

## Resumo dos marcadores

| `[NV]` original | Resultado |
|---|---|
| Stride issue #1666 sem correção na 4.4 | Resolvido documentalmente (issue aberta após 4.4.3; sem fix registrado); não testado empiricamente |
| Licença Dviglo | Resolvido: MIT (README + arquivo `licenses/dviglo/LICENSE`); API sem detecção por ausência de arquivo na raiz |
| Conta Defold | Resolvido: nenhuma conta exigida para editor/motor segundo download + manual de instalação |
| Manutenção/comunidade Stride, Fyrox, Urho3D, Dviglo | Verificada ao vivo via APIs e sites oficiais; números e datas acima |
