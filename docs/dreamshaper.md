# Submissão do Projeto de Extensão - ZelaJampa

Documentação consolidada de todas as respostas, justificativas e dados inseridos na plataforma de submissão da Atividade de Extensão Universitária (Cruzeiro do Sul Virtual).

---

## 1. Identificação Geral
- **Projeto:** ZelaJampa (Mapeamento & Agregação Espacial de Demandas Urbanas)
- **ODS Vinculado:** 9) Indústria, Inovação e Infraestrutura
- **Recorte Territorial:** João Pessoa e Cabedelo - PB (ênfase no corredor da BR-230 e bairro de Intermares)
- **Tecnologias Envolvidas:** Rust (Axum), PostgreSQL + PostGIS (ST_GeoHash), HTMX, Leaflet, Askama

---

## 2. Comunidade

### Público-alvo *(limite 800)*
> Cidadãos e moradores de João Pessoa, em especial quem circula bastante pelas vias públicas no cotidiano, como motoristas, pedestres, ciclistas e lideranças comunitárias de bairros que enfrentam falhas frequentes na infraestrutura urbana.

### Características demográficas *(limite 800)*
> Jovens e adultos entre 18 e 60 anos, residentes de bairros periféricos e centrais de João Pessoa, que utilizam smartphone com acesso à internet, mas evitam ou encontram barreiras em canais burocráticos tradicionais de atendimento público.

### Hábitos observados em relação à temática *(limite 800)*
> Observei que as pessoas costumam relatar os problemas em grupos de WhatsApp do bairro, em comentários nas redes sociais ou ligando para programas de rádio. Como esses relatos ficam dispersos, os moradores não conseguem saber se um vizinho já apontou o mesmo problema, gerando retrabalho e falta de visibilidade sobre quais pontos da rua realmente precisam de intervenção urgente.

### 3 Palavras-chave Representativas
1. **Mobilidade**
2. **Cidadania**
3. **Zeladoria**

### 3 Imagens Representativas
1. Rua com asfalto degradado / crateras no leito viário.
2. Poste com lâmpada queimada / trecho urbano em escuridão à noite.
3. Pedestres disputando espaço com veículos em vias sem calçada acessível.

---

## 3. Problemas e Desafios

### Problemas identificados *(limite 200 cada)*
1. **Asfalto e calçadas:**
   > Buracos no asfalto e calçadas deterioradas, gerando riscos de acidentes, danos a veículos e travando a locomoção de pedestres no dia a dia. *(138 caracteres)*
2. **Iluminação e segurança:**
   > Lâmpadas queimadas e vias escuras à noite, aumentando a sensação de insegurança dos moradores que precisam circular a pé pelo bairro. *(131 caracteres)*
3. **Comunicação com a prefeitura:**
   > Falta de um canal simples e direto para acompanhar se a demanda aberta foi repassada para o órgão responsável e quando será consertada. *(134 caracteres)*
4. **Dispersão de queixas:**
   > Relatos espalhados em redes sociais e WhatsApp, sem um mapa unificado que mostre onde estão os pontos mais críticos da cidade. *(126 caracteres)*

### Problema de partida escolhido *(limite 450)*
> A ausência de um canal ágil, transparente e unificado para mapear demandas de infraestrutura urbana (como buracos, postes apagados e saneamento). Como as queixas ficam espalhadas em redes sociais ou travadas na burocracia, os moradores não conseguem acompanhar soluções e o poder público não tem clareza visual dos pontos mais críticos para priorizar o atendimento. *(373 caracteres)*

### Importância das atividades de extensão universitária *(limite 800)*
> A extensão universitária é fundamental porque tira o conhecimento técnico de dentro do campus e o coloca a serviço de dores reais da população. Em vez de desenvolver uma aplicação puramente teórica, aplico engenharia de software e análise espacial para resolver um gargalo concreto de zeladoria urbana em João Pessoa. Essa integração aproxima a universidade dos bairros, devolvendo à comunidade uma ferramenta acessível e gratuita que organiza as demandas coletivas, fortalece a cidadania e ajuda a cobrar melhorias com base em dados geográficos reais. *(542 caracteres)*

