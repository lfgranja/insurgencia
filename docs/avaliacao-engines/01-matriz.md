# Matriz comparativa — avaliação de engines para o Insurgência

Data: 2026-11-19. Protocolo 019/040 do `mdmv-dark-factory` adaptado por `00-criterios.md`:
≥10 critérios, pesos com Σ=100%, escala 0–10, score ponderado = Σ(nota × peso)/10,
matriz com ≥10 opções, recomendação por fase, evidência e marcação `[NV]`.
Opções, nomes e IDs conforme `00b-contrato-escrita.md`. Notas extraídas dos oito relatórios em
`pesquisa/`; nenhuma nota foi inventada.

## 1. Critérios e pesos

| # | Critério | Peso | Definição operacional resumida (0–10) | Natureza |
|---|---|---:|---|---|
| C1 | Soberania e licença | 15 | OSI/SPDX (organização/padrão de software aberto) confirmado; MIT/permissiva favorecida; fork (bifurcação) e redistribuição sem permissão; sem EULA (contrato de usuário final) ou SDK (kit de desenvolvimento) proprietário no caminho crítico; licença de assets clara. | **Inviolável** (Pilar I) |
| C2 | Servidor autoritativo e federação | 15 | Servidor dedicado headless (sem interface gráfica) real; validação de input no servidor; snapshots (fotos de estado) versionados; tolerância a perda/reconexão; nenhum serviço central obrigatório. | **Inviolável** (GDD §14) |
| C3 | Modularidade Núcleo + Módulos | 12 | Regras, personagens e mapas por dados declarativos, sem recompilar o Núcleo; versionamento e hash de conteúdo no handshake (aperto de mão de conexão); fork como pacote, não como cópia da engine. | **Inviolável** (GDD §14.2) |
| C4 | Adequação aos Desafios de Ilha | 10 | Editor/lógica para sincronia de entrada (2 jogadores/5 s), coleta, sabotagem e negociação com PNJs (personagens não jogáveis); microssistemas independentes. | Alta |
| C5 | Tempo até protótipo e curva | 10 | Um mantenedor com base em engenharia de software; tempo até partida jogável; documentação; ausência de toolchain (cadeia de ferramentas) exótica. | Alta |
| C6 | Performance e escala da partida | 8 | Overhead do servidor headless, custo por tick (passo de simulação), memória, 5v5 + bots, latência; proxies (indicadores indiretos) se sem benchmark (teste comparativo) local. | Média |
| C7 | Pipeline de assets FOSS | 8 | Importação de CC0/low-poly, GLB/glTF, ferramentas livres, coerência com greybox e rejeição de assets AAA. | Média |
| C8 | Maturidade e governança | 8 | Releases estáveis, manutenção ativa, comunidade, entidade neutra, previsibilidade de roadmap. | Média |
| C9 | Portabilidade futura | 7 | Desktop primeiro; Linux/Windows/macOS e caminho eventual a mobile/web sem reescrita. | Média |
| C10 | Risco operacional e lock-in | 7 | Complexidade de CI/toolchain, dependências externas, substituibilidade da engine, custo de migração. | Média |

**Soma dos pesos = 15+15+12+10+10+8+8+8+7+7 = 100.** C1–C3 são invioláveis: nota baixa
nelas não pode ser compensada por C4–C10 na recomendação final.

## 2. Escala 0–10 e convenções

| Nota | Âncora |
|---:|---|
| 0 | Ausente ou incompatível por princípio. |
| 3 | Existe, mas exige trabalho desproporcional ou cria dependência problemática. |
| 5 | Adequação com esforço; sem evidência primária forte. |
| 7 | Bom; evidência documental primária; ressalvas menores. |
| 9 | Excelente; implementado e verificado em fonte primária ou projeto real equivalente. |
| 10 | Canônico para o requisito; zero atrito. |

- `*` = evidência parcial, proxy ou `[NV]`; nota conservadora e recalibrável. **Toda nota com `*` tem
  justificativa explícita** na subseção 3.1.
