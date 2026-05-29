/* app.js — Lógica de Simulação Interativa do BrainForge */

// --- Estado Inicial da Simulação ---
const STATE = {
    activeTab: 'context', // 'context' | 'user'
    cavemanLevel: 'full', // 'off' | 'lite' | 'full' | 'ultra'
    language: 'pt-BR',
    isBrainForgeOn: false,
    
    // Arquivo .context.md
    contextMemory: {
        title: '# Project Memory',
        charLimit: 2200,
        entries: [
            'Stack: Rust + Cargo (workspace)',
            'Estrutura: crates brainforge-core, brainforge-cli, brainforge-mcp',
            'Diretório de testes "teste" configurado com espelho de agentes'
        ],
        reference: `## Reference\n- Link: [README.md](file:///d:/Projetos/BrainForge/README.md)\n- CLI: [main.rs](file:///d:/Projetos/BrainForge/crates/brainforge-cli/src/main.rs)`
    },
    
    // Arquivo .user.md (Inspirado no real do projeto)
    userMemory: {
        title: '# User Profile',
        charLimit: 1375,
        entries: [
            'Replies **pt-BR** always; caveman **full** via `/brainforge`; off: modo normal, stop caveman/brainforge',
            'Agent runs commands/edits; commits only when asked; no force push main/master; minimal code scope',
            'PowerShell; RTK for large terminal output; session opt-out compress: `sem auto-compress` / `skip compress`',
            'Concise user text; code/diffs/commits readable; max one blocking question',
            'After structural work: offer `.cursor/project/.context.md` update; user prefs here',
            'No verbose greetings/closings; no activate all specialized skills on `/brainforge` alone',
            'Memory: prefs/workflow only; no ephemera/dumps; no copy rules/skills; save/skip = cavemem'
        ],
        reference: null
    },
    
    // Histórico de Conversa
    chatHistory: [
        {
            role: 'agent',
            thought: 'Iniciando sistema de simulação do BrainForge...',
            text: 'Olá! Este é o seu painel de simulação do **BrainForge**. Aqui você pode interagir comigo e testar como o ecossistema se comporta diante de diferentes inputs, comandos e estados de memória. Experimente digitar comandos como `/brainforge` ou clique em um dos cenários de teste à esquerda!'
        }
    ],
    
    // Logs do Terminal
    terminalLogs: [
        { text: 'BrainForge CLI v1.0.4 initialized.', type: 'info' },
        { text: 'Target project resolved: d:/Projetos/BrainForge/teste', type: 'bold' },
        { text: 'Run "brainforge doctor" or "/brainforge" to start simulation.', type: 'info' }
    ]
};

// --- Elementos do DOM ---
const DOM = {
    chatHistory: document.getElementById('chat-history'),
    chatInput: document.getElementById('chat-input'),
    btnSend: document.getElementById('btn-send'),
    tabContext: document.getElementById('tab-context'),
    tabUser: document.getElementById('tab-user'),
    capacityText: document.getElementById('capacity-text'),
    capacityBar: document.getElementById('capacity-bar'),
    memoryList: document.getElementById('memory-list'),
    terminalBody: document.getElementById('terminal-body'),
    selectCaveman: document.getElementById('select-caveman'),
    quickChips: document.querySelectorAll('.chip'),
    scenarios: {
        welcome: document.getElementById('scenario-welcome'),
        overflow: document.getElementById('scenario-overflow'),
        doctor: document.getElementById('scenario-doctor'),
        skill: document.getElementById('scenario-skill')
    }
};

// --- Funções Auxiliares de Memória ---

function getEntryChars(entries) {
    // Cada entrada consome o tamanho da string mais 2 caracteres do delimitador §...§
    return entries.reduce((acc, entry) => acc + entry.length + 2, 0);
}

function calculateCapacity(mem) {
    const used = getEntryChars(mem.entries);
    const limit = mem.charLimit;
    return {
        used,
        limit,
        pct: Math.min(100, Math.round((used * 100) / limit))
    };
}

function getCapacityColor(pct) {
    // Interpolação de cores simples baseada no percentual (Verde -> Laranja -> Vermelho)
    if (pct < 50) return '#2ec4b6'; // Verde success
    if (pct < 80) return '#ff9f1c'; // Laranja warning
    return '#ff5e62'; // Vermelho danger
}

// --- Renderização da Interface ---

