# Evidência externa — Lightyear ↔ Bevy (verificação ao vivo, 2026-11-19)

Escopo: resolver os `[NV]` das afirmações sobre matriz de compatibilidade, book WIP, cadência de versões e licença em `docs/avaliacao-engines/pesquisa/bevy.md:11,23,26`, `docs/avaliacao-engines/03-recomendacao.md:88` e `docs/avaliacao-engines/02-analise-por-opcao.md:45`. Método: HTTP direto ao repositório oficial, crates.io API e book publicado; navigator em 19/11/2026. Fontes primárias em inglês trianguladas com as afirmações em PT-BR do corpus.

## 1. Matriz de compatibilidade — CONFIRMADO

Tabela oficial "Supported bevy version" do README (verificação integral):

| Lightyear | Bevy |
|-----------|------|
| 0.31 | 0.20 |
| 0.28–0.30 | 0.19 |
| 0.26–0.27 | 0.18 |
| 0.25 | 0.17 |
| 0.20–0.24 | 0.16 |
| 0.18–0.19 | 0.15 |
| 0.16–0.17 | 0.14 |
| 0.10–0.15 | 0.13 |
| 0.1–0.9 | 0.12 |

O README ainda fixa explicitamente: "For Bevy 0.20 (Rust 1.97.1 or newer) ... `lightyear = \"0.31\"`".

Triangulação independente pelos manifests publicados (crates.io API `/dependencies`): `bevy_app`/`bevy_ecs` = `^0.19` em 0.28.0, 0.29.0, 0.30.0 e 0.30.1; `^0.20` em 0.31.0. O `Cargo.toml` do workspace em `main` registra `version = 0.31.0`, `rust-version = 1.97.1` e todas as dependências `bevy* = 0.20`. Portanto a afirmação "0.28–0.30 → Bevy 0.19; 0.31 → 0.20" é literalmente a tabela upstream, não inferência.

## 2. Book WIP — CONFIRMADO

O README chama o book de "WIP book" e linka `cbournhonesque.github.io/lightyear/book/`. O `book/src/introduction.md` é explícito: "This book does not aim to be a polished document, or a comprehensive reference ... It is more of a collection of notes and thoughts". O `book/src/SUMMARY.md` confirma o estado incompleto: a seção `Examples` contém apenas o título, sem entradas. A descrição "collection of notes" em `docs/avaliacao-engines/pesquisa/bevy.md:23` está confirmada na fonte.

## 3. Versões por ciclo do Bevy — CONFIRMADO COM NUANÇA

A crates.io API reporta 60 versões publicadas (crate criado em 04/02/2023; última em 21/10/2026). Contagem por faixa da tabela oficial:

- Bevy 0.12 (0.1–0.9): 13 publicações / 9 séries menores
- Bevy 0.13 (0.10–0.15): 8 / 6
- Bevy 0.14 (0.16–0.17): 7 / 2
- Bevy 0.15 (0.18–0.19): 3 / 2
- Bevy 0.16 (0.20–0.24): 11 / 5 (+4 RCs; 0.20.0 está yanked)
- Bevy 0.17 (0.25): 3 / 1
- Bevy 0.18 (0.26–0.27): 6 / 2
- **Bevy 0.19 (0.28–0.30): 4 publicações — 0.28.0 (26/06/2026), 0.29.0 (10/08), 0.30.0 (13/09), 0.30.1 (16/09) — em 3 séries menores**
- Bevy 0.20 (0.31): 1 até 19/11/2026 — 0.31.0 (21/10/2026)

A afirmação "4 versões do Lightyear por ciclo do Bevy" está **correta para o ciclo 0.19** (base da avaliação O3), mas não é lei geral: a contagem varia de 1 a 13 conforme o ciclo. Recomenda-se lê-la como observação do ciclo corrente, não como regularidade. A tabela do README conta séries menores (0.28–0.30 = 3); o número 4 vem das publicações reais incluindo o patch 0.30.1 — ambas as leituras são consistentes. Ressalva: a página de Releases do GitHub marca 0.30.0 como "Latest" enquanto o crates.io já tem 0.31.0; as notas de release ficam defasadas em relação ao registro.

## 4. Licença MIT/Apache — CONFIRMADO

`Cargo.toml` (workspace): `license = "MIT OR Apache-2.0"`. O repositório contém `LICENSE-MIT` (texto MIT integral) e `LICENSE-APACHE`. A API do crates.io devolve `license = "MIT OR Apache-2.0"` para 0.28.0, 0.30.1 e 0.31.0. A UI do GitHub exibe apenas "Apache-2.0 license" (detecção automática parcial), mas o manifest publicado e os dois arquivos de licença são a autoridade: a dupla licença confirmada em `docs/avaliacao-engines/pesquisa/licencas_lockin.md:36` está correta.

## Resolução dos [NV]

| Afirmação no corpus | Veredito |
|---------------------|----------|
| Matriz 0.28–0.30 → 0.19; 0.31 → 0.20 (`bevy.md:11`) | Verificado ao vivo — `[NV]` removível |
| Book WIP / "collection of notes" (`bevy.md:23`) | Verificado ao vivo — `[NV]` removível |
| "4 versões por ciclo do Bevy" (`bevy.md:26`) | Verificado para o ciclo 0.19; generalizar seria falso |
| MIT/Apache-2.0 (`licencas_lockin.md:36`) | Verificado ao vivo — `[NV]` removível |

Permanecem `[NV]` **fora deste escopo**: reconexão que restaura sessão, hash de conteúdo no handshake, ausência de preset "dedicated server", custo/tick e benefício (nenhum foi testado nesta rodada).

## Fontes (acesso em 19/11/2026)

1. "Lightyear — README.md" — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/README.md (tabela de compatibilidade, book WIP, requisito Rust 1.97.1).
2. "lightyear — crates.io API (metadados e 60 versões)" — https://crates.io/api/v1/crates/lightyear (datas, `num_versions=60`, `license`).
3. "lightyear 0.28.0/0.30.1/0.31.0 — dependências" — https://crates.io/api/v1/crates/lightyear/0.31.0/dependencies (e análogos; `bevy_app`/`bevy_ecs` ^0.19 vs ^0.20).
4. "Lightyear — Cargo.toml do workspace" — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/Cargo.toml (`license`, `rust-version`, deps Bevy 0.20).
5. "Lightyear book — introduction.md e SUMMARY.md" — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/book/src/introduction.md · https://raw.githubusercontent.com/cBournhonesque/lightyear/main/book/src/SUMMARY.md.
6. "Lightyear — LICENSE-MIT / LICENSE-APACHE" — https://raw.githubusercontent.com/cBournhonesque/lightyear/main/LICENSE-MIT · https://raw.githubusercontent.com/cBournhonesque/lightyear/main/LICENSE-APACHE.
7. "Releases · cBournhonesque/lightyear" — https://github.com/cBournhonesque/lightyear/releases (0.30.0 como "Latest" na UI; defasagem observada).

Idiomas: fontes primárias em inglês; afirmações auditadas em português (`docs/avaliacao-engines/`). Nenhuma afirmação deste arquivo repousa em fonte única.