- `[NV]` = não verificado ao vivo/localmente (sem build, protótipo ou benchmark nesta rodada).
- Fórmula: `score = Σ(nota × peso) / 10`, arredondado a 1 casa decimal.
- Rótulos de Fase 0: **ADOPT** (adotar como está), **ADAPT** (adotar com trabalho de integração),
  **DEFER** (postergar), **AVOID** (não usar salvo mudança de contexto), **REJECT** (descartado).

## 3. Matriz comparativa (ordenada por Score decrescente)

| Opção | C1 (15) | C2 (15) | C3 (12) | C4 (10) | C5 (10) | C6 (8) | C7 (8) | C8 (8) | C9 (7) | C10 (7) | Score | Fase 0 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| **O1 Godot 4.7.2** | 9 | 8 | 8 | 8 | 9 | 7* | 9 | 9 | 8 | 7 | **82.6** | **ADOPT** |
| **O3 Bevy 0.19.1 + Lightyear 0.28–0.30** | 10 | 9* | 7* | 7* | 5* | 8* | 8 | 7* | 8* | 6* | **77.1** | **ADAPT** |
| **O9 Panda3D** | 9 | 8 | 8 | 6 | 8 | 6* | 8 | 7 | 7 | 6 | **75.0** | **DEFER** |
| **O7 libGDX 1.14.2** | 9 | 6 | 6 | 5 | 8 | 7* | 7 | 9 | 8 | 8 | **72.3** | **DEFER** |
| **O6 Stride** | 10 | 6* | 7 | 6 | 7 | 6* | 5* | 8 | 7 | 8 | **71.1** | **AVOID** |
| **O2 Bevy 0.19.1** | 10 | 4* | 7* | 4* | 6 | 8* | 8 | 8 | 8 | 6 | **68.4** | **REJECT** |
| **O4 Defold** | 7 | 5* | 6 | 6 | 8 | 6* | 8 | 7 | 9 | 8 | **67.9** | **REJECT** |
| **O8 Urho3D / Dviglo** | 8 | 8 | 7 | 7 | 5 | 7* | 7 | 3 | 6 | 4 | **65.0** | **AVOID** |
| **O5 Flax Engine** | 4 | 7* | 6 | 7 | 6 | 6* | 8 | 8 | 8 | 4 | **62.7** | **REJECT** |
| **O10 MonoGame / FNA** | 8 | 4* | 4 | 5 | 6 | 5* | 6 | 9 | 7 | 7 | **59.6** | **REJECT** |
| **O13 macroquad + tokio (Rust custom)** | 9 | 5* | 3 | 3 | 5 | 6* | 5* | 5 | 8 | 5 | **54.5** | **REJECT** |
| **O12 Haxe / OpenFL** | 8 | 6 | 4 | 3 | 5 | 5* | 3 | 6 | 8 | 4 | **53.4** | **REJECT** |
| **O11 LÖVE** | 8 | 5 | 5 | 2 | 9 | 4* | 2 | 6 | 5 | 4 | **52.4** | **REJECT** |

### 3.1 Proveniência das notas e justificativas de `*`

Notas adotadas dos relatórios por engine, considerados primários para seus IDs. O relatório
`netcode_modularidade.md` aplica leitura mais estrita de C2/C3 e sugere notas alternativas
(O3 C2=9/C3=4; O4 C2=3; O6 C2=3; O7 C2=3; O8 C2=7; O9 C2=7; O10 C2=3/C3=3; O5 C2=7).
Elas ficam registradas aqui como divergência interpretativa e **não substituem** a nota do
relatório dedicado; onde coincidem (O1 C2=8/C3=8; O3 C2=9), reforçam a nota. As correções
faciais obrigatórias de `licencas_lockin.md` e `00b-contrato-escrita.md` já estão refletidas:
Panda3D BSD-3-Clause, MonoGame/FNA Ms-PL, compilador Haxe GPL-2.0-or-later, Flax
*source-available* sob EULA, Defold não-OSI, LÖVE zlib com OpenAL Soft LGPL embarcado,
Urho3D arquivado/Dviglo parado em 03/2024, Bevy sem editor visual oficial na 0.19 e Godot
4.7.2 como estável.