function renderMemory() {
    const isContext = STATE.activeTab === 'context';
    const mem = isContext ? STATE.contextMemory : STATE.userMemory;
    const { used, limit, pct } = calculateCapacity(mem);
    
    // Atualiza Abas no UI
    DOM.tabContext.classList.toggle('active', isContext);
    DOM.tabUser.classList.toggle('active', !isContext);
    
    // Atualiza Barra de Capacidade e Metadados
    DOM.capacityText.innerHTML = `Capacidade: <strong>${pct}%</strong> · ${used}/${limit} chars`;
    DOM.capacityBar.style.width = `${pct}%`;
    DOM.capacityBar.style.backgroundColor = getCapacityColor(pct);
    
    if (pct >= 80) {
        DOM.capacityBar.parentElement.style.boxShadow = '0 0 10px rgba(255, 94, 98, 0.4)';
    } else {
        DOM.capacityBar.parentElement.style.boxShadow = 'none';
    }
    
    // Renderiza Entradas
    DOM.memoryList.innerHTML = '';
    
    if (mem.entries.length === 0) {
        DOM.memoryList.innerHTML = '<div style="color: var(--text-dim); font-size: 0.8rem; text-align: center; padding: 2rem;">Nenhuma entrada § cadastrada.</div>';
        return;
    }
    
    mem.entries.forEach((entry, idx) => {
        const div = document.createElement('div');
        div.className = 'memory-entry';
        
        // Se a entrada foi modificada recentemente, adiciona destaque
        if (entry.includes('SIMULAÇÃO') || entry.includes('TESTE') || entry.includes('MODO')) {
            div.className += ' modified';
        }
        
        div.innerHTML = `§ ${escapeHtml(entry)} §`;
        DOM.memoryList.appendChild(div);
    });
    
    // Se existir seção de referência, mostra de forma compactada
    if (mem.reference) {
        const refDiv = document.createElement('div');
        refDiv.style.marginTop = '1rem';
        refDiv.style.borderTop = '1px solid var(--border-light)';
        refDiv.style.paddingTop = '0.5rem';
        refDiv.style.fontSize = '0.75rem';
        refDiv.style.color = 'var(--text-muted)';
        refDiv.innerHTML = `<strong>Reference:</strong><pre style="margin-top: 0.25rem; font-family: var(--font-mono); font-size: 0.7rem; opacity: 0.8;">${escapeHtml(mem.reference)}</pre>`;
        DOM.memoryList.appendChild(refDiv);
    }
}

function renderChat() {
    DOM.chatHistory.innerHTML = '';
    STATE.chatHistory.forEach(msg => {
        const wrapper = document.createElement('div');
        wrapper.className = `message message-${msg.role}`;
        
        if (msg.role === 'agent' && msg.thought) {
            const thoughtDiv = document.createElement('div');
            thoughtDiv.className = 'message-thought';
            thoughtDiv.innerText = `💡 TENSÕES DE AGENTE: ${msg.thought}`;
            wrapper.appendChild(thoughtDiv);
        }
        
        const textDiv = document.createElement('div');
        textDiv.innerHTML = formatMarkdown(msg.text);
        wrapper.appendChild(textDiv);
        
        DOM.chatHistory.appendChild(wrapper);
    });
    DOM.chatHistory.scrollTop = DOM.chatHistory.scrollHeight;
}

function addTerminalLog(text, type = 'info') {
    STATE.terminalLogs.push({ text, type });
    renderTerminal();
}

function renderTerminal() {
    DOM.terminalBody.innerHTML = '';
    // Mostra as últimas 25 linhas de log
    const recentLogs = STATE.terminalLogs.slice(-25);
    recentLogs.forEach(log => {
        const div = document.createElement('div');
        div.className = 'term-line';
        
        if (log.type === 'cmd') {
            div.innerHTML = `<span class="term-cmd">❯ </span><span class="term-bold">${escapeHtml(log.text)}</span>`;
        } else if (log.type === 'ok') {
            div.innerHTML = `<span class="term-ok">OK</span> ${escapeHtml(log.text)}`;
        } else if (log.type === 'warn') {
            div.innerHTML = `<span class="term-warn">WARN</span> ${escapeHtml(log.text)}`;
        } else if (log.type === 'err') {
            div.innerHTML = `<span class="term-err">FAIL</span> <span class="term-bold">${escapeHtml(log.text)}</span>`;
        } else if (log.type === 'bold') {
            div.className += ' term-bold';
            div.innerText = log.text;
        } else {
            div.innerText = log.text;
        }
        DOM.terminalBody.appendChild(div);
    });
    DOM.terminalBody.scrollTop = DOM.terminalBody.scrollHeight;
}

// --- Mecanismo do Agente Inteligente (Simulado) ---

function getCavemanReply(concept) {
    // Emula a linguagem simplificada e focada do Caveman mode (ultra/full)
    const replies = {
        welcome: "BrainForge ativado! Modo homem das cavernas FULL. Lendo memórias. Integridade de código prioridade. O que faremos agora?",
        doctor: "Roda verificação doctor. Arquivos checados. Status PASS. Kit saudável! Ajustar depois.",
        compress: "Compressão aplicada! Remover redundância. Unificar duplicatas. Economizar tokens. Espaço liberado!",
        skill: "Skill catalogada instalada com sucesso. Habilidade disponível em .cursor/skills/. Reinicie IDE se necessário.",
        unknown: "Comando não reconhecido. Modo conciso ativado. Diga tarefas. Fazer alterações limpas. Commits apenas quando solicitado.",
        general_prompt: "Entendido. Modificar código no escopo. Rodar testes em seguida. Manter integridade estrutural. Pronto para próximo passo!"
    };
    return replies[concept] || replies.general_prompt;
}