### Causas, consequências e pessoas afetadas *(limite 600)*
> As principais causas são a burocracia dos canais oficiais e a falta de uma ferramenta aberta que centralize queixas em tempo real com mapa. Como consequência, vias acumulam buracos gerando acidentes e prejuízos, ruas escuras aumentam a sensação de insegurança à noite e os moradores desanimam de registrar demandas. As pessoas mais afetadas são trabalhadores, pedestres, motoristas e estudantes que circulam diariamente pelo bairro e sofrem com a precariedade das vias. *(489 caracteres)*

---

## 4. Iniciativas e Boas / Más Práticas

### Iniciativas Existentes Mapeadas
1. **João Pessoa na Palma da Mão (App Oficial da Prefeitura)** *(limite 80)*
   - *Descrição (limite 300):* Canal oficial do município para solicitar reparos e serviços urbanos. Porém, funciona de forma burocrática por protocolo individual, sem um mapa público onde a vizinhança possa ver se um problema na rua já foi registrado por outra pessoa. *(249 caracteres)*
2. **Colab (Plataforma de Participação Cívica)** *(limite 80)*
   - *Descrição (limite 300):* Aplicativo brasileiro de zeladoria onde o cidadão fotografa e relata problemas da cidade. A limitação é que depende de convênio e contratação formal com as prefeituras para dar retorno, além de exigir cadastro prévio e download de app pesado. *(251 caracteres)*
3. **FixMyStreet (Plataforma Aberta de Zeladoria)** *(limite 80)*
   - *Descrição (limite 300):* Iniciativa global de código aberto para mapear queixas públicas em mapa interativo. Serve de referência técnica direta, mas não possui foco específico na realidade e nas secretarias municipais de João Pessoa. *(214 caracteres)*

### Boas práticas a integrar *(limite 800)*
> O aspecto mais positivo dessas iniciativas é o uso de mapas interativos para localizar as demandas no espaço geográfico real, o que é muito mais visual e intuitivo do que uma planilha ou um protocolo numérico frio. Também me inspiro na facilidade de categorizar os problemas (buraco, iluminação, saneamento) e na captura ágil da localização pelo GPS do celular. Quero trazer para a nossa ação extensionista essa transparência coletiva: permitir que o cidadão registre a queixa em segundos e veja no mapa que sua voz faz parte de um levantamento compartilhado com toda a vizinhança. *(575 caracteres)*

### Más práticas a evitar *(limite 800)*
> O que não funcionou bem e pretendo evitar a todo custo é a burocracia excessiva: forçar o morador a baixar um aplicativo pesado nas lojas virtuais, criar logins demorados e preencher dados pessoais extensos antes de conseguir relatar um poste quebrado. Isso desestimula a participação popular. Outra falha comum é a falta de tratamento de duplicidade; quando dez vizinhos relatam o mesmo buraco, os sistemas tradicionais criam dez chamados isolados e confusos. No nosso projeto, evitamos isso agrupando automaticamente queixas próximas por geolocalização para indicar a real gravidade do ponto. *(598 caracteres)*

---

## 5. Investigação e Pesquisa

### Questões que faltam responder *(limite 800)*
> Ainda preciso validar algumas questões práticas diretamente com os moradores:
>
> 1. Canais de acesso: qual é a forma mais natural para a comunidade acessar a ferramenta no dia a dia (links compartilhados no WhatsApp do bairro, QR Codes em pontos estratégicos ou redes sociais)?
> 2. Engajamento contínuo: o que motiva a comunidade a continuar registrando e acompanhando as queixas até a resolução definitiva?
> 3. Moderação e confiança: como garantir que o sistema continue simples e sem login burocrático, mas protegido contra denúncias falsas ou trotes?
> 4. Articulação pública: de que forma os dados consolidados no mapa podem ser entregues às secretarias municipais de João Pessoa para acelerar as ordens de serviço? *(749 caracteres)*

### Fontes de Informação *(limite 200 cada)*
1. **IBGE Cidades:** Dados estatísticos oficiais sobre pavimentação, iluminação pública e saneamento básico nos bairros de João Pessoa. *(129 caracteres)*
2. **Google Acadêmico:** Artigos científicos sobre tecnologia cívica, mapas colaborativos e participação popular na melhoria de cidades. *(129 caracteres)*
3. **Prefeitura de João Pessoa (SEINFRA):** Canais oficiais para checar serviços de zeladoria, programas de asfalto e secretarias responsáveis. *(131 caracteres)*
4. **Noticiários Locais (G1 PB):** Histórico de matérias sobre queixas reais de moradores sobre buracos e escuridão nas vias. *(135 caracteres)*