| Opção (fonte) | Notas `*` | Justificativa explícita (evidência do relatório) |
|---|---|---|
| O1 (`godot.md`) | C6 7* | Sem benchmark 5v5+bots `[NV]`; proxies documentais: headless, Strip Visuals, ALWAYS/ON_CHANGE, `max_sync_packet_size=1350`. |
| O2 (`bevy.md`) | C2 4*, C3 7*, C4 4*, C6 8* | C2: headless real (`ScheduleRunnerPlugin`), mas sem validação/snapshots/reconexão nativos. C3: ECS (entidades/componentes) e BSN (notação de cenas), sem loader `.bsn` de primeira parte nem hash no handshake. C4: sem editor oficial; BSN/Feathers/observers exigem código. C6: 55 mil entidades/60 fps, custo 5v5+bots `[NV]`. |
| O3 (`bevy.md`) | C2 9*, C3 7*, C4 7*, C5 5*, C6 8*, C8 7*, C9 8*, C10 6* | C2: autoritativo, netcode.io, inputs por tick, replicação/interesse; reconexão robusta ausente (`bevy_replicon_repair` desmantido) `[NV]`. C3: protocolo registrado em Rust, versionamento/hash cabe à aplicação. C4: predição/rollback e interpolação cobrem a janela de 5 s; sem editor. C5: book do Lightyear WIP e matriz de versões rígida. C6: benches Criterion, custo/tick `[NV]`. C8: projeto concentrado, 4 versões por ciclo do Bevy. C9: wasm via WebTransport `[NV]`. C10: acrescenta `aeronet`, `postcard`, `bevy_replicon`, Leafwing. |
| O4 (`defold_flax_stride.md`) | C2 5*, C6 6* | C2: `dmengine_headless` real, porém autoridade/snapshots manuais. C6: consumo conservador, sem benchmark `[NV]`. |
| O5 (`defold_flax_stride.md`) | C2 7*, C6 6* | C2: headless e replicação/RPC/autoridade oficiais, sem handshake de conteúdo. C6: C++ moderno e `-null`, sem benchmark `[NV]`. |
| O6 (`defold_flax_stride.md`) | C2 6*, C6 6*, C7 5* | C2: headless demonstrado em amostra da comunidade; netcode é terceiro/DIY. C6: Vulkan/D3D12 e Native AOT, sem benchmark `[NV]`. C7: GLB com texturas embarcadas falha na issue #1666 aberta (31/10/2026). |
| O7 (`libgdx_urho_panda.md`) | C6 7* | Proxy documental, sem benchmark local `[NV]`. |
| O8 (`libgdx_urho_panda.md`) | C6 7* | Melhor performance esperada em C++, sem benchmark 5v5 `[NV]`. |
| O9 (`libgdx_urho_panda.md`) | C6 6* | Python pode pesar no tick do servidor, mitigável em C++; sem benchmark `[NV]`. |
| O10 (`monogame_love_haxe.md`) | C2 4*, C6 5* | C2: issue #7121 (runtime headless) aberta desde 2020; sem rede embutida. C6: proxy, sem benchmark `[NV]`. Notas da coluna MonoGame; FNA é pior (C2 3*, C3 4, C4 4, C5 3, C7 4, C10 3). |
| O11 (`monogame_love_haxe.md`) | C6 4* | Natureza 2D (incompatível com as ilhas 3D low-poly); proxy sem benchmark `[NV]`. |
| O12 (`monogame_love_haxe.md`) | C6 5* | Proxy, sem benchmark `[NV]`; compilador Haxe GPL afeta a ferramenta, não o código gerado. |
| O13 (`alternativas.md`) | C2 5*, C6 6*, C7 5* | C2: tokio headless, mas sem rede; snapshots do zero. C6: eficiente, sem benchmark `[NV]`. C7: glTF por crate de terceiro `[NV]`. |

