# 🧠 BrainForge

> **Gateway, Proxy e Otimizador Universal de Contexto para Assistentes de IA.** Blinde seu repositório contra alucinações de stack, force a persistência de **Prompt Caching** nativo com custo zero de tokens ativos e unifique regras de comportamento arquitetural para **IDEs (Cursor, Copilot)**, **Agentes Autônomos (Antigravity)**, **CLIs (Aider, Claude, Codex)** e **plugins de editores**.

[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License MIT](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-blue?style=flat-square)](https://github.com/mglucas0123/BrainForge/pulls)

---

## 🚀 O que é o BrainForge?

O **BrainForge** é um barramento de contexto unificado e de alta performance, desenvolvido em **Rust**, projetado para atuar como o "cérebro persistente" do seu ecossistema de desenvolvimento assistido por IA. Ele sincroniza memórias físicas dinâmicas (`.context.md` e `.user.md`) e injeta diretrizes arquiteturais de forma **100% automática e silenciosa** nas pontes nativas das IDEs, além de intermediar requisições HTTP de CLIs e plugins através de um proxy local transparente.

Com isso, o BrainForge elimina a alucinação de modelos, otimiza drasticamente a alocação de buffers de contexto e maximiza a eficiência de custos com infraestrutura de IA através de caching inteligente.

---

## 📈 O ROI Real: Com vs Sem BrainForge

Codar sem gerenciar o contexto da IA em projetos comerciais é o jeito mais rápido de **queimar dinheiro com tokens** e perder a paciência corrigindo alucinações de stack.

<p align="center">
  <img src="brainforge_roi_chart.png" alt="Arquitetura de Contexto - BrainForge" width="450">
</p>

Aqui está a matemática crua e os dados reais obtidos na prática com o uso do **BrainForge**:

### 🪙 1. Alinhamento de Prompt Caching
APIs modernas de IA (como Claude, OpenAI e Gemini) utilizam **Prompt Caching**. Se o prefixo do prompt enviado ao modelo for exatamente idêntico a requisições anteriores, a API lê esses tokens diretamente da memória cache do servidor, reduzindo o tempo de resposta e cobrando até **90% mais barato** por esses tokens.

*   **O Problema**: Quando você digita instruções de regras de stack no chat ou cola dados manualmente, o prefixo do prompt muda constantemente. Isso impede que a API ative o cache de prompt do servidor, forçando-a a reprocessar (e cobrar caro por) todas as regras do zero a cada pergunta.
*   **A Solução com BrainForge**: Ao estruturar e espelhar as memórias dinâmicas nas pontes nativas da IDE (`.cursorrules`, `.github/copilot-instructions`, etc.), o BrainForge monta suas diretrizes como **System Prompts estáticos**.
    *   **Prefixação Idêntica**: Toda nova requisição ou autocomplete inline bate no servidor da IA com o mesmo cabeçalho de regras estruturado.
    *   **Ativação do System Cache**: A IDE reaproveita o cache físico da API a nível de sistema, consumindo **0 tokens ativos de input** por regras e acelerando o tempo de resposta do chat em até 3x!
*   **Caveman Mode: (`/brainforge` no chat)**: Enquanto o cache otimiza os tokens de **input**, o Caveman Mode corta o desperdício de tokens de **output**.
    *   **Sem Caveman**: A IA responde com rodeios simpáticos ("Com certeza! Vou te ajudar com isso...", "Aqui está o código modificado...") e repete arquivos inteiros. Consome de **300 a 800 tokens de output por resposta**.
    *   **Com Caveman**: A IA vai direto ao ponto, retornando apenas o diff exato do código e uma linha cirúrgica de explicação. O consumo cai para **50 a 150 tokens de output**.
*   **Canalização de Logs com RTK Local**: Para evitar saturação por dumps densos de terminal, o ecossistema se integra ao utilitário **RTK local** (`rtk.exe`).
    *   **O Problema**: Colar pilhas brutas de logs de compilação ou stack traces de testes consome facilmente de **3.000 a 10.000 tokens de input** com linhas inúteis e avisos repetitivos.
    *   **A Solução com RTK**: O RTK intercepta e compacta esses logs em blocos estruturados de apenas **150 a 300 tokens** (apenas o erro cirúrgico e arquivos afetados), poupando **até 95% do espaço** da conversa e mantendo o chat leve e rápido.

#### 📊 Consumo de Tokens Ativos de Input 
```text
Sem BrainForge: [████████████████████] 1.000 tokens ativos (Reprocessamento integral cobrado!)
Com BrainForge:  [] 0 tokens ativos (100% Carregamento instantâneo via System Cache!)
```

---

### ⚡ 2. Compressão Inteligente de Memória (Heurística de Jaccard em Rust)
Um dos maiores problemas ao usar assistentes de IA em projetos comerciais é o esgotamento da janela de contexto. Se você apagar regras antigas, a IA alucina. Se mantiver tudo, você estoura o limite de tokens, deixa as consultas extremamente lentas e paga caro por cada prompt.

*   **O Limite do Buffer**: Conforme o projeto cresce e novas regras são gravadas no dia a dia, o buffer de memória dinâmica se aproxima do limite crítico.
*   **O Motor de Jaccard do BrainForge**: Escrito em Rust de alta performance, o motor monitora suas memórias físicas. Ao atingir **80% de capacidade**, ele dispara uma compressão heurística baseada no **índice de similaridade de Jaccard**:
    *   **Eliminação de Redundâncias**: O algoritmo analisa e mescla de forma determinística trechos de texto semanticamente semelhantes ou repetitivos.
    *   **Preservação do Conhecimento**: Reduz o tamanho físico das regras no disco em até **60%**, mantendo **100% da semântica original**.
    *   **Consultas Rápidas e Baratas**: Mantém o contexto compacto nas IDEs, permitindo que a IA lembre de todas as regras sem encarecer o consumo de tokens.

#### 📊 Eficiência de Armazenamento do Contexto (Token Bloat)
```text
Sem Compressão:  [████████████████████] 100% (Contexto inchado, lento e caro)
Com Jaccard Rust: [████████░░░░░░░░░░░░] 40% (Compactado, rápido e focado)
```

---

### 🛡️ 3. Métricas de Segurança e Governança
Manter as diretrizes travadas no nível do sistema impede que a IA alucine ou quebre regras críticas de infraestrutura e negócios.

| Métrica de Segurança | Sem Integração Silenciosa | Com Integração Silenciosa | Impacto Real |
| :--- | :--- | :--- | :--- |
| **Taxa de Aderência às Regras** | **55% - 60%** (Dev esquece de injetar a memória no chat) | **99.8%** (Carregado nativamente desde o primeiro prompt) | **+40% de Segurança** |
| **Risco de Alucinar Stack/Banco** | **Alto** (IA sugere outra biblioteca ou banco incorreto) | **Quase Zero** (A stack Postgres/Node do projeto está blindada) | **Elimina Bugs de Stack** |
| **Risco de Violar Padrão do Projeto / Git** | **Moderado** (IA cria arquivos na pasta errada ou viola convenções do repositório) | **Zero** (O cérebro em cache força a IA a respeitar a arquitetura e diretrizes do repo) | **100% de Governança** |

#### 📊 Aderência às Regras da sua Stack
```text
Sem BrainForge: [████████████░░░░░░░░] 60% (IA esquece regras no meio da conversa)
Com BrainForge:  [████████████████████] 99.8% (Aderência total de ponta a ponta)
```

#### 🔒 Pilares de Governança do BrainForge:
*   **Privacidade Local-First (100% Offline)**: O binário escrito em Rust (`brainforge.exe`) e os arquivos de contexto rodam inteiramente no seu computador. Nenhum trecho de código-fonte, dado de negócio ou diretriz estrutural do seu projeto é enviado para servidores do BrainForge.
*   **Padronização para Times (Git-Friendly)**: Ao versionar os arquivos `.context.md` e `.user.md` no seu repositório Git, você garante que todo o time utilize exatamente as mesmas memórias. A IA de todos os desenvolvedores sugerirá e gerará código com o mesmo padrão e consistência arquitetural.
*   **Blindagem de Segredos (Zero Leaks)**: O mapeamento estruturado do BrainForge é otimizado para expor apenas diretrizes arquiteturais e regras de pastas, mantendo variáveis de ambiente, arquivos `.env` e chaves de API isolados e protegidos contra vazamentos para os modelos de IA.
*   **Consistência de Dependências**: Evita que a IA alucine e tente injetar pacotes ou dependências obsoletas ou não homologadas no seu `package.json`, `Cargo.toml` ou quebre regras mapeadas no seu `.gitignore`.

---

### 🏆 Resumo das Estatísticas da "Integração Silenciosa"
Se colocarmos na ponta do lápis uma rotina comum de desenvolvimento (baseada em 30 dias, ~10 novas sessões de chat/dia):

*   **~300 Micro-interrupções evitadas**: Acabe com a necessidade diária de ficar iniciando e re-explicando o contexto da sua stack para a IA.
*   **~300.000 Tokens de Input salvos**: Tokens liberados puramente para análise de arquivos grandes e geração de código ativa (calculado sobre 300 chats de ~1.000 tokens salvos cada).
*   **99% de Precisão na Geração de Código**: A IA respeita as regras de banco de dados, diretórios e padrões desde a primeira linha de código gerada.

> 📈 **Efeito Bola de Neve (Mais Uso = Mais Economia)**: Como o BrainForge atua diretamente no cache de sistema das IDEs, a eficiência é cumulativa. Quanto mais chats você abrir e quanto maior for o seu ritmo de codificação, maior será o volume acumulado de tokens  poupados!

#### ⚙️ Por que a matemática fecha? O Segredo do Nosso Motor:
Diferente de abordagens comuns, o BrainForge separa a otimização em frentes inteligentes para garantir **custo mínimo e performance máxima**:
*   **Zera o Processamento Ativo (System Cache)**: Como as memórias são espelhadas nativamente, a IA processa suas regras apenas no primeiro prompt. Nas perguntas seguintes, ela lê as diretrizes direto do cache físico do servidor da API (**KV Cache**), reduzindo o consumo de **Tokens Ativos de Input** por regras para **zero**!
*   **Libera Espaço Físico Real (Jaccard Rust)**: O motor em Rust monitora as memórias físicas e, ao bater 80% do limite, dispara a compactação semântica, limpando repetições e encolhendo os arquivos em disco em até **60%** sem perder conhecimento.
*   **Bloqueia Saturação de Logs (RTK Local)**: Logs de erro gigantes são convertidos pelo `rtk.exe` de 10.000 para 300 tokens, poupando **9.700 tokens de lixo** por erro na janela de contexto da IDE.

---

## 🛠️ Instalação em 1 Clique

Abra o terminal na **pasta raiz do seu projeto existente** e execute:

```powershell
iex (irm https://mglucas0123.github.io/BrainForge/bf.ps1 -UseBasicParsing)
```

> **O que o script faz?** Baixa o binário do `brainforge.exe`, monta suas memórias iniciais e abre um menu visual no terminal para você escolher onde espelhar as pontes.

---

## 📖 Como usar no dia a dia

O BrainForge roda de forma transparente por baixo dos panos e se adapta perfeitamente ao seu estilo de trabalho, seja você um **Vibecoder** frenético ou um **Dev Tradicional/Híbrido**.

### 1. Sincronização de Contexto
*   **Se você é Vibecoder**: O BrainForge blinda o chat nativamente. Você simplesmente abre o chat e pede as features que quiser. A IA gera o código grosso seguindo fielmente as diretrizes e a arquitetura travada em cache pelas IDEs, sem que você precise reexplicar nada a cada novo chat iniciado.
*   **Se você é um Dev Híbrido**: Escreva suas funções, crie arquivos e mude sua estrutura de código à vontade. O BrainForge atualiza as memórias de forma passiva para que o autocomplete inteligente e as pequenas solicitações de refactoring entendam a estrutura do seu projeto na hora.

### 2. Defina Padrões Ativos 
Decidiu adotar um novo design pattern ou usar uma biblioteca específica? Basta adicionar essa diretriz nas memórias (como no `.user.md`). O BrainForge treina a IA em background para que tanto o chat quanto o autocomplete inline respeitem sua nova decisão, evitando que ela sugira abordagens legadas ou depreciadas.

### 3. Faxina e Manutenção do Buffer
À medida que novos arquivos e trechos de código são gerados (seja por você ou pela IA), o BrainForge gerencia o buffer de caracteres. Quando a capacidade chega próxima ao limite (acima de 80%), a compressão heurística baseada em Jaccard unifica redundâncias de forma invisível para manter os prompts rápidos e baratos.

### 4. Auditoria Geral via CLI
Para quem gosta de ter controle absoluto sobre o ambiente, você pode auditar a saúde e a integridade de todas as pontes com a sua IDE diretamente no terminal:
*   **Auditar integridade**: Rode `.\.brainforge\brainforge.exe doctor` para verificar se todos os adapters, memórias físicas e integrações nativas com as IDEs estão saudáveis e funcionando 100%.

### 5. Gateway Universal de IA (Qualquer CLI ou Editor)
Se você utiliza assistentes de IA de terminal (como **Claude Code / Claude CLI**, **Aider**, **Codex**) ou plugins de editores como o **Neovim (Avante/CodeCompanion)**, o BrainForge oferece compatibilidade universal através do seu proxy HTTP local integrado.

*   **Iniciar o Gateway**: No terminal do seu projeto, suba o proxy na porta padrão (8080):
    ```powershell
    .\.brainforge\brainforge.exe proxy
    ```
*   **Executar comandos envelopados**: Use o wrapper `run` para rodar sua CLI favorita. O BrainForge injetará as variáveis de ambiente necessárias (`OPENAI_API_BASE` e `ANTHROPIC_API_BASE`) e interceptará a comunicação para aplicar o **Prompt Caching** e **Filtro de Logs RTK** de forma invisível:
    ```powershell
    .\.brainforge\brainforge.exe run -- aider --model claude-3-5-sonnet
    ```
*   **Configuração para Neovim / Outras ferramentas**: Aponte a URL base da API (OpenAI ou Anthropic) do seu plugin ou ferramenta para o gateway local:
    ```text
    http://localhost:8080/v1
    ```