### Documentos / Artigos Anexados
- **Arquivo Acadêmico Principal:** MACHADO, Adriana Alexandria; CAMBOIM, Silvana Philippi. *Mapeamento colaborativo como fonte de dados para o planejamento urbano: desafios e potencialidades*. urbe. Revista Brasileira de Gestão Urbana, v. 11, e20180142, 2019. [SciELO PDF](https://www.scielo.br/j/urbe/a/d6YmTQCPHhf5T5hsGVvrqHD/?lang=pt&format=pdf) / [DOI: 10.1590/2175-3369.011.e20180142](https://doi.org/10.1590/2175-3369.011.e20180142).
- **Dados Oficiais:** IBGE Cidades - Panorama de João Pessoa/PB.

### Autoavaliação da Pesquisa
- Marcadas todas as 5 frases de conformidade (fontes confiáveis, cruzamento de fontes, dados atualizados, base factual e uso de fontes originais).

---

## 6. Descobertas e Soluções

### Barreiras identificadas *(limite 800)*
> Identifiquei quatro barreiras principais que dificultam a solução do problema:
>
> 1. Burocracia governamental: os canais oficiais funcionam por protocolos individuais e fechados, sem transparência sobre prazos ou ordem de atendimento.
> 2. Informação dispersa: relatos ficam espalhados em redes sociais e grupos de mensagens, sem um mapa unificado que mostre onde estão as vias mais críticas.
> 3. Barreiras de acesso técnico: muitos aplicativos públicos são pesados e exigem cadastros longos, o que afasta moradores com celulares mais simples ou pouco tempo.
> 4. Desmotivação popular: a falta de retorno visível gera descrédito, fazendo o morador desanimar de fiscalizar e normalizar a convivência com buracos e escuridão na rua. *(736 caracteres)*

### Recursos existentes na comunidade *(limite 800)*
> A comunidade possui recursos valiosos que facilitam a solução:
>
> 1. Smartphones com GPS e internet: a maioria dos moradores e motoristas já possui celular conectado, permitindo marcar problemas na rua pelo navegador sem baixar aplicativos pesados.
> 2. Redes de comunicação locais: grupos de WhatsApp de vizinhança e perfis comunitários já ativos, perfeitos para divulgar e compartilhar o mapa da região.
> 3. Associações e lideranças de moradores: representantes que têm interesse direto em usar dados organizados para fundamentar cobranças junto à prefeitura.
> 4. Cultura de cooperação: motoristas e pedestres já têm o hábito de alertar uns aos outros sobre buracos e perigos na pista, bastando canalizar isso para uma ferramenta centralizada. *(744 caracteres)*

### Alternativas de solução *(limite 250 cada)*
1. **Plataforma Web com Mapa (ZelaJampa):**
   > Plataforma web aberta e leve onde o morador aponta queixas em um mapa interativo com GPS, com agrupamento automático de relatos na mesma rua para evidenciar os gargalos mais urgentes. *(180 caracteres)*
2. **Relatórios para Associações e Secretarias:**
   > Painel público que consolida as queixas em mapas de calor, permitindo a associações de moradores e cidadãos protocolarem dossiês organizados na SEINFRA e órgãos municipais. *(175 caracteres)*
3. **QR Codes em Pontos Estratégicos:**
   > Cartazes com QR Code em paradas de ônibus e comércios de bairro para que pedestres e motoristas acessem o mapa em segundos e registrem os buracos e falhas daquele trecho. *(174 caracteres)*

### Resumo das Conclusões / Achados *(limite 800)*
> A pesquisa mudou minha percepção em pontos fundamentais:
>
> 1. O problema não é falta de interesse: a comunidade quer participar e relata problemas diariamente, mas essas queixas ficam perdidas em redes sociais ou travadas na burocracia de canais oficiais.
> 2. O valor do agrupamento espacial: percebi que a chave não é abrir centenas de protocolos isolados, mas consolidar relatos por proximidade geográfica. Vários chamados no mesmo trecho de rua revelam visualmente onde estão os gargalos mais urgentes da cidade.
> 3. Acessibilidade supera complexidade: aplicativos pesados com cadastro demorado afastam o cidadão. Uma ferramenta web leve, sem login e baseada em mapa aberto é muito mais inclusiva e transforma desabafos soltos em cobrança cidadã fundamentada. *(757 caracteres)*

---

## 7. Documentário Fotográfico e Exibição

### Objetivo do Documentário *(limite 800)*
> O objetivo é documentar visualmente os impactos reais da precariedade viária na rotina da comunidade em João Pessoa. Desejo observar e registrar quatro situações:
>
> 1. Horários de pico: o malabarismo diário de motoristas, motociclistas e ônibus desviando de crateras no asfalto, criando risco iminente de acidentes.
> 2. Rotina dos pedestres: idosos, estudantes e trabalhadores obrigados a disputar espaço com os carros no meio da rua por falta de calçadas transitáveis.
> 3. Período noturno: o contraste e a escuridão provocados por postes apagados, mostrando o isolamento e o medo dos moradores nas paradas de ônibus.
> 4. Dias de chuva: o acúmulo de poças de lama e transbordamento em bueiros entupidos, dificultando o acesso a comércios e residências. *(757 caracteres)*

### Colaboradores *(limite 800)*
> Para retratar o cotidiano sob diferentes perspectivas, reuni pessoas da própria comunidade que vivenciam esses problemas diariamente:
>
> 1. Eu (responsável pelo projeto): coordenarei os roteiros de captura, padronização das imagens e localização geográfica de cada ponto.
> 2. Carlos Eduardo (vizinho e motorista): registrará fotos dos piores trechos de asfalto e desníveis durante seus deslocamentos pelas vias do bairro.
> 3. Mariana Souza (moradora e estudante universitária): colaborará com registros do trajeto a pé até o ponto de ônibus, com foco na iluminação noturna e acessibilidade das calçadas.
> 4. José Ribamar (comerciante local): registrará imagens do impacto dos buracos e da poeira acumulada na porta de pequenos negócios da rua. *(749 caracteres)*

### Locais Definidos para Visita *(limite 90 cada)*
1. `Marginais da BR-230 no trecho de ligação entre João Pessoa e Cabedelo` *(69 caracteres)*
2. `Av. Mar Vermelho em Intermares: via principal com buracos e calçadas irregulares` *(79 caracteres)*
3. `Orla de Intermares (Av. Oceano Atlântico): trechos com falhas na iluminação pública` *(83 caracteres)*
4. `Paradas de ônibus e passarelas ao longo da BR-230 com pouca luz à noite` *(70 caracteres)*
5. `Ruas residenciais internas de Intermares com pavimento danificado e poças` *(74 caracteres)*

### Observações e Descobertas de Campo *(limite 500 cada)*
1. **Marginais da BR-230 e acessos (Iluminação, Sinalização e Estreitamento):**
   > Nas marginais e alças de acesso da BR-230 rumo a Intermares, a iluminação pública é extremamente precária à noite. A situação piora pela falta de sinalização e ausência de placas alertando sobre o estreitamento repentino da pista. Motoristas e motociclistas só percebem o afunilamento da via em cima da hora, no escuro, forçando freadas e desvios bruscos que causam acidentes e colocam pedestres das marginais em perigo constante. *(432 caracteres)*
2. **Intermares (Av. Mar Vermelho e vias internas):**
   > Ao percorrer a Av. Mar Vermelho e vias internas de Intermares, constatei poças acumuladas de água servida e calçamento afundado. Comerciantes relataram que clientes evitam parar em frente às lojas por causa da lama e dos desníveis. Os pedestres e idosos são forçados a caminhar pela rua junto aos carros, já que as calçadas são irregulares ou inexistentes. *(365 caracteres)*
3. **Percepção e reação dos moradores:**
   > Em conversa com moradores e motoristas de aplicativo que circulam pela região, ficou claro o desânimo com os canais tradicionais. Quase todos já conhecem os buracos "de cor", mas não abrem chamados oficiais por acharem o processo lento e inútil. Todos concordaram que ver as queixas agrupadas em um mapa público daria muito mais peso para cobrar reparos. *(363 caracteres)*

### Formato de Exibição do Trabalho *(limite 500)*
> A exibição será digital e interativa através de um site próprio (a plataforma ZelaJampa) combinado com redes sociais locais. Na página web, as fotografias e relatos de Intermares e da BR-230 ficarão vinculados a um mapa interativo aberto, permitindo que a população visualize os pontos críticos no espaço geográfico. O link e um resumo das imagens serão compartilhados nos grupos de WhatsApp e perfis de moradores do bairro para engajamento e validação coletiva. *(466 caracteres)*
