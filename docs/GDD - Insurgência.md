# Insurgência — Game Design Document (GDD — Documento de Projeto de Jogo)

| Campo | Valor |
| :-- | :-- |
| **Projeto** | Insurgência — framework livre para criar MOBAs (Arenas de Batalha Multijogador Online) |
| **Versão** | 1.0 |
| **Data da primeira redação** | 2026-11-19 |
| **Estado** | Documento vivo — derivado de duas conversas de pesquisa e verificado contra fontes primárias |
| **Fontes canônicas** | `docs/Esporte vs. E-sport_ Controle das Regras.md`; `docs/Jogos MOBA Open Source e Frameworks.md` |
| **Idioma** | Português brasileiro. Termos estrangeiros são explicados entre parênteses na primeira ocorrência |
| **Escopo** | Design do jogo base 5v5 e do framework (Núcleo + Módulos); viabilidade técnica e governança |

## Índice

1. Resumo Executivo
2. Contexto e Fundamentação: Esporte, E-sport e Soberania das Regras
3. Promessa ao Jogador, Público-alvo e Posicionamento
4. Pilares de Design
5. Fantasia do Jogador, Tom e Identidade
6. Conceito Central e Condição de Vitória
7. Loop Central e Loop de Sessão
8. O Mapa: Arquipélago, Rotas e Ilhas
9. Sistemas de Jogo: Desafios de Ilha, Súditos, Recursos
10. Funções e Habilidades
11. Economia, Progressão e Balanceamento
12. Controles, Interface e Experiência do Jogador
13. Direção de Arte e Áudio
14. Arquitetura Técnica e Viabilidade
15. Governança, Comunidade e Ecossistema
16. MVP (Produto Mínimo Viável), Escopo e Roadmap
17. Métricas de Sucesso
18. Riscos e Mitigações
19. Fora do Escopo
20. Suposições, Hipóteses e Perguntas Abertas
21. Referências e Glossário

---
## 1. Resumo Executivo

**Data de redação:** 2026-11-19. **Estado:** documento vivo, derivado de duas conversas de pesquisa. As decisões aqui registradas são reconduzidas a cada nova sessão de projeto; as seções seguintes são extensão deste raciocínio, não revisão dele.

Insurgência não é um jogo isolado. É um framework (estrutura de software reutilizável) FOSS (Free and Open Source Software — Software Livre e de Código Aberto) para criar MOBAs (Multiplayer Online Battle Arena — Arena de Batalha Multijogador Online). O produto distribuído é um protocolo de regras implementado num Núcleo (motor de regras) e estendido por Módulos (pacotes de conteúdo). Qualquer pessoa pode baixar, compilar, hospedar e alterar o jogo sem autorização de terceiros. Onde o modelo proprietário oferece um serviço, Insurgência oferece uma infraestrutura comum.

O Arquipélago da Insurgência é o jogo base: uma arena 5v5 de estratégia assimétrica. A vitória não se obtém por destruição da base adversária, mas por estabelecer a Autogestão na Ilha Central: controlar os meios de produção e a hegemonia ideológica do Arquipélago da Insurgência. Para isso, a equipe percorre rotas de ilhas, resolve Desafios de Ilha em vez de derrubar torres, organiza a logística com Súditos (substitutos dos minions, tropas tradicionais do gênero) e coordena funções — Intelectual, Artesão, Guerreiro, Camponês, Agitador e Aliança Mercantil — cujas habilidades são ações materiais, não magia.

**Tese central — soberania das regras:** a diferença entre esporte e e-sport não é física versus mental, nem tecnologia versus tradição. É a resposta a uma pergunta: quem controla as regras e o espaço de jogo? Futebol e xadrez são bens comuns culturais; suas regras pertencem a quem as pratica. League of Legends é software licenciado; suas regras, servidores e código pertencem a uma corporação. Insurgência existe para que uma prática competitiva volte a ser domínio de quem a joga.

**Problema que resolve:** hoje, a prática competitiva depende de infraestrutura privada e propriedade intelectual restritiva. O jogador aceita um EULA (Contrato de Licença de Usuário Final) para entrar num círculo mágico pré-fabricado; o acesso é privilégio revogável; as regras mudam por decisão unilateral; se o servidor desliga, o jogo deixa de existir. Insurgência substitui esse arranjo por código aberto, servidores federados e governança comunitária, sem perder a densidade estratégica que tornou o gênero relevante.

**Critérios de verificação da tese:**
- Uma pessoa baixa o Núcleo, compila e disputa uma partida em servidor próprio sem aceitar contrato de licença e sem autenticar em serviço central.
- Uma comunidade altera um Desafio de Ilha e distribui o Módulo resultante sem autorização do projeto original.
- O desligamento de qualquer servidor, inclusive o de referência, não interrompe a existência do jogo em outros servidores.

[NOTA PARA O DESIGNER: a tese é critério de corte. Toda mecânica proposta nas seções 6 a 21 que reintroduza permissão central, conta vinculada a empresa ou regra imutável por decisão unilateral deve ser rejeitada ou sinalizada.]

**Descrição curta (fonte 1, 08/11/2025)**

> Insurgência é um framework de código aberto (FOSS — Software Livre e de Código Aberto) para a criação de jogos MOBA (Arenas de Batalha Multijogador Online), permitindo que comunidades criem e hospedem suas próprias versões. O jogo base é uma arena 5v5 com uma abordagem decolonial e materialista: não há magia, apenas ação coletiva. Em vez de torres, os jogadores conquistam ilhas ao superar desafios de lógica, sabotagem e organização. A vitória exige estratégia e cooperação entre funções como Artesãos, Intelectuais e Camponeses, não apenas poder de combate.

## 2. Contexto e Fundamentação: Esporte, E-sport e Soberania das Regras

### 2.1 Bens comuns culturais e software licenciado

As regras do futebol e do xadrez são um *common* (bem comum) cultural: na prática, pertencem a quem as joga. FIFA e FIDE regulam competições de alto nível, mas a FIDE apenas recomenda as regras para partidas não ranqueadas — não as impõe, e não há licença para jogar xadrez. Nada impede duas pessoas de jogarem num parque ou crianças de improvisarem uma pelada com chinelos como traves. A prática é descentralizada e resiliente: sobrevive à falência de qualquer federação. [A afirmação de que as regras são "domínio público" é inferência jurídica a validar — ver 2.8.]

League of Legends, Valorant e World of Warcraft não são práticas nesse sentido. São software, serviço licenciado. Código-fonte, balanceamento e servidores são propriedade intelectual de uma corporação. Jogar exige permissão; organizar um campeonato exige autorização; criar uma variante exige que a empresa a tolere.

### 2.2 O círculo mágico alugado

Huizinga (*Homo Ludens*, 1938) definiu o jogo como atividade livre, "não séria" mas absorvente, sem interesse material ou lucro, cercada por limites próprios de tempo e espaço, segundo regras fixas e com agrupamentos que se cercam de segredo. Ele enumerou vários *play-grounds* (lugares de jogo: arena, mesa, círculo mágico, templo, palco, tela, quadra, tribunal) como "mundos temporários dentro do mundo ordinário". A leitura do círculo como fronteira do espaço de jogo foi desenvolvida por Salen e Zimmerman (*Rules of Play*, 2004) — Zimmerman admitiu terem "mais ou menos inventado o conceito". No futebol de rua, os jogadores criam esse limite por contrato social temporário: "três cantos é pênalti". O limite existe porque eles o inventam e podem desinventá-lo.

No League of Legends, o jogador aceita um EULA (Contrato de Licença de Usuário Final) para *alugar* — metáfora analítica deste documento, não doutrina de Huizinga — o direito de entrar num espaço de jogo pré-fabricado, desenhado e controlado pela Riot Games. O jogador não cria as regras; consentem-nas. A liberdade e a voluntariedade que Huizinga atribuía ao jogo são, aqui, delegadas a uma empresa. A atividade deixa de ser livre no sentido original e passa a ser serviço ou bem de consumo: o "e-sport" é menos "esporte" (prática cultural) e mais "e" (tecnologia como produto).

### 2.3 Mutabilidade, acesso e natureza do serviço

Três consequências decorrem do círculo alugado e devem ser tratadas como o problema a resolver, não como efeitos colaterais:

- **Mutabilidade por patch (atualização unilateral de regras):** a Riot pode alterar fundamentalmente o jogo, em escala global, por decisão unilateral e a qualquer momento. Mudanças nas regras do futebol, como o VAR (árbitro de vídeo), são processos culturais e burocráticos lentos, debatidos e negociados. No serviço proprietário, a regra muda sem deliberação de quem a pratica.
- **Acesso como privilégio revogável:** entrar não é direito, é concessão. As licenças de Chess.com e Riot autorizam encerramento "com ou sem causa, sem aviso" e "a qualquer tempo, por qualquer motivo ou nenhum" (URLs em 2.8). A empresa pode revogar o acesso de qualquer jogador, organização ou país e pode fazê-lo cessar, desligando os servidores — como em *The Crew* (Ubisoft, 2014–2024, 12 milhões de jogadores). Não há recurso: o contrato é de adesão.
- **Natureza orientada a receita de acionistas:** o jogo não existe para ser jogado; existe para gerar receita. A experiência do jogador é secundária ao modelo de negócios. Balanceamento, calendário e conteúdo são decididos conforme o retorno financeiro, não conforme a saúde da prática.

Quando a prática vira propriedade intelectual, o que se perde não é o pedido de permissão em si — é a capacidade de a comunidade adaptar, preservar e transmitir o jogo. Um esporte tradicional acumula variações como memória viva; um serviço proprietário acumula versões descartadas por decisão de produto.

### 2.4 O limite da definição: xadrez não é e-sport — por escolha de critério

Sob o critério adotado por este projeto — soberania sobre regras e infraestrutura — o xadrez não é e-sport; essa é uma escolha de critério, não um consenso. A literatura corrente define e-sport como competição videogame estruturada e o debate se concentra em fisicalidade, não em propriedade das regras; a Lei Geral do Esporte brasileira (14.597/2023) chegou a excluir o "desporto virtual". É, portanto, posição contestada, não fato.

Chess.com é empresa privada que fornece uma plataforma para *jogar* xadrez; não controla as regras do xadrez. Se falir e desligar os servidores amanhã, a prática migra para o tabuleiro físico ou para outro servidor. Ser banido do Chess.com não é ser banido do xadrez. O Lichess (servidor de xadrez de código aberto, associação caritativa francesa sob a lei de 1901, código AGPL-3.0 — licença livre de copyleft em rede) reforça a resiliência da prática, mas **não é prova cabal**: seus termos reservam encerramento de conta "por qualquer motivo, sem aviso" e afirmam que "nossa palavra é final". O software livre garante o fork (bifurcação do código), não a governança da instância hospedada.

League of Legends é produto que *é* o jogo; Chess.com é produto para *jogar* o jogo. O que decide a classificação adotada aqui é a soberania sobre as regras e a infraestrutura, não o suporte físico ou mental. O Champions Chess Tour *parece* e-sport; o jogo em si não pertence a ninguém.

### 2.5 Precedente histórico: o gênero nasceu como mod

A proposta de Insurgência não é inédita; é a formalização de um processo que o gênero já inventou acidentalmente dentro de um jogo proprietário:

- DotA original foi um mod (modificação) criado por Eul dentro de Warcraft III, usando o Editor de Mapas. O anfitrião escolhia qual versão do mapa carregar (por exemplo, DotA Allstars 5.84c); quem entrava "baixava" e validava as regras daquele mapa. Guinsoo e, depois, IceFrog bifurcaram o mapa, acrescentando heróis, itens e regras.
- Counter-Strike foi mod de Half-Life.

Em ambos os casos, o "círculo alugado" (Warcraft III, Half-Life) foi usado para abrigar um "círculo criado" (DotA, Counter-Strike). O que Insurgência faz é tornar esse direito a característica principal e protegê-lo por licença, arquitetura e governança, em vez de depender da tolerância de um editora. O modelo é o de um protocolo — como o e-mail: o Gmail pode cair, o e-mail não deixa de existir — e não o de um produto polido e centralizado.

### 2.6 Tabela comparativa

| Dimensão | Esporte tradicional (futebol, xadrez) | E-sport proprietário (League of Legends) | Modelo proposto (Insurgência) |
| :--- | :--- | :--- | :--- |
| Natureza do jogo | Prática cultural, bem comum | Software licenciado, serviço | Protocolo livre, framework FOSS |
| Tipologia (Caillois) | *agon* (competição: xadrez, futebol, MOBA) | *ludus* (cálculo e regras) dominante | *agon* nos Desafios de Ilha; *alea* (azar) em caixas de saque; *paidia* (modificações) e *ludus* (patch oficial) como polos de um contínuo |
| Propriedade das regras | Tratadas como bem comum (inferência jurídica a validar); FIDE recomenda, não impõe | Propriedade intelectual da corporação | Do bem comum: versionadas em repositório aberto, sob soberania da comunidade |
| Entrada no espaço de jogo (metáfora do "círculo alugado") | Criada por contrato social entre participantes | Alugada mediante EULA (Contrato de Licença de Usuário Final) | Criada por quem hospeda; nada é alugado |
| Mudança de regra | Processo cultural e burocrático lento | Patch (atualização) unilateral e global | Proposta pública, debate e aprovação comunitária; divergência via Módulo |
| Acesso | Direito decorrente da prática | Privilégio revogável (banimento, bloqueio geográfico) | Autonomia: servidor próprio ou comunitário |
| Persistência | Existe mesmo sem federação ou plataforma | Cessa quando o servidor desliga | Persiste em servidores federados; queda de um não encerra o jogo |
| Finalidade | Prática, sociabilidade, competição | Receita para acionistas | Autogestão da prática; financiamento sem apropriação |
| Variação e torneios | Livres; qualquer um organiza | Dependentes de autorização da empresa | Livres; ligas oficiais e ligas com regras próprias |
| Riscos estruturais | Disputa cultural sobre a regra | Concentração, dependência, revogação | Fragmentação da base de jogadores e trapaça em cliente aberto |

### 2.7 Critérios de verificação da fundamentação

- Nenhuma regra de Insurgência depende de aceitação de contrato de adesão para ser praticada em servidor privado.
- Uma partida continua disputável após o desligamento do servidor de referência, desde que exista outro servidor com o mesmo Módulo.
- Toda mudança de regra no Arquipélago da Insurgência é acompanhada de registro público da decisão e de mecanismo para a comunidade divergir sem precisar copiar o Núcleo inteiro.

[NOTA PARA O DESIGNER: fragmentação e trapaça são riscos reconhecidos na fonte e devem ser tratados nas seções 14, 15 e 18. Não disfarçá-los como decididos: liberdade de bifurcar é faca de dois gumes — uma comunidade dividida em dez versões pode não ter jogadores suficientes para pareamento rápido.]

### 2.8 Notas de precisão teórica

Registro das correções feitas sobre as fontes de origem, para que a fundamentação não repita imprecisões. São notas de precisão, não nova tese: a soberania sobre regras e infraestrutura permanece a premissa de design.

- **Huizinga (1938).** A definição literal de *Homo Ludens* inclui atividade livre, "não séria" mas absorvente, sem interesse material ou lucro, dentro de limites próprios de tempo e espaço, segundo regras fixas, com agrupamentos que se cercam de segredo. A fonte original registra apenas "livre e voluntária" e omite esses elementos. Edição brasileira: *Homo Ludens: O Jogo como Elemento da Cultura*, Perspectiva, tradução de João Paulo Monteiro, revisão de Newton Cunha, ISBN 9788527311571. https://archive.org/stream/homoludensstudyo1950huiz/homoludensstudyo1950huiz_djvu.txt · https://editoraperspectiva.com.br/produtos/homo-ludens-nova-edicao-revisada/
- **"Círculo mágico": atribuição correta.** Huizinga lista *play-grounds* (arena, mesa, círculo mágico, templo, palco, tela, quadra, tribunal) como "mundos temporários dentro do mundo ordinário"; o termo aparece apenas seis vezes na tradução inglesa. A leitura do círculo como fronteira do espaço de jogo é de Salen e Zimmerman (*Rules of Play*, 2004), que Zimmerman admitiu terem "mais ou menos inventado o conceito". Chamar "exata" a aplicação do conceito de Huizinga é impreciso. https://todigra.org/index.php/todigra/article/download/1701/1701
- **"Alugar o círculo mágico".** Metáfora analítica adotada neste documento para descrever acesso licenciado a um espaço de jogo pré-fabricado; não é doutrina de Huizinga e não deve ser lida como tal.
- **Caillois (1958) como complemento.** *Les jeux et les hommes* propõe seis caracteres (livre, separado, incerto, improdutivo, regrado, fictício) e quatro categorias: *agon* (competição: xadrez, futebol, MOBA), *alea* (azar), *mimicry* (simulação) e *ilinx* (vertigem). *Paidia* (espontaneidade) e *ludus* (cálculo e regras) são polos de um contínuo dentro de cada categoria, não categorias separadas. A distinção separa o *agon* dos Desafios de Ilha da *alea* das caixas de saque (*loot boxes*) e a *paidia* dos mods (modificações) do *ludus* do patch (atualização) oficial. https://creativegames.org.uk/modules/Intro_Game_Studies/Caillois_Man_Play_Games_Chapters1_2-2001.pdf
- **Contratos de licença (EULA — Contrato de Licença de Usuário Final).** Chess.com: licença "limitada, não exclusiva, revogável", encerramento "com ou sem causa, sem aviso, sem apelação". Riot: término "a qualquer tempo, por qualquer motivo ou nenhum" e possibilidade de "parar de fornecer acesso". https://www.chess.com/legal/user-agreement · https://raw.communitydragon.org/9.5/plugins/rcp-be-lol-license-agreement/riot/en_us/eula.txt
- **Lichess não é prova cabal.** É associação caritativa francesa (lei de 1901) com código AGPL-3.0, mas seus termos reservam encerramento de conta "por qualquer motivo, sem aviso" e declaram que "nossa palavra é final". O FOSS (software livre e de código aberto) garante o fork (bifurcação do código), não a governança da instância hospedada. https://lichess.org/terms-of-service
- **"Xadrez não é e-sport" é posição contestada.** A literatura define e-sport como competição videogame estruturada, e o debate incide sobre fisicalidade, não sobre propriedade das regras; a Lei Geral do Esporte (14.597/2023) chegou a excluir o "desporto virtual". Formulação correta: sob o critério deste projeto — soberania sobre regras e infraestrutura — o xadrez não é e-sport; escolha de critério, não consenso. https://link.springer.com/article/10.1007/s12662-025-01054-9 · http://www.planalto.gov.br/ccivil_03/_ato2023-2026/2023/lei/l14597.htm
- **"Regras são domínio público" é inferência a validar.** O direito autoral protege a expressão, não regras abstratas; não há fonte primária decisiva. A FIDE recomenda as regras para partidas não ranqueadas, não as impõe, e não existe licença para jogar xadrez. https://handbook.fide.com/chapter/E012023
- **Encerramentos documentados.** *The Crew* (Ubisoft, 2014–2024) foi desligada com 12 milhões de jogadores; a campanha *Stop Killing Games* reuniu cerca de 1,3 milhão de assinaturas à Comissão Europeia e teve audiência no Parlamento Europeu em abril de 2026. https://www.bbc.com/news/articles/c8e8e7g0r82o

## 3. Promessa ao Jogador, Público-alvo e Posicionamento

### 3.1 Promessa ao jogador

**"Você não aluga o círculo mágico — você o constrói."**

A promessa é de posse, não de consume. O jogador recebe as ferramentas para produzir o espaço de jogo, alterar suas regras e transmiti-las. Onde o serviço proprietário pergunta se ele aceita as condições, Insurgência pergunta o que ele quer que a condição seja. A promessa vale para a partida (a comunidade decide o que é vitória e como se chega a ela) e para o ecossistema (a comunidade decide qual versão do jogo disputa).

### 3.2 Público-alvo

- **Pessoas com base em engenharia de software** que nunca desenvolveram um jogo, mas conseguem ler um repositório e compreender uma arquitetura cliente-servidor. É o público que a fonte original nomeia: quem tem fundamentos de engenharia, não estúdio orçamentário. O design deve ser legível a esse público — cenas e módulos com responsabilidades claras, não mágica de engine.
- **Comunidades que querem autonomia sobre as regras:** grupos que discordam de um balanceamento, querem um "mod clássico", um servidor regional, uma liga amadora ou um torneio com regras próprias, e que hoje só podem esperar que a empresa decida.
- **Jogadores cansados de serviços proprietários:** pessoas que já foram banidas, já viram um servidor encerrar ou já viram seu personagem mudar por decisão unilateral, e que preferem um jogo imperfeito e seu para um serviço polido e alheio.

[SUPOSIÇÃO: o público inicial é a interseção desses três conjuntos, não sua união. Isso implica um primeiro produto mais próximo de uma ferramenta compreensível do que de um jogo acabado; a validação dessa suposição cabe nas métricas da seção 17.]

### 3.3 Posicionamento frente a referências

| Referência | O que representa | Relação com Insurgência |
| :--- | :--- | :--- |
| League of Legends | Serviço proprietário de MOBA; círculo mágico alugado, regra mutável por patch, acesso revogável | Antítese explícita. Insurgência preserva a densidade estratégica do gênero e rejeita o modelo de posse, de servidor único e de receita como finalidade |
| Paragon | MOBA cancelado cujos assets (recursos de arte, animação e áudio) foram liberados pela Epic Games | Referência de *assets* apenas como substitutos temporários. A estética de fantasia tecnológica e armaduras brilhantes contradiz o tom materialista; usar esqueletos e colisão, não a aparência |
| Kenney.nl | Biblioteca FOSS de recursos visuais de baixa poligonalidade | Fonte preferencial de arte para protótipo: leve, licenciada e coerente com a proposta. Complementa o greyboxing (protótipo com formas geométricas sem textura), não substitui a decisão de estética final |
| Codebases (bases de código) de referência (MOBA CSharp Unity, Prime World, OpenRA, Veloren) | Código e infraestrutura de MOBAs e jogos em rede já existentes | Estudo de netcode (código de rede), servidor autoritativo e gerenciamento de estado; a lógica de jogo tradicional é descartada por ser engessada em torno de súditos em linha reta, torres e base a destruir |

O posicionamento não é "um MOBA mais barato". É uma mudança de categoria: de produto para protocolo, de serviço para prática, de plateia para participantes. Um jogador de LoL é cliente; um jogador de Insurgência é mantenedor em potência.

### 3.4 Síntese "X meets Y"

> **Insurgência é DotA como mod de Warcraft III, mas como protocolo livre — a criação coletiva de regras que o gênero inventou por acidente, tornada princípio de projeto.**

Formulação alternativa, para uso em comunicação: **xadrez, mas com Desafios de Ilha** — a soberania do domínio público aplicada a uma arena 5v5.

### 3.5 Critérios de verificação do posicionamento

- A descrição pública do projeto declara a condição de vitória como Autogestão na Ilha Central, e não destruição da base adversária.
- Nenhum material de apresentação usa armaduras, heróis fantásticos ou linguagem de "campeão" como imagem do jogo.
- O protótipo inicial roda com formas geométricas sem textura ou recursos de baixa poligonalidade; nenhum asset de Paragon aparece como representação final de personagem.

## 4. Pilares de Design

Os quatro pilares abaixo são regras operatórias, não slogans. Cada um declara o que habilita, o que proíbe, como se verifica e qual mecânica o concretiza. Mecânica que não serve a nenhum pilar é cortada ou sinalizada explicitamente.

### 4.1 Pilar I — Soberania do Jogador

- **Enunciado:** o jogo pertence a quem o joga. Regras, servidores e conteúdo são possíveis de inspecionar, modificar e redistribuir sem pedir permissão a autoridade central.
- **Habilita:** servidores comunitários e federados; Módulos com regras, personagens e mapas próprios; divergência por bifurcação (`fork`, bifurcação do código) ou por pacote de configuração; ligas amadoras com regras adaptadas.
- **Proíbe:** EULA (Contrato de Licença de Usuário Final) de adesão; servidor único como ponto de falha; patch (atualização) unilateral; conta obrigatória vinculada a empresa; regra que só pode ser mudada por quem não a pratica.
- **Critério de verificação:** dois servidores com Módulos diferentes coexistem e são alcançáveis por qualquer jogador sem autorização do Núcleo; a remoção do servidor de referência não impede novas partidas; a mudança de uma regra é possível por edição de arquivo de configuração do servidor, sem recompilar o Núcleo.
- **Mecânica que concretiza:** a separação entre Núcleo (motor de regras) e Módulos (pacotes de conteúdo), com carregamento de regras por servidor no início da partida.

### 4.2 Pilar II — Materialismo Histórico sem Sobrenatural

- **Enunciado:** não há magia no mundo de Insurgência. Toda habilidade é uma ação material sobre o terreno, a produção ou as relações sociais, com causa e consequência observáveis. A economia é a base de todo o poder.
- **Habilita:** construir barricadas e pontes; sabotar máquinas; espalhar panfletos que convertem Súditos; organizar uma greve que desativa defesas; negociar com trabalhadores; revelar pontos fracos por análise.
- **Proíbe:** feitiços, invocação supernatural, cura sem causa material, "poder divino”, heróis com origem mítica, abates como única fonte de poder.
- **Critério de verificação:** cada habilidade do elenco é descrita por uma ação física ou social e por um efeito mensurável em jogo; nenhuma habilidade resolve um Desafio de Ilha por intervenção inexplicada; nenhuma linha de texto do jogo atribui eventos a deuses, magia ou destino.
- **Mecânica que concretiza:** os Desafios de Ilha (quebra-cabeça de lógica, sincronia de entrada, coleta e sabotagem) e os recursos — Materiais para o Artesão e Suprimentos para o Camponês — como fundamento da progressão.

### 4.3 Pilar III — Luta Decolonial e Assimétrica

- **Enunciado:** o conflito padrão não é império contra império. É uma Comuna autônoma contra uma força opressora — Corporação Colonial ou Estado burocrático — em um mapa que é ele próprio hostil. A vitória é emancipação, não aniquilação.
- **Habilita:** oposição assimétrica PVE+PVP (jogador contra ambiente e jogador contra jogador); rotas que testam capacidades distintas; a Ilha Central como tomada dos meios de produção; disputa ideológica por influência sobre Súditos e defensores.
- **Proíbe:** duas facções simétricas com bases idênticas; "conquistar" como sinônimo de explodir; retratar a opressão como estética decorativa; qualquer cenário em que o lado opressor seja apenas uma skin (aparência) do oprimido.
- **Critério de verificação:** a condição de vitória da partida padrão é ativar a Autogestão na Ilha Central, não destruir a base inimiga; a força opressora possui defesas e regras diferentes das da Comuna; nenhuma ilha temática é representada como território vazio a ser "limpo".
- **Mecânica que concretiza:** a progressão por cadeias de ilhas — Rota Superior / Ilhas da Autossuficiência, Rota Central / Ilhas da Ação Direta e Rota Inferior / Delta do Comércio — cada uma com seu Desafio de Ilha, culminando na Ilha Central.

### 4.4 Pilar IV — Ação Coletiva sobre Brilho Individual

