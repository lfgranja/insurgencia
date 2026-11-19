# Guia de Aplicação dos Princípios de Engenharia de Software

Este documento traduz os princípios clássicos de engenharia de software para a prática de desenvolvimento do projeto "Insurgência".

## SOLID

-   **S (Responsabilidade Única):** Uma classe faz uma coisa só. `Personagem.cs` lida com vida/movimento; `HabilidadeConstruirBarricada.cs` lida com a lógica da barricada.
-   **O (Aberto/Fechado):** O código do Core (`src/core`) é fechado para modificação, mas aberto para extensão. Criamos novos personagens em `src/modules` herdando do Core, sem alterá-lo.
-   **L (Substituição de Liskov):** O código deve poder tratar um `Guerreiro` ou um `Artesao` simplesmente como um `Personagem`, sem precisar saber o tipo específico.
-   **I (Segregação de Interfaces):** Usar interfaces pequenas e específicas (`IColetavel`, `ISabotavel`) em vez de uma interface monolítica `IInteragivel`.
-   **D (Inversão de Dependência):** Classes devem depender de abstrações, não de implementações. O `Personagem` depende da classe abstrata `Habilidade`, não de uma habilidade concreta.

## DRY (Don't Repeat Yourself)

-   **Aplicação:** Usar a herança de cenas do Godot. Se todo personagem tem uma barra de vida, ela é criada uma única vez na cena `PersonagemBase.tscn` e herdada por todos.

## YAGNI (You Ain't Gonna Need It)

-   **Aplicação:** Focar exclusivamente no escopo do MVP. Se o MVP define 1 habilidade por personagem, não implementaremos um sistema de status (veneno, lentidão) até que seja realmente necessário.

## KISS (Keep It Simple, Stupid)

-   **Aplicação:** Implementar a solução mais simples que funcione. Para um quebra-cabeça de "ativar 3 alavancas na ordem correta", um script com uma ordem hardcoded é suficiente para o MVP. A complexidade só é adicionada quando necessária.

## TDD (Test-Driven Development)

-   **Aplicação:** Usar um framework de testes (como GdUnit) para a lógica do Core. Escrever testes para os métodos de `Personagem.cs` (como `ReceberDano`) antes de implementar o método em si, garantindo que as regras do jogo sejam robustas e testáveis.

## DDD (Domain-Driven Design)

-   **Aplicação:** O código fala a língua do jogo. As classes se chamam `Personagem`, `Ilha`, `Agitador`, refletindo o domínio do jogo. Os "Contextos Delimitados" do DDD são mapeados para nossa arquitetura: `src/core` (Domínio Central) e `src/modules/*` (Subdomínios).
