# Setup Pré-Desenvolvimento: A Fundação do Código

Este documento descreve os passos a serem executados dentro do ambiente Godot **antes** de iniciar o desenvolvimento do protótipo (MVP). O objetivo é criar a arquitetura base do framework.

## 1. Configuração do Projeto no Godot

1.  **Abra o Godot Engine.**
2.  Clique em **"Novo Projeto"**.
3.  Em "Caminho do Projeto", navegue e selecione a pasta raiz do projeto `Insurgência/` (que já contém o `.git/` e as subpastas).
4.  Nomeie o projeto como "Insurgência" e confirme a criação. Isso gerará o arquivo `project.godot`.

## 2. Definição das Classes Base Abstratas (`src/core/`)

Criar os scripts que servirão como "contratos" para todos os objetos do jogo.

-   **`Personagem.cs`:**
    -   **Propósito:** Define um personagem jogável.
    -   **Propriedades:** `vidaMaxima`, `vidaAtual`, `velocidadeMovimento`.
    -   **Métodos Abstratos:** `Mover()`, `ReceberDano()`, `UsarHabilidade()`, `Morrer()`.

-   **`Habilidade.cs`:**
    -   **Propósito:** Define uma habilidade.
    -   **Propriedades:** `tempoDeRecarga`, `custoDeRecurso`.
    -   **Métodos Abstratos:** `Ativar()`.

-   **`Ilha.cs`:**
    -   **Propósito:** Define uma ilha no mapa.
    -   **Propriedades:** `listaDeDesafios`, `estaConquistada`.
    -   **Métodos Abstratos:** `IniciarIlha()`, `VerificarCondicaoDeVitoria()`.

-   **`DesafioDeIlha.cs`:**
    -   **Propósito:** Define um desafio dentro de uma ilha.
    -   **Propriedades:** `estaCompleto`.
    -   **Métodos Abstratos:** `IniciarDesafio()`, `VerificarProgresso()`.

## 3. Criação da Arquitetura de Cenas Base (`src/core/`)

Criar cenas no Godot que usarão os scripts acima e servirão de modelo para o conteúdo real do jogo.

-   **`PersonagemBase.tscn`:**
    -   **Estrutura:** Um nó `CharacterBody3D` com o script `Personagem.cs` anexado.
    -   **Propósito:** Todos os personagens do jogo (Guerreiro, Artesão) serão **cenas herdadas** desta.

-   **`IlhaBase.tscn`:**
    -   **Estrutura:** Um nó `Node3D` com o script `Ilha.cs` anexado.
    -   **Propósito:** Todas as ilhas do mapa herdarão desta cena.

## 4. Primeiro Commit da Arquitetura

Após criar os scripts e cenas base, um commit deve ser feito com uma mensagem clara, como: `feat(core): Define classes e cenas base para o framework`.
