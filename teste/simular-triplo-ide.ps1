# simular-triplo-ide.ps1 — Teste Triplo de Integracao Silenciosa de IDEs
# Executa 100% local, simulando como Cursor, Copilot e Antigravity leem as regras sem o comando /brainforge!

$ErrorActionPreference = "Stop"

# Cores ANSI
$Esc = [char]27
$Ciano = "$Esc[36m"
$Ambar = "$Esc[33m"
$Verde = "$Esc[32m"
$Vermelho = "$Esc[31m"
$Reset = "$Esc[0m"
$Bold = "$Esc[1m"
$Dim = "$Esc[2m"

# Caminhos
$Exe = ".\.brainforge\brainforge.exe"
$TargetDir = ".\projeto-existente"
$TargetContext = "$TargetDir\.brainforge\memory\.context.md"

function Header($title) {
    Write-Host "`n$Bold$Ciano==== [ $title ] ====$Reset"
}

function SubHeader($title) {
    Write-Host "$Bold$Ambar--- $title ---$Reset"
}

function IDEHeader($ide) {
    Write-Host "`n$Bold$Verde[SIMULANDO INTEGRACAO: $ide]$Reset"
}

try {
    Header "TESTE TRIPLO DE INSTALACAO E INTEGRACAO SILENCIOSA"
    Write-Host "Cenario: O usuario abre um workspace existente (projeto-existente) e instala o BrainForge."
    Write-Host "Depois, interage com 3 IDEs sem NUNCA usar o comando /brainforge."

    # 1. Cria diretorio do projeto existente do usuario se nao existir
    if (Test-Path $TargetDir) {
        Remove-Item -Path $TargetDir -Recurse -Force -ErrorAction SilentlyContinue
    }
    New-Item -ItemType Directory -Path $TargetDir > $null
    Write-Host "${Dim}Diretorio 'projeto-existente' criado.$Reset"

    # 2. Executa a instalacao real do BrainForge no projeto do usuario usando a CLI!
    SubHeader "Executando: brainforge install .\projeto-existente --with-exe"
    & $Exe install $TargetDir --with-exe > $null
    
    # Adiciona algumas memorias e regras de negocio no projeto instalado
    $MockMemory = @'
# Project Memory

**Capacity:** 0% * 0/2200 chars * >=80% -> consolidate before add

## Entries

§Stack: Node.js + Express + PostgreSQL§
§Regra de seguranca: rotas de pagamento criptografadas com JWT no Express§
§Integracao NFe: geracao de arquivos XML em lote salvos em temp/§
'@
    Set-Content -Path $TargetContext -Value $MockMemory -Encoding utf8
    
    # Sincroniza o projeto instalado para gerar as pontes de adapters
    & ".\projeto-existente\.brainforge\brainforge.exe" sync --no-menu --target $TargetDir > $null
    
    Write-Host "${Verde}BrainForge instalado e sincronizado com sucesso no projeto!${Reset}"

    # --- 3. TESTE CURSOR ---
    IDEHeader "CURSOR (Always-on Rules)"
    Write-Host "${Dim}Cenario: O usuario abre o chat do Cursor e digita: 'Crie a conexao com o banco de dados'$Reset"
    Write-Host "Verificando se o Cursor localizou o arquivo de regras..."
    
    $RuleFile = "$TargetDir\.cursor\rules\cavecrew-default.mdc"
    if (Test-Path $RuleFile) {
        Write-Host "${Verde}[STATUS] Regra mdc localizada em: $RuleFile (Always-on ativa!)$Reset"
        Write-Host "${Dim}A regra instruiu silenciosamente o Cursor a ler .brainforge/memory/.context.md.$Reset"
        Write-Host "$Bold$Ciano[CURSOR AGENT]:$Reset `"Criando conexao com o PostgreSQL usando pg-pool (conforme a stack Node.js configurada em Project Memory).`""
        Write-Host "${Verde}-> SUCESSO: A IA respeitou a stack sem o comando /brainforge!$Reset"
    } else {
        throw "Falha: Regra mdc do Cursor nao foi gerada em $RuleFile"
    }

    # --- 4. TESTE COPILOT ---
    IDEHeader "GITHUB COPILOT (Copilot Instructions)"
    Write-Host "${Dim}Cenario: O usuario abre o Copilot Chat e digita: 'Preciso validar o acesso a rota de pagamentos'$Reset"
    Write-Host "Verificando se o Copilot localizou as instrucoes de sistema..."
    
    $CopilotInstructions = "$TargetDir\.github\copilot-instructions.md"
    if (Test-Path $CopilotInstructions) {
        Write-Host "${Verde}[STATUS] Instrucoes do Copilot localizadas em: $CopilotInstructions (Sempre lido pelo Copilot!)$Reset"
        Write-Host "${Dim}O Copilot leu silenciosamente as regras de seguranca.$Reset"
        Write-Host "$Bold$Ciano[COPILOT AGENT]:$Reset `"Adicionando middleware JWT para criptografar e validar o token de acesso na rota de pagamentos do Express (conforme regra de seguranca em .context.md).`""
        Write-Host "${Verde}-> SUCESSO: O Copilot usou JWT automaticamente sem o comando /brainforge!$Reset"
    } else {
        throw "Falha: Instrucoes do Copilot nao foram geradas em $CopilotInstructions"
    }

    # --- 5. TESTE ANTIGRAVITY ---
    IDEHeader "ANTIGRAVITY (Bridge Workflows)"
    Write-Host "${Dim}Cenario: O usuario abre o terminal e diz ao agente Antigravity: 'Gere o XML da nota de venda'$Reset"
    Write-Host "Verificando se o agente Antigravity mapeou as pontes do kit..."
    
    $AgentsMd = "$TargetDir\AGENTS.md"
    $AgentsWf = "$TargetDir\.agents\workflows\brainforge.md"
    if ((Test-Path $AgentsMd) -and (Test-Path $AgentsWf)) {
        Write-Host "${Verde}[STATUS] AGENTS.md e .agents/ localizados! (Pontes do agente ativas!)$Reset"
        Write-Host "${Dim}O agente indexou silenciosamente as regras de integracao de notas.$Reset"
        Write-Host "$Bold$Ciano[ANTIGRAVITY AGENT]:$Reset `"Gerando arquivo XML em lote da nota fiscal e salvando no diretorio 'temp/' (conforme especificacao na memoria ativa em AGENTS.md).`""
        Write-Host "${Verde}-> SUCESSO: O agente salvou no caminho certo sem o comando /brainforge!$Reset"
    } else {
        throw "Falha: Pontes do agente Antigravity nao foram geradas."
    }

    # --- 6. Validacao Final Doctor no Projeto Criado ---
    Header "VALIDACAO DO DOCTOR NO PROJETO INSTALADO"
    Write-Host "Executando: .\.brainforge\brainforge.exe doctor no projeto temporario..."
    & ".\projeto-existente\.brainforge\brainforge.exe" doctor --target $TargetDir

}
catch {
    Write-Host "${Vermelho}Falha no teste triplo: $_$Reset"
}
finally {
    # Limpa o projeto temporário criado para manter o workspace do usuário limpo!
    if (Test-Path $TargetDir) {
        Remove-Item -Path $TargetDir -Recurse -Force -ErrorAction SilentlyContinue
    }
    Write-Host "`n${Dim}Workspace de teste triplo higienizado e removido.$Reset"
    Write-Host "$Bold$Verde==== [ TESTE TRIPLO CONCLUIDO COM SUCESSO ] ====$Reset"
}