function processUserMessage(msgText) {
    const textClean = msgText.trim().toLowerCase();
    
    // Inicializa resposta simulada do Agente
    let agentThought = "";
    let agentText = "";
    
    // Log do comando digitado no Terminal
    addTerminalLog(msgText, 'cmd');

    if (textClean.startsWith('/brainforge')) {
        if (textClean === '/brainforge-doctor' || textClean.includes('doctor')) {
            // EXECUTA SIMULAÇÃO DO DOCTOR
            agentThought = "Verificando integridade das memórias, RTK local e pontes de adapters.";
            
            addTerminalLog('Running brainforge doctor checks...', 'bold');
            setTimeout(() => {
                addTerminalLog('RTK (d:/Projetos/BrainForge/teste/brainforge/tools/rtk/rtk.exe) ...', 'ok');
                addTerminalLog('.brainforge/memory/.context.md (format OK) ...', 'ok');
                addTerminalLog('.brainforge/memory/.user.md (format OK) ...', 'ok');
                addTerminalLog('antigravity output (.agents/workflows/brainforge.md) ...', 'ok');
                addTerminalLog('cursor output (.cursor/commands/brainforge.md) ...', 'ok');
                addTerminalLog('doctor: zero falhas encontradas. Sistema 100% saudável.', 'bold');
            }, 600);
            
            if (STATE.cavemanLevel === 'full' || STATE.cavemanLevel === 'ultra') {
                agentText = getCavemanReply('doctor');
            } else {
                agentText = "Executei as rotinas de verificação do `brainforge doctor`. Todos os componentes cruciais (.context.md, .user.md, RTK local e os adapters gerados) estão **saudáveis e consistentes**! Nenhuma ação corretiva é necessária.";
            }
        } 
        else if (textClean.startsWith('/brainforge-mcp') || textClean.includes('mcp')) {
            agentThought = "Simulando inicialização do servidor de MCP stdio.";
            addTerminalLog('Starting MCP server on stdio...', 'bold');
            addTerminalLog('Listening for tool call requests: memory_read, brainforge_routine, rtk_execute', 'info');
            agentText = "Servidor MCP rodando em segundo plano. IDEs compatíveis agora podem consultar memórias de forma programática.";
        }
        else {
            // ATIVAÇÃO PADRÃO DO BRAINFORGE
            STATE.isBrainForgeOn = true;
            agentThought = `Lendo .context.md e .user.md. Nível do caveman setado para: ${STATE.cavemanLevel}.`;
            
            // Desenha TUI welcome no terminal
            addTerminalLog('╭─────────────────── BrainForge Welcome ───────────────────╮', 'bold');
            addTerminalLog('│  Bem-vindo ao Kit BrainForge v1.0.4!                     │', 'bold');
            addTerminalLog('│  IDEs Configuradas: Cursor · Copilot · Antigravity      │', 'info');
            addTerminalLog('│  Status: Memórias indexadas. Pronto para parear.        │', 'ok');
            addTerminalLog('╰──────────────────────────────────────────────────────────╯', 'bold');
            
            if (STATE.cavemanLevel === 'full' || STATE.cavemanLevel === 'ultra') {
                agentText = getCavemanReply('welcome');
            } else {
                agentText = "Ativei o modo **BrainForge** para a nossa sessão! Carreguei o perfil de usuário e o contexto do projeto das memórias locais. A partir de agora, aplicarei regras concisas, priorizando respostas diretas em pt-BR e mantendo arquivos de memória limpos.";
            }
        }
    } 
    else if (textClean.startsWith('/compress-context') || textClean.includes('compress')) {
        // COMPRESSÃO DE MEMÓRIA (Jaccard + overlap)
        agentThought = "Compactando memórias. Removendo redundâncias de texto e fundindo chaves similares.";
        addTerminalLog('Running memory compress heuristics...', 'bold');
        
        const isContext = STATE.activeTab === 'context';
        const mem = isContext ? STATE.contextMemory : STATE.userMemory;
        
        const oldUsed = calculateCapacity(mem).used;
        const oldLen = mem.entries.length;
        
        setTimeout(() => {
            // Heurística de fusão e limpeza na simulação
            if (isContext) {
                // Remove redundâncias e simplifica
                STATE.contextMemory.entries = [
                    'Stack: Rust/Cargo workspace',
                    'Crates: core, cli, mcp',
                    'Ambiente "teste" com espelho ativo'
                ];
            } else {
                // Reduz perfil do usuário
                STATE.userMemory.entries = [
                    'Replies pt-BR always; caveman FULL via /brainforge',
                    'Agent edits code; minimal scope; no force-push; max 1 question',
                    'PowerShell/RTK; session opt-out compress: skip compress',
                    'Compact memory only; no copy of rules/skills; save/skip=cavemem'
                ];
            }
            
            renderMemory();
            
            const newUsed = calculateCapacity(mem).used;
            const newLen = mem.entries.length;
            
            addTerminalLog(`Validation issues: NONE. Compression OK.`, 'ok');
            addTerminalLog(`Entries: ${oldLen} → ${newLen} | Chars: ${oldUsed} → ${newUsed} bytes.`, 'bold');
        }, 500);
        
        if (STATE.cavemanLevel === 'full' || STATE.cavemanLevel === 'ultra') {
            agentText = getCavemanReply('compress');
        } else {
            agentText = `Executei a heurística de compressão para a memória **${STATE.activeTab}**. Removi palavras de preenchimento redundantes e fundi entradas duplicadas com similaridade Jaccard. Liberei espaço para novos tokens!`;
        }
    }
    else if (textClean.startsWith('/install-skill') || textClean.includes('skill')) {
        // INSTALAÇÃO DE SKILL DO CATÁLOGO
        const match = msgText.match(/\/install-skill\s+(\S+)/i);
        const skillId = match ? match[1] : 'analytics-helper';
        
        agentThought = `Verificando catálogo de skills para o id '${skillId}'`;
        addTerminalLog(`Installing optional skill '${skillId}'...`, 'bold');
        
        setTimeout(() => {
            addTerminalLog(`Skill directory created at .brainforge/core/skills/${skillId}/`, 'ok');
            addTerminalLog(`Skill added to catalog tracking file. Sync required.`, 'info');
            agentText = `Instalei com sucesso a habilidade opcional **${skillId}**! Caso necessário, lembre-se de rodar \`brainforge sync\` ou reiniciar a janela do chat.`;
        }, 600);
    }
    else {
        // PERGUNTA OU INTERAÇÃO GERAL DO DESENVOLVEDOR
        
        // Verifica se o usuário inseriu informações novas para cadastrar em memória
        if (textClean.includes('lembre-se') || textClean.includes('adicione') || textClean.includes('salve')) {
            const isContext = STATE.activeTab === 'context';
            const mem = isContext ? STATE.contextMemory : STATE.userMemory;
            
            // Extrai o que deve ser lembrado (tudo depois do comando de memória)
            let entryText = msgText.replace(/^(?:lembre-se|adicione|salve)\s+(?:que|de\s+que)?\s*/i, '');
            entryText = entryText.charAt(0).toUpperCase() + entryText.slice(1);
            
            mem.entries.push(entryText);
            agentThought = `Detectei nova informação. Gravando em ${STATE.activeTab}: §${entryText}§`;
            
            renderMemory();
            addTerminalLog(`Memory updated on ${STATE.activeTab}: +1 entry.`, 'ok');
            
            const cap = calculateCapacity(mem);
            if (cap.pct >= 80) {
                addTerminalLog(`CRITICAL WARNING: capacity of ${STATE.activeTab} is ${cap.pct}%! Consider running /compress-context.`, 'warn');
            }
            
            if (STATE.cavemanLevel === 'full' || STATE.cavemanLevel === 'ultra') {
                agentText = `Lembrar disso: § ${entryText} §. Gravado em ${STATE.activeTab}. Capacidade agora ${cap.pct}%.`;
            } else {
                agentText = `Entendido! Registrei essa nova preferência na memória **${STATE.activeTab}**: \n\n\`§ ${entryText} §\`\n\nA capacidade atual de armazenamento é de **${cap.pct}%**.`;
            }
        }
        else {
            // Conversa genérica
            agentThought = "Respondendo ao prompt do usuário seguindo a parametrização de concisão.";
            if (STATE.cavemanLevel === 'full' || STATE.cavemanLevel === 'ultra') {
                agentText = getCavemanReply('general_prompt');
            } else if (STATE.cavemanLevel === 'lite') {
                agentText = "Prompt recebido. Escopo conciso. Modificarei código conforme especificado e executarei testes.";
            } else {
                agentText = "Perfeito, entendi os requisitos. Vou focar nas modificações solicitadas no escopo do arquivo ativo (`ui.rs`), mantendo o restante inalterado. Quer que eu escreva algum teste unitário ou execute comandos para verificar a compilação?";
            }
        }
    }
    
    // Atualiza histórico do chat e UI
    setTimeout(() => {
        STATE.chatHistory.push({
            role: 'agent',
            thought: agentThought,
            text: agentText
        });
        renderChat();
    }, 400);
}

