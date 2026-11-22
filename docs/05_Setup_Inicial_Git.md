# 05: Log da Configuração Inicial do Repositório Git

Este documento registra o processo passo a passo, incluindo erros e soluções, para conectar o repositório local ao GitHub e enviar o código inicial do projeto "Insurgência".

## 1. Estratégia de Branches

Foi definido que a estratégia de usar o branch `dev` como o principal de desenvolvimento e proteger o `main` para releases estáveis é uma excelente prática profissional, alinhada com fluxos de trabalho como o **Git Flow**.

- **`main`**: Apenas para versões estáveis e lançadas.
- **`dev`**: Branch de integração para o desenvolvimento contínuo.
- **`feature/*`**: Branches para desenvolvimento de novas funcionalidades, que serão mescladas no `dev` via Pull Request (PR).

## 2. Processo de Sincronização Inicial

O objetivo era enviar a estrutura de pastas e arquivos de documentação para o repositório recém-criado no GitHub.

### Passo 1: Conexão e Primeiro Push (Falha 1)

- **Ação:** Conectamos o repositório local ao remoto com `git remote add origin ...`.
- **Ação:** Criamos o branch `dev` localmente e fizemos o primeiro commit assinado.
- **Ação:** Tentamos o push com `git push -u origin dev`.
- **Resultado:** Rejeitado. O motivo foi que o repositório remoto continha trabalho que não existia localmente (um arquivo `.gitignore` criado pelo GitHub na inicialização).

### Passo 2: Resolução de Conflito (Falha 2)

- **Ação:** Tentamos integrar as mudanças remotas com `git pull origin dev --rebase`.
- **Resultado:** Um conflito de merge ocorreu no arquivo `.gitignore`.
- **Ação:** O conflito foi resolvido manualmente, unificando os dois arquivos `.gitignore` em um só. O processo de rebase foi continuado com `git add .gitignore` e `git rebase --continue`.
- **Ação:** Tentamos o push novamente.
- **Resultado:** Rejeitado. Desta vez, por um erro de "Large files detected". O arquivo do executável do Godot (`Godot_v4.5.1-stable_linux.x86_64`) excedia o limite de 100MB do GitHub.

### Passo 3: Remoção de Arquivo Grande (Falha 3)

- **Lição:** Executáveis e binários de ferramentas não devem ser versionados no repositório.
- **Ação:** O arquivo grande foi removido do stage do Git com `git rm --cached <arquivo>`.
- **Ação:** O nome do executável foi adicionado ao `.gitignore` para prevenir futuros uploads.
- **Ação:** O commit anterior foi corrigido com `git commit --amend` para registrar a remoção do arquivo.
- **Ação:** Tentamos o push novamente.
- **Resultado:** Rejeitado. Desta vez, pelo hook de proteção de branch (`protected branch hook declined`), pois as regras do repositório exigem que as alterações no `dev` sejam feitas via Pull Request e que os commits tenham assinaturas verificadas (que podem ter se perdido no processo de rebase/amend).

### Passo 4: A Solução via Pull Request (Sucesso)

- **Lição:** As proteções de branch estavam funcionando como esperado.
- **Ação:** Para seguir as regras, um novo branch foi criado a partir do estado atual: `git checkout -b feature/initial-setup`.
- **Ação:** O novo branch foi enviado ao GitHub com `git push -u origin feature/initial-setup`.
- **Resultado:** **Sucesso!** O Git forneceu o link para criar um Pull Request no GitHub.

## 3. Próximos Passos Definidos

1.  Mover o executável do Godot para uma pasta fora do projeto (ex: `/usr/local/bin`).
2.  Acessar a URL do GitHub para abrir o PR de `feature/initial-setup` para `dev`.
3.  Fazer o merge do PR no GitHub.
4.  Localmente, voltar ao branch `dev` (`git checkout dev`) e sincronizar com o remoto (`git pull origin dev`).

Após estes passos, o repositório estará finalmente sincronizado e pronto para o desenvolvimento.
