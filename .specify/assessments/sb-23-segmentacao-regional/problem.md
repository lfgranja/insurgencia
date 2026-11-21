# Problem — Segmentação regional (sb-23)

**Data**: 2026-11-21

## Problema concreto no projeto
O interesse por segmentação regional nasce de números que o projeto **não consegue verificar**: os percentuais 68/93/79/77 vêm só do vídeo, e a API de reviews por idioma redirecionou **[NV]**. Enquanto isso, o projeto tem uma dívida concreta e local: texto em `docs/` está só em pt-BR e a tendência é colocar rótulos em imagens/capturas, o que torna futura localização caríssima. Decidir "para onde ir" com base em percentuais não reproduzidos inverte a ordem: primeiro é preciso tornar o produto localizável; a campanha é pré-requisito de uma superfície de publicação que hoje não existe.

## Sucesso observável
1. Texto de interface/documentação vive em **dados** (strings externas), não embutido em imagens — pronto para i18n sem reeditar assets.
2. Nenhuma decisão de campanha regional é justificada pelos percentuais do vídeo; eles permanecem rotulados **[NV]**.
3. Nenhuma loja/campanha regional é lançada antes de existir superfície de distribuição e capacidade de tradução — essa parte fica **DEFER**.
4. O 79% global, quando citado, vem com data e fonte (dinâmico: 921 em 17/11; 1.598 em 21/11).
5. Nenhum serviço externo entra no caminho crítico; CI/merge permanecem **[NV]** até verificação ao vivo.

## Custo de não agir
Texto embutido em imagens cria retrabalho: cada novo idioma exige reeditar e revalidar capturas, em vez de trocar uma tabela de strings. Por outro lado, agir sobre os percentuais não verificados significa planejar para um mercado inferido de caso único — o mesmo erro de sobrevivência de sb-25 — e queimar esforço em campanha sem canal onde publicá-la.