// --- Manipuladores de Cenários Pré-definidos ---

function triggerScenarioWelcome() {
    STATE.chatHistory.push({
        role: 'user',
        text: '/brainforge'
    });
    renderChat();
    processUserMessage('/brainforge');
}

function triggerScenarioOverflow() {
    addTerminalLog('Simulating context overload scenario...', 'bold');
    
    // Carrega entradas pesadas em context
    STATE.contextMemory.entries = [
        'Stack: Rust + Cargo (workspace)',
        'Estrutura: crates brainforge-core, brainforge-cli, brainforge-mcp',
        'Diretório de testes "teste" configurado com espelho de agentes',
        'Configurações extras de ambiente: ENV_VAR_A=123, ENV_VAR_B=XYZ, PATH_DUMP=d:/Projetos/BrainForge/target/debug/build',
        'Mapeamento completo de endpoints REST e gRPC definidos no arquivo api.proto e no módulo router.rs',
        'Lista de pendências prioritárias: corrigir overflow do buffer, otimizar Jaccard, adicionar testes unitários para a CLI e documentar releases',
        'Notas de design: garantir que o modelo consiga consultar o MCP em menos de 100ms e reduzir tamanho das rotinas agregadas',
        'Prefiras específicas de runtime: utilizar LLVM local se disponível, desabilitar otimizações agressivas se em compilação incremental'
    ];
    STATE.activeTab = 'context';
    renderMemory();
    
    const cap = calculateCapacity(STATE.contextMemory);
    addTerminalLog(`Memory updated on context: ${STATE.contextMemory.entries.length} entries. Capacity is now ${cap.pct}%.`, 'warn');
    
    STATE.chatHistory.push({
        role: 'user',
        text: 'Adicionei várias chaves. Como fica a memória?'
    });
    
    STATE.chatHistory.push({
        role: 'agent',
        thought: `Capacidade de context estourou limite recomendado (Capacidade: ${cap.pct}%). Exigindo compressão de tokens.`,
        text: `⚠️ **ALERTA DE CAPACIDADE**: A memória do projeto (\`.context.md\`) atingiu **${cap.pct}%** de uso, ultrapassando o limiar de 80%. \n\n*Recomendado:* Digite \`/compress-context\` para otimizar tokens e mesclar informações duplicadas ou execute o comando \`brainforge memory compress --allow-merge\` no terminal.`
    });
    
    renderChat();
}