- **Enunciado:** reflexos rápidos são subordinados à estratégia coletiva. Nenhum jogador vence sozinho; a vitória exige coordenação entre funções com capacidades complementares, inclusive situações em que a cooperação é limitada por penalidade deliberada.
- **Habilita:** entrada sincronizada de três jogadores em cinco segundos; preparação prévia do terreno por um Agitador ou Intelectual; defesa de ponto enquanto outro jogador canaliza; escolta logística; interceptação de rotas de coleta.
- **Proíbe:** design que recompense apenas o abate individual; "carry" (jogador que sozinho decide a partida) como plano de jogo; habilidade que substitua a necessidade de coordenar; vitória por soma de desempenhos independentes.
- **Critério de verificação:** ao menos um Desafio de Ilha da rota central é impossível de iniciar por um único jogador; a penalidade de grupo na Rota Superior aumenta a dificuldade quando mais aliados entram, tornando a ajuda uma decisão custosa; a vitória requer funções distintas atuando em janelas coincidentes.
- **Mecânica que concretiza:** o Desafio de Ilha sincronizado da Rota Central e a penalidade de grupo da Rota Superior / Ilhas da Autossuficiência.

[NOTA PARA O DESIGNER: os quatro pilares podem entrar em tensão. Quando entrarem, a ordem de precedência é I → III → II → IV: preservar a soberania, depois a assimetria decolonial, depois a materialidade, depois a coordenação. Toda resolução de conflito deve ser registrada como decisão de design, não como exceção silenciosa.]

### 4.5 Rastreabilidade: fantasia → pilares → loop → mecânicas

| Fantasia do jogador | Pilar | Loop (visão geral; detalhamento na seção 7) | Mecânica que fecha o ciclo |
| :--- | :--- | :--- | :--- |
| Organizar uma Comuna autônoma | I — Soberania do Jogador | Escolher servidor e Módulo → entrar na partida → sair com regras adaptadas | Núcleo + Módulos; servidores federados |
| Agir sobre o mundo sem magia | II — Materialismo Histórico | Coletar Materiais e Suprimentos → agir → transformar o terreno | Coleta, construção, sabotagem, conversão de Súditos |
| Enfrentar uma força opressora | III — Luta Decolonial e Assimétrica | Libertar ilhas nas rotas → pressionar a Ilha Central → estabelecer Autogestão | Desafios de Ilha; condição de vitória por ativação da Ilha Central |
| Confiar no aliado que entra na ilha | IV — Ação Coletiva | Preparar terreno → sincronizar entrada → resolver juntos → repetir | Janela de 5 s para três jogadores; penalidade de grupo na Rota Superior |

**Critérios de verificação da cadeia:** toda mecânica listada nas seções 6 a 21 declara o pilar que atende; nenhuma mecânica aparece sem conexão com pelo menos um pilar; nenhum pilar fica sem ao menos uma mecânica verificável na partida.

## 5. Fantasia do Jogador, Tom e Identidade

### 5.1 O que o jogador é

O jogador é um organizador de uma Comuna autônoma. Não é um campeão, não é um herói escolhido, não é um "jogador profissional" em busca de troféu. Sua competência não é acertar mais tiros: é saber o que a comunidade precisa naquele momento — um Artesão ergue a barricada, um Intelectual prepara o terreno, um Camponês sustenta a coleta, um Agitador desativa a defesa, um Guerreiro protege quem canaliza, uma Aliança Mercantil abre um caminho. A fantasia é a de quem participa de uma construção coletiva sob pressão, não a de quem brilha sozinho diante de uma plateia.

### 5.2 O que o jogador sente

- **Agência coletiva:** a sensação de que o que a equipe faz altera o mundo do mapa de forma visível — uma ilha muda de mãos, um ponto de coleta passa a render, uma defesa cai porque alguém preparou o terreno antes.
- **Tensão da confiança:** na Rota Superior / Ilhas da Autossuficiência, a ajuda de um aliado dobra as forças de segurança. O jogador solitário precisa decidir se confia no companheiro que entra em sua ilha; o companheiro precisa decidir se atrapalha para ajudar. Na Rota Central / Ilhas da Ação Direta, a confiança é literal: sem três entradas na janela de cinco segundos, todos são repelidos. A coordenação é vivida como risco, não como senão decorativo.
- **Pertencimento sem pertence:** o jogador sabe que pode levar as regras consigo para outro servidor. Isso produz segurança material, não nostalgia de marca.

### 5.3 Tom

Materialista, sóbrio, sem heroísmo fantástico e sem militarismo vazio. O texto do jogo nomeia ações e consequências; não exalta sacrifício, não promete glória, não transforma opressão em cenário pitoresco. A iconicidade vem do gesto concreto — sabotar, negociar, organizar, defender — e não de títulos pomposos. A fonte de humor, quando houver, é a burocracia e a logística, não a caricatura do adversário. Nada de "lenda", "destino", "poder ancestral" ou "glória eterna".

### 5.4 Identidade visual e narrativa

A identidade é ancorada nas ilhas temáticas, não num elenco de personagens carismáticos:

- **Rota Superior / Ilhas da Autossuficiência:** ilhas densas, selvagens ou isoladas — floresta, montanha —, com aqueduto, alavancas, quebra-cabeça de lógica e forças de segurança. Tom de solidão vigiada.
- **Rota Central / Ilhas da Ação Direta:** ilhas industrializadas e urbanas — fábrica, cidade —, com maquinário, gás lacrimogêneo, greve e ocupação. Tom de tensão coletiva.
- **Rota Inferior / Delta do Comércio:** delta e pântano, rede de ilhas pequenas sem caminho linear, postos de coleta e rotas negociadas com ilhas neutras. Tom de logística e interceptação.
- **Ilha Central:** o ponto de chegada; não é ruína sagrada nem trono, é o lugar onde a Autogestão é estabelecida. Tom de decisão.

[SUPOSIÇÃO: a narrativa é ambiental — contada pelo estado das ilhas e pelos objetos presentes, não por cutscenes (cenas de narrativa gravadas) ou diálogo longo. A decisão de implementar ou não cutscenes cabe na seção 13.]

### 5.5 Citações de tom (texto do jogo)

> "A Ilha Industrial não cai. Ela é ocupada. Três de vocês, agora — ou nenhum."

> "O aqueduto não foi defendido por um deus. Foi quebrado por mãos, e mãos vão consertá-lo. Outro aliado aqui só levantaria o alarme."

[NOTA PARA O DESIGNER: as duas citações acima são amostras de tom, não texto definitivo de interface. Mantê-las curtas, materiais e sem heroísmo; evitar qualquer termo de fantasia épica na revisão de texto da seção 13.]
## 6. Conceito Central e Condição de Vitória

### 6.1 A assimetria fundante

**Insurgência** é um MOBA (Arena de Batalha Multijogador Online) de código aberto jogado em equipes de cinco combatentes. A assimetria não é tema decorativo: a configuração padrão opõe uma **Comuna** Autônoma (Assembleia Popular) à Sede da Corporação Colonial ou ao Palácio do Governo Opressor. O mapa em si é hostil — forças de segurança e animais selvagens são PNJs (personagens não-jogadores) que defendem o território. Daí o PVE+PVP: PVE (jogador contra ambiente) na disputa das ilhas e PVP (jogador contra jogador) quando uma segunda Comuna reivindica as mesmas ilhas.

[HIPÓTESE] O Arquipélago da Insurgência tem 12 ilhas: 3 na Rota Superior, 3 na Rota Central, 5 na Rota Inferior e 1 Ilha Central. Justificativa: três ilhas por cadeia linear reproduzem o ritmo "preparar → ocupar → greve" das fontes; cinco ilhas no delta sustentam uma rede não linear sem pulverizar a defesa; um único objetivo preserva a leitura do mapa em servidores comunitários de baixa latência.

**Pilar atendido:** Luta Decolonial (o opressor é sistêmico, não um espelho da Comuna) e Materialismo Histórico (o território disputado é meio de produção).

### 6.2 Vitória: autogestão, não demolição

A condição de vitória é **estabelecer a autogestão na Ilha Central**. A ilha não é explodida; é convertida. A Sede da Corporação pode permanecer de pé ao fim da partida.

[HIPÓTESE] A Ilha Central possui 3 Nós de Controle — produção, comunicação e segurança — e 100 pontos de Resistência Ideológica. Cada Nó convertido reduz 30 pontos; o pulso de autogestão seguinte reduz os 10 restantes e encerra a partida com a mensagem "Autogestão estabelecida". Dano direto não converte Nó e reduz no máximo 20 pontos da barra, nunca abaixo de 10 enquanto restar Nó não convertido; os 30 pontos de cada Nó só caem por Propaganda canalizada de 20 s. Justificativa: o teto de dano impede que a vitória degrade para corrida de destruição e obriga a presença do Intelectual; três Nós espelham a tríade Artesão–Guerreiro–Intelectual da Greve (§8.3).

[HIPÓTESE] Derrota: a Comuna é dissolvida se o relógio de 20 min se esgotar sem a conversão ou se a Corporação reconquistar 3 ilhas já libertadas. Justificativa: 20 min cabem numa sessão única e o limite de reconquistas puniu a expansão sem defesa, não a lentidão.

**Variante duas Comunas.** O módulo base mantém o objetivo único: converter a Ilha Central antes da Comuna adversária, sem atacar sua base. Derrotar a Comuna rival por destruição é regra opcional de servidor (fork, bifurcação do código), nunca padrão — preserva a tese de que o conflito padrão é assimétrico.

**Critérios de aceitação — condição de vitória**
- Zerar os pontos de um Nó com armas o deixa inativo por 20 s, mas não altera sua contribuição de 30 pontos na Resistência Ideológica; somente Propaganda canalizada remove essa contribuição.
- A partida termina no primeiro pulso em que a Resistência chega a 0, com a Ilha Central intacta e nenhuma estrutura da Sede destruída.
- Na variante duas Comunas, ambas podem converter os mesmos 3 Nós; a primeira a zerar a Resistência vence sem causar dano à Comuna rival.
- Reconquistar 3 ilhas libertadas encerra a partida com "Comuna dissolvida", mesmo que a Resistência da Ilha Central já esteja abaixo de 50.

### 6.3 Rastreabilidade do conceito

| Mecânica | Pilar atendido | Por quê |
| :-- | :-- | :-- |
| Converter em vez de destruir | Luta Decolonial; Materialismo Histórico | Nega a lógica de "destruir o Nexus" e afirma a tomada dos meios de produção. |
| Teto de 20 pontos de dano na Ilha Central | Ação Coletiva | Obriga Intelectual e funções de apoio a participarem do ato final. |
| Variante duas Comunas por fork | Soberania do Jogador | O servidor escolhe o módulo de vitória; a comunidade altera a regra sem pedir permissão. |

## 7. Loop Central e Loop de Sessão

### 7.1 Pulso de 30 segundos

O Núcleo (motor de regras) emite um **pulso de autogestão a cada 30 s**: renda de pontos neutros, surgimento de Súditos e conversão passiva são resolvidos no mesmo instante. [HIPÓTESE] 30 s; justificativa: é o intervalo curto o bastante para dar retorno visível a cada decisão e longo o bastante para que um quebra-cabeça de 3 peças e uma canalização de 6 s caibam no mesmo compasso. Em equipe, as fases se sobrepõem, mas o pulso continua sendo a unidade de recompensa.

| Fase | Ação (função) | Decisão do jogador | Recompensa no pulso | Tempo alvo |
| :-- | :-- | :-- | :-- | :-- |
| 1. Coletar | Camponês em ponto neutro | ficar e arriscar emboscada ou recuar | 3 Suprimentos + 2 Materiais | 6 s |
| 2. Preparar | Intelectual ou Agitador canaliza Organizar Célula | proteger a canalização ou abandoná-la | defesas da ilha seguinte −25% por 45 s | 6 s |
| 3. Entrar | 3 aliados pisam na ilha | entrar já ou esperar o terceiro | desafio destravado; entrada escalonada expulsa o grupo | janela de 5 s |
| 4. Defender | Guerreiro na barricada, Artesão reforça | segurar o ponto ou perseguir a emboscada | +1 ponto de autogestão a cada 10 s de barricada intacta | 8 s |
| 5. Converter | Intelectual panfleteia; Súditos param de atacar | converter ou abater o defensor | 1 Súdito convertido e −5 de Resistência da ilha | 6 s |

**Critérios de aceitação — loop de 30 s**
- Um Camponês que permanece 6 s em ponto neutro recebe exatamente 3 Suprimentos e 2 Materiais no pulso; sair antes zera o acúmulo parcial.
- Três aliados que entram em 4,9 s destravam o desafio; entrando em 5,2 s, todos são repelidos por gás lacrimogêneo e nenhum progresso é registrado.
- Uma canalização interrompida por 1 s de dano reinicia os 6 s de Preparar e não concede o redutor de 25%.

**Pilar atendido:** Ação Coletiva e Materialismo Histórico (toda recompensa vem de coleta, preparação ou conversão — nunca de abate isolado).

### 7.2 Loop de partida: da cadeia de ilhas à Ilha Central

| Fase | Intervalo | Objetivo | Número verificável |
| :-- | :-- | :-- | :-- |
| Embarque | 0–2 min | escolher 5 funções e definir rotas | 5 funções distintas por Comuna |
| Disputa das rotas | 2–14 min | libertar ilhas nas três rotas, em paralelo | Superior 3 ilhas × 3 min; Central 3 × 3 min; Inferior 5 × 2 min |
| Convergência | 14–17 min | alcançar e preparar a Ilha Central | Plano Mestre do Intelectual: 3 s de canalização |
| Conversão final | 17–20 min | converter os 3 Nós | 20 s de propaganda por Nó + 1 pulso |

[HIPÓTESE] Duração alvo de 20 min por partida. Justificativa: abaixo de 15 min a rede de 5 ilhas do Delta do Comércio não completa sua malha logística; acima de 30 min aumenta o custo de formar sala em servidores federados, onde a população é menor que a de um serviço centralizado. O Plano Mestre (habilidade definitiva do Intelectual) revela a estratégia de defesa da ilha e enfraquece permanentemente seus defensores PNJs, conforme as fontes.

**Critérios de aceitação — loop de partida**
- Duas equipes de cinco completam a conversão final entre 18 e 22 min em 9 de 10 partidas automatizadas.
- A Ilha Central só aceita o Plano Mestre após 2 ilhas convertidas em cada rota — 6 no total.
- Nenhuma função pode ser escolhida duas vezes na mesma Comuna no módulo base; a repetição só é habilitada por fork.

### 7.3 Loop de meta: forks, ligas e gestão comunitária

A partida alimenta a assembleia: resultados de conversão, funções mais usadas e desafios repetidos viram proposta pública. A Fundação sem fins lucrativos mantém a Liga base (regras do Núcleo sem modificações); ligas regionais e amadoras usam as regras oficiais ou as próprias. Se um servidor adotar regras impopulares, a comunidade migra para outro fork levando personagens e balanceamentos consigo.

[HIPÓTESE] Toda mudança segue 7 dias de debate aberto e 21 dias em liga experimental antes de entrar na Liga base; cada servidor mantém no máximo 3 conjuntos de regras ativos. Justificativa: 28 dias dão tempo suficiente para medir o efeito em pelo menos 4 ciclos semanais de torneio sem travar a inovação.

[SUPOSIÇÃO] A Fundação terá moderadores voluntários para arbitrar os 21 dias de teste em cada servidor; sem eles, o prazo vira apenas formalidade e a validação perde valor.

O servidor é autoritativo: valida 100% das ações e aceita apenas comandos dos clientes, o que sustenta a confiança das ligas mesmo com o código aberto.

**Critérios de aceitação — loop de meta**
- Uma regra aprovada em servidor custom não altera a Liga base antes de 28 dias decorridos (7 de debate + 21 de teste).
- O servidor rejeita qualquer efeito calculado no cliente; somente comandos do jogador são aceitos como entrada.
- Um fork troca o elenco e o balanceamento sem modificar o Núcleo, que continua carregando módulos de mapa e de regras como arquivos separados.

**Pilar atendido:** Soberania do Jogador (o círculo mágico é mantido pela comunidade, não alugado de uma corporação).

## 8. O Mapa: Arquipélago da Insurgência, Rotas e Ilhas

### 8.1 Topologia e tabela de rotas

As rotas não são caminhos retos: são cadeias de ilhas que precisam ser libertadas. **Convenção de números:** valores de efetivo, dano e raio ausentes nas fontes são `[HIPÓTESE]`; cada tabela é seguida da justificativa correspondente.

| Rota | Nome canônico | Ilhas | Regra de entrada | Penalidade central | Funções | Pilar |
| :-- | :-- | :-- | :-- | :-- | :-- | :-- |
| Superior | Ilhas da Autossuficiência | 3 | desafio solo; 1 jogador por ilha | aliado adicional dobra as forças de segurança (4 → 8); emboscada (*gank*, ataque surpresa a alvo isolado) é decisão tática | qualquer função isolada | Materialismo Histórico; Ação Coletiva via emboscada |
| Central | Ilhas da Ação Direta | 3 | Ilha 1: preparação interrompível; Ilhas 2 e 3: 3 jogadores em janela de 5 s | gás lacrimogêneo (dano contínuo) e expulsão se a entrada for escalonada | Agitador/Intelectual, Artesão, Guerreiro | Ação Coletiva |
| Inferior | Delta do Comércio | 5 | rede não linear; entrada livre | interceptação de caravanas e envenenamento de postos | Camponês, Aliança Mercantil, Agitador, Artesão, Guerreiro | Materialismo Histórico; Soberania do Jogador |

[HIPÓTESE] 4 forças de segurança por ilha da Rota Superior e 8 dmg/s por 3 s de gás lacrimogêneo na Rota Central. Justificativa: 4 guardas permitem que um jogador sozinho resolva o quebra-cabeça em ~90 s sem se tornar um duelo de tiros; 8 dmg/s por 3 s totaliza 24 pontos — doloroso, mas não letal para um combatente com 100 pontos de vida, preservando a expulsão como punição e não como morte.

**Decisão de design (2026-11-19).** A valoração canônica da penalidade de grupo na Rota Superior é **×2 (dobra)**: um aliado adicional dobra as forças de segurança de 4 para 8, conforme o briefing (regra de jogo). O valor `DificuldadeEvento = Dificuldade * 5` do roteiro de protótipo fica registrado como **variante de estresse do protótipo de caixas** — teste de robustez do sistema, não regra da liga vanilla. Justificativa: a dobra preserva a previsibilidade para o jogador solo, que lê a penalidade e decide entre ajudar ou emboscar; o ×5 destruiria essa leitura ao converter 4 guardas em 20 e inviabilizar a resolução do quebra-cabeça.

[NOTA PARA O DESIGNER] Os nomes Delta-1 a Delta-5 são rótulos funcionais; o módulo de mapa pode substituí-los por nomes definitivos sem alterar qualquer regra.

### 8.2 Rota Superior — Ilhas da Autossuficiência

Ilhas densas, selvagens ou isoladas. O desafio é de raciocínio, coleta e sobrevivência. A penalidade de grupo ativa um alarme: a presença de qualquer aliado além do primeiro dobra o efetivo de 4 para 8 guardas, sem segundo dobramento. A estratégia não é ajudar o aliado a terminar mais rápido — é emboscar o adversário enquanto ele está sozinho, distraído com o quebra-cabeça e sob fogo dos PNJs.

[HIPÓTESE] Janela de vulnerabilidade de 6 s após cada erro de quebra-cabeça; justificativa: tempo suficiente para um emboscador chegar de uma ilha adjacente e curto o bastante para punir a distração.

| Ilha | Temática | Tipo de desafio | Pré-requisito | Recompensa | Penalidade |
| :-- | :-- | :-- | :-- | :-- | :-- |
| Ilha da Floresta | madeira, alimentos, abrigos, animais selvagens | Restaurar o Aqueduto: 3 peças em raio de 120 m, ordem lógica, instalação sob ataque animal | nenhum | +4 Materiais, +3 Suprimentos, 1 ponto de coleta | aliado adicional → 8 forças de segurança |
| Ilha da Montanha | minério, isolamento, neblina | 3 alavancas na ordem correta | Floresta convertida | +6 Materiais | erro reinicia as 3 alavancas e atrai +2 guardas |
| Ilha do Vale [HIPÓTESE] | passagem estreita, vento | sobrevivência de 45 s enquanto coleta 2 peças | Montanha convertida | +5 Materiais, +2 Ouro | 2 aliados → 8 guardas e as peças reaparecem em novas posições |

**Critérios de aceitação — Rota Superior**
- Um único jogador na Floresta enfrenta 4 forças de segurança com multiplicador de dificuldade 1×; ao entrar um segundo aliado, o efetivo sobe para 8 em menos de 2 s e o multiplicador passa a 2× — nunca 5×, reservado ao protótipo de caixas.
- Erro na ordem das alavancas da Montanha reinicia as 3 e adiciona 2 guardas; nenhuma peça permanece instalada.
- Um adversário atacado nos 6 s seguintes a um erro sofre +50% de dano do emboscador; fora da janela, o bônus não existe.

### 8.3 Rota Central — Ilhas da Ação Direta

Ilhas industrializadas e urbanas. A Ilha 1 exige preparação prévia: um Agitador (também chamado Sabotador nas fontes) ou o Intelectual canaliza Organizar Célula por 6 s; a canalização é interrompível e enfraquece as defesas das ilhas seguintes. A Propaganda (W do Intelectual) não prepara ilha: converte Súditos e reduz visão. Nas Ilhas 2 e 3, três jogadores aliados devem entrar simultaneamente em janela de 5 s; entrando escalonados, são repelidos por gás lacrimogêneo e expulsos. A Greve exige Artesão sabotando, Guerreiro defendendo e Intelectual negociando ao mesmo tempo.

| Ilha | Temática | Tipo de desafio | Pré-requisito | Recompensa | Penalidade |
| :-- | :-- | :-- | :-- | :-- | :-- |
| Ilha da Cidade (Ilha 1) | barricadas, produção de armas, forças policiais | Preparar o Terreno: canalização interrompível de 6 s | nenhum | defesas das Ilhas 2 e 3 −25% por 45 s | interrupção reinicia; 3 interrupções → +2 policiais na Ilha 2 |
| Ilha Industrial — A Ocupação da Fábrica (Ilha 2) | tomar a produção | 3 aliados em 5 s + capturar 2 postos de produção | Ilha da Cidade preparada | +8 Materiais por minuto de produção operária | entrada escalonada → 8 dmg/s por 3 s e expulsão |
| Ilha Industrial — A Greve (Ilha 3) | sabotar máquinas, propaganda, negociação | Artesão Sabotar Maquinário (8 s), Guerreiro Defender Barricada (10 s), Intelectual Negocia com os Trabalhadores (3 perguntas) | Ilha 2 tomada + 3 aliados em 5 s | 2 acertos: +1 Súdito propagandista; 3 acertos: também −50% de produção por 90 s | ≤1 acerto na negociação → +2 policiais e nenhum bônus |

**Critérios de aceitação — Rota Central**
- Três aliados que pisam na Ilha 2 em 4,8 s iniciam a Ocupação; em 5,3 s, todos são repelidos por 8 dmg/s durante 3 s e nenhum posto é capturado.
- Na Greve, Sabotar Maquinário só avança se o Guerreiro mantiver a barricada por 10 s; sem ela, a canalização é interrompida e reinicia do zero.
- A Negociação apresenta 3 perguntas com 8 s cada; 2 acertos convertem 1 propagandista, 3 acertos acrescentam os −50% de produção por 90 s.

### 8.4 Rota Inferior — Delta do Comércio

Rede não linear de pequenas ilhas em delta de rio e pântano. O sucesso não é matar, mas estrangular a economia inimiga. O Camponês coleta, a Aliança Mercantil abre rotas temporárias com ilhas neutras (PNJs), o Agitador intercepta e envenena, o Artesão constrói ponte provisória para emboscada e o Guerreiro escolta.

| Ilha | Temática | Tipo de desafio | Pré-requisito | Recompensa | Penalidade |
| :-- | :-- | :-- | :-- | :-- | :-- |
| Delta-1 (Ilha do Mangue) | coleta de alimentos | Camponês Coleta Eficiente em posto neutro | nenhum | 3 Suprimentos a cada 6 s (renda passiva) | posto envenenado rende 0 por 20 s |
| Delta-2 (Ilha da Foz) | comércio com ilha neutra | Aliança Mercantil Estabelece Rotas Comerciais | Delta-1 | nova rota temporária por 60 s | expira se ninguém pagar 2 Ouro; atalho ligado fecha junto |
| Delta-3 (Ilha da Barca) | logística e travessia | Artesão constrói Ponte Provisória | Delta-2 | atalho de 8 s entre ilhas por 45 s | ponte destrutível com 60 PV |
| Delta-4 (Ilha do Dique) | interceptação | Agitador Intercepta caravana ou Envenena posto | Delta-1 | −5 Suprimentos inimigos, +2 Ouro | Agitador revelado no mapa por 6 s |
| Delta-5 (Ilha da Confluência) | estrangulamento econômico | 2 rotas mercantis ativas + escolta do Guerreiro | 2 rotas ativas | pulso de renda em dobro por 30 s | perder a escolta custa 6 Suprimentos |

**Critérios de aceitação — Rota Inferior**
- Um Camponês em Delta-1 gera 3 Suprimentos a cada 6 s sem combate; um posto envenenado rende 0 por 20 s mesmo com o Camponês presente.
- A rota aberta pela Aliança Mercantil expira em 60 s se não receber 2 Ouro; ao expirar, a ponte de Delta-3 também deixa de existir.
- Interceptar rende 2 Ouro e subtrai 5 Suprimentos do adversário, mas revela o Agitador no mapa por 6 s.

### 8.5 Ilha Central e exemplos de desafios

| Ilha | Temática | Tipo de desafio | Pré-requisito | Recompensa | Penalidade |
| :-- | :-- | :-- | :-- | :-- | :-- |
| Ilha Central | meios de produção e hegemonia ideológica | converter 3 Nós de Controle; Resistência Ideológica 100 → 0 | 2 ilhas por rota (6) + Plano Mestre | vitória por autogestão | dano não converte Nó; 3 ilhas reconquistadas = derrota |

