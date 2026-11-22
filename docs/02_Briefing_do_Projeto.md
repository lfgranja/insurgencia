# Briefing de Projeto: "Insurgência" (Provisório)

**Data:** 27 de outubro de 2025
**Preparado por:** Gemini
**Para:** lfgranja

## 1. Resumo Executivo

"Insurgência" não é um único jogo, mas um framework de código aberto (FOSS) para a criação de Arenas de Batalha Multijogador Online (MOBAs). O projeto visa desafiar o modelo de "serviço como software" proprietário, devolvendo o controle das regras e do espaço de jogo à comunidade.

O "jogo base" (a primeira implementação deste framework) será um MOBA 5v5 focado em estratégia assimétrica, ação coletiva e temas decoloniais e materialistas.

## 2. Visão e Pilares Filosóficos

- **Soberania do Jogador (Anarquista):** O jogo pertence a quem joga (arquitetura federada e modular).
- **Materialismo Histórico (Design):** Habilidades são ações materiais, não "magia".
- **Luta Decolonial (Temático):** O conflito é assimétrico (Comuna vs. Opressor).
- **Ação Coletiva (Jogabilidade):** A estratégia coletiva supera o brilho individual.

## 3. Conceito Central da Jogabilidade

- **Objetivo:** Controlar e ativar a Ilha Central.
- **Mapa:** Um arquipélago onde "rotas" são cadeias de ilhas.
- **Defesas:** Desafios de Ilha (lógicos, cooperativos) em vez de "torres".
- **Recursos:** "Materiais" e "Suprimentos" além do "ouro".

## 4. Pilares de Design

- **Rotas Assimétricas:** Superior (1v1, lógica), Central (equipe, sincronia), Inferior (rede, economia).
- **Funções Baseadas em Ações:** Intelectuais, Artesãos, Guerreiros, Camponeses, Agitadores.

## 5. Estrutura Técnica (Arquitetura do Framework)

- **Motor de Jogo:** Godot Engine (com suporte a C#).
- **Arquitetura:**
    - **Núcleo "Insurgência" (Framework):** Lógica de rede, APIs de jogo (Habilidade, Personagem, etc.), software de servidor dedicado. Não contém conteúdo de jogo.
    - **Módulos (Jogos/Forks):** Pacotes de conteúdo (personagens, mapas, regras) que o servidor carrega.

## 6. Escopo Inicial (Produto Mínimo Viável - MVP)

- **Funcionalidade:** Conexão de rede direta (Cliente-Servidor).
- **Mapa:** 1 Mapa "Arquipélago" com 3 ilhas + 1 Ilha Central.
- **Desafios:** 1 de lógica (Topo), 1 de sincronia (Meio), 1 ponto de coleta (Inferior).
- **Personagens:** 2 Funções (Artesão e Guerreiro), cada um com 1 habilidade.
- **Condição de Vitória:** Ativar o desafio da Ilha Central.
- **Objetivo do MVP:** Provar a arquitetura "Núcleo + Módulo" e a jogabilidade baseada em desafios.