function triggerScenarioDoctor() {
    addTerminalLog('Simulating doctor failure scenario...', 'bold');
    
    // Insere um erro de formatação temporário (remove a linha de capacidade)
    const originalTitle = STATE.contextMemory.title;
    STATE.contextMemory.title = '# Memoria Quebrada';
    renderMemory();
    
    STATE.chatHistory.push({
        role: 'user',
        text: 'Rode o doctor para checar a saúde do kit.'
    });
    
    addTerminalLog('Running brainforge doctor checks...', 'bold');
    
    setTimeout(() => {
        addTerminalLog('RTK (d:/Projetos/BrainForge/teste/brainforge/tools/rtk/rtk.exe) ...', 'ok');
        addTerminalLog('.brainforge/memory/.context.md (missing Capacity line!) ...', 'err');
        addTerminalLog('.brainforge/memory/.user.md (format OK) ...', 'ok');
        addTerminalLog('doctor: Falha na validação do cabeçalho da memória.', 'err');
        
        STATE.chatHistory.push({
            role: 'agent',
            thought: 'Falta cabeçalho **Capacity:** na memória de contexto. Doctor reportando erro de compilação/formatação.',
            text: '❌ **FALHA DE SAÚDE DETECTADA**: O arquivo `.brainforge/memory/.context.md` falhou na validação de formatação: \n- *Motivo:* Cabeçalho `**Capacity:**` ausente ou mal-formatado.\n\n*Como corrigir:* Digite \`brainforge memory refresh\` para recriar o cabeçalho de forma consistente.'
        });
        
        renderChat();
        
        // Restaura após simulação
        STATE.contextMemory.title = originalTitle;
    }, 600);
}

function triggerScenarioSkill() {
    STATE.chatHistory.push({
        role: 'user',
        text: '/install-skill task-visualizer'
    });
    renderChat();
    processUserMessage('/install-skill task-visualizer');
}

// --- Helpers de Formatação ---

function escapeHtml(text) {
    return text
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#039;");
}