## 4. Verificação aritmética (conferência de cada conta)

Pesos na ordem C1…C10 = 15, 15, 12, 10, 10, 8, 8, 8, 7, 7.

- O1: (9×15 + 8×15 + 8×12 + 8×10 + 9×10 + 7×8 + 9×8 + 9×8 + 8×7 + 7×7)/10 = (135+120+96+80+90+56+72+72+56+49)/10 = 826/10 = **82.6**
- O3: (10×15 + 9×15 + 7×12 + 7×10 + 5×10 + 8×8 + 8×8 + 7×8 + 8×7 + 6×7)/10 = (150+135+84+70+50+64+64+56+56+42)/10 = 771/10 = **77.1**
- O9: (9×15 + 8×15 + 8×12 + 6×10 + 8×10 + 6×8 + 8×8 + 7×8 + 7×7 + 6×7)/10 = (135+120+96+60+80+48+64+56+49+42)/10 = 750/10 = **75.0**
- O7: (9×15 + 6×15 + 6×12 + 5×10 + 8×10 + 7×8 + 7×8 + 9×8 + 8×7 + 8×7)/10 = (135+90+72+50+80+56+56+72+56+56)/10 = 723/10 = **72.3**
- O6: (10×15 + 6×15 + 7×12 + 6×10 + 7×10 + 6×8 + 5×8 + 8×8 + 7×7 + 8×7)/10 = (150+90+84+60+70+48+40+64+49+56)/10 = 711/10 = **71.1**
- O2: (10×15 + 4×15 + 7×12 + 4×10 + 6×10 + 8×8 + 8×8 + 8×8 + 8×7 + 6×7)/10 = (150+60+84+40+60+64+64+64+56+42)/10 = 684/10 = **68.4**
- O4: (7×15 + 5×15 + 6×12 + 6×10 + 8×10 + 6×8 + 8×8 + 7×8 + 9×7 + 8×7)/10 = (105+75+72+60+80+48+64+56+63+56)/10 = 679/10 = **67.9**
- O8: (8×15 + 8×15 + 7×12 + 7×10 + 5×10 + 7×8 + 7×8 + 3×8 + 6×7 + 4×7)/10 = (120+120+84+70+50+56+56+24+42+28)/10 = 650/10 = **65.0**
- O5: (4×15 + 7×15 + 6×12 + 7×10 + 6×10 + 6×8 + 8×8 + 8×8 + 8×7 + 4×7)/10 = (60+105+72+70+60+48+64+64+56+28)/10 = 627/10 = **62.7**
- O10: (8×15 + 4×15 + 4×12 + 5×10 + 6×10 + 5×8 + 6×8 + 9×8 + 7×7 + 7×7)/10 = (120+60+48+50+60+40+48+72+49+49)/10 = 596/10 = **59.6**
- O13: (9×15 + 5×15 + 3×12 + 3×10 + 5×10 + 6×8 + 5×8 + 5×8 + 8×7 + 5×7)/10 = (135+75+36+30+50+48+40+40+56+35)/10 = 545/10 = **54.5**
- O12: (8×15 + 6×15 + 4×12 + 3×10 + 5×10 + 5×8 + 3×8 + 6×8 + 8×7 + 4×7)/10 = (120+90+48+30+50+40+24+48+56+28)/10 = 534/10 = **53.4**
- O11: (8×15 + 5×15 + 5×12 + 2×10 + 9×10 + 4×8 + 2×8 + 6×8 + 5×7 + 4×7)/10 = (120+75+60+20+90+32+16+48+35+28)/10 = 524/10 = **52.4**

Conferência cruzada: os totais de O4 (67,9), O5 (62,7), O6 (71,1), O10 (59,6), O11 (52,4) e
O12 (53,4) reproduzem os índices já publicados nos relatórios de origem.

## 5. Trava inviolável

Nota < 5 em C1, C2 ou C3 **impede adoção na Fase 0**, independentemente do score:

