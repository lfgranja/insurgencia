# Concept — Tela de resultado: exibição da vitória declarada pelo servidor

**Data:** 2026-11-20T11:39Z

## Contexto

`docs/GDD - Insurgência.md:1539` fecha o critério de aceitação: "Com as três ilhas, Artesão e Guerreiro vivos e presentes na Ilha Central e 4 s de canalização conjunta contínua, o servidor declara vitória e exibe tela de resultado". É o último item `[ ]` do checklist 16.7 (`:1569`). O servidor já produz o evento real em `nucleo/desafios/ilha_central.gd`: constante `EVENTO_VITORIA="vitoria_ilha_central"` (`:11`), sinal `vitoria_ilha_central(resultado: Dictionary)` (`:14`) e resultado com as chaves `evento`, `declarada_por="servidor"` e `tempo_segundos` (`:74-78`), além de `estado_persistido=EVENTO_VITORIA` (`:73`). A tela é apenas consumidora desse resultado; a decisão permanece no servidor. Escopo solo/single confirmado pela SE (`.specify/memory/STATE.md:9`): sem 5v5, lobby, NAT ou rede nova.

## Opções consideradas

1. **Componente `Control` programático, sem `.tscn` — escolhida** — `nucleo/apresentacao/tela_resultado.gd` estende `Control`, cria seus `Label` filhos com `Label.new()`/`add_child()` e expõe `definir_resultado(resultado: Dictionary)`. É instanciável por `.new()` com `preload` (tipos customizados não resolvem por `class_name` em `--headless --script`), testável em `SceneTree` sem editor e sem carregar cena principal. Consome literalmente as chaves reais do evento (`evento`, `declarada_por`, `tempo_segundos`) e só exibe vitória quando `evento=="vitoria_ilha_central"` e `declarada_por=="servidor"`; sem evento, permanece em `AGUARDANDO`. É a fatia vertical mínima que fecha `:1539` sem antecipar interface de produção, partidas 5v5 ou rede.
2. **Cena `.tscn` declarativa com nós `Label` (rejeitada)** — escrever a cena à mão introduz risco de formato (cabeçalho, extensões, `ext_resource`, UID de recursos e caminhos `res://`) que só poderia ser validado carregando a cena, e o fluxo verificado do repo é `--headless --script`, sem editor Godot disponível para gerar ou inspecionar o arquivo. A opção 1 elimina esse risco ao construir a árvore em código, com o mesmo resultado visual e verificação determinística.
3. **Saída de texto puro no console/`print` (rejeitada)** — imprimir "Vitória" em `stdout` não é uma tela: não há componente de interface, não há `Control`, não há `Label` e nada pode ser exibido ao jogador. O GDD pede explicitamente "exibe tela de resultado"; essa opção satisfaria apenas o log de teste, não o requisito, e não permitiria distinguir estado neutro de vitória na interface.

## Consequências

- Arquivos restritos a `nucleo/apresentacao/tela_resultado.gd` e `tests/test_tela_resultado.gd`, mais os artefatos documentais deste ciclo; nenhum `.tscn`, nenhuma cena principal, nenhuma dependência externa e nenhuma alteração de gate, baseline, `Makefile` ou portaria nesta etapa de escrita.
- O componente mantém exatamente dois estados observáveis (`VITORIA` e `AGUARDANDO`) e três `Label` criados programaticamente; a fonte única é o dicionário do evento do servidor, com as chaves literais de `ilha_central.gd`, nunca um campo de comando do cliente.
- A suíte em `SceneTree` usa `preload` para o tipo customizado, TAB, tipos explícitos e contagem binária de `PASS`/`OK`/exit 0; cobre o evento real, o estado neutro sem evento e a recusa em inventar vitória com entradas inválidas.
- Execuções de Godot, testes, `make check` e gates não são observadas nesta etapa e permanecem **[NV]** até registro com data/hora na portaria própria; nenhum artefato R4/R7 é escrito aqui.
- Fora de escopo: 5v5, lobby, NAT/Internet, UDP novo, servidor adicional, regra de produção (seis ilhas/Plano Mestre/20 s), deploy, remote e qualquer alegação de execução ao vivo.