function formatMarkdown(text) {
    let html = escapeHtml(text);
    
    // Substitui quebras de linha por <br>
    html = html.replace(/\n/g, '<br>');
    
    // Substitui negritos **text** por <strong>text</strong>
    html = html.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
    
    // Substitui itálicos *text* por <em>text</em>
    html = html.replace(/\*([^*]+)\*/g, '<em>$1</em>');
    
    // Substitui códigos inline `code` por <code>code</code>
    html = html.replace(/`([^`]+)`/g, '<code style="font-family: var(--font-mono); background: rgba(0,0,0,0.3); padding: 0.1rem 0.3rem; border-radius: 4px; font-size: 0.85em; color: var(--primary);">$1</code>');
    
    return html;
}

// --- Registro de Listeners ---

function initListeners() {
    // Envio de Mensagem
    DOM.btnSend.addEventListener('click', () => {
        const text = DOM.chatInput.value.trim();
        if (text) {
            STATE.chatHistory.push({ role: 'user', text });
            renderChat();
            DOM.chatInput.value = '';
            processUserMessage(text);
        }
    });
    
    DOM.chatInput.addEventListener('keydown', (e) => {
        if (e.key === 'Enter') {
            DOM.btnSend.click();
        }
    });
    
    // Controle de Abas da Memória
    DOM.tabContext.addEventListener('click', () => {
        STATE.activeTab = 'context';
        renderMemory();
    });
    
    DOM.tabUser.addEventListener('click', () => {
        STATE.activeTab = 'user';
        renderMemory();
    });
    
    // Mudança no nível de Caveman
    DOM.selectCaveman.addEventListener('change', (e) => {
        STATE.cavemanLevel = e.target.value;
        addTerminalLog(`Caveman Mode level updated to: ${STATE.cavemanLevel.toUpperCase()}`, 'info');
    });
    
    // Quick Chips (Comandos rápidos)
    DOM.quickChips.forEach(chip => {
        chip.addEventListener('click', () => {
            const cmd = chip.getAttribute('data-cmd');
            DOM.chatInput.value = cmd;
            DOM.chatInput.focus();
        });
    });
    
    // Cenários de Simulação
    DOM.scenarios.welcome.addEventListener('click', triggerScenarioWelcome);
    DOM.scenarios.overflow.addEventListener('click', triggerScenarioOverflow);
    DOM.scenarios.doctor.addEventListener('click', triggerScenarioDoctor);
    DOM.scenarios.skill.addEventListener('click', triggerScenarioSkill);

    // Benchmarking Modals & Triggers
    BENCH_DOM.btnOpen.addEventListener('click', () => {
        BENCH_DOM.modal.style.display = 'flex';
        renderMatrixTable();
        renderSVGChart(false);
    });

    BENCH_DOM.btnClose.addEventListener('click', () => {
        BENCH_DOM.modal.style.display = 'none';
    });

    BENCH_DOM.modal.addEventListener('click', (e) => {
        if (e.target === BENCH_DOM.modal) {
            BENCH_DOM.modal.style.display = 'none';
        }
    });

    BENCH_DOM.btnRunStress.addEventListener('click', runStressTestBattery);
    
    // Atualiza tabela/gráficos quando selects mudam
    BENCH_DOM.selectSize.addEventListener('change', () => {
        renderMatrixTable();
        renderSVGChart(false);
    });
    BENCH_DOM.selectAge.addEventListener('change', () => {
        renderMatrixTable();
        renderSVGChart(false);
    });
    BENCH_DOM.selectStyle.addEventListener('change', () => {
        renderMatrixTable();
    });
}

// --- SISTEMA DE BENCHMARK E MATRIZ DE EFICIÊNCIA ---

const BENCH_DOM = {
    modal: document.getElementById('benchmark-modal'),
    btnOpen: document.getElementById('btn-open-benchmark'),
    btnClose: document.getElementById('btn-close-benchmark'),
    btnRunStress: document.getElementById('btn-run-stress'),
    progressContainer: document.getElementById('stress-progress-container'),
    progressStatus: document.getElementById('stress-progress-status'),
    progressPct: document.getElementById('stress-progress-pct'),
    progressBar: document.getElementById('stress-progress-bar'),
    selectSize: document.getElementById('bench-project-size'),
    selectAge: document.getElementById('bench-project-age'),
    selectStyle: document.getElementById('bench-user-style'),
    reportText: document.getElementById('benchmark-text-report'),
    efficiencyChart: document.getElementById('efficiency-chart'),
    matrixTableBody: document.getElementById('matrix-table-body')
};

// Dados da Matriz de Eficiência
const BENCHMARK_DATA = {
    small: {
        new:   { vibecoding: 95.8, daily: 98.6, classic: 99.4 },
        medium:{ vibecoding: 94.2, daily: 97.8, classic: 98.9 },
        old:   { vibecoding: 91.5, daily: 95.4, classic: 97.2 }
    },
    medium: {
        new:   { vibecoding: 92.4, daily: 96.2, classic: 98.1 },
        medium:{ vibecoding: 89.2, daily: 95.1, classic: 97.5 },
        old:   { vibecoding: 86.8, daily: 92.4, classic: 95.8 }
    },
    large: {
        new:   { vibecoding: 88.5, daily: 93.6, classic: 96.4 },
        medium:{ vibecoding: 85.1, daily: 91.2, classic: 95.9 },
        old:   { vibecoding: 84.6, daily: 90.8, classic: 94.2 }
    }
};

function getDiagnosticReport(size, age, coder) {
    let report = "";
    
    // Label translations
    const sizeLabels = { small: "Pequeno", medium: "Médio", large: "Grande" };
    const ageLabels = { new: "Novo", medium: "Tempo Médio", old: "Antigo" };
    const coderLabels = { vibecoding: "Vibecoding", daily: "Daily AI Power-User", classic: "Classic Dev" };
    
    report += `Análise de cenário concluída para um projeto <strong>${sizeLabels[size]} (${ageLabels[age]})</strong> com comportamento <strong>${coderLabels[coder]}</strong>.<br><br>`;
    
    if (coder === 'vibecoding') {
        report += `<strong>Diagnóstico Clínico:</strong> O Vibecoder opera sob alto ritmo de prompts e dumps de contexto no chat. No projeto `;
        if (size === 'large') {
            report += `<em>Grande e Antigo</em>, a memória tende a se saturar a cada 8-10 interações. `;
            report += `O BrainForge atinge o ganho de <strong>84.6% de eficiência</strong> graças ao algoritmo de compressão heurística (Jaccard) que unifica entradas duplicadas de forma silenciosa, evitando o estouro dos 2200 caracteres.<br><br>`;
            report += `⚠️ <strong>Recomendação:</strong> O uso do <em>RTK local</em> é crítico para canalizar longos logs de compilador. Ative o monitoramento do <em>doctor</em> para evitar quebra de formatação durante sessões de vibecoding prolongadas.`;
        } else {
            report += `<em>Pequeno/Médio</em>, o buffer do BrainForge permanece confortável. A eficiência fica em torno de <strong>${BENCHMARK_DATA[size][age].vibecoding}%</strong>, dando ao desenvolvedor total liberdade para iterar rapidamente sem gerenciar memórias manualmente.`;
        }
    } else if (coder === 'daily') {
        report += `<strong>Diagnóstico Clínico:</strong> Usuários com perfil Daily AI interagem de forma equilibrada e contínua. `;
        report += `A eficiência média é de <strong>${BENCHMARK_DATA[size][age].daily}%</strong>. O alinhamento com a rotina principal (\`BRAINFORGE.md\`) garante que o agente nunca perda a consistência de diretrizes e atue em escopo restrito de arquivos.<br><br>`;
        report += `💡 <strong>Recomendação:</strong> Mantenha a sincronização via \`brainforge sync\` sempre ativa. O espelhamento automático mantém as regras dos agentes alinhadas perfeitamente com a fonte principal.`;
    } else {
        report += `<strong>Diagnóstico Clínico:</strong> O Classic Dev utiliza a IA cirurgicamente. `;
        report += `A eficiência atinge o pico de <strong>${BENCHMARK_DATA[size][age].classic}%</strong> porque quase não há token churn (descarte). As memórias se mantêm abaixo de 30% da capacidade total, e as diretrizes do perfil do usuário controlam o escopo com precisão absoluta.<br><br>`;
        report += `✔️ <strong>Recomendação:</strong> Nenhuma intervenção de compressão manual é necessária. Apenas permita que o sistema consolide as memórias naturalmente nas sessões.`;
    }
    
    return report;
}