| Opção | Nota inviolável | Motivo | Fase 0 |
|---|---|---|---|
| O2 Bevy 0.19.1 | C2 = 4* | Sem validação/snapshots/reconexão nativos; a solução é O3 (com Lightyear). | REJECT |
| O5 Flax Engine | C1 = 4 | EULA proprietária, usuário único, royalty de 4%, foro na Polônia — fere o Pilar I. | REJECT |
| O10 MonoGame / FNA | C2 = 4*, C3 = 4 | Headless e netcode inexistentes no core; content pipeline é build-time. | REJECT |
| O12 Haxe / OpenFL | C3 = 4 | Sem recarga declarativa do Núcleo; 2D/Stage3D e compilador GPL. | REJECT |
| O13 macroquad + tokio | C3 = 3 | Sem loader declarativo; exige recompilar; reportado como "EXCLUÍDA como engine-base". | REJECT |

Portanto **O2, O5, O10, O12 e O13 não podem ser adotadas na Fase 0**, ainda que O2 tenha score
superior a O4/O8 e O13 superior a O11/O12. O4 (C2=5), O11 (C2=5, C3=5) não são travadas por esta
regra; seus rótulos REJECT vêm de licença não-OSI e ausência de autoridade (O4) e
incompatibilidade 2D com as ilhas 3D (O11). Rótulos provisórios alinhados ao relatório de
recomendação; as opções auxiliares F1/F2/F3 de `alternativas.md` não integram a matriz canônica.

## 6. Sensibilidade — C4 e C5 em dobro, C6 e C7 pela metade

Novos pesos: C1=15, C2=15, C3=12, **C4=20, C5=20, C6=4, C7=4**, C8=8, C9=7, C10=7.
A nova soma é **112** (não 100). Para manter a comparação com a matriz original, apresento o
score bruto `Σ(nota×peso)/10` e o **score normalizado** `Σ(nota×peso)×10/112`, que volta à escala
0–100. A ordenação é idêntica nos dois critérios.

| Opção | Somatório com novos pesos | Bruto | Normalizado (÷112×10) | Original | Δ |
|---|---|---:|---:|---:|---:|
| O1 Godot 4.7.2 | (9×15+8×15+8×12+8×20+9×20+7×4+9×4+9×8+8×7+7×7) = 932 | 93.2 | **83.2** | 82.6 | +0.6 |
| O2 Bevy 0.19.1 | (10×15+4×15+7×12+4×20+6×20+8×4+8×4+8×8+8×7+6×7) = 720 | 72.0 | **64.3** | 68.4 | −4.1 |
| O3 Bevy + Lightyear | (10×15+9×15+7×12+7×20+5×20+8×4+8×4+7×8+8×7+6×7) = 827 | 82.7 | **73.8** | 77.1 | −3.3 |

**Efeito na ordem** (normalizado, todas as 13 opções recalculadas):
O1 83.2 → O9 74.5 → O3 73.8 → O6 71.2 = O7 71.2 (empate) → O4 68.1 → O2 64.3 →
O8 63.8 → O5 62.6 → O10 59.1 → O11 54.5 → O12 52.0 → O13 51.9.

- O1 **permanece em 1º** (C4=8 e C5=9 altos); a liderança é insensível ao cenário.
- O3 **cai de 2º para 3º**: O9 (C4=6, C5=8) sobe para 2º com 74.5, enquanto O3 sofre com C5=5*
  (book do Lightyear WIP) e perde o ganho relativo que tinha em C7 (8→peso 4).
- O2 **cai de 6º para 7º** e a distância para O3 aumenta de 8,7 para 9,5 pontos: penalizado por
  C4=4* (sem editor) e C5=6, mas com C6=8* desvalorizado. A trava de C2=4* permanece.
- O11 sobe de 13º para 11º (C5=9) e O13 cai de 11º para 13º (C4=3, C5=5): a troca de pesos
  recompensa prototipagem rápida e puni a ausência de editor, sem alterar o topo Godot-first.