**Ilha da Floresta — Restaurar o Aqueduto.** Três peças (bomba de madeira, filtro de carvão, roda d'água) estão espalhadas em raio de 120 m; a instalação só funciona na ordem bomba → filtro → roda. Quatro animais selvagens atacam durante a busca. Recompensa: +4 Materiais, +3 Suprimentos e um ponto de coleta. É o arquétipo de desafio de raciocínio e sobrevivência da Rota Superior.

**Ilha da Cidade — Organizar Célula.** A canalização de 6 s ocorre numa praça patrulhada por 3 forças policiais em ciclo de 20 s. O Artesão pode bloquear 2 saídas com barricadas de 200 PV; sem esse bloqueio, a patrulha interrompe o Intelectual em 4 de 5 tentativas. A produção de armas da Corporação na ilha só cai quando a Greve (Ilha 3) é concluída.

**Ilha Industrial — A Greve.** Três ações simultâneas: o Artesão sabota o maquinário por 8 s, o Guerreiro defende a barricada por 10 s e o Intelectual negocia com os trabalhadores por 3 perguntas de 8 s. Negociação bem-sucedida transforma PNJs resistentes em propagandistas; sabotagem bem-sucedida reduz a produção inimiga em 50% por 90 s.

**Negociação com PNJs.** Trabalhadores de ilha respondem a perguntas de lógica e materialismo histórico; 2 acertos bastam para virar um propagandista. O mesmo Intelectual não pode negociar com o mesmo PNJ por 60 s, regra que impede a exploração repetida do mesmo diálogo.

**Critérios de aceitação — Ilha Central**
- A Ilha Central rejeita o Plano Mestre enquanto menos de 2 ilhas de cada rota estiverem convertidas.
- Converter os 3 Nós consome 60 s acumulados de Propaganda e reduz exatamente 90 pontos da Resistência; o pulso seguinte aplica os 10 finais.
- Um Nó atingido apenas por dano permanece contabilizado na Resistência após seus 20 s de inatividade.

## 9. Sistemas de Jogo: Desafios de Ilha, Súditos, Recursos

### 9.1 Desafios de Ilha (substituto das torres)

Não há torres que disparam projéteis. Cada ilha roda seu próprio Desafio de Ilha como microssistema com estado próprio; ao concluí-lo, emite um sinal (evento) para o servidor central, que é a única autoridade a registrar a conversão. [HIPÓTESE] O servidor descarta sinais duplicados da mesma ilha em intervalo de 2 s; justificativa: elimina ganho de renda por repetição acidental sem exigir reconciliação complexa de estado.

| Estado | Condição de entrada | Sinal emitido | Feedback visível |
| :-- | :-- | :-- | :-- |
| Latente | ilha ainda não alcançada | `ilha_bloqueada` | névoa, PNJs neutros |
| Preparado | canalização de 6 s concluída | `terreno_preparado` | bandeira da Comuna, defesas −25% |
| Ativo | regra de entrada satisfeita (1 jogador na Superior; 3 em 5 s na Central) | `desafio_iniciado` | barra de progresso |
| Contestado | dano inimigo ou interrupção | `contestacao` | indicador piscando |
| Convertido | objetivo do desafio cumprido | `ilha_convertida` | cores da Comuna, Súditos panfletando |
| Reconvertido | Corporação retoma a ilha | `ilha_perdida` | indicador da Corporação |

**Critérios de aceitação — Desafios de Ilha**
- Uma ilha emite no máximo um sinal `ilha_convertida`; um segundo sinal da mesma ilha em 2 s é descartado pelo servidor.
- Nenhuma torre, projétil automático ou estrutura destrutível para fins de vitória existe no módulo base.
- Um sinal `contestacao` interrompe a barra de progresso, mas não apaga o progresso já feito na canalização.

**Pilar atendido:** Ação Coletiva e Soberania do Jogador (ilhas são cenas independentes que qualquer fork pode substituir).

### 9.2 Súditos

Os Súditos não são soldados irracionais: são grupos de trabalhadores, propagandistas e milicianos que avançam pelas rotas representando o avanço material e ideológico da Comuna. Um Intelectual próximo pode inspirá-los: eles param de atacar estruturas e passam a panfletar, convertendo passivamente as defesas da ilha.

[HIPÓTESE] A cada pulso de 30 s, cada ilha convertida libera 2 Súditos (1 trabalhador e 1 propagandista); ilhas da Rota Central liberam 1 miliciano adicional. Justificativa: dois por pulso mantêm pressão visível sem exigir mobilização de entidades em massa; o miliciano extra compensa o gargalo da entrada sincronizada. Propaganda do Intelectual tem área de 6 m, exige 4 s de exposição e converte até 3 Súditos inimigos; convertidos conservam a função original e permanecem assim por 60 s.

**Critérios de aceitação — Súditos**
- Um Súdito inimigo exposto 4 s à Propaganda para de atacar estruturas e passa a panfletar; se levar dano de jogador, reverte em 3 s.
- Um trabalhador convertido continua gerando renda, agora para a Comuna, durante 60 s; após isso volta à condição neutra.
- Uma ilha convertida libera exatamente 2 Súditos por pulso, ou 3 se pertencer à Rota Central.

**Pilar atendido:** Materialismo Histórico e Ação Coletiva (a influência avança pelo trabalho e pela persuasão, não por exércitos automáticos).

### 9.3 Recursos

| Recurso | Fonte | Uso principal | Valor de referência |
| :-- | :-- | :-- | :-- |
| Ouro | abate de jogador (12) e de Súdito inimigo (12); interceptação (2) | negociação, rotas mercantis, propaganda | +12 / +12 / +2 |
| Materiais | pontos neutros; Floresta (+4), Montanha (+6) | barricada (20), ponte provisória (15), sabotagem | +2 por pulso por ponto |
| Suprimentos | pontos neutros; Camponês (+3 por 6 s) | milícia camponesa (20), manutenção de caravana | +3 por pulso por ponto |

[HIPÓTESE] 18 pontos de coleta neutros no mapa completo — 6 na Rota Superior (3 ilhas × 2), 10 no Delta do Comércio (5 ilhas × 2) e 2 na Ilha Central. Cada ponto tem 30 pontos de vida e rende 3 unidades a cada 6 s enquanto um aliado estiver presente no raio de 4 m. Atributos distintos: 30 PV e 60 s de respawn medem a vida e a indisponibilidade do ponto; o rendimento é 3 unidades por 6 s (15 por minuto) por aliado presente. Justificativa: a densidade acompanha as ilhas produtivas e mantém o ciclo de 30 s legível sem exigir que o jogador percorra longas distâncias entre pulsos.

**Critérios de aceitação — Recursos**
- Um ponto neutro rende a cada 6 s somente com aliado em 4 m; dois aliados no mesmo ponto não duplicam a renda.
- Destruir um ponto (30 PV de vida) interrompe seu rendimento (3 unidades a cada 6 s) por 60 s de respawn, após o que ele reaparece neutro.
- Uma barricada custa exatamente 20 Materiais e uma ponte provisória 15; sem saldo, a ação é recusada sem consumo parcial.

**Pilar atendido:** Materialismo Histórico (a economia precede e condiciona toda ação militar).

### 9.4 Controle de ilha e avanço

Controlar uma ilha exige duas condições: superar o Desafio de Ilha e manter ao menos 1 Súdito da Comuna na ilha por 10 s contínuos. Sem o Súdito, a ilha volta a Latente e o sinal `ilha_perdida` é emitido.

| Situação | Pré-requisito | Efeito |
| :-- | :-- | :-- |
| Rota Superior | desafio solo concluído | destrava a próxima ilha da cadeia |
| Rota Central | Ilha 1 preparada | destrava Ilha 2; Ilha 2 tomada destrava a Greve |
| Rota Inferior | qualquer rota mercantil ativa | destrava ilhas vizinhas na rede, em qualquer ordem |
| Ilha Central | 2 ilhas por rota + Plano Mestre | habilita a conversão dos 3 Nós |

[HIPÓTESE] A Corporação reconverte uma ilha desprotegida em 20 s; justificativa: iguala o tempo de inatividade do Nó e dá ao jogador uma janela real para retornar e defender.

**Critérios de aceitação — controle e avanço**
- Uma ilha cujo desafio foi concluído, mas sem Súdito da Comuna por 10 s, retorna a Latente e emite `ilha_perdida`.
- Na Rota Inferior, converter Delta-4 antes de Delta-2 é válido desde que Delta-1 esteja sob controle.
- A Ilha Central permanece bloqueada com 5 ilhas convertidas mesmo que todas sejam da Rota Superior e da Central.
## 10. Funções e Habilidades

As funções são papéis materiais na insurgência, não classes mágicas: cada habilidade modifica o mundo físico ou as relações sociais, nunca conjura energia (P2). O Núcleo expõe a interface de habilidade; os Módulos definem os conjuntos Q/W/E/R e podem alterar seus valores sem bifurcar (fazer *fork*) o código. Este documento fixa o conjunto base do Módulo "Heróis da Comuna".

**Legenda de pilares** (usada nas marcações de rastreabilidade):

- **P1 — Soberania do Jogador:** regras controladas pela comunidade, servidores federados, Módulos configuráveis.
- **P2 — Materialismo Histórico:** ações materiais, sem magia; a economia é a base de todo poder.
- **P3 — Luta Decolonial:** a Comuna contra a força opressora; conflito assimétrico.
- **P4 — Ação Coletiva:** o brilho individual é subordinado à estratégia coletiva; ninguém vence sozinho.

### 10.1 Convenções de leitura e recurso comum

Toda habilidade consome **Fôlego**, reserva física que se regenera por tempo — não é mana (energia mágica) nem recurso sobrenatural. Os valores abaixo são chutes de design e se aplicam a todas as funções.

| Parâmetro | Valor | Justificativa / marcação |
|---|---|---|
| Fôlego máximo | 100 | Permite 3 a 4 habilidades seguidas antes da exaustão. `[HIPÓTESE]` |
| Regeneração | 10/s fora de combate; 5/s em combate | Punir o gasto em combate sem travar o jogador. `[HIPÓTESE]` |
| Alcance contato | 2 m | Corpo a corpo. `[HIPÓTESE]` |
| Alcance curto | 6 m | Exige aproximação, mas permite reação. `[HIPÓTESE]` |
| Alcance médio | 12 m | Faixa de habilidades de área e marcação. `[HIPÓTESE]` |
| Alcance de ilha | 25 m | Atinge o Desafio de Ilha sem sair da margem. `[HIPÓTESE]` |
| Desbloqueio | Q no nível 1, W no 2, E no 3, R no 6 | A Ultimate (R) coroa a fase inicial de progressão. `[HIPÓTESE]` |
| Teto de nível | 12 | Mantém a curva curta o suficiente para partidas de 20 a 30 min. `[HIPÓTESE]` |

**Aura** significa área circular contínua centrada no personagem. **Canalização** significa que a ação exige permanência imóvel e é cancelada por dano ou por silêncio (bloqueio de habilidades). **Silêncio** impede usar habilidades, mas não ataques básicos nem itens. **NPCs** são personagens controlados pelo servidor, como defensores de ilha e Súditos.

**Decisão de design (2026-11-19).** A preparação prévia da ilha na Rota Central / Ilhas da Ação Direta passa a se chamar sempre **Organizar Célula** (nome também presente nas fontes); a habilidade W do Intelectual permanece **Propaganda**. Organizar Célula é um estado do Desafio de Ilha ativado ao canalizar E (Agitador) na ilha preparatória, com **6 s de canalização interrompível**, reduzindo as defesas da ilha seguinte em **−25% por 45 s** — não uma quinta habilidade e nunca pela W do Intelectual. Distinção: **Organizar Célula = preparação do cenário da ilha (6 s, −25% por 45 s); Propaganda = habilidade do Intelectual que converte Súditos e reduz visão, sem preparar ilhas**.

---

### 10.2 Intelectual (Organizador)

*Função:* estrategista, influenciador, apoio que converte e revela. *Pilares:* P2, P4.

**Q — Discurso Inspirador.** Concede moral (velocidade de ataque e movimento temporária) aos aliados e aos Súditos próximos. **Custo:** 25 Fôlego; **execução:** 0,2 s; **alcance:** 8 m (aura); **recarga:** 8 s. **Efeito:** +15% de velocidade de ataque e +20% de velocidade de movimento por 4 s. **Desafio de Ilha:** Súditos sob a aura largam o ataque a estruturas e passam a panfletar, iniciando a conversão passiva da defesa da ilha.

**Critérios de aceitação:**
- Três aliados dentro de 8 m recebem os dois bônus por exatamente 4 s.
- Súditos na aura mudam o comportamento de ataque para panfletagem e reduzem a integridade da defesa da ilha em 5%/s enquanto panfletam.
- Um segundo Discurso Inspirador na mesma janela não acumula os bônus (aplica-se o maior valor).

**W — Propaganda.** Lança panfletos numa área; inimigos jogadores perdem visão (confusão) e Súditos inimigos podem parar de atacar e se converter. **Custo:** 30 Fôlego; **execução:** 0,5 s de arremesso; **alcance:** 12 m, área de 4 m de raio; **recarga:** 12 s. **Efeito:** -30% de raio de visão por 3 s nos jogadores na área; cada Súdito inimigo na área tem 40% de chance de ficar neutro por 5 s. **Desafio de Ilha:** converte Súditos inimigos em panfletadores e reduz a pressão sobre a defesa da ilha; **não prepara a ilha** — a preparação pertence a Organizar Célula (6 s de canalização, −25% por 45 s), executada por E do Agitador.

**Critérios de aceitação:**
- Jogadores na área perdem 30% do raio de visão por 3 s; fora da área não há efeito.
- Ao menos um Súdito inimigo na área muda de facção quando o teste de 40% passa.
- Propaganda não aplica enfraquecimento de ilha: a redução de defesas da ilha seguinte só advém de Organizar Célula, nunca deste efeito.

**E — Análise Materialista.** Revela um ponto fraco num inimigo ou estrutura e reduz sua defesa temporariamente. **Custo:** 20 Fôlego; **execução:** 0,3 s; **alcance:** 12 m, alvo único; **recarga:** 15 s. **Efeito:** -25% de resistência a dano por 5 s; estruturas marcadas ficam visíveis através de terreno por 10 s. **Desafio de Ilha:** marca a máquina do desafio e aumenta em 25% o dano da Sabotagem Estrutural do Artesão.

**Critérios de aceitação:**
- O alvo marcado sofre 25% de dano extra por 5 s; o valor é idêntico para jogadores, NPCs e estruturas.
- Uma estrutura marcada permanece visível ao time por 10 s mesmo após sair da linha de visão.
- Marcas de dois Intelectuais no mesmo alvo não se acumulam.

**R — Plano Mestre (Ultimate).** Canaliza por **3 s (valor canônico da fonte)** para revelar a estratégia de defesa da ilha e enfraquecer permanentemente seus defensores (NPCs). **Custo:** 40 Fôlego; **execução:** canalização de 3 s interrompível; **alcance:** ilha alvo a 25 m; **recarga:** 90 s. `[HIPÓTESE]` na recarga e nos valores abaixo. **Efeito:** revela torres, armadilhas e postos de defensores por 10 s e aplica -10% de dano permanente aos NPCs daquela ilha, acumulável até três vezes (-30%). **Desafio de Ilha:** é a preparação da Ilha Central — a leitura dos meios de produção antes da tomada.

**Critérios de aceitação:**
- Interromper a canalização em qualquer instante consome metade do custo e não revela nem enfraquece nada.
- Após 3 s completos, o layout defensivo aparece no mapa da equipe por 10 s.
- A quarta aplicação na mesma ilha não aumenta o enfraquecimento.

---

### 10.3 Artesão (Artífice)

*Função:* construtor, defensor de ponto, gerenciador de recursos. *Pilares:* P2, P4.

**Q — Construir Barricada.** Cria uma pequena parede destrutível. **Custo:** 30 Fôlego e **20 Materiais**; **execução:** **1,5 s de canalização**; **alcance:** 4 m; **recarga:** 10 s. **Efeito:** parede de 3 m de largura por 2 m de altura, **200 pontos de vida**, **12 s de duração**; bloqueia passagem e projéteis. `[HIPÓTESE]` nos valores de Fôlego, alcance, recarga e dimensões; Materiais, canalização, pontos de vida e duração são canônicos. **Desafio de Ilha:** viabiliza a tarefa **Defender Barricada (tarefa d'A Greve, 10 s)** no desafio "A Greve" da Rota Central; na Rota Inferior / Delta do Comércio, a mesma mecânica gera a Ponte Provisória usada em emboscadas. `[SUPOSIÇÃO]`

**Critérios de aceitação:**
- A parede interdita um corredor de 3 m por **12 s** e é destruída ao receber **200** de dano.
- Projéteis inimigos colidem com a parede em vez de atravessá-la.
- Cada Artesão mantém no máximo duas barricadas ativas; a terceira remove a mais antiga.

**W — Oficina de Campo.** Cria uma pequena área onde aliados compram itens básicos sem voltar à Comuna. **Custo:** 35 Fôlego e 20 Materiais; **execução:** 2 s de canalização; **alcance:** 3 m, raio de atendimento 5 m; **recarga:** 20 s. **Efeito:** banca ativa por 45 s que vende itens de nível 1 (tier 1) com Ouro e Materiais da reserva compartilhada do time. `[HIPÓTESE]` na duração e no custo. **Desafio de Ilha:** ponto de reabastecimento durante "A Ocupação da Fábrica", onde retornar à Comuna custa a janela de sincronia.

**Critérios de aceitação:**
- Aliados a até 5 m da banca compram itens de nível 1 sem retornar à Comuna.
- Cada compra debita Ouro e Materiais da reserva do time; a banca dura 45 s.
- Itens de nível 2 e 3 não aparecem na Oficina de Campo.

**E — Armadilha Improvisada.** Coloca uma armadilha realista (armadilha de urso ou rede) que imobiliza quem pisa. **Custo:** 25 Fôlego e 5 Materiais; **execução:** 1 s; **alcance:** 3 m; **recarga:** 14 s. **Efeito:** artefato terrestre de 1 m de raio, imobiliza por 2 s, dura 60 s, máximo de três ativas; aliados não a ativam. `[HIPÓTESE]` **Desafio de Ilha:** protege pontos de coleta neutros e intercepta as rotas do Camponês inimigo no Delta do Comércio.

**Critérios de aceitação:**
- O primeiro inimigo que entra no raio fica imobilizado por 2 s, sem receber dano mágico.
- Aliados e Súditos aliados atravessam a armadilha sem ativá-la.
- A armadilha expira após 60 s mesmo sem ter sido ativada.

**R — Sabotagem Estrutural (Ultimate).** Causa dano massivo a uma estrutura (torre ou máquina de desafio) após longa canalização. **Custo:** 45 Fôlego e 30 Materiais; **execução:** canalização de **4 s** interrompível; **alcance:** 5 m, exige contato com a estrutura; **recarga:** 100 s. `[HIPÓTESE]` **Efeito:** 600 de dano e 8 s de inatividade da máquina; +25% de dano se a estrutura estiver marcada por Análise Materialista. **Distinção:** Sabotagem Estrutural é a habilidade individual do Artesão (4 s); não se confunde com **Sabotar Maquinário (tarefa d'A Greve, 8 s)**, tarefa conjunta do desafio executada por múltiplas funções. **Desafio de Ilha:** a Sabotagem Estrutural do Artesão contribui para a tarefa Sabotar Maquinário (8 s) do desafio "A Greve" na Rota Central / Ilhas da Ação Direta.

**Critérios de aceitação:**
- Dano ou silêncio durante os 4 s cancelam a canalização sem consumir os Materiais.
- A estrutura recebe 600 de dano e fica inoperante por 8 s ao completar a canalização.
- O bônus de 25% só incide sobre alvo marcado por Análise Materialista ativa.

---

### 10.4 Agitador (Sabotador)

*Função:* infiltrador, eliminação de alvos de alto valor, controle de mapa. *Pilares:* P3, P4.

**Q — Ato de Vandalismo.** Ataque rápido que causa dano extra e silencia (impede o uso de habilidades) temporariamente. **Custo:** 20 Fôlego; **execução:** 0,2 s; **alcance:** 2 m; **recarga:** 7 s. **Efeito:** 120 de dano físico e 1,5 s de silêncio. `[HIPÓTESE]` **Desafio de Ilha:** interrompe a Sabotagem Estrutural do Artesão inimigo e o Plano Mestre do Intelectual inimigo.

**Critérios de aceitação:**
- O alvo recebe 120 de dano e não usa habilidades por 1,5 s.
- Acertar uma canalização em andamento a cancela imediatamente.
- Itens e ataques básicos continuam disponíveis durante o silêncio.

**W — Fuga pelas Sombras.** Fica invisível por um curto período, especialmente perto de estruturas ou muros. **Custo:** 25 Fôlego; **execução:** 0,3 s; **alcance:** pessoal; **recarga:** 12 s. **Efeito:** 3 s de invisibilidade, estendidos para 5 s a até 3 m de estrutura ou muro; atacar ou usar habilidade quebra o efeito. `[HIPÓTESE]` **Desafio de Ilha:** permite infiltração prévia nas ilhas da Rota Central e aproximação a postos de coleta no Delta.

**Critérios de aceitação:**
- O Agitador fica invisível por 3 s, ou 5 s junto a estrutura, e nenhum marcador de interface o exibe nesse período.
- Atacar ou lançar habilidade encerra a invisibilidade no mesmo instante.
- Uma torre de defesa a 4 m ou Análise Materialista revela o Agitador.

**E — Espalhar Rumores.** Confunde os defensores (NPCs) de uma ilha, fazendo-os atacar uns aos outros brevemente. **Custo:** 30 Fôlego; **execução:** 0,5 s; **alcance:** 10 m, área de 5 m de raio; **recarga:** 16 s. **Efeito:** NPCs da área trocam de alvo por 4 s; jogadores são imunes; defensores de elite sofrem apenas 2 s. `[HIPÓTESE]` **Desafio de Ilha:** é a segunda forma de "Preparar o Terreno" na ilha preparatória da Rota Central.

**Critérios de aceitação:**
- Defensores comuns na área atacam outros NPCs da mesma ilha por 4 s.
- Jogadores aliados e inimigos não são afetados pela confusão.
- Defensores de elite resistem à metade da duração.

**R — Greve Geral (Ultimate).** Habilidade de desafio que desativa todas as defesas de uma ilha por um curto período, mas revela o Agitador no mapa. **Custo:** 45 Fôlego; **execução:** 0,5 s; **alcance:** ilha alvo a 25 m; **recarga:** 120 s. `[HIPÓTESE]` **Efeito:** torres, NPCs e armadilhas da ilha ficam inoperantes por 6 s; o Agitador aparece no mapa inimigo por 8 s. **Restrição (Decisão de design, 2026-11-19):** Greve Geral **não afeta a Ilha Central** — o objetivo final é protegido; incide apenas sobre ilhas intermediárias das três rotas. **Desafio de Ilha:** abre a janela de 5 s para a entrada sincronizada de três aliados na Rota Central / Ilhas da Ação Direta.

**Critérios de aceitação:**
- Todas as defesas da ilha alvo param de agir por 6 s; NPCs não retomam o ataque antes disso.
- O marcador do Agitador fica visível no mapa inimigo por 8 s, mesmo após sair da ilha.
- A habilidade não remove o desafio em si: os três aliados ainda devem entrar na janela de 5 s.
- Em ilhas intermediárias das três rotas, Greve Geral desativa as defesas por 6 s e abre a janela de 5 s; a Ilha Central nunca entra na lista de alvos válidos.
- Ao tentar usar Greve Geral na Ilha Central, a habilidade é recusada com feedback visual e sem consumo de recurso.

**Decisão de design (2026-11-19).** Greve Geral não afeta a Ilha Central; a habilidade incide apenas sobre ilhas intermediárias das três rotas, preservando a cadeia de capturas. Justificativa: um atalho na Ilha Central anularia a cadeia de capturas e esvaziaria o loop de preparar → ocupar → greve.

---

### 10.5 Camponês (Coletor / Miliciano)

*Função:* geração de renda, resistência, absorção de dano (tanque — que suporta golpes). *Pilares:* P2, P4.

**Q — Coleta Eficiente.** Passiva: ganha mais recursos (Ouro e experiência) ao coletar em ilhas neutras como a selva. Ativa: aumenta a velocidade de coleta. **Custo:** passiva nenhuma; ativa 15 Fôlego; **execução:** 0,2 s; **alcance:** ponto de coleta a 2 m; **recarga:** 10 s. **Efeito:** passiva de +25% de Ouro e +25% de experiência em pontos neutros; ativa dobra a taxa de coleta por 5 s. `[HIPÓTESE]` **Desafio de Ilha:** sustenta a renda passiva do Delta do Comércio e cumpre desafios de coleta da Rota Superior.

**Critérios de aceitação:**
- Coletar num ponto neutro rende 25% de Ouro e experiência a mais do que o valor base.
- A ativação dobra a taxa de coleta por 5 s e não se acumula com outra ativação.
- Nenhum bônus incide sobre pontos controlados pelo time inimigo.

**W — Ferramentas Agrícolas.** Alterna entre duas posturas: "Enxada" (ataques causam lentidão) e "Familiaridade com o Terreno" (velocidade de movimento). **Custo:** 10 Fôlego por troca; **execução:** 0,2 s; **alcance:** pessoal; **recarga:** 3 s. **Efeito:** Enxada aplica -20% de velocidade por 2 s nos alvos dos ataques básicos; Familiaridade concede +25% de velocidade de movimento e +10% de resistência à lentidão em ilhas do Delta. `[HIPÓTESE]` **Desafio de Ilha:** Familiaridade atravessa o pântano do Delta e escapa de emboscadas; Enxada defende o ponto de coleta.

**Critérios de aceitação:**
- As posturas são mutuamente exclusivas; trocar substitui a anterior imediatamente.
- Com Enxada, cada acerto básico reduz a velocidade do alvo em 20% por 2 s.
- Com Familiaridade, o bônus de 25% só vale dentro das ilhas da Rota Inferior / Delta do Comércio.

**E — Resiliência.** Recupera vida passivamente, mais rápido perto de recursos naturais. **Custo:** nenhum; **execução:** passiva; **alcance:** pessoal, com aura de 4 m em torno de recursos naturais; **recarga:** nenhuma. **Efeito:** 2% da vida máxima por segundo; 5%/s a até 4 m de árvore, fonte ou plantio. `[HIPÓTESE]` **Desafio de Ilha:** mantém o Camponês em desafios longos de coleta sem retornar à Comuna.

**Critérios de aceitação:**
- A regeneração base é de 2%/s e sobe para 5%/s junto a recurso natural.
- Dano contínuo de gás lacrimogêneo na Rota Central suspende a regeneração enquanto durar.
- A passiva continua ativa enquanto o Camponês coleta.

**R — A Revolta (Ultimate).** Invoca uma milícia de camponeses (NPCs) que segue o personagem e o defende por tempo limitado. **Custo:** 45 Fôlego e 20 Suprimentos; **execução:** 1 s de canalização; **alcance:** pessoal; **recarga:** 110 s. **Efeito:** três milicianos com 200 de vida cada, que seguem e atacam alvos próximos por 12 s. `[HIPÓTESE]` **Desafio de Ilha:** fornece os Súditos que o Discurso Inspirador converte e protege pontos de coleta no Delta.

**Critérios de aceitação:**
- Três milicianos surgem ao redor do Camponês e o seguem por 12 s.
- Milicianos contam como Súditos para Discurso Inspirador e Propaganda.
- Ao expirar, os milicianos desaparecem sem deixar corpo ou recurso.

---

### 10.6 Guerreiro (Milícia)

*Função:* defesa de aliados e combate direto; escolta opcional, nunca obrigatória. *Pilares:* P3, P4. As fontes citam a função e registram "Ataque Forte" no protótipo inicial, mas não detalham o conjunto; apenas o nome do Q é canônico. Todo o restante é proposta.

**Q — Ataque Forte.** Golpe concentrado corpo a corpo; nome canônico do protótipo, efeito detalhado aqui. **Custo:** 20 Fôlego; **execução:** 0,4 s de preparação do golpe; **alcance:** 2,5 m; **recarga:** 6 s. **Efeito:** 150 de dano físico a um alvo, +20% contra estruturas de desafio. `[HIPÓTESE]` **Desafio de Ilha:** remove defensores que bloqueiam as placas de sincronia da Rota Central.

**Critérios de aceitação:**
- O golpe acerta um alvo por vez e causa 150 de dano, ou 180 contra estruturas.
- Acertar uma canalização inimiga a interrompe.
- O tempo de preparação de 0,4 s permite contra-ataque ou esquiva.

**W — Escudo de Milícia.** `[HIPÓTESE]` Ergue um escudo que absorve dano e protege um aliado. **Custo:** 25 Fôlego; **execução:** 0,3 s; **alcance:** 3 m, si mesmo ou aliado; **recarga:** 14 s. **Efeito:** 250 de absorção por 5 s e +15% de resistência a dano contínuo. **Desafio de Ilha:** protege o Artesão durante a Sabotagem Estrutural e os três entrantes contra o gás lacrimogêneo.

**Critérios de aceitação:**
- O escudo absorve até 250 de dano e expira após 5 s.
- Enquanto ativo, reduz em 15% o dano contínuo de gás lacrimogêneo.
- O escudo pode ser aplicado a um aliado ou ao próprio Guerreiro, nunca a dois alvos ao mesmo tempo.

**E — Carga de Piquete.** `[HIPÓTESE]` Avança em linha empurrando inimigos. **Custo:** 25 Fôlego; **execução:** 0,5 s; **alcance:** 6 m; **recarga:** 12 s. **Efeito:** desloca o Guerreiro 6 m, empurra inimigos 2 m e reduz sua velocidade em 15% por 2 s. **Desafio de Ilha:** abre o corredor para a entrada sincronizada de três jogadores.

**Critérios de aceitação:**
- O deslocamento percorre exatamente 6 m e para ao encontrar terreno sólido.
- Inimigos atingidos são empurrados 2 m e ficam 15% mais lentos por 2 s.
- A carga não atravessa barricadas nem paredes de desafio.

**R — Linha de Defesa (Ultimate).** `[HIPÓTESE]` Forma uma barreira que bloqueia a passagem. **Custo:** 45 Fôlego e 15 Materiais; **execução:** 1 s de canalização; **alcance:** linha de 8 m; **recarga:** 105 s. **Efeito:** barreira de 8 m por **8 s**, sem dano, que concede +20% de resistência a aliados atrás dela. **Distinção:** Linha de Defesa é a Ultimate do Guerreiro (8 s de duração); não se confunde com **Defender Barricada (tarefa d'A Greve, 10 s)**, tarefa do desafio. **Desafio de Ilha:** a Linha de Defesa protege os participantes da tarefa Defender Barricada no desafio "A Greve".

**Critérios de aceitação:**
- A barreira interdita uma linha de 8 m por 8 s e não causa dano a quem a toca.
- Aliados posicionados atrás da linha recebem +20% de resistência.
- A barreira não pode ser instalada sobre um Desafio de Ilha ativo.

---

### 10.7 Aliança Mercantil

*Função:* logística, comércio e abertura de caminhos; o "suporte" aqui é de rotas, não de um único combatente. *Pilares:* P1, P2, P4. As fontes citam a função e a frase "Estabelecer Rotas Comerciais", mas não fornecem o conjunto; as quatro habilidades abaixo são propostas.

**Q — Escambo.** `[HIPÓTESE]` Troca rápida com um ponto neutro. **Custo:** 10 Fôlego; **execução:** 0,5 s; **alcance:** 3 m de posto neutro; **recarga:** 8 s. **Efeito:** converte 20 Ouro em 15 Materiais ou 15 Suprimentos, taxa configurável no Módulo. **Desafio de Ilha:** abastece Artesãos e Camponeses no Delta do Comércio sem retorno à Comuna.

**Critérios de aceitação:**
- A troca converte exatamente 20 Ouro em 15 Materiais ou 15 Suprimentos, nunca em ambos.
- O Módulo pode alterar a taxa sem mudar o Núcleo.
- A conversão falha se o time não tiver 20 Ouro disponíveis.

**W — Estabelecer Rotas Comerciais.** `[HIPÓTESE]` Abre um caminho temporário entre ilhas neutras. **Custo:** 35 Fôlego; **execução:** 2 s de canalização; **alcance:** ilha neutra a 12 m; **recarga:** 25 s. **Efeito:** cria passagem navegável de 20 s entre duas ilhas e gera 10 de Ouro por travessia aliada. **Desafio de Ilha:** viabiliza a exploração do Delta e o flanqueio da Rota Inferior / Delta do Comércio.

**Critérios de aceitação:**
- A passagem existe por 20 s e permite travessia em ambas as direções.
- Cada aliado que a atravessa rende 10 de Ouro ao time, até o limite de 8 travessias.
- Dano durante a canalização cancela a rota e não devolve o custo.

**E — Monopólio Local.** `[HIPÓTESE]` "Compra" um ponto de coleta neutro por meio de preço. **Custo:** 30 Ouro e 30 Fôlego; **execução:** 1 s; **alcance:** 10 m; **recarga:** 18 s. **Efeito:** ninguém além da Aliança coleta no ponto por 8 s, gerando 5 de Ouro por segundo. **Desafio de Ilha:** transforma a disputa de recursos em estrangulamento econômico.

**Critérios de aceitação:**
- Durante 8 s, apenas a Aliança Mercantil extrai daquele ponto.
- A renda de 5 Ouro/s entra na reserva do time, não no personagem.
- Outra Aliança Mercantil não pode comprar o mesmo ponto enquanto o monopólio estiver ativo.

**R — Boicote Geral (Ultimate).** `[HIPÓTESE]` Fecha as rotas comerciais inimigas do Delta. **Custo:** 45 Fôlego; **execução:** 1,5 s de canalização; **alcance:** rotas do Delta do Comércio; **recarga:** 120 s. **Efeito:** congela a renda passiva inimiga no Delta por 10 s e revela a Aliança no mapa por 5 s. **Desafio de Ilha:** é o caminho ativo para a vitória por estrangulamento econômico descrita na seção 11.

**Critérios de aceitação:**
- A renda passiva inimiga no Delta cai a zero por 10 s, inclusive a gerada por Camponeses.
- Rotas inimigas abertas por Estabelecer Rotas Comerciais são fechadas durante o boicote.
- A Aliança que executa o boicote fica visível no mapa inimigo por 5 s.

---

### 10.8 Tabela-resumo das funções

| Função | Papel | Rota preferencial | Sinergia principal |
|---|---|---|---|
| Intelectual | Apoio, visão, conversão e preparação | Rota Central / Ilhas da Ação Direta | Converte Súditos com Discurso Inspirador; marca alvos para o Artesão e prepara a Ilha Central com Plano Mestre |
| Artesão | Construção, armadilhas, sabotagem | Rota Central / Ilhas da Ação Direta e Rota Inferior / Delta do Comércio | Recebe a marcação do Intelectual; depende da proteção do Guerreiro durante a canalização da Ultimate |
| Agitador | Infiltração, silêncio, confusão de NPCs | Rota Central / Ilhas da Ação Direta e Rota Inferior / Delta do Comércio | Prepara ilhas com Espalhar Rumores; abre a janela de três aliados com Greve Geral; intercepta o Camponês inimigo |
| Camponês | Coleta, renda passiva, absorção de dano | Rota Superior / Ilhas da Autossuficiência e Rota Inferior / Delta do Comércio | Alimenta o Discurso Inspirador com milícia; escoa a produção pelas rotas da Aliança Mercantil |
| Guerreiro (Milícia) | Defesa de ponto, escolta, combate direto | Rota Central / Ilhas da Ação Direta | Protege o Artesão e os três entrantes; escolta opcional do Camponês, sem vínculo obrigatório |
| Aliança Mercantil | Logística, comércio, estrangulamento | Rota Inferior / Delta do Comércio | Converte a coleta do Camponês em vantagem de mapa; viabiliza o abastecimento do Artesão |

**Critérios de aceitação do conjunto:**
- Cada uma das seis funções possui exatamente quatro habilidades nomeadas Q, W, E e R.
- Nenhuma habilidade causa dano ou efeito classificado como mágico, sobrenatural ou espiritual (P2).
- Em toda partida de teste, ao menos três funções diferentes participam da captura da Ilha Central (P4).

---

## 11. Economia, Progressão e Balanceamento

A economia é a base de todo poder (P2): nenhuma habilidade altera o mundo sem consumir um recurso material ou social. O desenho evita o arquétipo de "carregador" e distribui a geração de renda entre funções, de modo que a coordenação — e não o acúmulo individual — decida a partida (P4).

### 11.1 Recursos e fluxo

São três os recursos de jogo: **Ouro** (abates e renda), **Materiais** (construção, consumo do Artesão) e **Suprimentos** (sustento, milícia, comércio). A fonte fixa que Materiais e Suprimentos são gerados em pontos de coleta neutros e que o Ouro vem de abates; os valores são propostos.

| Recurso | Fonte | Uso | Consumo | Limite |
|---|---|---|---|---|
| Ouro | Abate de Súditos e NPCs, pontos neutros, renda passiva | Compra de itens, Escambo, Monopólio Local | Debitado no ato da compra; a taxa é configurável pelo Módulo (ex.: "Taxa de Ouro: 150%", valor canônico da fonte) | Sem teto; `[HIPÓTESE]` para o valor base de 12 Ouro por Súdito abatido |
| Materiais | Pontos de coleta neutros da Rota Superior e do Delta do Comércio | Barricada, Oficina de Campo, Armadilha, Sabotagem, Linha de Defesa | Consumidos no instante da construção ou compra | Reserva de 200 por time `[HIPÓTESE]` |
| Suprimentos | Pontos de coleta neutros do Delta do Comércio | Milícia de A Revolta, rações da Oficina de Campo, Escambo | Consumidos ao convocar milícia e ao comprar ração | Reserva de 150 por time `[HIPÓTESE]` |
| Fôlego | Regeneração temporal (10/s fora de combate, 5/s em combate) | Custo de todas as habilidades | Gasto ao ativar habilidade | 100 por personagem |
| Experiência | Abate, coleta, conclusão de Desafio de Ilha | Progressão de nível | Gasta ao cruzar cada nível | Nível 1 a 12 |

**Pilares atendidos:** P2 (recursos como única fonte de poder) e P1 (taxas e limites reescritos por Módulos).

**Critérios de aceitação:**
- Abater um Súdito entrega 12 Ouro ao time, antes da aplicação da Taxa de Ouro do Módulo.
- **Atributo de rendimento** do ponto: começa com 60 unidades do seu recurso e se regenera 30 s após ser esgotado — distinto do **atributo de integridade/respawn** do ponto (30 PV, reposição em 60 s), definido na seção 9. `[HIPÓTESE]`
- Nenhum recurso aparece no inventário sem registrar a fonte no registro de partida do servidor.

### 11.2 Renda passiva e pontos neutros

Cada ponto neutro controlado rende Ouro contínuo ao time, não ao indivíduo. A renda é atribuída à reserva coletiva para impedir que um único jogador concentre o produto do mapa. `[HIPÓTESE]` nos valores abaixo.

- Ponto controlado por Camponês com Coleta Eficiente ativa: **4 Ouro/s** na reserva do time.
- Ponto controlado sem Camponês: **2 Ouro/s** na reserva do time.
- Ilha capturada após seu Desafio de Ilha: **+3 Ouro/s** enquanto o time mantiver o controle.
- O controle cai quando o ponto fica 10 s sem presença aliada ou quando um inimigo conclui a interdição.

**Pilares atendidos:** P2 e P4.

**Critérios de aceitação:**
- A renda de todos os pontos entra na reserva do time e aparece igualmente na interface de todos os aliados.
- Perder a presença aliada por 10 s interrompe a renda daquele ponto sem afetar os demais.
- Um único jogador não pode sacar a reserva: compras debiram o fundo comum por igual.

### 11.3 Estrangulamento econômico como vitória alternativa

Vencer não exige apenas ativar a Ilha Central. Um time também vence ao levar a economia inimiga ao colapso, convertendo a disputa de recursos em condição de vitória. `[HIPÓTESE]` na condição abaixo.

> Se a renda passiva inimiga permanecer em **0 Ouro/s por 120 s consecutivos** e o time controlar ao menos **5 dos 10 pontos neutros do Delta do Comércio**, declara-se a vitória por Estrangulamento. O cronômetro zera sempre que o inimigo recuperar qualquer renda.

Monopólio Local, Boicote Geral, Armadilha Improvisada e a escolta do Guerreiro são as vias para aplicar a pressão; a janela de 120 s impede que um único golpe de Boicote (10 s) decida a partida.

**Pilares atendidos:** P2 (a economia como campo de batalha) e P4 (a ação coletiva substitui o abate como caminho de vitória).

**Critérios de aceitação:**
- O contador de 120 s só avança enquanto a renda inimiga for exatamente zero.
- O painel de ambos os times exibe o contador e os pontos necessários (5 de 10).
- Capturar a Ilha Central por um time encerra a partida imediatamente, mesmo que o contador de estrangulamento do adversário esteja ativo.

### 11.4 Curva de custos e progressão

A progressão é curta e compartilhada por todas as funções: níveis desbloqueiam habilidades e concedem incrementos fixos, sem árvore de talentos no Módulo base. `[HIPÓTESE]` em toda a tabela.

| Nível | Experiência acumulada | Desbloqueio / ganho |
|---|---|---|
| 1 | 0 | Habilidade Q |
| 2 | 100 | Habilidade W |
| 3 | 250 | Habilidade E |
| 4 | 450 | +10% de vida máxima |
| 5 | 700 | +5% de dano |
| 6 | 1000 | Ultimate (R) |
| 7 | 1350 | +10% de vida máxima |
| 8 | 1750 | +5% de dano |
| 9 | 2200 | +5% de regeneração de Fôlego |
| 10 | 2700 | +10% de vida máxima |
| 11 | 3250 | +5% de dano |
| 12 | 3850 | +5% de regeneração de Fôlego; teto da partida |

**Curva de custos de itens.** Itens de nível *tier* (categoria) 1, 2 e 3 custam, respectivamente, **100, 140 e 196 Ouro**, seguindo `custo = 100 × 1,4^(tier−1)`. O crescimento de 40% por nível impede que o item superior seja comprado antes de um Desafio de Ilha ter sido resolvido. `[HIPÓTESE]`

**Critérios de aceitação:**
- Subir do nível 5 para o 6 exige 300 pontos de experiência e desbloqueia a Ultimate no mesmo instante.
- Nenhum personagem passa do nível 12; experiência adicional é convertida em 5 Ouro por nível excedente.
- Um item de nível 3 custa 96 Ouro a mais que um de nível 2.

### 11.5 Aquisição de itens na Oficina de Campo

A Oficina de Campo (W do Artesão) vende apenas itens de nível 1, com Ouro e Materiais da reserva do time. Itens de nível 2 e 3 exigem o retorno à Comuna, o que cria tensão logística: sair do Desafio de Ilha para se equipar custa posição no mapa. `[HIPÓTESE]` na tabela.

| Item | Nível | Preço | Efeito |
|---|---|---|---|
| Bandagem de Lona | 1 | 50 Ouro + 10 Materiais | Restaura 150 de vida instantaneamente |
| Alavanca | 1 | 60 Ouro + 15 Materiais | +10% de dano a estruturas de desafio por 60 s |
| Ração | 1 | 40 Ouro + 10 Suprimentos | +20% de regeneração de vida por 60 s |
| Capacete de Milícia | 2 | 140 Ouro (somente na Comuna) | +80 de vida máxima |
| Bigorna Portátil | 3 | 196 Ouro (somente na Comuna) | +15% de dano a estruturas, cumulativo com Alavanca |

**Critérios de aceitação:**
- A Oficina de Campo oferece exatamente os três itens de nível 1 e nenhum item superior.
- Comprar Bandagem de Lona debita 50 Ouro e 10 Materiais da reserva do time e restaura 150 de vida.
- Comprar Bigorna Portátil fora da Comuna é impossível mesmo com 196 Ouro disponíveis.

### 11.6 Regras de balanceamento e válvulas anti-neve

O sistema pressupõe que uma vantagem inicial tende a se autoamplificar. As válvulas abaixo limitam essa inércia sem punir a boa execução e sem recorrer a emparelhamento artificial de jogadores.

| Válvula | Regra | Marcação |
|---|---|---|
| Abate decrescente | Cada abate sem resposta reduz o Ouro recebido em 10%, até o piso de 60% do valor base; um abate inimigo zera a penalidade | `[HIPÓTESE]` |
| Renda diluída | A partir do quarto ponto controlado, cada ponto extra reduz em 8% a renda por ponto | `[HIPÓTESE]` |
| Penalidade de grupo | Desafios da Rota Superior / Ilhas da Autossuficiência: **dobra única ×2** — com 1 aliado adicional, 4 → 8 guardas. Sem segundo dobramento e sem curva exponencial | Canônico da fonte |
| Janela de sincronia | Três aliados devem entrar na ilha da Rota Central em 5 s; entradas escalonadas sofrem **8 pontos de dano por segundo de gás lacrimogêneo, com expulsão após 3 s de exposição contínua**, e são repelidas | Canônico da fonte |
| Reconstituição de defesas | Ilha perdida recupera 50% das defesas após 90 s se o time dominante controlar mais de 60% do mapa | `[HIPÓTESE]` |
| Teto de experiência | Nível máximo 12 e conversão do excedente em Ouro evitam escalada infinita | `[HIPÓTESE]` |

**Regras gerais de balanceamento:**

- Nenhuma habilidade de controle (imobilização, silêncio, confusão) dura mais de 2 s quando aplicada a um jogador; efeitos sobre NPCs podem durar mais. `[HIPÓTESE]`
- Todo efeito é físico ou social; nenhuma habilidade introduz dano mágico ou cura sobrenatural (P2).
- Taxas, custos e durações vivem em `RegrasDoServidor.cfg` e são alteráveis por Módulo sem tocar no Núcleo (P1). O exemplo canônico é "Taxa de Ouro: 150%".
- Toda alteração de valor deve declarar o pilar atendido; alteração que não serve a nenhum pilar é cortada.

**Critérios de aceitação:**
- Após cinco abates consecutivos sem resposta, o sexto rende 60% do Ouro base, não menos.
- Um time com 6 pontos controlados recebe menos Ouro total do que um time com 4 pontos.
- Três jogadores que entram em 5 s iniciam o desafio; dois que entram separados por 6 s são repelidos pelo gás lacrimogêneo.

### 11.7 Suporte logístico flexível — o suporte não é um "Carregador"

**Carregador** (*carry*: personagem que acumula itens e escala com Ouro) é o arquétipo que o Insurgência recusa. Nenhuma regra do Núcleo vincula a função de apoio a proteger um único Carregador, e nenhuma função é obrigada a seguir outra. O apoio é logístico e pode ser exercido por quem abrir rotas, coletar, construir ou converter — conforme a fonte, "o sucesso aqui não é matar, mas estrangular a economia inimiga".

- O Camponês gera renda para a reserva do time, nunca para um indivíduo.
- A Aliança Mercantil escoa essa produção e abre atalhos para qualquer aliado, sem escolher um beneficiário fixo.
- O Intelectual aplica Discurso Inspirador a todos os aliados e Súditos na aura, não a um alvo privilegiado.
- O Artesão constrói a Oficina de Campo para o time inteiro e pode erguer uma Ponte Provisória para emboscadas, em vez de apenas defender um colega.
- O Guerreiro escolta quando a situação pede; a escolta é decisão tática, não obrigação de função.

**Pilares atendidos:** P4 (ação coletiva) e P1 (a comunidade de jogadores define a divisão de trabalho, não uma hierarquia fixa).

**Critérios de aceitação:**
- Nenhuma regra, interface ou Módulo base designa um único personagem como "Carregador" ou concede renda exclusiva a ele.
- Em partida de teste, a remoção de qualquer jogador que esteja acumulando Ouro não interrompe a renda do time.
- Ao menos três funções diferentes são capazes de gerar ou proteger renda para a reserva coletiva.
## 12. Controles, Interface e Experiência do Jogador

### 12.1. Intenção e rastreabilidade

A interface não ensina o jogo por texto longo; ela torna visível a regra que já está em ação. O jogador deve conseguir responder, sem explicação prévia, a três perguntas: quantos aliados estão na ilha, quanto tempo resta para a sincronia e o que falta para a canalização terminar.

Cadeia de rastreabilidade: fantasia (organizar a tomada coletiva dos meios de produção, sem magia — seção 5) → pilares (Ação Coletiva, Materialismo Histórico, Luta Decolonial e Soberania do Jogador — seção 4) → loop (ciclo de jogo: entrar na ilha, superar o Desafio de Ilha, liberar a rota, avançar à Ilha Central — seção 7) → mecânicas desta seção (janela de sincronia de 5 s, penalidade de grupo da Rota Superior, canalização interrompível, painel de recursos). Cada elemento abaixo declara o pilar que serve; o que não serve a nenhum não entra no protótipo.

Intenção antes de implementação: este capítulo define o que o jogador vê e o que a interface precisa comunicar. Nomes de classes, chamadas de API e caminhos de arquivo ficam na seção 14.

### 12.2. Esquema de controle do protótipo — teclado e mouse

O esquema padrão reserva Q/W/E/R para as habilidades e desloca o movimento para o mouse, evitando a colisão entre a tecla W de movimento e a tecla W de habilidade.

| Ação | Controle padrão (teclado e mouse) | Intenção / valor |
| :--- | :--- | :--- |
| Mover | Botão direito do mouse: clique no destino desloca o personagem até ele; manter pressionado repete o ponto sob o cursor. Alternativa sem mouse: setas direcionais (↑ ↓ ← →) | Movimento de 4 m/s [HIPÓTESE — ritmo suficiente para atravessar a ilha do protótipo em poucos segundos sem transformar a entrada em espera] |
| Interagir com Desafio de Ilha | Botão esquerdo sobre o elemento destacado (alavanca, placa, máquina) a até 2 m de distância [HIPÓTESE]; alternativa por teclado: F | A proximidade é a permissão: não há menu de contexto |
| Canalizar | Manter F pressionado (ou manter o botão da habilidade) até a barra encher; soltar antes do fim interrompe | Canalização é ação material e vulnerável, não um encantamento automático |
| Usar habilidades | Q, W, E e R ativam, respectivamente, a primeira, a segunda, a terceira habilidade e a ultimate da função ativa (nomes das habilidades na seção 10) | R é sempre a ultimate; "Plano Mestre" (Intelectual) canaliza 3 s e "Sabotagem Estrutural" (Artesão) exige canalização longa — ambas usam a regra de interrupção acima |
| Entrar / sair de ilha | Entrar: cruzar a ponte — a entrada é automática ao penetrar a zona de 1 m da ponte [HIPÓTESE]. Sair: manter G por 1 s [HIPÓTESE] ou clicar na ponte de retorno | Entrar é consequência do movimento, não uma tecla de habilidade |
| Sinalizar sincronia | Barra de espaço: registra a presença do jogador no contador da ilha e acende o indicador de sincronia | O primeiro sinal inicia a janela de 5 s; os demais devem sinalizar dentro dela |

**Alternativa para controle de console — [SUPOSIÇÃO].** As fontes não especificam controles de console; o mapeamento abaixo é proposto e deve ser validado em teste com jogador.

| Ação | Controle proposto — [SUPOSIÇÃO] |
| :--- | :--- |
| Mover | Analógico esquerdo |
| Posicionar o cursor / olhar | Analógico direito |
| Interagir e canalizar | Manter o botão de ação secundário (B no controle Xbox; ○ no controle PlayStation) |
| Q / W / E / R | Controle direcional (D-pad) para escolher a habilidade + botão superior direito (RB no Xbox; R1 no PlayStation) para confirmar |
| Sinalizar sincronia | Botão de ação principal (A no Xbox; × no PlayStation) |
| Entrar / sair de ilha | Entrada ao cruzar a ponte; saída com manter o botão superior esquerdo (LB no Xbox; L1 no PlayStation) por 1 s |

**Critérios de aceitação do esquema de controle**

- No protótipo, o jogador se move com o botão direito e executa Q/W/E/R sem conflito: mover nunca dispara habilidade e usar habilidade nunca desloca o personagem.
- Três jogadores sinalizam com a barra de espaço em 5,0 s → o desafio sincronizado inicia; se o terceiro sinalizar em 5,1 s, os três são repelidos com dano contínuo e a ilha não é capturada.
- Soltar F durante a canalização zera o progresso e emite o retorno de interrupção; nenhuma parte da ação é concluída.
- Manter G por 1 s fora da janela de sincronia retira o personagem da ilha e decrementa o contador de presença em 1 s.

### 12.3. Elementos de interface

#### Janela de sincronia de 5 s

Intenção: tornar o prazo da entrada coordenada legível sem depender de contagem verbal entre jogadores. Serve a Ação Coletiva.

- Uma barra circular aparece no centro da interface no instante do primeiro sinal de presença e esvazia linearmente em exatamente 5,0 s.
- O contador numérico "0/3" atualiza a cada sinal; ao atingir "3/3" antes de 5,0 s, a barra fica verde e o desafio inicia.
- Se "3/3" não for atingido até 5,0 s, a barra fica vermelha, os presentes sofrem 8 pontos de dano por segundo — o gás lacrimogêneo das fontes — e, após 3 s de exposição contínua, são expulsos; a ilha permanece não capturada.
- Os três jogadores veem o mesmo estado da barra, com diferença máxima de 0,2 s [HIPÓTESE — tolerância de sincronização de rede].

> [NOTA PARA O DESIGNER] As fontes descrevem a regra completa como três aliados em 5 s, mas o escopo inicial do protótipo usa duas placas pisadas ao mesmo tempo. A seção 16 deve dizer explicitamente qual das duas versões vale no MVP (Produto Mínimo Viável); a interface acima foi dimensionada para a regra completa de 5 s.

#### Indicador de "ilha preparada"

Intenção: mostrar que o Desafio Preparatório ("Organizar Célula", executado por Agitador ou Intelectual) terminou e que a ilha seguinte está vulnerável. Serve à Ação Coletiva e ao Materialismo Histórico.

- O ícone da ilha aparece com borda tracejada no mapa e na barra lateral; ao concluir a canalização de 6 s de preparação, a borda fica preenchida e o rótulo "ilha preparada" aparece.
- O estado preenchido persiste por 45 s; expirado, volta a tracejado e a preparação deve ser repetida.
- Ao entrar na ilha preparada, a interface na tela mostra "defesas enfraquecidas" e o dano sofrido dos personagens controlados pelo jogo cai 25% enquanto durar o estado de 45 s.
- A visibilidade do indicador para a equipe adversária é assimétrica: inimigos veem apenas a borda tracejada [HIPÓTESE — as fontes não definem essa regra de informação].

#### Contador de presença na ilha

Intenção: avisar, antes do erro, que ajudar um aliado na Rota Superior torna o desafio mais difícil. Serve à assimetria das rotas e a Ação Coletiva.

- O contador exibe "1" em cinza quando há um jogador na ilha; ao entrar o segundo aliado, passa a "2" em vermelho e o ícone de alerta pisca duas vezes.
- Com contador igual ou superior a 2, a dificuldade do evento sobe de 1× para 2× — regra canônica do briefing: dobra única, sem segundo dobramento; as forças de segurança passam de 4 para 8 (DificuldadeEvento = Dificuldade × 2), conforme decisão de design de 2026-11-19 —; o ×5 permanece apenas como variante de estresse do protótipo de caixas, não como regra da liga vanilla.
- O contador decrementa no prazo de 1 s após a saída do aliado e a dificuldade retorna a 1× nesse mesmo intervalo.
- O contador é por ilha, nunca global: entrar na Ilha da Floresta não altera o contador da Ilha da Montanha.

#### Painel de recursos

Intenção: manter a economia material sempre visível, pois é dela que saem as construções do Artesão e a coleta do Camponês. Serve ao Materialismo Histórico.

- Três linhas fixas no canto inferior esquerdo mostram ícone, rótulo e número: Ouro, Materiais e Suprimentos, atualizados em até 0,5 s [HIPÓTESE] após cada abate ou coleta.
- Ouro é gerado por abates; Materiais alimentam construções e armadilhas do Artesão; Suprimentos vêm dos pontos de coleta do Delta do Comércio e alimentam o Camponês.
- Ao gastar, o número pisca em âmbar por 0,5 s e o custo aparece na dica da habilidade por 2 s [HIPÓTESE].
- O painel ocupa no máximo 15% da largura da tela e nunca cobre a barra circular de sincronia.

#### Retorno de canalização interrompida

Intenção: uma ação material pode falhar; a falha precisa ser lida como falha, não como ausência de resposta. Serve ao Materialismo Histórico.

- Uma barra de progresso aparece acima do personagem em 0,2 s [HIPÓTESE] no início da canalização.
- Soltar o controle, sofrer dano ou sair do raio de 2 m interrompe a ação: a barra quebra em vermelho e desaparece em 0,8 s [HIPÓTESE].
- A mensagem curta "Canalização interrompida" permanece por 1,5 s, acompanhada apenas do estalo mecânico descrito na seção 13 — sem trilha musical de derrota.
- O progresso perdido não é reembolsado: recomeçar inicia em 0%.

#### Mapa de rotas não linear

Intenção: mostrar que o Arquipélago da Insurgência é uma rede de ilhas e que o Delta do Comércio não é um caminho reto. Serve a Luta Decolonial e a Ação Coletiva.

- O mapa desenha as três rotas como cadeias de ilhas e o Delta do Comércio como uma malha de pelo menos 5 nós [HIPÓTESE — as fontes não fixam a quantidade de ilhas do Delta].
- Ilha não capturada tem contorno tracejado; capturada, contorno preenchido; a Ilha Central tem destaque duplo.
- Rotas temporárias criadas por "Estabelecer Rotas Comerciais" (Aliança Mercantil) ou "Ponte Provisória" (Artesão) aparecem como linha pontilhada e desaparecem quando a habilidade termina.
- Pressionar M abre e fecha o mapa em até 0,3 s [HIPÓTESE] sem pausar a partida.

### 12.4. Regras de acessibilidade

- **Contraste:** razão mínima de 4,5:1 entre texto e fundo, conforme a diretriz WCAG (Diretrizes de Acessibilidade para Conteúdo Web), nível AA. Nenhum estado de jogo depende somente de cor.
- **Legendas:** todo retorno sonoro tem texto equivalente; os estados da janela de sincronia se distinguem por cor, número e forma de barra ao mesmo tempo.
- **Remapeamento:** todas as ações da tabela de controles podem ser reatribuídas, incluindo perfis para uso com uma mão. O remapeamento altera apenas a entrada, nunca a regra de 5 s da janela.

**Critérios de aceitação de acessibilidade**

- Com filtro para daltonismo ativo, os estados "pendente", "pronta" e "falha" da janela de sincronia continuam distinguíveis pelo número e pelo formato da barra.
- Desativar todo o áudio não remove nenhuma informação necessária para completar um Desafio de Ilha.
- Após remapear a sinalização de sincronia para outra tecla, o contador e a janela de 5 s continuam funcionando com o mesmo prazo.
- A janela de 5 s nunca é estendida por configuração de acessibilidade; extensões de prazo são [NOTA PARA O DESIGNER] e exigem decisão de balanceamento na seção 11.

### 12.5. Onboarding aprendendo fazendo

O primeiro Desafio de Ilha é a aula. A ilha de lógica do início pede três alavancas na ordem correta: ao aproximar-se, a alavanca é destacada e o ícone da tecla de interação aparece — o jogador aprende a interagir interagindo. A ilha seguinte exige duas placas no mesmo instante, ensinando a sinalização e a janela. A terceira ensina o painel de recursos ao transformar coleta em número visível. Nenhuma dessas etapas usa um tutorial longo.

**Critérios de aceitação do onboarding**

- Nenhum tutorial em vídeo ou texto com mais de três linhas aparece nos primeiros 2 minutos de partida.
- A dica contextual com o ícone de F aparece em até 1 s quando o jogador chega a 2 m da primeira alavanca e desaparece após o primeiro acerto.
- Um jogador que complete as três alavancas sem ajuda externa tenha executado movimento, interação e leitura de destaque — medido por eventos de telemetria (seção 17).
- Erro na ordem das alavancas reinicia apenas a sequência daquela ilha, nunca a partida.

## 13. Direção de Arte e Áudio

### 13.1. Rastreabilidade e recusa da base estética dos recursos de Paragon

Cadeia de rastreabilidade: fantasia (tomada coletiva dos meios de produção, sem magia — seção 5) → pilares (Materialismo Histórico, Luta Decolonial e Ação Coletiva — seção 4) → loop (ciclo de jogo: cada ilha é um microssistema de desafio — seção 7) → decisão desta seção: greyboxing (protótipo com formas geométricas sem textura) e baixa poligonalidade no MVP.

Os recursos AAA (jogos de grande orçamento e acabamento industrial) de fantasia tecnológica e ficção científica do pacote liberado pelo jogo Paragon **não são a base visual do Insurgência**. As fontes são diretas: esse pacote carrega "uma estética de 'fantasia épica/tecnológica' muito ligada aos padrões de consumo da indústria tradicional"; "modelos hiper-realistas de ficção científica e fantasia medieval não se alinham com a estética materialista e decolonial"; e usar um "herói de armadura tecnológica brilhante" quebra a imersão de um jogo sobre Camponeses e Artesãos — risco de a estética "colonizar" a ideia.

Uso residual, permitido apenas como substituto temporário (esqueleto de animação e lógica de colisão em teste), nunca como direção visual, e removido antes do MVP. Essa é uma decisão de pilar, não de orçamento: adotar a estética seria importar a fantasia épica que o projeto recusa.

Justificativa filosófica: a mensagem da mecânica deve brilhar mais que o gráfico. Formas simples permitem que a cooperação — e não o brilho do modelo — seja o que se lê na tela.

### 13.2. Greyboxing e baixa poligonalidade no MVP

- **Greyboxing (protótipo com formas geométricas sem textura):** cubos e cilindros representam personagens, pontes, alavancas, placas e máquinas. As fontes recomendam explicitamente "cubos e cilindros sem textura para testar mecânicas" e, no primeiro passo, "substitua os heróis por cubos coloridos".
- **Baixa poligonalidade (poucos polígonos — faces que compõem o modelo 3D — por objeto):** bibliotecas de recursos FOSS (Software Livre e de Código Aberto), como as do portal de recursos livres Kenney.nl, citadas nas fontes. Carregam mais rápido e deslocam o protagonismo para a mecânica de cooperação.
- A ausência de textura não é falha acabamento: é o que permite ler contorno, cor de estado e posição dos elementos do desafio sem ruído visual.

**Critérios de aceitação da direção visual do MVP**

- 100% dos personagens jogáveis do MVP usam formas geométricas sem textura aplicada.
- Nenhum modelo do pacote de Paragon aparece em captura de tela do MVP; seu uso, se houver, limita-se a esqueleto e colisão em teste interno.
- Cada ilha é reconhecível apenas por silhueta e cor, sem depender de letreiro [HIPÓTESE].
- Cada modelo carrega em até 2 s em conexão de 5 Mbps [HIPÓTESE].

### 13.3. Paleta por ilha e elementos sonoros

As cores e os sons específicos da tabela abaixo são propostas derivadas do princípio materialista e da recusa da trilha épica; as fontes não definem paletas nem sonoridades, portanto cada linha é [HIPÓTESE].

| Ilha | Paleta — [HIPÓTESE] | Elementos sonoros — [HIPÓTESE] |
| :--- | :--- | :--- |
| Floresta (Rota Superior / Ilhas da Autossuficiência) | Verde-água, terracota e cinza-pedra | Passos em terra batida, estalo de galho, respiração; alarme de forças de segurança quando um segundo aliado entra na ilha |
| Cidade (Rota Central / Ilhas da Ação Direta) | Cinza-concreto, ocre e faixa vermelha de sinalização | Multidão abafada, alto-falantes de propaganda, passos em calçamento, clique das placas ao sincronizar |
| Industrial (Rota Central / A Ocupação da Fábrica) | Ferrugem, grafite e amarelo de segurança | Máquinas em ciclo, metal, vapor; o zumbido cessa quando a sabotagem termina |
| Delta do Comércio (Rota Inferior) | Verde-pântano, barro e azul de água rasa | Água rasa, madeira de pontes, vozes de negociação; silêncio quando a Propaganda converte súditos |

> [NOTA PARA O DESIGNER] A Ilha Central ainda não tem cor definida nas fontes. Proposta [HIPÓTESE]: paleta neutra de gesso, com a cor da equipe que a ativa dominando a ilha no último segundo da partida. A seção 8 decide.

### 13.4. Sonoridade materialista, sem trilha épica

Não há trilha orquestral épica: ela reintroduziria a fantasia épica rejeitada. O som é diegético (nascido de dentro do mundo do jogo, e não de uma trilha externa): máquinas, multidão, passos, papel de panfleto, gás lacrimogêneo, metal. A recompensa sonora da Propaganda é o silêncio — quando os súditos param de atacar, as vozes de combate se apagam por 2 s [HIPÓTESE]. O dano do gás lacrimogêneo é válvula e tosse, sem música de derrota.

**Critérios de aceitação da direção sonora**

- Nenhum arquivo de música orquestral é carregado no MVP.
- Todo evento de Desafio de Ilha tem ao menos um som diegético correspondente e uma legenda textual (seção 12.4).
- A conversão de súditos por Propaganda reduz o volume de combate em até 0,5 s [HIPÓTESE].
- O retorno de canalização interrompida usa apenas um estalo mecânico de 0,3 s [HIPÓTESE], sem alerta musical.

### 13.5. Princípios de representação

- **Sem estereótipos:** personagens são definidos pela função material (Artesão, Camponês, Guerreiro, Intelectual, Agitador, Aliança Mercantil), não por traços étnicos, de gênero ou religiosos. Nenhum grupo aparece como naturalmente servo ou naturalmente violento.
- **Sem "pornografia da pobreza"** (estetização da miséria para impacto visual, sem agência para as pessoas representadas): a Comuna não é suja por decoração. A precariedade aparece como condição material a ser transformada — e como alvo da ação coletiva — nunca como cenário exótico para consumo.
- **Sem heroificação vazia** (exaltação do indivíduo isolado como salvador): a câmera e a luz não isolam um "escolhido". O destaque visual vai para o grupo e para a ilha libertada. Isso serve diretamente ao pilar Ação Coletiva.

**Critérios de aceitação dos princípios de representação**

- Nenhum modelo, nome ou texto do MVP associa uma função a etnia, gênero ou religião.
- A tomada de uma ilha enquadra pelo menos dois personagens aliados [HIPÓTESE]; não há cena de "salvador único".
- Nenhuma função recebe aura, brilho exclusivo ou iconografia de "escolhido".
- A quebra de qualquer princípio desta subseção bloqueia a aprovação de um Módulo (seção 15).

### 13.6. Cooperação, representação e responsabilidade

Rastreabilidade: fantasia (tomada coletiva dos meios de produção — seção 5) → pilares (Ação Coletiva, Luta Decolonial, Materialismo Histórico — seção 4) → desafios (entrada sincronizada e funções complementares — seção 8) → decisão desta subseção: cooperação por interdependência de objetivos, não por punição de presença; representação como prática jogável, não texto decorativo. Pesquisa verificada em 19/11/2026.

**Interdependência é mecânica, não texto.** Em *Never Alone / Kisima Inŋitchuŋa* (Upper One Games/CITC, primeiro estúdio indígena a lançar jogo próprio), Nuna é forte e carrega/escala; a raposa é leve e cabe em espaços pequenos — nenhuma das duas avança sozinha. O projeto registrou co-jogo intergeracional e 24 filmes com anciões. Regra derivada: todo Desafio de Ilha é insolúvel por uma função única e solúvel pela diferença de situação entre funções, nunca por diferença de poder. https://www.theguardian.com/technology/2014/sep/29/never-alone-alaskas-indigenous-game-never-alone-teaches-cooperation-through-stories

**Quatro átomos da cooperação (Lauret, Ubisoft Quebec, 2018).** Gate (porta compartilhada; colaboração opcional), Comfort (difícil o bastante para um "superplayer" — jogador muito habilidoso — resolver sozinho, como em Left 4 Dead), Class (habilidades fixas e únicas, como em Keep Talking and Nobody Explodes) e Job (todos têm as mesmas habilidades, mas ninguém avança sozinho; trocar de tarefa a qualquer momento gera empatia, como em Portal 2). Decisão: adotar **Job** como núcleo — assimetria de SITUAÇÃO, não de poder — com Gate e Comfort como reforço. A coordenação nasce da interdependência de objetivos, não de punição. Tradução no jogo: a entrada sincronizada de 3 aliados em 5 s (seção 8.3) e a Greve (Artesão + Guerreiro + Intelectual, seção 8.5) são tarefas intercambiáveis dentro de um objetivo comum. https://www.gamedeveloper.com/design/the-four-atoms-of-cooperative-video-games

**A mecânica é a mensagem.** Carneiro e Dias (ANTARES 12(28), 2020) mostram que os estudos de jogos (game studies) pós-coloniais negligenciam estrutura mecânica e agência em favor de narrativa e representação — "turismo identitário". A tese política do Insurgência deve morar na economia e nos desafios, não apenas nos textos. https://sou.ucs.br/etc/revistas/index.php/antares/article/download/9209/4482/35299

**O problema da visão divina.** *Half-Earth Socialism* (Trust/Verso, 2022): jogos são fantasias de controle unilateral, o que conflita com planejamento democrático — o parlamento do jogo pode derrubar o jogador. Distinguir fricção legítima (adversário difícil, boss) de fricção ilegítima (decisões ignoradas). Aplicação: decisões coletivas no Insurgência são regras jogáveis — quórum, mandato, rodízio —, nunca cortinas. https://spaceandtim.es/projects/half_earth_socialism/

**Vitória compartilhada.** *STRIKE!* (TESA Collective, 2020), tabuleiro cooperativo criado com o sindicato Jobs with Justice: todos ganham ou perdem juntos. A condição de vitória do Insurgência (seção 6) é de equipe, não placar individual. https://www.tesacollective.com/teamed-up-with-labor-movement-make-strike-game-worker-rebellion/

**Riscos de representação.** Linderoth, Chapman e Deterding (Routledge, 2023), *The Limits of 'Serious' Play*: "boas intenções não salvam o designer de criar algo lido como insensível"; o "aha" pretendido não é garantido; "Slave Tetris" foi removido após protestos. Protocolo OFELAŠ (International Sámi Film Institute): "Nada sobre nós sem nós"; quatro perguntas — Responsabilidade, Reciprocidade, Consciência, Intenção — e revisão por especialistas. Aplicação: nenhum Módulo (seção 15) que represente um grupo é aprovado sem essa revisão. https://spiral.imperial.ac.uk/server/api/core/bitstreams/d61c28a6-8dbb-4ad7-b359-178cbdbb8a18/content ; https://isfi.no/guidelines/

**Diretriz final.** Nunca fazer o jogador ENCENAR opressão esperando reflexão automática. Funções das Ilhas são práticas (coleta, sabotagem, negociação, escolta — seção 8), não "culturas" essencializadas. Saber cultural é ação jogável, não texto decorativo.

**Critérios de aceitação — cooperação, representação e responsabilidade**

- A entrada sincronizada usa interdependência de objetivos (Job) e não punição por presença: o gás lacrimogêneo (seção 8.3) incide sobre entrada escalonada, nunca sobre aliados coordenados dentro da janela de 5 s.
- Todo Desafio de Ilha do MVP exige ao menos duas funções — inclusive a Ilha Central do MVP, cuja canalização de 4 s exige Artesão e Guerreiro vivos e presentes, não "qualquer membro" (seção 6); um jogador isolado falha o mesmo desafio em 3 tentativas [HIPÓTESE de limiar de teste].
- Nenhum aliado sozinho conclui a Greve: sabotagem (8 s), barricada (10 s) e negociação (3 perguntas) devem estar ativas no mesmo intervalo (seção 8.5).
- A tese política aparece na economia: ao menos 1 ilha por rota altera recursos ou produção inimiga (seções 8.2–8.4), não apenas diálogo.
- Decisões coletivas são regras do Núcleo, não texto: quórum de 2/3 dos aliados, mandato de 60 s e rodízio de funções [HIPÓTESE].
- Nenhuma decisão do jogador é descartada sem efeito jogável; fricção ilegítima (decisão ignorada) bloqueia a aprovação de um Módulo (seção 15).
- Todo Módulo que represente um grupo passa pelas quatro perguntas OFELAŠ e por revisão de especialistas do grupo antes da aprovação (seção 15).
- Nenhuma mecânica pede ao jogador que encene opressão; nenhuma função é descrita como "cultura" essencializada; todo saber cultural verificado é vinculado a uma ação jogável.
## 14. Arquitetura Técnica e Viabilidade

Esta seção converte os pilares em requisitos verificáveis. A separação **Núcleo + Módulos** e a topologia federada servem à **Soberania do Jogador**: ninguém precisa de permissão para alterar regras ou hospedar uma partida. A simulação autoritativa e o anti-cheat servem à **Ação Coletiva**: nenhum indivíduo pode decidir sozinho o resultado de uma ocupação. A escolha de assets CC0 (Creative Commons Zero — domínio público) e do *greyboxing* (protótipo com formas geométricas sem textura) serve à **Luta Decolonial**: recusar a estética de fantasia épica proprietária. A validação de ações materiais no servidor serve ao **Materialismo Histórico**: o sistema reage a custo, alcance e tempo de recarga, não a "magia" declarada pelo cliente.

### 14.1 Requisitos técnicos de nível GDD

Os números abaixo são metas de engenharia, não promessas de marketing. Onde o valor é chute de design, ele está marcado; onde é inferência, está marcado como `[SUPOSIÇÃO]`.

| Requisito | Meta | Intenção e marcador |
| :-- | :-- | :-- |
| Plataformas alvo | Cliente: Windows e Linux. Servidor: Linux *headless* (sem interface gráfica). macOS e navegador (HTML5 — linguagem de marcação para páginas web) como alvos secundários. | `[SUPOSIÇÃO]` Priorizar o par que permite hospedagem comunitária barata; o navegador depende da decisão de linguagem (14.6). |
| Desempenho | 60 FPS (quadros por segundo) sustentados por **10 min** de carga contínua; mediana ≥ 58 FPS e 1º percentil (1% dos quadros mais lentos) ≥ 45 FPS. | `[HIPÓTESE]` O período de 10 min é um teste de estabilidade (vazamento de memória, acúmulo de partículas), não a duração da partida. |
| Carga de referência | 10 jogadores (5v5), 30–40 **Súditos** e até 20 personagens não jogáveis por ilha. | `[HIPÓTESE]` Define o cenário do teste de desempenho acima. |
| Latência | RTT (tempo de ida e volta, do inglês *round-trip time*) ≤ **80 ms** na mesma região; controle ainda responsivo com **150 ms** injetados. | `[HIPÓTESE]` Faixa típica de servidores comunitários regionais; acima disso a predição local mantém a jogabilidade, sem prometer competição intercontinental. |
| Orçamento de assets | ≤ 100 MB de assets por módulo no cliente; texturas ≤ 2048 × 2048 pixels; ≤ 10 mil triângulos por personagem. | `[HIPÓTESE]` Compatível com *low-poly* (baixa poligonalidade) e com download rápido em servidores pequenos. |
| Pacote de módulo | ≤ **25 MB** (personagens + mapa + regras + assets); *hash* (resumo criptográfico) SHA-256 de 32 bytes transmitido no *handshake* (troca inicial de parâmetros de conexão). | `[HIPÓTESE]` O limite cabe em links comunitários; o *hash* é escolha de projeto, não estimativa. |

**Critérios de aceitação:**

- Executar a carga de referência por 10 min em hardware de referência: mediana ≥ 58 FPS, 1º percentil ≥ 45 FPS, sem crescimento de memória.
- Medir RTT ≤ 80 ms cliente–servidor na mesma região; com 150 ms injetados, o movimento permanece contínuo (sem saltos visíveis).
- O módulo de referência empacota em ≤ 25 MB e é carregado sem recompilar o **Núcleo**.

### 14.2 Arquitetura Núcleo + Módulos

**Contrato conceitual.** O **Núcleo** é um executável sem nenhum personagem, ilha ou balanceamento: contém rede, simulação, validação e as Interfaces de Programação de Aplicação (APIs — *Application Programming Interfaces*) de Habilidade, Personagem, Item e **Desafio de Ilha**. Os **Módulos** são pacotes declarativos lidos na inicialização: `PacoteDePersonagens` (em JSON — formato de dados em texto), `PacoteDeMapa`, `RegrasDoServidor` (arquivo `.cfg` de configuração) e assets. Assim, a comunidade cria variações sem tocar no código do Núcleo.

**Handshake e versionamento.** Na conexão, cliente e servidor trocam versão do protocolo, identificador do módulo e o *hash* SHA-256 do conjunto de conteúdo. Constantes de protocolo separadas — como o OpenRA faz com `Mod`, `Version`, `AuthToken` e `OrdersProtocol` — evitam que uma mudança de regra pareça uma mudança de rede. Incompatibilidade recusa a conexão com erro legível; nunca baixa código silenciosamente. O OpenRA usa ainda *hash* de mapa e limite de 128 kB por pacote, referência para o nosso tamanho de mensagem. Fontes: [OpenRA — ProtocolVersion.cs](https://raw.githubusercontent.com/OpenRA/OpenRA/bleed/OpenRA.Game/Server/ProtocolVersion.cs) e [configuração declarativa do OpenRA](https://deepwiki.com/OpenRA/OpenRA/4-data-driven-configuration).

**Configuração orientada a dados.** MiniYaml (formato de configuração declarativa com herança, do OpenRA) e *modifiers* (modificadores orientados a dados, como em For Honor) são os modelos: o mesmo motor roda conteúdos diferentes sem recompilação. Fontes: [For Honor — GDC 2017](https://gdcvault.com/play/1024580/Modify-Everything-Data-Driven-Dynamic).

**Recarga e forks.** Regras só são recarregadas **entre partidas**: ao iniciar, o servidor congela o módulo, o que garante que o *replay* (reexecução da partida) seja reproduzível. Um `fork` (bifurcação do código) da comunidade altera dados do Módulo, nunca o Núcleo; mudanças no Núcleo seguem o processo da Fundação (seção 15). Para poucas dezenas de entidades por ilha, composição de cenas mais *resources* (recursos reutilizáveis do Godot) basta — ECS (*Entity-Component-System*, padrão de entidades, componentes e sistemas) só compensa com dezenas de milhares de objetos. Fonte: [Linietsky — por que Godot não é baseado em ECS](https://godotengine.org/article/why-isnt-godot-ecs-based-game-engine/).

**Critérios de aceitação:**

- Trocar o módulo A pelo B entre partidas recarrega as regras sem recompilar o Núcleo.
- Cliente com versão de protocolo diferente é recusado no *handshake* com código de erro legível; *hash* divergente recusa o download.
- Alterar um valor em `RegrasDoServidor.cfg` afeta a partida seguinte, nunca a partida em andamento.
- O *diff* (comparação) de um fork publicado contra o Núcleo é vazio: apenas arquivos de Módulo mudaram.

### 14.3 Topologia federada

Qualquer pessoa baixa o binário `dedicated_server` e hospeda uma partida. O servidor da Fundação é **um entre muitos**: é anfitrião, não autoridade de regras. O catálogo de servidores (*server browser*, lista de descoberta) é **opcional** — clientes podem conectar por endereço direto (IP — Protocolo da Internet — e porta). Se o catálogo e o servidor da Fundação saírem do ar, partidas em andamento e servidores comunitários continuam: é o modelo do e-mail, em que o Gmail pode cair sem que o e-mail deixe de existir. Essa resiliência é a tradução técnica da soberania do jogador; a governança do catálogo pertence à seção 15.

**Critérios de aceitação:**

- Desligar catálogo e servidor da Fundação não interrompe partidas em andamento em dois servidores comunitários independentes.
- Servidor recém-criado aceita conexão direta por IP:porta sem registro prévio no catálogo.
- O cliente exibe explicitamente o *hash* do módulo do servidor antes de entrar, para que o jogador saiba quais regras vai encontrar.

### 14.4 Modelo de simulação e latência

**Servidor autoritativo.** O cliente envia **inputs** (comandos), nunca posição; o servidor é a fonte única da verdade. Cada comando carrega número de sequência e o cliente mantém uma janela deslizante de comandos não confirmados, com histórico de posições marcado por *timestamp* (registro de tempo). A reconciliação usa o "último comando processado" informado pelo servidor; o código de movimento é compartilhado cliente–servidor para que a predição local coincida com a simulação remota. Fontes: [Gambetta — predição e reconciliação](https://www.gabrielgambetta.com/client-side-prediction-server-reconciliation.html) e [Bernier/Valve — compensação de latência](https://www.gamedevs.org/uploads/latency-compensation-in-client-server-protocols.pdf). O modelo do Valve usa `usercmd` com milissegundos, ângulos, movimento e botões; adotamos a mesma forma. A compensação de atraso ("atirar por trás da esquina") é consequência desse histórico com *timestamp*.

**Passo e snapshots.** Simulação em passo fixo de **30–60 Hz** `[HIPÓTESE]` e *snapshots* (fotos do estado da partida) a **15–20 Hz** `[HIPÓTESE]` com compressão diferencial (envio apenas do que mudou desde o *snapshot* anterior). Entidades remotas são renderizadas com interpolação de **100–200 ms**, isto é, cerca de um passo no passado; essa janela absorve perda de pacotes sem tremer a imagem. Fonte: [Gambetta — interpolação de entidades](https://gabrielgambetta.com/entity-interpolation.html).

**Área de interesse por ilha.** Cada ilha é uma bolha de causalidade: o servidor aplica *culling* (remoção de entidades fora da área relevante) por distância e oclusão (bloqueio de visão por obstáculos). Isso economiza banda **e** evita vazamento de informação — o *wallhack* (trapaça que revela inimigos através de paredes) deixou de existir se o dado nunca chega ao cliente. O VELVET propõe raio variável; o Godot expõe filtros de visibilidade em `MultiplayerSynchronizer`. Fontes: [VELVET — área de interesse](https://www.ee.ucl.ac.uk/lcs/previous/LCS2011/LCS1121.pdf) e [documentação de MultiplayerSynchronizer](https://docs.godotengine.org/en/latest/classes/class_multiplayersynchronizer.html).

**Transporte.** `MultiplayerAPI` e `MultiplayerPeer` com ENet (biblioteca sobre UDP — Protocolo de Datagramas de Usuário), WebRTC (comunicação em tempo real na web) e WebSocket (canal bidirecional sobre TCP — Protocolo de Controle de Transmissão). O servidor é sempre o ID 1; RPCs (chamadas de procedimento remoto) exigem o mesmo caminho de nó (*NodePath*) e a mesma assinatura nos dois lados. HTML5 não tem UDP cru — logo, cliente web depende de WebRTC/WebSocket — e portas UDP precisam de redirecionamento (*port forwarding*) no roteador do anfitrião. O aviso oficial é decisivo: **nunca confiar no cliente**. Fonte: [Godot — multiplayer de alto nível](https://docs.godotengine.org/en/stable/tutorials/networking/high_level_multiplayer.html).

**Critérios de aceitação:**

- Com 120 ms de RTT injetados, o jogador se move sem atraso perceptível; após a reconciliação, o erro de posição é ≤ 0,5 m `[HIPÓTESE]`.
- Dois clientes veem o mesmo evento de **Desafio de Ilha** com diferença ≤ 200 ms (janela de interpolação).
- Jogador na **Ilha Central** não recebe dados de jogador na **Rota Inferior / Delta do Comércio**; verificado por inspeção de pacotes.
- Cliente modificado que envia posição em vez de input é recusado pelo servidor.

### 14.5 Anti-cheat sem kernel

**Validação no servidor.** Todo limite, custo, *cooldown* (tempo de recarga), alcance e a janela de sincronia de 5 s dos **Desafios de Ilha** é checado no servidor. O servidor recalcula dano e efeitos; a palavra do cliente não vale.

**Replay e adjudicação.** Cada partida é gravada como sequência de inputs mais semente e pode ser reexecutada de forma determinística. O OpenRA usa *lockstep determinístico* (passos idênticos em todas as máquinas) com `SharedRandom` semeado pelo servidor, *sync hashes* (resumos de sincronização) e *replay* no servidor para adjudicar disputas. Fonte: [OpenRA — processamento de ordens e rede](https://deepwiki.com/OpenRA/OpenRA/3.4-order-processing-and-networking).

**Detecção pós-partida e revisão humana.** A detecção roda sobre telemetria (dados coletados remotamente) **do servidor**, não do cliente, que é manipulável. O Xonotic detecta apenas no servidor, com médias móveis de *speedhack* (aceleração ilícita), *snap-aim* (mira que salta), *strafebot* e *snapback*, sem *driver* (controlador em nível de sistema) de *kernel* (núcleo do sistema operacional). Fonte: [Xonotic — anticheat.qc](https://rsync.xonotic.org/release/Xonotic-high/source/qcsrc/server/anticheat.qc). O XGUARDIAN (USENIX Security 2026) detecta *aim-assist* (assistência de mira) por *replay* pós-partida com 90,7% de recall e 4,1% de falsos positivos, argumentando justamente que a telemetria de cliente é manipulável. Fonte: [XGUARDIAN — USENIX Security 2026](https://www.usenix.org/system/files/conference/usenixsecurity26/sec26_prepub_zhang-jiayi.pdf). Nenhuma punição é automática: suspeito vira caso para **revisão humana**.

**Rejeição explícita.** *Drivers* de kernel invasivos (como o Vanguard da Riot) são incompatíveis com software livre e com a soberania do jogador: o projeto não instala, não exige e não recomenda nenhum.

**Critérios de aceitação:**

- Cliente modificado que declara "causo 500 de dano" é corrigido pelo servidor para o dano real da habilidade.
- Replay da mesma partida reproduz o mesmo *hash* final em duas execuções independentes.
- Relatório pós-partida marca suspeito como "revisão humana"; nenhuma punição é disparada apenas por estatística.
- Nenhum componente do projeto instala *driver* em nível de kernel.

### 14.6 Motor e ferramentas

**Versão.** Godot **4.7.2** é a versão estável (18/08/2026); **4.8 beta 4** é pré-release (versão prévia) e não deve ser usada no protótipo. Fontes: [versões oficiais](https://raw.githubusercontent.com/godotengine/godot-website/master/_data/versions.yml) e [notas da versão 4.7](https://godotengine.org/releases/4.7/).

**Licença.** MIT (licença permissiva, originalmente do *Massachusetts Institute of Technology*) cobre motor, editor e *templates* (modelos) de exportação. Jogos criados **não herdam** a licença; componentes de terceiros são listados em `COPYRIGHT.txt`. As marcas "GODOT", "GODOT ENGINE" e o logotipo **não** são MIT e seguem a política de marcas. Fontes: [licença do Godot](https://godotengine.org/license/), [cumprimento de licenças](https://docs.godotengine.org/en/stable/about/complying_with_licenses.html) e [política de marcas](https://godot.foundation/policies-and-procedures/trademark-policy).

**Servidor e linguagem.** Servidor dedicado *headless* existe desde 4.0 (`--headless`, com *driver* Dummy); o preset "Export as dedicated server" adiciona a *feature tag* (marcador de funcionalidade de exportação) `dedicated_server`, detectável com `OS.has_feature("dedicated_server")`. Recomendações oficiais de hospedagem: `systemd` (gerenciador de serviços do Linux) ou Docker (plataforma de contêineres). Fonte: [exportação para servidores dedicados](https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_dedicated_servers.html).

C# exige build .NET e *templates* .NET: desktop é suportado, Android/iOS são experimentais e **web não é suportado em 4.x**. Fonte: [C# no Godot](https://docs.godotengine.org/en/stable/tutorials/scripting/c_sharp/index.html). Decisão: protótipo em **GDScript** (linguagem de script do Godot), sem etapa de build; C# fica condicionado a abrir mão do cliente web e a um ganho medido.

**Correção obrigatória à fonte.** A afirmação de que "Godot é o único motor que realmente respeita a licença MIT" é **falsa**. Bevy 0.20.1 é MIT/Apache-2.0, Urho3D é MIT, libGDX é Apache-2.0 e Defold tem licença própria derivada de Apache-2.0. Substituir por: **Godot é um motor FOSS (Software Livre e de Código Aberto) sob MIT que reúne editor, exportação *headless*, C# e comunidade madura — critérios decisivos para o protótipo —, mas não é o único motor MIT/FOSS disponível.**

**Critérios de aceitação:**

- O binário do servidor roda com `--headless` em Linux, sem GPU (placa gráfica) ou driver gráfico.
- `COPYRIGHT.txt` é preservado; nenhuma arte do projeto usa o logotipo do Godot sem autorização.
- O protótipo compila sem etapa de build .NET; se C# for adotado, o cliente web sai do escopo até nova avaliação.

### 14.7 Avaliação de codebases e assets

Verificação feita via GitHub API e páginas oficiais em **19/11/2026**. O princípio orientador da seção 14.2 se aplica aqui: infraestrutura pode ser estudada, gameplay tradicional é descartado.

**Tabela 1 — Codebases avaliados**

| Projeto | Situação verificada (19/11/2026) | Licença | Veredito | URL |
| :-- | :-- | :-- | :-- | :-- |
| nival/Prime-World | C++ e engine própria confirmados; **não há código de servidor no repositório** (58.431 entradas, nenhum caminho com "server"). Último commit 19/08/2026. | Proprietária, não-OSS: uso comercial proibido, licença revogável com cessação em 3 dias, modificações grátis obrigatórias, multa mínima de €10.000 ou 80% da receita. | Descartar como base; no máximo referência de editor de dados. | [Link](https://github.com/nival/Prime-World) · [LICENSE.md](https://github.com/nival/Prime-World/blob/main/LICENSE.md) |
| yasgamesdev/MOBA_CSharp_Unity | Cerca de 3.000 linhas (autoafirmação); cliente Unity C#, servidor console C# via Mono (tempo de execução .NET multiplataforma), ENet/UDP, MessagePack (formato binário compacto), *snapshots*, NavMesh (malha de navegação), visão compartilhada e arbustos. Último commit 06/03/2019. | MIT | **Melhor referência de infraestrutura**; descartar gameplay (torres e *minions*/súditos). | [Link](https://github.com/yasgamesdev/MOBA_CSharp_Unity) |
| tammukul/UNION-OpenSource-MOBA | Unity + Photon PUN + PlayFab; SDKs (conjuntos de ferramentas de desenvolvimento) obsoletos. Último commit 10/07/2015. | Apache-2.0 | Baixa utilidade; estudo de RPC e sincronização. | [Link](https://github.com/tammukul/UNION-OpenSource-MOBA) |
| DevZhav/unity-moba | Unity C# + 8 arquivos Go + RethinkDB; microsserviços de conta e sala. Último commit 08/09/2018. | GPL-3.0 (`copyleft`: exige compartilhar código derivado sob a mesma licença) | Referência de serviços; obrigações de `copyleft` se copiado. | [Link](https://github.com/DevZhav/unity-moba) |
| yasgamesdev/OpenMOBA | UE4 (Unreal Engine 4), somente Blueprints (programação visual); último commit 22/02/2018. | MIT cobre o código, mas assets UE4 seguem o EULA (Contrato de Licença de Usuário Final) da Epic. | Descartar: engine proprietária e projeto parado. | [Link](https://github.com/yasgamesdev/OpenMOBA) |
| danielholmes839/GoBA2 | Refactor genérico de framework em tempo real (App, fila/sala, WebSocket); **sem evidência de "milhares de conexões"**; README especula "may create a polished version in the future". Último commit 08/07/2022. | **Sem licença** (todos os direitos reservados) | Referência conceitual; não reutilizável sem autorização. | [Link](https://github.com/danielholmes839/GoBA2) |
| RichardAragon/MOBASwarmAgents | **Não é Sakana AI** (autor sem vínculo); script Python de ~70 linhas (DEAP, PSO, GPT-2) mais pasta `Unity_Setup`; commit 18/06/2024. | MIT | Quase nula; exercício didático. | [Link](https://github.com/RichardAragon/MOBASwarmAgents) |
| Assets Paragon | Epic anunciou US$ 12 mi em 2018 + US$ 5 mi depois; uso comercial **somente em projetos Unreal Engine**, sem usar a marca PARAGON. Predecessor tem universo e heróis próprios e segue ativo (patch 1.18, 03/11/2026). | EULA de Marketplace ("conteúdo exclusivo da Unreal Engine"), não é código aberto. | Descartar: armadilha legal e estética incompatível. | [Fórum Epic](https://forums.unrealengine.com/t/12-million-worth-of-paragon-assets-released-updated/108140) |
| Xonotic / OpenRA / Veloren | Comunidades ativas (commits em 18/11, 17/11 e 10/11/2026). | GPLv3+ / GPL-3.0 / GPL-3.0 + CC-BY-SA-4.0 (assets) | Referência de *netcode* (código de rede), pipeline de assets e governança — não de gameplay MOBA. | [Xonotic](https://github.com/xonotic/xonotic) · [OpenRA](https://github.com/OpenRA/OpenRA) · [Veloren](https://github.com/veloren/veloren) |

**Tabela 2 — Fontes de assets livres**

| Fonte | Licença verificada | Restrição e cuidado | Uso no Insurgência | URL |
| :-- | :-- | :-- | :-- | :-- |
| Kenney | CC0 / domínio público, uso comercial livre, atribuição opcional. | Conferir o arquivo de licença incluído; não usar o logotipo isolado. | Fonte principal de *low-poly* após o *greybox*. | [Link](https://kenney.nl/assets) |
| OpenGameArt | Licença **por item** (CC0, CC-BY 3/4, CC-BY-SA, OGA-BY, GPL 2/3, WTFPL). | Filtrar na busca; **previews não são licenciados**. | Usar só após conferir a licença do item. | [FAQ](https://opengameart.org/content/faq) |
| Poly Pizza | Creative Commons **por modelo no momento do download**, não CC0 global. | Proibido *scraping* (raspagem automatizada) ou treino de IA sem permissão. | Admissível com registro individual. | [Termos](https://poly.pizza/docs/tos) |
| Quaternius | **QAL v1.0 (28/08/2026), não é CC0.** | Uso pessoal, educacional e comercial sem crédito; vedado revender ou redistribuir os assets como assets, mesmo modificados. | Admissível como asset integrado; nunca como pacote redistribuível. | [Licença](https://quaternius.com/license.html) |
| itch.io (filtro CC0) | CC0 1.0 declarado pelo item; 182 itens em nov/2026. | Combinar com a tag *low-poly*; conferir a página individual. | Complemento eventual. | [Link](https://itch.io/game-assets/assets-cc0/tag-cc0) |
| "OpenGL Fonts" | Não é uma fonte canônica única; são fontes bitmap GLUT sob licença XFree/MIT-style, com status legal "um pouco vago". | Evitar; preferir CC0 ou SIL OFL (licença de fontes livres). | **Não usar.** | Sem URL canônica. |

**Correções obrigatórias às fontes (o que a fonte afirma e o que foi verificado):**

- "O código do servidor de batalha do Prime World está disponível" — **falso**: nenhum caminho com "server" entre 58.431 entradas; além disso a licença proíbe uso comercial, é revogável com 3 dias, exige modificações grátis e prevê multa mínima de €10.000 ou 80% da receita. [LICENSE.md](https://github.com/nival/Prime-World/blob/main/LICENSE.md).
- "Godot é o único motor que respeita MIT" — **falso**; correção integral em 14.6.
- "Sakana AI — MOBA Swarm Agents é o estado da arte" — **falso**: RichardAragon não tem vínculo com a Sakana AI; o repositório é um exercício didático de ~70 linhas.
- "Assets Paragon como *placeholder* são seguros" — **falso**: uso comercial restrito à Unreal Engine e sem uso da marca PARAGON; Predecessor tem propriedade intelectual própria e permanece ativo. [Fórum Epic](https://forums.unrealengine.com/t/12-million-worth-of-paragon-assets-released-updated/108140).
- "GoBA2 gerencia milhares de conexões" — **sem evidência**; o README apenas especula uma versão futura, e o repositório não tem licença.
- "Começar com MOBA_CSharp_Unity como base do jogo" — **corrigido**: serve apenas como referência de infraestrutura; todo o gameplay tradicional (torres, *minions*, dano por segundo) é descartado para não gerar débito técnico.
- "OpenMOBA é opção viável" — **falso**: engine proprietária, parada desde 2018, e os assets continuam sob EULA da Epic.

**Decisão explícita.** Usar `yasgamesdev/MOBA_CSharp_Unity` **somente como referência de infraestrutura** (rede, servidor desacoplado, *snapshots*); descartar todo gameplay tradicional; production visual começa em **greybox + assets Kenney CC0**.

**Critérios de aceitação:**

- Nenhum asset entra no repositório sem entrada no arquivo de atribuição (fonte, licença, data do download e restrições).
- Todo item de OpenGameArt e itch.io tem licença confirmada na página individual no momento do download; *previews* não são considerados licenciados.
- Nenhum asset Paragon/UE-only, nenhum código de Prime-World ou GoBA2 e nenhum logotipo Kenney isolado aparece no cliente.
- O arquivo de atribuição é validado antes do empacotamento de cada módulo.

### 14.8 Decisão técnica final e justificativa

Adota-se **Godot-first (4.7.2, MIT, servidor *headless*)**, **Núcleo + Módulos declarativos**, **servidor autoritativo** e **anti-cheat no servidor com *replay* e revisão humana**. É a única combinação que satisfaz simultaneamente soberania (forks sem recompilar), viabilidade de protótipo (GDScript; cenas correspondem às ilhas) e confiança (o servidor valida, o cliente apenas solicita). Godot não é o único motor MIT — ver 14.6 —, mas é o que reúne editor, exportação *headless*, C# opcional e comunidade madura neste escopo.

`[NOTA PARA O DESIGNER]`

- O cliente web (HTML5) entra no protótipo? Se sim, C# fica fora em 4.x e GDScript é obrigatório; se não, a decisão de linguagem pode ser reavaliada após o primeiro teste de carga.
- O catálogo de servidores será um serviço da Fundação ou um conjunto de instâncias autônomas? Decisão compartilhada com a seção 15.
- Fixar 30 Hz ou 60 Hz de simulação após medir o custo com 10 jogadores e 40 Súditos; os valores atuais são `[HIPÓTESE]`.
- Quem assina a revisão humana de uma suspeita de trapaça? Política a definir com a governança (seção 15).
## 15. Governança, Comunidade e Ecossistema

Se as regras são o produto, quem decide a mudança de regra é o dono do jogo. Esta seção define quem é esse dono e como ele decide. Ela converte os quatro pilares — Soberania do Jogador, Ação Coletiva, Materialismo Histórico e Luta Decolonial — em mecanismos verificáveis e fecha os pontos deixados em aberto pela seção 14: mudanças no **Núcleo**, governança do catálogo de servidores, assinatura da revisão humana de trapaça e aprovação de **Módulos**.

**Ressalva geral, válida para toda a seção.** Fundação sem fins lucrativos, estatuto público, eleição, licença de software livre e direito de saída são **freios**, não garantias. Nenhum deles assegura, por si, governança justa: captura por corporação, por voluntários influentes ou por maioria tácita continua possível. Toda afirmação abaixo sobre fundações e licenciamento é lida com essa ressalva.

### 15.1 Por que governança é mecânica, não burocracia

**Tese.** Huizinga definiu o jogo como atividade livre e voluntária. No futebol de rua, o "círculo mágico" é criado por contrato social temporário ("três cantos é pênalti"); no *League of Legends*, o jogador aceita um EULA (Contrato de Licença de Usuário Final) e **aluga** um círculo mágico pré-fabricado. A Riot altera regras por decisão unilateral, revoga acesso e pode desligar os servidores. A governança do **Insurgência** é a garantia de que esse círculo não seja alugado de novo: não é setor administrativo, é a mecânica que mantém a soberania jogável. Serve à **Soberania do Jogador**.

**A "FIFA open source" sem idealização ingênua.** A proposta é uma fundação sem fins lucrativos que guarda a especificação das regras canônicas, não um clube de administradores. Ela não é santa: estatuto e eleição podem ser capturados. Por isso a governança é desenhada como mecanismo de saída e de contestação, não como confiança depositada em pessoas.

**Critérios de aceitação:**

- Nenhuma partida exige EULA ou conta corporativa para começar; conexão por endereço direto funciona sem registro (seção 14.3).
- Qualquer participante publica proposta de mudança de regra sem autorização prévia; o registro de propostas é público e versionado.
- O estatuto declara que a fundação não possui as regras como propriedade privada e que o direito de saída prevalece sobre a permanência.

### 15.2 Modelo proposto: fundação em três camadas e comitês

**Separação de poderes.** Três camadas; nenhuma tem autoridade sobre as regras das outras.

| Camada | O que é | Quem decide | Não pode |
| :-- | :-- | :-- | :-- |
| (i) Especificação | Protocolo e regras canônicas: vitória na **Ilha Central**, janela de sincronia do **Desafio de Ilha**, funções | Comitê de Regras de Jogo; versão canônica publicada | Alterar instância hospedada |
| (ii) Implementação | **Núcleo** (motor, rede, validação) e **Módulos** declarativos | Mantenedores do **Núcleo**, por proposta aprovada | Mudar regra canônica sem versão nova |
| (iii) Instância | Servidores comunitários, catálogos e ligas | Cada anfitrião | Alterar o protocolo ou falsificar o *hash* (resumo criptográfico) do módulo |

**Modelo de fundação.**

| Mecanismo | Regra proposta | Intenção e marcador |
| :-- | :-- | :-- |
| Estatuto público | Texto versionado; alteração exige maioria de 3:1 | `[SUPOSIÇÃO]` Calqueado no Debian; impede alterar as regras do próprio estatuto por maioria simples. |
| Eleições | Líder e comitês eleitos, mandato de 2 anos, reeleição permitida | `[HIPÓTESE]` Tempo suficiente para trabalho contínuo, curto o bastante para responsabilizar. |
| Quórum explícito | 2/3 dos membros presentes para regras canônicas; 3:1 para documentos fundacionais | `[HIPÓTESE]` Dificulta captura por maioria estreita sem paralisar a decisão. |
| Direito de saída | Qualquer membro ou desenvolvedor sai a qualquer tempo, por declaração pública | Regra canônica do Debian; tradução institucional da Soberania do Jogador. |
| Transparência | Orçamento, atas e votações nominais publicados a cada 3 meses | `[HIPÓTESE]` Periodicidade suficiente para detectar desvio. |

**Ressalva.** Estatuto público não é governança horizontal automática: a Linux Foundation é corporação sem fins lucrativos de Oregon, sob 501(c)(6) do Código Tributário dos EUA, com diretores indicados por membros Platinum (até 20). "Sem fins lucrativos" não significa "sem dono".

**Comitês.**

| Comitê | Poder | Composição | Pilar atendido |
| :-- | :-- | :-- | :-- |
| Regras de Jogo | Aprova a versão canônica de regras e balanceamento | Eleição aberta; decisão por 2/3 | Ação Coletiva: a regra muda por deliberação, não por *patch* unilateral |
| Compatibilidade de Protocolo | Testes de conformidade, *hash* e versionamento | Mantenedores do **Núcleo** mais delegados de instâncias | Soberania: um fork não quebra a partida alheia |
| Conteúdo e Assets | Aprova **Módulos**, licenças e atribuição | Curadoria mais revisão por especialistas | Luta Decolonial: bloqueia Módulo que estereotipa ou encena opressão |
| Conduta | Sanções graduadas, apelação e revisão humana de trapaça | Eleição aberta; decisão por 2/3 | Ação Coletiva: nenhum indivíduo pune sozinho |

**Processo de mudança** — aplica-se ao **Núcleo**; mudanças de regra canônica seguem o mesmo rito.

| Etapa | Prazo | Artefato | Quórum |
| :-- | :-- | :-- | :-- |
| Proposta pública | Aberto | Documento com *diff* (comparação), impacto em regras e plano de reversão | Nenhum |
| Debate | 30 dias `[HIPÓTESE]` | Registro de objeções e de membros afetados | Nenhum |
| Revisão | 14 dias `[HIPÓTESE]` | Parecer do comitê competente e teste de conformidade | Comitê |
| Votação | 14 dias | Versão canônica numerada e nota de mudança | 2/3; 3:1 para documentos fundacionais |

**Ressalva.** Processo público não garante decisão justa: quórum pode ser capturado e debate pode ser dominado por poucos. Direito de saída e fork (bifurcação do código) são o contrapeso, não o processo em si.

**Decisões que fecham pendências da seção 14.** O catálogo de servidores é um conjunto de **instâncias autônomas**, não serviço exclusivo da Fundação: ela pode manter um catálogo de referência replicável, sem autoridade sobre regras. A revisão humana de suspeita de trapaça é assinada por dois membros do Comitê de Conduta (decisão por 2/3), com apelação a painel misto; antes da fundação formal, assinam os mantenedores do **Núcleo**. A aprovação de **Módulos** cabe ao Comitê de Conteúdo e Assets, que bloqueia todo Módulo com fricção ilegítima (decisão do jogador descartada sem efeito jogável) e exige as quatro perguntas OFELAŠ mais revisão por especialistas quando o Módulo representar um grupo.

**Critérios de aceitação:**

- Alteração no **Núcleo** sem versão canônica numerada é rejeitada pelo teste de conformidade.
- Proposta, debate, parecer e votação ficam registrados com autor, data e resultado; nenhuma etapa é privada.
- Catálogo desligado não impede conexão direta por IP:porta em dois servidores independentes (reforço do critério 14.3).
- Módulo que represente um grupo sem registro das quatro perguntas OFELAŠ e da revisão por especialistas não recebe aprovação.

### 15.3 Bens comuns e policentricidade

Elinor Ostrom, *Governing the Commons* (1990), descreve oito princípios de instituições duradouras para recursos compartilhados. Aqui o "recurso" é o protocolo e as regras canônicas: uso não rival, mas vulnerável à captura. Governança policêntrica (múltiplos centros de decisão parcialmente independentes, coordenados por regras comuns) é a forma institucional do que a seção 14 faz na arquitetura: nenhum servidor é autoridade única. Fontes: [Oficina Ostrom — ferramentas de ensino](https://ostromworkshop.indiana.edu/courses-teaching/teaching-tools/ostrom-design/index.html); [Elinor Ostrom — princípios de desenho de instituições](https://ecologyandsociety.org/vol9/iss1/art18/table3.html).

| # | Princípio (Ostrom, 1990) | Tradução no **Insurgência** |
| :-- | :-- | :-- |
| 1 | Limites claros | Domínio da regra canônica e quem pode modificá-la estão no estatuto; fora dele, cada instância é livre |
| 2 | Equivalência entre benefícios e custos | Quem propõe mudança publica custo de manutenção e impacto em quem a usa |
| 3 | Arranjos de escolha coletiva | Comitês eleitos; votam os afetados pela regra |
| 4 | Monitoramento responsável aos usuários | Testes de conformidade e atas públicas; o monitor responde à comunidade, não a acionistas |
| 5 | Sanções graduadas | Advertência → restrição temporária → exclusão, com apelação (15.7) |
| 6 | Resolução de conflitos barata e local | Disputa resolvida na instância; apelação ao comitê sem custo para o jogador |
| 7 | Reconhecimento do direito de se organizar | Estatuto reconhece ligas, servidores e fork (bifurcação do código) sem autorização da Fundação |
| 8 | Empreendimentos aninhados | Comitês locais → fundação → federação de instâncias, cada camada com poder próprio |

**Ressalva.** Ostrom identifica condições associadas à persistência de bens comuns, não garantia de justiça: os oito princípios podem ser implementados e ainda assim produzir decisões injustas. Policentricidade não é sinônimo de horizontalidade — é distribuição de autoridade com coordenação.

**Critérios de aceitação:**

- Toda regra do estatuto mapeia para ao menos um dos oito princípios; regra sem mapeamento é removida ou justificada por escrito.
- Um servidor comunitário resolve disputa local sem consultar a Fundação e registra a decisão na própria instância.
- Nenhum comitê altera regra de outro comitê sem o rito de 15.2.

### 15.4 Fundações não são automaticamente horizontais

| Organização | Estrutura verificada | Lição para o **Insurgência** |
| :-- | :-- | :-- |
| Linux Foundation | Corporação sem fins lucrativos de Oregon, sob 501(c)(6) do Código Tributário dos EUA; membros Platinum (nível corporativo) indicam até 20 diretores; associados sem voto | "Sem fins lucrativos" não impede hierarquia nem peso corporativo |
| Mozilla | Autoridade distribuída por contribuição, com governança documentada publicamente | Contribuição é critério legítimo, mas quem contribui mais pode concentrar poder |
| Debian | Líder eleito, Resoluções Gerais, Comitê Técnico; maioria de 3:1 emenda a constituição e supera documentos fundacionais; qualquer desenvolvedor sai a qualquer tempo, por declaração pública | **Modelo de referência**: quórum alto para documentos fundacionais e saída incondicional |

Fontes: [Estatutos da Linux Foundation](https://www.linuxfoundation.org/legal/bylaws); [Governança da Mozilla](https://www.mozilla.org/en-US/about/governance/); [Constituição do Debian](https://www.debian.org/devel/constitution).

**Ressalva.** Mesmo o Debian, tomado como modelo, não livra a governança de captura: maioria organizada pode controlar Resoluções Gerais e concentração de mantenedores cria poder de fato. Copiar o quórum de 3:1 copia uma barra de proteção, não a ausência de captura. Licença de código também não garante governança justa — garante apenas o direito de ler, modificar e redistribuir o programa.

**Critérios de aceitação:**

- O estatuto reproduz o quórum de 3:1 para documentos fundacionais e a saída a qualquer tempo, com texto público.
- A composição dos comitês inclui assentos de contribuidores sem vínculo corporativo; nenhuma entidade indica diretores por cotização.
- Alteração de estatuto sem registro de votação nominal é considerada nula.

### 15.5 Protocolo não é instância: a lição do fediverso

O fediverso (rede de servidores independentes que conversam entre si) separa três camadas de governança: o padrão ActivityPub do W3C (World Wide Web Consortium — consórcio que define padrões da web), os pacotes de software que o implementam e as instâncias que hospedam comunidades. O artigo do *International Journal of Communication* descreve o "Permissionless Completeness Problem" (problema da completude sem permissão): protocolo aberto não impede que instâncias se tornem jardins murados (*walled gardens* — ambientes fechados que impedem a saída de dados). URL: [IJOC — fediverso e governança](https://ijoc.org/index.php/ijoc/article/download/22254/4864).

O Lichess é associação caritativa francesa sob a lei de 1901 e publica o código sob AGPL-3.0 (GNU Affero General Public License — licença *copyleft* que exige compartilhar o código modificado quando o software é usado em rede). Seus termos, porém, reservam encerramento de conta "a qualquer tempo, por qualquer motivo, sem aviso" e declaram: "você concorda que nossa palavra é final", sem obrigação de apresentar evidências — embora exista direito de apelação. URL: [Termos de serviço do Lichess](https://lichess.org/terms-of-service).

**Ressalva central.** Software livre garante o **garfo do código** (a possibilidade de copiar e manter o programa), não a justiça da instância hospedada. Um servidor de terceiros sob AGPL (como o do Lichess) pode encerrar contas sem recurso; fundação caritativa pode ser mais opaca que empresa. A soberania depende de portabilidade — direito de sair com dados, personagens e histórico —, não da licença do servidor. A AGPL citada é de terceiros; o servidor do Insurgência permanece MIT conforme ADR-002.

**Decisão.** O catálogo de servidores é instância autônoma (15.2); nenhuma instância do **Insurgência** pode ser a única portadora de identidade, progressão ou histórico. Exportação de dados é requisito do **Núcleo**, não cortesia do anfitrião.

**Critérios de aceitação:**

- Conta, personagem e histórico são exportáveis em formato aberto; nenhuma instância guarda a chave de exportação.
- Termos de instância referenciada pelo catálogo não podem revogar a licença do código nem o direito de exportação.
- Cliente conecta a servidor novo por endereço direto sem depender de registro no catálogo (seção 14.3).

### 15.6 Forks como mecanismo de responsabilização, não apenas risco

Após a aquisição da Sun pela Oracle, contribuidores de OpenOffice.org fundaram The Document Foundation em setembro de 2010 e bifurcaram o projeto como LibreOffice. Cerca de quatro anos e meio depois (LWN, 25/03/2015), o LibreOffice reunia mais de 250 desenvolvedores ativos, enquanto o OpenOffice era movido por quatro desenvolvedores de uma única empresa. URLs: [História da The Document Foundation](https://www.documentfoundation.org/history/); [LWN — atividade de desenvolvimento LibreOffice e OpenOffice](https://lwn.net/Articles/637735).

**Ressalva.** O fork mostra que a comunidade pode responder à captura com saída, não que a nova fundação seja justa: o próprio LWN registrava concentração de patches em poucas empresas. Bifurcar resolve o monopólio; não resolve a governança interna do fork.

**Ligas *vanilla* e *custom*.** A liga *vanilla* (regras canônicas sem modificação) é o referencial de compatibilidade; ligas *custom* (variantes com regras, personagens e balanceamento alterados) competem por jogadores e prêmios. A competição de forks é o mecanismo de responsabilização: se um servidor estraga o balanceamento, a comunidade migra. Para que a migração não pulverize o pareamento de partidas (*matchmaking*), existe um **núcleo mínimo** obrigatório — protocolo de rede, identidade de conta e testes de compatibilidade — que todo fork preserva. Mudanças de regra ficam nos **Módulos**, não no **Núcleo** (seção 14.2).

**Critérios de aceitação:**

- Fork publicado altera apenas arquivos de **Módulo**; o *diff* (comparação) contra o **Núcleo** é vazio (reforço do critério 14.2).
- Dois forks aprovados nos testes de conformidade entram na mesma partida com o mesmo *hash* de protocolo.
- Liga *custom* declara qual versão canônica usa e quais regras alterou, visível no cliente antes da entrada.

### 15.7 Mecanismos anticaptura e de coordenação

| Mecanismo | Regra | Pilar atendido | Ressalva |
| :-- | :-- | :-- | :-- |
| Sanções graduadas e apelação | Advertência → suspensão → exclusão; apelação gratuita. Ausentes no Chess.com (plataforma privada), presentes no Lichess, que ainda assim afirma "nossa palavra é final" | Ação Coletiva | Apelação existir não impede decisão arbitrária; exige registro do motivo |
| Torneios com regras canônicas versionadas | Todo torneio declara versão da regra e *hash* do módulo; resultado reproduzível por *replay* (reexecução da partida) | Materialismo Histórico | Versão declarada pode ser falsificada sem teste de conformidade |
| Direito de saída e exportação | Qualquer jogador sai com dados e personagens; nenhuma instância retém a chave | Soberania do Jogador | Portabilidade depende de formato aberto, não de promessa |
| Compatibilidade por testes de conformidade | Servidor que falha o teste não entra no catálogo de referência | Soberania e coordenação | Teste é escrito por alguém: quem escreve pode capturá-lo |
| Orçamento e transparência | Prestação trimestral de contas, atas e votos nominais | Ação Coletiva | Transparência sem auditoria é cosmética |
| Financiamento sem acionistas | Doações, patrocínio e cosméticos cujo lucro vai para a fundação | Luta Decolonial | Gera menos capital imediato que uma Riot; depende de voluntariado |

**Ressalva final do quadro.** Nenhum mecanismo elimina captura: são freios que aumentam o custo da captura e reduzem o custo da saída. Nenhum substitui a vigilância dos jogadores.

**Critérios de aceitação:**

- Toda sanção registra motivo, autor e prazo; apelação é respondida em até 30 dias `[HIPÓTESE]`.
- Torneio anuncia versão canônica e *hash*; o *replay* reproduz o mesmo resultado em duas execuções (seção 14.5).
- Exportação de conta e personagem gera um arquivo legível, verificável por ferramenta independente.
- Relatório trimestral de finanças é publicado no mesmo repositório das atas.

### 15.8 Contexto legal e urgência

O risco que a governança endereça é legal, não teórico. *The Crew* (Ubisoft, 2014–2024) reuniu mais de 12 milhões de jogadores e foi desligada, ficando injogável. A campanha Stop Killing Games entregou cerca de 1,3 milhão de assinaturas à Comissão Europeia em janeiro de 2026, com audiência pública no Parlamento Europeu em abril de 2026. Em março de 2026, a associação de consumidores UFC-Que Choisir processou a Ubisoft pela descontinuação. Nos Estados Unidos, o Protect Our Games Act da Califórnia passou na Assembleia estadual e aguarda o Senado. URL: [BBC — Stop Killing Games](https://www.bbc.com/news/articles/c8e8e7g0r82o).

**Ressalva.** Pressão legal é aliada, não substituto da governança: lei pode obrigar plano de fim de vida sem dizer quem decide a regra. Fundação e licença continuam sem garantir governança justa — a diferença do **Insurgência** é tornar saída e portabilidade parte do produto antes que a lei exija.

**Critérios de aceitação:**

- Estatuto prevê plano de fim de vida para instância mantida pela Fundação, com exportação garantida.
- Nenhum **Módulo** do protótipo depende de serviço hospedado que possa ser desligado sem substituto local.
- Projeto publica licença e atribuição desde a primeira versão, sem esperar decisão legal.

### 15.9 Relação com o MVP (Produto Mínimo Viável)

Entra na primeira versão o que custa pouco e protege a soberania; espera o que exige comunidade formada.

| Mecanismo | No MVP? | Critério de aceitação |
| :-- | :-- | :-- |
| Licença MIT (licença permissiva do Massachusetts Institute of Technology) para o **Núcleo** e o servidor headless (ADR-002) | Sim | `LICENSE` e `COPYRIGHT.txt` presentes; nenhum arquivo sem licença declarada |
| Contrato de contribuição (DCO — *Developer Certificate of Origin*, certificado de origem do desenvolvedor; ou CLA — Contrato de Licença de Contribuição) | Sim | Todo *commit* assinado; contribuição sem assinatura é recusada |
| Arquivo de atribuição (fonte, licença, data e restrições) | Sim | Validado antes de empacotar cada **Módulo** (seção 14.7) |
| Especificação de versão do protocolo e *hash* do módulo | Sim | A troca inicial (*handshake*) recusa versão divergente com erro legível (seção 14.2) |
| Quórum, comitês e votação formal | Não — `[NOTA PARA O DESIGNER]` | Substituídos pelos mantenedores do **Núcleo** até a fundação; decisão registrada |
| Fundação formal (estatuto, eleições, quórum de 3:1) | Não | Depende de comunidade e financiamento; é marco a decidir na seção 16 |
| Circuito de ligas *vanilla* e *custom* com torneios | Não | Requer catálogo e testes de conformidade maduros |

**Ressalva.** Licença MIT (ADR-002) e contrato de contribuição protegem o código e a autoria; não protegem o jogador de instância injusta. Mesmo no MVP, governança justa depende de portabilidade e direito de saída — por isso exportação entra cedo e fundação pode esperar.

**Critérios de aceitação consolidados do MVP:**

- Cliente inicia partida sem conta corporativa e conecta por endereço direto.
- Conta, personagem e histórico são exportáveis antes do lançamento do MVP.
- Toda contribuição tem assinatura de origem e entrada no arquivo de atribuição.
- Versão do protocolo e *hash* do **Módulo** aparecem no cliente antes da entrada na partida.
## 16. MVP (Produto Mínimo Viável), Escopo e Roadmap

### 16.1 Intenção do MVP

O MVP (Produto Mínimo Viável) não entrega "um MOBA pequeno". Ele entrega uma prova: o Núcleo (motor de regras) carrega um Módulo de jogo, um servidor dedicado autoritativo aceita conexão direta e a captura de ilhas por desafios — não por destruição de torres — produz vitória. Se essa prova falhar, mais funções, mapas ou arte não a consertam.

Escopo de jogadores: 2 a 4 por partida. [HIPÓTESE — o jogo base mira 5v5; o MVP testa a sincronia com o menor número que exercita duas placas distintas, e 4 permite dois pares simultâneos.]

### 16.2 Decisão de motor: Godot-first e a contradição das fontes

As fontes discordam, e o documento não pode fingir o contrário.

- **Texto A** (`Jogos MOBA Open Source e Frameworks.md`, resposta de 22/02/2026 às 11:44): Passo 1 — protótipo de caixas no Unity usando `MOBA CSharp Unity` (yasgamesdev), classificado como "Essencial. Comece por aqui"; Passo 2 — "migração filosófica" para a Godot entre as semanas 5 e 12.
- **Texto B** (mesmo arquivo, resposta de 22/02/2026 às 11:18; reforçado pela seção 5 de `Esporte vs. E-sport_ Controle das Regras.md`): usar frameworks MOBA tradicionais é "armadilha arquitetural" — cerca de 80% do tempo é gasto desligando lógica de dano de torre e rotas de súditos. Recomenda Godot desde o início: a arquitetura de Cenas e Nós espelha Ilhas e Módulos.

**Decisão: Godot-first. A alternativa Unity fica registrada como descartada.**

Justificativa: (1) licença MIT — a Godot é FOSS (Software Livre e de Código Aberto), enquanto o Unity é proprietário; adiar a migração adia o teste da premissa que motiva o projeto; (2) coerência filosófica — Cena, Nó e sinal (evento) mapeiam diretamente Ilha, Módulo e conclusão de desafio, sem camada de tradução; (3) débito técnico — a rota Unity→Godot paga duas vezes o custo de reescrever a arquitetura que se quer validar; (4) escopo — o MVP precisa de um servidor autoritativo simples, não de netcode (código de rede) pronto de terceiros. A licença protege o garfo do código, não a governança da instância hospedada: o caso Lichess mostra que um serviço FOSS pode manter termos unilaterais de encerramento. Por isso o Núcleo separa protocolo de implementação e prevê direito de saída (item 18 e subseção 21.4).

Consequência operacional: os repositórios citados entram como referência de infraestrutura (reconciliação de servidor, validação de ações, empacotamento de estado), nunca como base de código. O pacote AAA de Paragon fica fora por custo e incompatibilidade estética; o protótipo usa greyboxing (protótipo com formas geométricas sem textura) e assets de baixa poligonalidade de Kenney.nl.

### 16.3 Escopo exato do MVP

**Motor e rede.** Godot; conexão direta cliente-servidor, sem lobby (sala de espera) nem matchmaking (pareamento de partidas). Intenção: isolar o Núcleo.

**Mapa.** O mapa do Arquipélago da Insurgência em greyboxing, com três ilhas de rota e uma Ilha Central.

**Decisão de escala — sincronia da Rota Central.** A regra canônica de produção exige 3 jogadores em janela de 5 s (seção 8). O protótipo usa 2 placas ocupadas por 2 jogadores dentro da mesma janela de 5 s como redução de escala: o objetivo é validar o princípio da entrada sincronizada com o menor número de clientes. A redução vale somente para o MVP; o Módulo de produção mantém 3 jogadores em 5 s, e nenhum critério desta seção a revoga. [NOTA PARA O DESIGNER] A janela de 5 s é preservada nas duas escalas; alterá-la na produção exige recalibrar a métrica da seção 17.

**Decisão de escala — condição de vitória.** A produção exige seis ilhas de rota articuladas pelo Plano Mestre e 20 s de canalização de Propaganda na Ilha Central. O MVP reduz a prova a três ilhas de rota — uma por rota canônica — e a 4 s de canalização contínua na Ilha Central, realizada por Artesão e Guerreiro vivos e presentes, nunca por qualquer membro. [HIPÓTESE — justificativa de escopo: com dois a quatro clientes, o menor conjunto que exercita as três rotas já valida a arquitetura Núcleo + Módulo; exigir as duas funções preserva o pilar Ação Coletiva e a regra de que todo desafio do MVP requer ao menos duas funções.] A redução é exclusiva do protótipo: a regra de produção (seis ilhas + Plano Mestre, 20 s de canalização) permanece e nenhum critério desta seção a revoga.

A identidade visual da Ilha Central segue a definição da seção 8; esta seção não redefine paleta e mantém o protótipo em greyboxing.

| Elemento | Especificação canônica da fonte | Valores de design | Pilares atendidos |
|---|---|---|---|
| Ilha Topo (Rota Superior / Ilhas da Autossuficiência) | Um desafio de lógica simples: ativar 3 alavancas na ordem correta | Ordem fixa por partida; 8 s por alavanca; erro reinicia a sequência em 2 s; limite de 45 s. [HIPÓTESE] | Materialismo Histórico; Soberania do Jogador |
| Ilha Meio (Rota Central / Ilhas da Ação Direta) | Um desafio de sincronia: 2 jogadores pisam em 2 placas ao mesmo tempo (redução de escala do protótipo; produção: 3 jogadores em 5 s) | Janela de 5 s; falha aplica gás lacrimogêneo: 8 pontos de dano por segundo e expulsão após 3 s de exposição contínua. [HIPÓTESE] | Ação Coletiva |
| Ilha Inferior (Rota Inferior / Delta do Comércio) | Um ponto de coleta de recurso | Ciclo de 3 s; 5 unidades de "Materiais" por ciclo; estoque de 3 ciclos; recarga em 20 s. [HIPÓTESE] | Materialismo Histórico |
| Artesão | Uma habilidade: Construir Barricada | Custo 20 Materiais; 1,5 s de canalização; 200 pontos de vida; 12 s de duração; interrompida por dano. [HIPÓTESE] | Ação Coletiva; Materialismo Histórico |
| Guerreiro | Uma habilidade: Ataque Forte | 1,2 s de canalização; 2× o dano base; 6 s de recarga. [HIPÓTESE] | Ação Coletiva; Luta Decolonial |
| Ilha Central | A equipe que ativar o desafio da Ilha Central vence | MVP: três ilhas de rota sob controle da mesma equipe (em vez de seis na produção) e 4 s de canalização contínua (em vez de 20 s de Propaganda na produção) por Artesão e Guerreiro vivos e presentes, não por qualquer membro. [HIPÓTESE de escala — justificativa em 16.3] | Luta Decolonial; Ação Coletiva |

**Objetivo do MVP:** provar que a arquitetura Núcleo + Módulo de Jogo funciona e que a jogabilidade baseada em desafios de ilha, em vez de combate puro, é viável e divertida. A validação objetiva dessa intenção está nas métricas da seção 17.

### 16.4 Critérios de aceitação do MVP

**Rede direta e Núcleo.**
- Dois clientes informam o endereço do servidor na inicialização e entram na mesma partida sem serviço intermediário.
- O servidor aceita apenas intenções (inputs): posição, vida e estado de ilha enviados prontos pelo cliente são descartados.
- Um cliente modificado que solicita "capturar ilha" sem condição prévia recebe recusa, e a ilha permanece neutra.

**Ilha Topo.**
- A sequência correta das três alavancas em até 45 s captura a ilha e emite o evento ao servidor.
- Uma alavanca fora de ordem reinicia a sequência em 2 s, com aviso visual, sem captura.
- Um cliente que envia as três ativações fora da janela recebe falha; a captura só existe se validada no servidor.

**Ilha Meio.**
- No protótipo, dois jogadores distintos nas duas placas em janela de até 5 s travam as placas e capturam a ilha; a janela é idêntica à da regra de produção.
- Entrada pingada (acima de 5 s) aplica gás lacrimogêneo — 8 pontos de dano por segundo e expulsão após 3 s de exposição contínua — e não captura.
- Um único jogador ocupando as duas placas não captura a ilha.
- A regra de produção (3 jogadores em 5 s) permanece no Módulo base; o critério de 2 jogadores é exclusivo do protótipo e não a substitui.

**Ilha Inferior.**
- A cada 3 s o servidor credita 5 Materiais ao coletor; o ponto esgota após 3 ciclos e recarrega em 20 s.
- Um cliente que inventa Materiais sem passar pelo ponto tem o crédito recusado.

**Funções.**
- Construir Barricada exige 20 Materiais e 1,5 s parados; a barricada bloqueia passagem por 12 s e desaparece ao sofrer 200 pontos de dano.
- Ataque Forte aplica 2× o dano base após 1,2 s e fica indisponível por 6 s; o servidor recusa o uso durante a recarga.

**Condição de vitória (escala do MVP).**
- Sem as três ilhas de rota controladas pela mesma equipe, a canalização da Ilha Central não inicia; na produção, o pré-requisito é o Plano Mestre sobre seis ilhas.
- Com as três ilhas, Artesão e Guerreiro vivos e presentes na Ilha Central e 4 s de canalização conjunta contínua, o servidor declara vitória e exibe tela de resultado.
- A canalização não avança se uma das duas funções morrer ou deixar a Ilha Central; a presença de outros jogadores não a substitui.
- Interromper a canalização zera o progresso; a vitória é declarada pelo servidor, não pelo cliente.

### 16.5 Rastreabilidade da fantasia às mecânicas

Cadeia ancorada nas seções 5 a 7.

| Fantasia | Pilar | Loop | Mecânica do MVP |
|---|---|---|---|
| Uma Comuna autônoma converte ilhas sob domínio de uma corporação colonial por organização material, não por destruição | Luta Decolonial | Capturar rotas → ativar a Ilha Central | Vitória por ativação da Ilha Central |
| Nenhum proprietário das regras: qualquer servidor hospeda o mesmo Núcleo | Soberania do Jogador | Conectar → jogar → hospedar | Conexão direta e Módulo carregável |
| Ações materiais, não magia | Materialismo Histórico | Ler o terreno → agir sobre ele | Alavancas, placas, coleta, barricada |
| Ninguém vence sozinho | Ação Coletiva | Coordenar funções → superar desafio | Duas placas sincronizadas e defesa do Artesão |

### 16.6 Roadmap com marcos verificáveis

Durações são [HIPÓTESE — ritmo de um desenvolvedor com base em engenharia de software, uma entrega por fase].

| Fase | Semanas | Entrega | Marco verificável |
|---|---|---|---|
| 0 — Fundação | 1–2 | Núcleo Godot em servidor dedicado sem interface; conexão direta; validação de inputs | Dois clientes conectam, movimento é replicado e nenhum estado pronto é aceito |
| 1 — Ilha Topo | 3–4 | Greyboxing do arquipélago e desafio das três alavancas | Sequência correta captura; sequência errada não captura |
| 2 — Ilhas Meio e Inferior | 5–7 | Janela de 5 s e ponto de coleta | Dois jogadores em 5 s capturam (redução de escala, ver 16.3); coleta é creditada pelo servidor |
| 3 — Funções | 8–9 | Artesão e Guerreiro com uma habilidade cada | Barricada custa e bloqueia; Ataque Forte respeita recarga |
| 4 — Fechamento do MVP | 10–12 | Ilha Central, condição de vitória e carregamento de Módulo | Servidor inicia com Módulo externo; com três ilhas de rota, Artesão e Guerreiro vivos canalizam 4 s na Ilha Central e o servidor declara vitória; cliente sem Módulo não entra |
| 5 — Pós-MVP | além da 12 | Governança, editor e funções restantes | Fora do escopo desta seção; ver item 19 |

O MVP está fechado quando todos os critérios de aceitação acima passarem em duas partidas consecutivas, uma delas com cliente modificado.

### 16.7 Checklist de implementação

> Estado verificado em 2026-11-20T03:08Z. **[x]** = concluído com comando executado e saída colada; **[~]** = parcial (lógica validada, integração pendente); **[ ]** = não implementado. Nenhum marcador afirma execução remota sem confirmação ao vivo.

**Governança e aparato**
- [x] Constituição local, factory como fonte (HEAD `b1deab1`) e gate autoritativo do alvo
- [x] 9 ciclos SDD com R1/R4/R7; 5 gates determinísticos PASS
- [x] ADR-001 (GDScript) e ADR-002 (MIT) aprovadas
- [x] CI remoto, proteção de `dev` e PRs #1–#14 mesclados (verificados ao vivo via `gh`)
- [x] Godot 4.7.2 executável localmente; benchmark headless executado

**Fatia 80/20 (spec CE-1..CE-5)**
- [x] CE-1 — estrutura e contrato versionado (16 `PASS`; hash `b2c11e6d…`; 14/39 caminhos)
- [~] CE-2 — servidor headless e validação (6 + 8 + 9 `PASS`; `SERVIDOR_PRONTO porta=7777`); conexão UDP real entre clientes **[NV]**
- [x] CE-3 — Ilha Meio (14 `PASS` + `OK test_ilha_sincronia`; `DELTA_EVENTO_MS 0` + 3 `PASS`)
- [x] CE-4 — manifesto e gate local (7 `PASS`; `COMPAT_OK module_id=insurgencia.exemplo`; 3 execuções idênticas; divergência de hash recusada)
- [x] CE-5 — evidência auditável (`make check` exit 0; CSV de 9 colunas; 1 `NAO_VERIFICADO` explícito)

**Critérios de aceitação do MVP (§16.4 — 18 itens)**
- [~] 1514 — 2 clientes em IP:porta na mesma partida: handshake em teste unitário; conexão real **[NV]**
- [~] 1515 — servidor aceita só intenções: `normalizar_pacote` e `handle_command` testados; integração de rede pendente
- [ ] 1516, 1519–1521 — Ilha Topo (alavancas, ordem, janela)
- [x] 1524–1527 — Ilha Meio (2 placas/5 s, gás 8/amostra, 24 em 3 s, regra de produção 3 no Módulo base)
- [ ] 1530–1531 — Ilha Inferior (Materiais, recarga, material forjado)
- [ ] 1534–1535 — Artesão (Barricada) e Guerreiro (Ataque Forte)
- [ ] 1538–1541 — Ilha Central e condição de vitória

**Fora do MVP / pendente de autorização**
- [ ] Fase 5 — governança de comunidade, editor e funções restantes
- [ ] Deploy/publicação, Postgres/Coolify remotos, Zitadel, `pgmq` e federação forte — **[NV]**

Este checklist registra estado; não substitui os critérios normativos de §16.4 nem o fechamento em duas partidas consecutivas (§16.6). Histórico datado em `docs/PROGRESSO.md`.

## 17. Métricas de Sucesso

A instrumentação registra eventos no Núcleo, nunca apenas no cliente. Toda meta abaixo é [HIPÓTESE] e deve ser recalibrada após 20 partidas instrumentadas. [HIPÓTESE — amostra mínima para separar sorte de padrão.]

| Métrica | Definição operacional | Meta | Intenção / pilar |
|---|---|---|---|
| Tempo até primeira captura de ilha | Mediana, desde o início da partida até a primeira ilha de rota capturada | ≤ 90 s na mediana; p95 ≤ 150 s [HIPÓTESE] | Abaixo disso o loop não se forma; acima, a sessão perde ritmo |
| Taxa de conclusão do desafio sincronizado | Ativações bem-sucedidas da Ilha Meio ÷ tentativas na janela de 5 s | ≥ 60% e ≤ 90% [HIPÓTESE] | Se for baixa, a janela é punitiva; se for quase 100%, não houve coordenação |
| Latência percebida | RTT mediano e p95 reportados pelo cliente, mais resposta "sem atraso perceptível" | RTT mediano ≤ 80 ms em servidor local e ≤ 150 ms em comunitário; p95 ≤ 250 ms; ≥ 90% de respostas positivas [HIPÓTESE] | A janela de 5 s só é justa se o servidor decidir rápido |
| Servidores comunitários ativos | Servidores com ao menos uma partida completa nos últimos 7 dias | ≥ 5 em 3 meses; ≥ 20 em 12 meses [HIPÓTESE] | Prova da Soberania do Jogador fora do servidor oficial |
| Diversidade de módulos e forks publicados | Módulos públicos distintos com partida registrada; forks (bifurcações do código) contados à parte | ≥ 3 módulos em 3 meses; ≥ 10 em 12 meses; ≥ 2 com pacote de personagens diferente [HIPÓTESE] | Variação sem fragmentação é o objetivo da arquitetura Núcleo + Módulo |
| Taxa de partidas concluídas | Partidas terminadas por vitória da Ilha Central ÷ partidas iniciadas | ≥ 50% [HIPÓTESE] | Se o objetivo nunca é alcançado, a condição de vitória está mal calibrada |

Critérios de aceitação do conjunto:
- Nenhuma métrica entra no painel sem definição operacional e fonte de dados declaradas.
- O Núcleo grava capturas, tentativas de sincronia e divergências cliente-servidor antes da Fase 4.
- "Servidor ativo" e "módulo publicado" usam exatamente os limiares acima em todos os relatórios.
- As metas são revisadas uma única vez após 20 partidas, com o novo valor ainda marcado como [HIPÓTESE].

## 18. Riscos e Mitigações
O risco não é abstrato: *The Crew* reuniu 12 milhões de jogadores e foi desligada em 2024, após dez anos; a campanha Stop Killing Games superou 1,3 milhão de assinaturas na Comissão Europeia e teve audiência no Parlamento em abril de 2026. Um jogo que depende de servidor de terceiros pode desaparecer por decisão unilateral — exatamente a falha que a federação existe para eliminar.

| Risco (origem nas fontes) | Impacto | Probabilidade | Mitigação | Sinal de alerta | Pilar ameaçado |
|---|---|---|---|---|---|
| Custo e inconsistência de assets (arte e som nível AAA) | Alto | Alta | Greyboxing no MVP; Kenney.nl e baixa poligonalidade; paleta e escala únicas validadas por Módulo; nenhum asset sem licença compatível | Mais de 20% do tempo de uma fase em ajuste visual; asset externo sem licença verificada [HIPÓTESE] | Materialismo Histórico |
| Sustentabilidade financeira | Alto | Alta | Fundação sem fins lucrativos; doações e patrocínio; servidor leve e desacoplado; gate local (`make check`) desde o primeiro dia; integração contínua remota DEFER até o primeiro commit de código [NV] (nenhum gate/pipeline remota neste ciclo; ADR-001); estatuto com eleições, quórum e direito de saída — fundação não é sinônimo de horizontalidade (Linux Foundation 501(c)(6) com diretores indicados por Platinum; Mozilla distribui autoridade por contribuição; Debian combina líder eleito, Resoluções Gerais, Comitê Técnico e maioria 3:1 para documentos fundacionais); desenho policêntrico segundo os oito princípios de Ostrom | Menos de 3 mantenedores ativos por 2 meses; nenhuma doação em 90 dias; concentração de assentos ou votos em um único patrocinador [HIPÓTESE] | Soberania do Jogador |
| Fragmentação por forks (bifurcações do código) | Alto | Média | Variações vivem no Módulo, não no Núcleo; catálogo de módulos e convenção de versão; servidores rodam regras próprias sem copiar o Núcleo; especificação de protocolo separada da implementação, com testes de conformidade; regras canônicas versionadas para torneios; direito de saída com exportação de dados e personagens — o precedente OpenOffice → LibreOffice mostra a comunidade se deslocando (cerca de 250 desenvolvedores ativos contra 4 em cinco anos) | Dois forks do Núcleo em 6 meses ou queda acima de 30% em servidor oficial após publicação de fork; módulo conforme que roda em uma única instância [HIPÓTESE] | Soberania do Jogador |
| Trapaça com cliente aberto; anti-cheat (anti-trapaça) de kernel proprietário é inaceitável | Alto | Alta | Servidor autoritativo aceita só inputs; validação de alcance, tempo e custo; detecção estatística de padrões; nenhum driver de kernel; nenhuma leitura da memória do jogador | Mais de 1% das partidas com divergência de estado cliente-servidor; acusações em 3 servidores distintos em uma semana [HIPÓTESE] | Soberania do Jogador; Ação Coletiva |
| Complexidade de netcode (código de rede): janela de 5 s, reconciliação e estado de ilha | Alto | Alta | Estudar os repositórios citados só como referência de infraestrutura; começar com 2 a 4 jogadores; servidor sem interface; janela decidida no servidor; testes com clientes simulados | p95 acima de 250 ms ou mais de 2 h semanais em depuração de dessincronia [HIPÓTESE] | Ação Coletiva |
| Temática política afastar jogadores | Médio | Média | Tom materialista e decolonial sem panfleto: o conflito emerge das mecânicas de controle dos meios de produção; módulos permitem outros temas; onboarding pela ação, não pela leitura | Queda acima de 40% entre a primeira e a segunda sessão; reclamação recorrente de "política demais" em 3 servidores [HIPÓTESE] | Luta Decolonial |
| Protocolo aberto que gera jardins murados ("Permissionless Completeness Problem" do fediverso: padrão W3C ActivityPub, software e instâncias são camadas distintas) | Alto | Média | Separar especificação, implementação e hospedagem; testes de conformidade públicos; exportação de dados e personagens para mudança de instância; sanções graduadas com apelação, como no Lichess, em vez da cláusula "nossa palavra é final" do Chess.com | Uma instância concentrar mais de 50% dos servidores ativos ou exigir conta própria para torneios canônicos [HIPÓTESE] | Soberania do Jogador |

Critérios de aceitação da mitigação anti-trapaça:
- Um cliente modificado que envia posição pronta é ignorado; o servidor deriva a posição dos inputs.
- Habilidade com custo insuficiente ou recarga ativa é recusada sem efeito no mundo.
- Ativação de ilha sem condição prévia não gera evento de captura.
- Um relatório diário lista divergências por cliente para auditoria, sem exigir acesso ao equipamento do jogador.

## 19. Fora do Escopo

Limites da v1, com o motivo e a condição de revisão. Nada aqui é veto permanente: cada item volta quando o marco da última coluna for observado nas métricas da seção 17.

| Item | Justificativa | Revisitar quando |
|---|---|---|
| Matchmaking centralizado e lobby | Contradiz servidores federados; o MVP exige conexão direta; um serviço único pode encerrar a prática, como mostram The Crew e os termos unilaterais de instâncias FOSS hospedadas | Houver ao menos 5 servidores ativos e fila real; preferir navegador de servidores federado com exportação de dados |
| Anti-cheat de kernel proprietário | Invasivo, proprietário e incompatível com FOSS; a fonte o aponta como inaceitável | surgir alternativa livre e auditável — hoje descartado por princípio |
| Assets AAA, incluindo o pacote de Paragon | Custo proibitivo e estética de fantasia épica incompatível com o materialismo decolonial | Existir pacote FOSS de baixa poligonalidade com direção de arte já validada |
| Campanha narrativa longa | O produto é uma arena de regras, não uma sequência de missões | Após a estabilidade da arena, como Módulo opcional |
| Suporte a 10 mil jogadores simultâneos | Escala de serviço centralizado, oposta à federação; o Núcleo mira partidas pequenas | Um servidor único demandar mais de 200 jogadores simultâneos [HIPÓTESE] |
| Mobile e controles touch | A fonte cita suporte touch em projeto alheio; amplia a superfície de teste da interface | Depois de estabilizados netcode e interface desktop |
| Elenco completo (Intelectual, Camponês, Agitador, Aliança Mercantil) | O MVP valida apenas Artesão e Guerreiro | Fase 5, após a condição de vitória funcionar |
| Partidas 5v5 completas | Meta do jogo base, não do protótipo | Com servidores estáveis e latência dentro da meta |
| Penalidade de grupo da Ilha Topo (dobra única ×2: com 1 aliado adicional, 4 → 8 guardas, sem segundo dobramento) | O desafio do MVP é o da lógica simples; a penalidade alteraria o teste da ilha. A variante ×5 pertence apenas ao protótipo de caixas, não ao jogo | Assim que a captura solo estiver estável e medida |
| Editor visual de conteúdo | A fonte o chama de ferramenta mais importante do framework, mas ele não é necessário para provar a arquitetura | Imediatamente após o formato de Módulo ser validado na Fase 4 |
| Neblina de guerra e furtividade por arbustos | Exige estado de visão por equipe e mais sincronia | Fase 5, com o protocolo de rede consolidado |
| IA de enxame e comportamento de grupos | Projeto citado serve de estudo para NPCs, não de entrega do MVP | Quando as ilhas passarem a ter forças de segurança próprias |
| Cosméticos, monetização e ligas oficiais | Dependem de governança definida; receita não pode virar controle das regras; circuito de torneios exige regras canônicas versionadas e testes de conformidade | Após o estatuto, a especificação de protocolo e a política de módulos existirem |
| Localização ampla e acessibilidade completa | Custo sem efeito sobre a prova arquitetural | Após a interface e os critérios de aceitação estarem fechados |
| Direito de saída e portabilidade de conta e personagens | Fork do código não garante liberdade de sair da instância hospedada (casos Chess.com e Lichess) | Após o formato de Módulo e a especificação de protocolo estarem estáveis |

## 20. Suposições, Hipóteses e Perguntas Abertas

### 20.1 Suposições [SUPOSIÇÃO]

- S1 — Os quatro pilares canônicos são Soberania do Jogador, Materialismo Histórico, Luta Decolonial e Ação Coletiva; as seções 16 a 19 usam essa nomenclatura.
- S2 — Escala do MVP de 2 a 4 jogadores por partida; o jogo base permanece 5v5.
- S4 — "Servidor comunitário ativo" significa ao menos uma partida completa nos últimos 7 dias.
- S5 — A cadeia fantasia → pilares → loop → mecânicas da subseção 16.5 é a das seções 5 a 7.
- S6 — O build de servidor sem interface da Godot atende ao Núcleo; validar na Fase 0.
- S7 — A governança será uma Fundação, mas fundações não são automaticamente horizontais (Linux Foundation, Mozilla e Debian têm arranjos de autoridade distintos); o modelo específico fica pendente e não deve ser presumido a partir da licença MIT.

### 20.2 Hipóteses numéricas [HIPÓTESE]

- H1 — Ilha Topo: 8 s por alavanca, reinício em 2 s, limite de 45 s.
- H2 — Ilha Meio: janela de 5 s em ambas as escalas, gás lacrimogêneo com 8 pontos de dano por segundo e expulsão após 3 s de exposição contínua; protótipo com 2 jogadores e produção com 3.
- H3 — Ilha Inferior: 3 s por ciclo, 5 Materiais, 3 ciclos de estoque, 20 s de recarga.
- H4 — Barricada: 20 Materiais, 1,5 s de canalização, 200 pontos de vida, 12 s de duração.
- H5 — Ataque Forte: 1,2 s de canalização, 2× dano base, 6 s de recarga.
- H6 — Ilha Central do MVP: três ilhas de rota como pré-requisito (em vez de seis na produção) e 4 s de canalização (em vez de 20 s de Propaganda na produção), com Artesão e Guerreiro vivos e presentes — não qualquer membro. [HIPÓTESE de escopo — justificativa na subseção 16.3.] A regra de produção permanece seis ilhas + Plano Mestre.
- H7 — Duração das fases 0 a 4: semanas 1–2, 3–4, 5–7, 8–9 e 10–12.
- H8 — Tempo até primeira captura: mediana de 90 s e p95 de 150 s.
- H9 — Conclusão do desafio sincronizado entre 60% e 90%.
- H10 — Latência: 80 ms e 150 ms de mediana, 250 ms de p95, 90% de percepção positiva.
- H11 — Comunidade: 5 e 20 servidores; 3 e 10 módulos em 3 e 12 meses.
- H12 — Limiares de alerta dos riscos: 20% do tempo em arte, 3 mantenedores, 90 dias sem doação, 2 forks, 30% de queda, 1% de divergência, 3 servidores com reclamação, 250 ms, 2 h semanais, 40% de queda entre sessões.
- H13 — Calibração das metas após 20 partidas instrumentadas.
- H14 — Quórum de 2/3 para alterações no protocolo canônico e apelação de sanções em até 7 dias [HIPÓTESE — referência: maioria 3:1 da Debian para documentos fundacionais e sanções graduadas com apelação no Lichess; ajustar à nossa escala].

### 20.3 Notas para o designer [NOTA PARA O DESIGNER]

- N3 — Licença do Núcleo (resolvida, ADR-002): MIT para todo o código do ciclo, inclusive o servidor headless; sem cláusula "ou GPL" e sem AGPL. A Godot é MIT (§14.6).
- N4 — Formato canônico do Módulo: arquivo `.pck` da Godot ou JSON/XML? Trata-se da API de carregamento.
- N6 — Quais intenções o cliente pode submeter sem validação? Essa fronteira define o anti-cheat no servidor.
- N8 — Qual desenho de governança adotar: oito princípios de Ostrom com governança policêntrica, ou estatuto de Fundação com eleições, quórum, sanções graduadas, apelação e direito de saída? A escolha define se a Soberania do Jogador é mecanismo ou retórica.

### 20.4 Perguntas abertas bloqueantes

| Pergunta | O que bloqueia | Responder até |
|---|---|---|
| N3 — licença do Núcleo (resolvida: MIT — ADR-002) | Publicação do repositório e do catálogo de módulos | Primeira publicação |
| N4 — formato do Módulo | API de carregamento do Núcleo | Fase 4 |
| N6 — fronteira de confiança do cliente | Protocolo de rede e anti-cheat | Fase 0 |
| N8 — modelo de governança, quórum e direito de saída | Estatuto da Fundação, especificação de protocolo e catálogo de módulos | Primeira publicação |

Enquanto N4, N6 e N8 não são respondidos (N3 resolvida por ADR-002 — MIT), a Fase 0 segue com S1, S6 e S7 e com a validação mínima de inputs do item 18; o desenho de governança (N8) fica registrado como pendência, não como decisão.

### 20.5 Decisões registradas (não são suposições abertas)

- S3 (resolvida) — A sincronia de 2 jogadores no protótipo é redução de escala decidida, não suposição: a regra de produção permanece 3 jogadores em 5 s. Decisão registrada na subseção 16.3.
- N7 — A decisão Godot-first da subseção 16.2 é vinculante; revertê-la invalida o roadmap e a mitigação de débito técnico.

## 21. Referências e Glossário

### 21.1 Fontes locais

- `docs/Esporte vs. E-sport_ Controle das Regras.md` — soberania das regras, arquitetura Núcleo/Módulos, pilares, funções e escopo do MVP.
- `docs/Jogos MOBA Open Source e Frameworks.md` — mapeamento de codebases e assets, armadilha dos frameworks tradicionais, opção Godot e roadmap de engenharia.

### 21.2 Skills carregadas

`game-design-core`, `gds-gdd`, `gdd-writing`, `gdd-writer` — disciplina de escrita de documento de projeto, densidade, rastreabilidade e separação entre intenção e implementação. [SUPOSIÇÃO — atribuição de papel inferida pelos nomes.]

### 21.3 Sites e repositórios citados nas fontes

- godotengine.org — documentação e distribuição da Godot; base da decisão Godot-first.
- Kenney.nl — biblioteca FOSS de assets de baixa poligonalidade para o protótipo.
- GDQuest — referência citada para multiplayer em Godot 4.x.
- github.com/nival/Prime-World — MOBA comercial liberado; uso comercial proibido.
- github.com/yasgamesdev/MOBA_CSharp_Unity — referência de netcode desacoplado; não adotado como base (item 16.2).
- github.com/tammukul/UNION-OpenSource-MOBA — referência de integração em tempo real e controles touch.
- github.com/DevZhav/unity-moba — referência de microsserviços de conta e partida.
- github.com/yasgamesdev/OpenMOBA — referência de prototipagem visual em Unreal; licença MIT.
- github.com/danielholmes839/GoBA2 — referência de escalabilidade de conexões WebSocket.
- github.com/RichardAragon/MOBASwarmAgents — referência de comportamento de enxame para NPCs.
- Unreal Marketplace (pacote Paragon) — citado apenas como contraste de custo e estética.


### 21.4 Referências verificadas — governança e ecossistema

- Ostrom, *Governing the Commons* (1990) — oito princípios de desenho institucional (limites claros; equivalência entre benefícios e custos; escolha coletiva; monitoramento pelos usuários; sanções graduadas; resolução barata e local; direito de se organizar; empreendimentos aninhados) e governança policêntrica. https://ostromworkshop.indiana.edu/courses-teaching/teaching-tools/ostrom-design/index.html
- Debian Constitution — líder eleito, Resoluções Gerais, Comitê Técnico, maioria 3:1 para documentos fundacionais e direito de saída: fundações organizam poder de formas diferentes. https://www.debian.org/devel/constitution
- *Permissionless Completeness Problem* no fediverso — camadas de padrão W3C ActivityPub, software e instâncias; protocolo aberto não impede jardins murados. https://ijoc.org/index.php/ijoc/article/download/22254/4864
- The Document Foundation — histórico da migração OpenOffice → LibreOffice (2010) e da comunidade resultante. https://www.documentfoundation.org/history/
- Lichess Terms of Service — associação francesa sob AGPL-3.0, mas encerramento de conta sem aviso e cláusula "nossa palavra é final": o código é livre, a governança da instância não. https://lichess.org/terms-of-service
- BBC — encerramento de The Crew (2014–2024, 12 milhões de jogadores) e campanha Stop Killing Games (cerca de 1,3 milhão de assinaturas na Comissão Europeia; audiência no Parlamento em abril de 2026). https://www.bbc.com/news/articles/c8e8e7g0r82o

### 21.5 Glossário

- **Arquipélago da Insurgência** — nome canônico do jogo base: conjunto de ilhas interligadas que substitui o mapa de rotas contínuas dos MOBAs.
- **Ação Coletiva** — pilar segundo o qual a vitória depende da coordenação de funções, não de desempenho individual.
- **Anti-cheat (anti-trapaça)** — conjunto de medidas contra trapaça; aqui restrito à validação no servidor.
- **Comuna** — base inicial autônoma da equipe; substitui o castelo ou Nexus.
- **Desafio de Ilha** — provação que substitui a torre: a ilha só é capturada ao superá-la.
- **Fantasia** — a experiência imaginada que orienta as decisões de design; precede pilares, loop e mecânicas.
- **FOSS** — sigla para *Free and Open Source Software* (Software Livre e de Código Aberto).
- **Fork (bifurcação do código)** — cópia de um projeto para criar versão independente; no Insurgência, a variação deve ocorrer no Módulo.
- **Greyboxing (protótipo com formas geométricas sem textura)** — uso de cubos e cilindros para testar mecânicas antes da arte.
- **Ilha Central** — objetivo final: ativar seu desafio define a vitória.
- **Luta Decolonial** — pilar temático do conflito entre uma Comuna e uma força opressora.
- **Matchmaking (pareamento de partidas)** — serviço que monta partidas; ausente no MVP.
- **Materialismo Histórico** — pilar que exige ações materiais e consequências reais, sem magia.
- **Módulo** — pacote de conteúdo e regras lido pelo Núcleo, sem alterar o código-base.
- **Netcode (código de rede)** — conjunto de técnicas de conexão, predição, reconciliação e sincronização de estado.
- **Núcleo** — motor de regras e rede, sem personagens ou mapas específicos.
- **RTT** — tempo de ida e volta de um pacote entre cliente e servidor, em milissegundos.
- **Servidor autoritativo** — servidor que decide o estado da partida e aceita apenas intenções dos clientes.
- **Soberania do Jogador** — pilar anarquista: regras e infraestrutura pertencem a quem joga.
- **Súditos** — grupos de trabalhadores, propagandistas ou milicianos que avançam pelas rotas; substituem os súditos (*minions*) tradicionais.
### 21.6 Referências verificadas na pesquisa (19/11/2026)

As URLs abaixo foram consultadas durante a redação deste documento. A distinção entre fato verificado e inferência está feita no corpo das seções correspondentes.

**Fontes primárias do projeto**
- `docs/Esporte vs. E-sport_ Controle das Regras.md` — conversa de 27/10/2025 a 08/11/2025, atualizada em 22/02/2026.
- `docs/Jogos MOBA Open Source e Frameworks.md` — conversa de 22/02/2026.

**Metodologia de GDD**
- BMad `gds-gdd`: https://github.com/bmad-code-org/bmad-module-game-dev-studio/blob/main/src/workflows/2-design/gds-gdd/SKILL.md
- BMad, template de GDD: https://github.com/bmad-code-org/bmad-module-game-dev-studio/blob/main/src/workflows/2-design/gds-gdd/assets/gdd-template.md
- BMad, checklist de validação: https://github.com/bmad-code-org/bmad-module-game-dev-studio/blob/main/src/workflows/2-design/gds-gdd/assets/gdd-validation-checklist.md
- Fullerton, *Game Design Workshop*, cap. 14: https://flylib.com/books/en/2.489.1.112/1/
- Schreiber, *Game Design Concepts*: http://gamedesignconcepts.pbworks.com/f/Game+Design+Concepts+-+An+experiment+in+game+design+and+teaching.pdf
- Codecks, documentos de design vivos: https://www.codecks.io/learning/writing-modern-game-design-documents/
- Ella Romanos, como escrever um GDD útil: https://fundamentally.games/knowledge-base/how-to-write-a-useful-game-design-document/
- Brenda Romero, "a mecânica é a mensagem": https://www.museumofplay.org/blog/brenda-romero-the-mechanic-is-the-message/
- Elizabeth Sampat, GDC 2017: https://www.gdcvault.com/play/1023994/Systems-Make-Statements-Simulations-and/
- Riot, framework de balanceamento do Wild Rift: https://wildrift.leagueoflegends.com/en-us/news/dev/dev-balance-framework-update/
- Riot, determinismo em *League of Legends*: https://engineering.riotgames.com/news/determinism-league-legends-introduction

**Teoria do jogo, regras e governança**
- Huizinga, *Homo Ludens* (1938), texto integral: https://archive.org/stream/homoludensstudyo1950huiz/homoludensstudyo1950huiz_djvu.txt
- Edição brasileira de *Homo Ludens*, Perspectiva: https://editoraperspectiva.com.br/produtos/homo-ludens-nova-edicao-revisada/
- Caillois, *Les jeux et les hommes* (1958): https://creativegames.org.uk/modules/Intro_Game_Studies/Caillois_Man_Play_Games_Chapters1_2-2001.pdf
- Salen e Zimmerman, *Rules of Play*, cap. 9: http://www.systemsforplay.com/wp-content/uploads/2014/02/Salen_Zimmerman_Rules_of_Play_Ch9.pdf
- Stenros, sobre o "círculo mágico": https://todigra.org/index.php/todigra/article/download/1701/1701
- FIDE, *Laws of Chess*: https://handbook.fide.com/chapter/E012023
- Contrato de usuário do Chess.com: https://www.chess.com/legal/user-agreement
- EULA da Riot Games: https://raw.communitydragon.org/9.5/plugins/rcp-be-lol-license-agreement/riot/en_us/eula.txt
- Termos de serviço do Lichess: https://lichess.org/terms-of-service
- BBC, encerramento de *The Crew* e campanha Stop Killing Games: https://www.bbc.com/news/articles/c8e8e7g0r82o
- Lei Geral do Esporte (Lei 14.597/2023): http://www.planalto.gov.br/ccivil_03/_ato2023-2026/2023/lei/l14597.htm
- Debates sobre a definição de e-sport: https://link.springer.com/article/10.1007/s12662-025-01054-9
- Ostrom, *Governing the Commons* (1990), princípios: https://ostromworkshop.indiana.edu/courses-teaching/teaching-tools/ostrom-design/index.html
- Cox et al., tabela dos princípios de Ostrom: https://ecologyandsociety.org/vol9/iss1/art18/table3.html
- Constituição do Debian: https://www.debian.org/devel/constitution
- Estatutos da Linux Foundation: https://www.linuxfoundation.org/legal/bylaws
- Governança da Mozilla: https://www.mozilla.org/en-US/about/governance/
- Fediverso e "Permissionless Completeness Problem": https://ijoc.org/index.php/ijoc/article/download/22254/4864
- História da LibreOffice / The Document Foundation: https://www.documentfoundation.org/history/

**Motor, licenças e redes**
- Godot, versões: https://raw.githubusercontent.com/godotengine/godot-website/master/_data/versions.yml
- Godot 4.7: https://godotengine.org/releases/4.7/
- Godot, licença MIT: https://godotengine.org/license/
- Godot, conformidade com licenças: https://docs.godotengine.org/en/stable/about/complying_with_licenses.html
- Godot Foundation, política de marcas: https://godot.foundation/policies-and-procedures/trademark-policy
- Godot, servidores dedicados: https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_dedicated_servers.html
- Godot, multiplayer de alto nível: https://docs.godotengine.org/en/stable/tutorials/networking/high_level_multiplayer.html
- Godot, C#/.NET: https://docs.godotengine.org/en/stable/tutorials/scripting/c_sharp/index.html
- Godot, `MultiplayerSynchronizer`: https://docs.godotengine.org/en/latest/classes/class_multiplayersynchronizer.html
- Linietsky, por que Godot não é baseado em ECS: https://godotengine.org/article/why-isnt-godot-ecs-based-game-engine/
- Gambetta, predição e reconciliação: https://www.gabrielgambetta.com/client-side-prediction-server-reconciliation.html
- Gambetta, interpolação de entidades: https://gabrielgambetta.com/entity-interpolation.html
- Bernier (Valve), compensação de latência: https://www.gamedevs.org/uploads/latency-compensation-in-client-server-protocols.pdf
- Gerenciamento de área de interesse: https://www.ee.ucl.ac.uk/lcs/previous/LCS2011/LCS1121.pdf
- Anti-cheat do Xonotic: https://rsync.xonotic.org/release/Xonotic-high/source/qcsrc/server/anticheat.qc
- XGUARDIAN (USENIX Security 2026): https://www.usenix.org/system/files/conference/usenixsecurity26/sec26_prepub_zhang-jiayi.pdf
- OpenRA, processamento de ordens e rede: https://deepwiki.com/OpenRA/OpenRA/3.4-order-processing-and-networking
- OpenRA, configuração declarativa: https://deepwiki.com/OpenRA/OpenRA/4-data-driven-configuration
- OpenRA, versionamento de protocolo: https://raw.githubusercontent.com/OpenRA/OpenRA/bleed/OpenRA.Game/Server/ProtocolVersion.cs
- Overwatch, arquitetura de jogabilidade (GDC): https://www.gdcvault.com/play/1024001/Overwatch-Gameplay-Architecture-and
- For Honor, modificadores orientados a dados (GDC): https://gdcvault.com/play/1024580/Modify-Everything-Data-Driven-Dynamic

**Codebases avaliadas**
- Prime World: https://github.com/nival/Prime-World — licença: https://github.com/nival/Prime-World/blob/main/LICENSE.md
- MOBA CSharp Unity: https://github.com/yasgamesdev/MOBA_CSharp_Unity
- UNION: https://github.com/tammukul/UNION-OpenSource-MOBA
- unity-moba: https://github.com/DevZhav/unity-moba
- OpenMOBA: https://github.com/yasgamesdev/OpenMOBA
- GoBA2: https://github.com/danielholmes839/GoBA2
- MOBASwarmAgents: https://github.com/RichardAragon/MOBASwarmAgents
- Assets de Paragon, anúncio da Epic: https://forums.unrealengine.com/t/12-million-worth-of-paragon-assets-released-updated/108140
- Xonotic: https://github.com/xonotic/xonotic · OpenRA: https://github.com/OpenRA/OpenRA · Veloren: https://github.com/veloren/veloren

**Assets livres**
- Kenney: https://kenney.nl/assets
- OpenGameArt, FAQ de licenças: https://opengameart.org/content/faq
- Poly Pizza, termos: https://poly.pizza/docs/tos
- Quaternius, QAL v1.0: https://quaternius.com/license.html
- itch.io, filtro CC0: https://itch.io/game-assets/assets-cc0/tag-cc0

**Design decolonial, cooperação e representação**
- Jogo educativo Kaingang, SBGames 2026: https://sol.sbc.org.br/index.php/sbgames_estendido/article/download/45985/45746/
- Banzo — Marks of Slavery (Sabbatini, 2022), DOI: 10.55738/alaic.v20i38.754
- Carneiro e Dias, ANTARES 12(28), 2020: https://sou.ucs.br/etc/revistas/index.php/antares/article/download/9209/4482/35299
- *Never Alone / Kisima Inŋitchuŋa*: https://www.theguardian.com/technology/2014/sep/29/never-alone-alaskas-indigenous-game-never-alone-teaches-cooperation-through-stories
- Tripathy, *Humanities* 12(83), 2023: https://www.mdpi.com/2076-0787/12/4/83
- TESA Collective, *STRIKE!*: https://www.tesacollective.com/teamed-up-with-labor-movement-make-strike-game-worker-rebellion/
- *Half-Earth Socialism*: https://spaceandtim.es/projects/half_earth_socialism/
- Jogos e anarquismo (de la Vega, 2025): https://theanarchistlibrary.org/library/don-diego-de-la-vega-the-strategic-potential-of-games-in-anarchist-organization.a4.pdf
- Lauret, os quatro átomos da cooperação: https://www.gamedeveloper.com/design/the-four-atoms-of-cooperative-video-games
- Linderoth, Chapman e Deterding, *The Limits of 'Serious' Play*: https://spiral.imperial.ac.uk/server/api/core/bitstreams/d61c28a6-8dbb-4ad7-b359-178cbdbb8a18/content
- OFELAŠ Pathfinder, princípios Sámi: https://isfi.no/guidelines/