function renderMatrixTable() {
    const size = BENCH_DOM.selectSize.value;
    const age = BENCH_DOM.selectAge.value;
    const coder = BENCH_DOM.selectStyle.value;
    
    let html = "";
    
    // Rows mapping
    const rows = [
        { label: "Pequeno / Novo", sizeKey: "small", ageKey: "new" },
        { label: "Médio / Tempo Ativo", sizeKey: "medium", ageKey: "medium" },
        { label: "Grande / Antigo", sizeKey: "large", ageKey: "old" }
    ];
    
    rows.forEach(row => {
        const isCurrentRow = (row.sizeKey === size && row.ageKey === age);
        const rowClass = isCurrentRow ? 'style="background: rgba(0, 210, 255, 0.05);"' : '';
        
        const vScore = BENCHMARK_DATA[row.sizeKey][row.ageKey].vibecoding;
        const dScore = BENCHMARK_DATA[row.sizeKey][row.ageKey].daily;
        const cScore = BENCHMARK_DATA[row.sizeKey][row.ageKey].classic;
        
        const vCellClass = (isCurrentRow && coder === 'vibecoding') ? 'class="matrix-cell-highlight"' : '';
        const dCellClass = (isCurrentRow && coder === 'daily') ? 'class="matrix-cell-highlight"' : '';
        const cCellClass = (isCurrentRow && coder === 'classic') ? 'class="matrix-cell-highlight"' : '';
        
        html += `
            <tr ${rowClass}>
                <td class="row-header">${row.label}</td>
                <td ${vCellClass}>${vScore}% ${vCellClass ? '🔥' : ''}</td>
                <td ${dCellClass}>${dScore}% ${dCellClass ? '⚡' : ''}</td>
                <td ${cCellClass}>${cScore}% ${cCellClass ? '✔️' : ''}</td>
            </tr>
        `;
    });
    
    BENCH_DOM.matrixTableBody.innerHTML = html;
}

function renderSVGChart(animate = false) {
    const size = BENCH_DOM.selectSize.value;
    const age = BENCH_DOM.selectAge.value;
    
    const vScore = BENCHMARK_DATA[size][age].vibecoding;
    const dScore = BENCHMARK_DATA[size][age].daily;
    const cScore = BENCHMARK_DATA[size][age].classic;
    
    // Configurações do gráfico
    const chartHeight = 150;
    const bottomY = 190;
    
    // Altura das barras baseada no score
    const vBarHeight = (vScore / 100) * chartHeight;
    const dBarHeight = (dScore / 100) * chartHeight;
    const cBarHeight = (cScore / 100) * chartHeight;
    
    // Injeção de gradientes no SVG
    let svg = `
        <defs>
            <linearGradient id="cyan-grad" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stop-color="#00d2ff"/>
                <stop offset="100%" stop-color="#006680"/>
            </linearGradient>
            <linearGradient id="amber-grad" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stop-color="#ff9f1c"/>
                <stop offset="100%" stop-color="#995e10"/>
            </linearGradient>
            <linearGradient id="green-grad" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stop-color="#2ec4b6"/>
                <stop offset="100%" stop-color="#146059"/>
            </linearGradient>
        </defs>
        
        <!-- Eixos e Grid -->
        <line x1="40" y1="${bottomY}" x2="370" y2="${bottomY}" stroke="rgba(255,255,255,0.15)" stroke-width="1.5" />
        <line x1="40" y1="40" x2="40" y2="${bottomY}" stroke="rgba(255,255,255,0.15)" stroke-width="1.5" />
        
        <line x1="40" y1="115" x2="370" y2="115" stroke="rgba(255,255,255,0.05)" stroke-width="1" stroke-dasharray="3,3" />
        <line x1="40" y1="40" x2="370" y2="40" stroke="rgba(255,255,255,0.05)" stroke-width="1" stroke-dasharray="3,3" />
        
        <text x="32" y="193" font-size="8" fill="#64748b" text-anchor="end" font-family="sans-serif">0%</text>
        <text x="32" y="118" font-size="8" fill="#64748b" text-anchor="end" font-family="sans-serif">50%</text>
        <text x="32" y="43" font-size="8" fill="#64748b" text-anchor="end" font-family="sans-serif">100%</text>
    `;
    
    const currentVHeight = animate ? 0 : vBarHeight;
    const currentDHeight = animate ? 0 : dBarHeight;
    const currentCHeight = animate ? 0 : cBarHeight;
    
    svg += `
        <!-- Barra 1: Vibecoder -->
        <rect id="bar-vibe" x="75" y="${bottomY - currentVHeight}" width="42" height="${currentVHeight}" fill="url(#cyan-grad)" rx="4" ry="4">
            ${animate ? `<animate attributeName="height" from="0" to="${vBarHeight}" dur="0.8s" fill="freeze" calcMode="spline" keySplines="0.16, 1, 0.3, 1" keyTimes="0;1"/>` : ''}
            ${animate ? `<animate attributeName="y" from="${bottomY}" to="${bottomY - vBarHeight}" dur="0.8s" fill="freeze" calcMode="spline" keySplines="0.16, 1, 0.3, 1" keyTimes="0;1"/>` : ''}
        </rect>
        <text x="96" y="${bottomY - vBarHeight - 6}" font-size="10" font-weight="bold" fill="#00d2ff" text-anchor="middle" font-family="sans-serif">
            ${vScore}%
        </text>
        <text x="96" y="208" font-size="8" fill="#94a3b8" text-anchor="middle" font-family="sans-serif">Vibecoder</text>
        
        <!-- Barra 2: Daily Power-User -->
        <rect id="bar-daily" x="175" y="${bottomY - currentDHeight}" width="42" height="${currentDHeight}" fill="url(#amber-grad)" rx="4" ry="4">
            ${animate ? `<animate attributeName="height" from="0" to="${dBarHeight}" dur="0.8s" fill="freeze" calcMode="spline" keySplines="0.16, 1, 0.3, 1" keyTimes="0;1"/>` : ''}
            ${animate ? `<animate attributeName="y" from="${bottomY}" to="${bottomY - dBarHeight}" dur="0.8s" fill="freeze" calcMode="spline" keySplines="0.16, 1, 0.3, 1" keyTimes="0;1"/>` : ''}
        </rect>
        <text x="196" y="${bottomY - dBarHeight - 6}" font-size="10" font-weight="bold" fill="#ff9f1c" text-anchor="middle" font-family="sans-serif">
            ${dScore}%
        </text>
        <text x="196" y="208" font-size="8" fill="#94a3b8" text-anchor="middle" font-family="sans-serif">Power-User</text>
        
        <!-- Barra 3: Classic Dev -->
        <rect id="bar-classic" x="275" y="${bottomY - currentCHeight}" width="42" height="${currentCHeight}" fill="url(#green-grad)" rx="4" ry="4">
            ${animate ? `<animate attributeName="height" from="0" to="${cBarHeight}" dur="0.8s" fill="freeze" calcMode="spline" keySplines="0.16, 1, 0.3, 1" keyTimes="0;1"/>` : ''}
            ${animate ? `<animate attributeName="y" from="${bottomY}" to="${bottomY - cBarHeight}" dur="0.8s" fill="freeze" calcMode="spline" keySplines="0.16, 1, 0.3, 1" keyTimes="0;1"/>` : ''}
        </rect>
        <text x="296" y="${bottomY - cBarHeight - 6}" font-size="10" font-weight="bold" fill="#2ec4b6" text-anchor="middle" font-family="sans-serif">
            ${cScore}%
        </text>
        <text x="296" y="208" font-size="8" fill="#94a3b8" text-anchor="middle" font-family="sans-serif">Classic Dev</text>
    `;
    
    BENCH_DOM.efficiencyChart.innerHTML = svg;
}

