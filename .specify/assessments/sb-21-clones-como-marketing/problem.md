# Problem — Clones como marketing (sb-21)

**Data**: 2026-11-21

## Problema concreto no projeto
Insurgência é um protocolo FOSS cuja tese é **"fork é feature"** (GDD §1, §3.3, §4.1; constituição IV). Forks e redistribuições são esperados e permitidos — inclusive sem autorização. Ainda assim, quando um projeto ganha alcance, clones desonestos (com ads, ou que se passam pelo original) podem aparecer; o caso Slotbound documenta dois em Douyin/WeChat. Hoje o projeto **não tem registro cronológico de autoria** que permita responder com evidência, e não tem política que impeça a reação de se tornar uma campanha ofensiva — o que contradiria a própria tese de soberania. O risco não é o clone em si: é reagir sem recibos ou fabricar o incidente.

## Sucesso observável
1. Cada release versionada carrega commits, hashes, datas e capturas íntegras que identificam autoria e anterioridade.
2. Se — e somente se — ocorrer um incidente real, a resposta pública cita **evidência datada existente**, não especulação.
3. Zero clones provocados, fabricados ou simulados pelo projeto; nenhuma campanha de "fake clone".
4. Nenhum serviço externo no caminho crítico; registro vive em commits/releases locais e espelhos já usados.
5. Qualquer alegação sobre alcance em ZH permanece **[NV]** até verificação ao vivo; CI/merge não são chamados de verdes sem comando executado (VI).

## Custo de não agir
Sem registro, a reação ao incidente vira assertiva — "somos o original" sem prova — e abre espaço para a leitura de que o projeto apelou a marketing fabricado, que é infalsificável e eticamente reprovável. Também se perde a distinção entre *alcance* (sintoma externo) e *soberania* (estratégia), desviando esforço do gate determinístico que já é a resposta local (achado 6 de `fit-local`).
