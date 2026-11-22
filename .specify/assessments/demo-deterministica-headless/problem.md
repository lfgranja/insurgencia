# Problem — Demo determinística headless

**Problema**: o repositório não tem uma forma executável de demonstrar a simulação determinística sem abrir a janela do cliente; toda verificação visual depende da janela `--minimizado` e de captura pela SE.

**Sucesso observável**: exemplo executável com saída idêntica entre duas execuções, sem Bevy/janela/rede, coberto por `verify.sh --all-targets`.

**Custo de não agir**: nenhum "recibo" reproduzível para `sb-15`/`sb-20`, e a lição mais verificável do vídeo (demo) continua sem materialização no alvo.