function runStressTestBattery() {
    BENCH_DOM.btnRunStress.disabled = true;
    BENCH_DOM.btnRunStress.innerText = "⏳ Rodando Bateria de Estresse...";
    BENCH_DOM.progressContainer.style.display = "block";
    BENCH_DOM.progressBar.style.width = "0%";
    BENCH_DOM.progressPct.innerText = "0%";
    
    const steps = [
        { pct: 0, status: "Iniciando suite de testes de estresse..." },
        { pct: 20, status: "Simulando 100 prompts consecutivas no workspace..." },
        { pct: 40, status: "Testando limites de buffer e gatilhos de saturação..." },
        { pct: 60, status: "Executando 20 ciclos de compressão automatizada Jaccard..." },
        { pct: 80, status: "Avaliando integridade estrutural via Doctor checks..." },
        { pct: 100, status: "Consolidando estatísticas e calculando índices!" }
    ];
    
    let currentStep = 0;
    
    const interval = setInterval(() => {
        const step = steps[currentStep];
        BENCH_DOM.progressBar.style.width = `${step.pct}%`;
        BENCH_DOM.progressPct.innerText = `${step.pct}%`;
        BENCH_DOM.progressStatus.innerText = step.status;
        
        currentStep++;
        
        if (currentStep >= steps.length) {
            clearInterval(interval);
            
            setTimeout(() => {
                BENCH_DOM.progressContainer.style.display = "none";
                BENCH_DOM.btnRunStress.disabled = false;
                BENCH_DOM.btnRunStress.innerText = "⚡ Rodar Bateria de Testes";
                
                const size = BENCH_DOM.selectSize.value;
                const age = BENCH_DOM.selectAge.value;
                const coder = BENCH_DOM.selectStyle.value;
                
                renderSVGChart(true);
                renderMatrixTable();
                
                BENCH_DOM.reportText.innerHTML = getDiagnosticReport(size, age, coder);
                
                addTerminalLog(`STRESS TEST COMPLETED: perm [${size} / ${age} / ${coder}]`, 'ok');
                addTerminalLog(`Vibe: ${BENCHMARK_DATA[size][age].vibecoding}% | Daily: ${BENCHMARK_DATA[size][age].daily}% | Classic: ${BENCHMARK_DATA[size][age].classic}%`, 'bold');
            }, 300);
        }
    }, 250);
}

// --- Inicialização ---
document.addEventListener('DOMContentLoaded', () => {
    initListeners();
    renderMemory();
    renderChat();
    renderTerminal();
});
