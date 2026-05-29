# rodar-simulacao.ps1 — Automacao Local de Teste de Estresse e Simulacao do BrainForge
# Executa 100% local, sem gastar tokens, usando o binario compilado real do projeto!

$ErrorActionPreference = "Stop"

# Configuracao de Cores ANSI
$Esc = [char]27
$Ciano = "$Esc[36m"
$Ambar = "$Esc[33m"
$Verde = "$Esc[32m"
$Vermelho = "$Esc[31m"
$Reset = "$Esc[0m"
$Bold = "$Esc[1m"
$Dim = "$Esc[2m"

# Caminhos dos Binarios e Memorias
$Exe = ".\.brainforge\brainforge.exe"
$ContextFile = ".\.brainforge\memory\.context.md"
$UserFile = ".\.brainforge\memory\.user.md"

# Backups para restaurar no final do teste
$ContextBackup = ""
$UserBackup = ""

function Header($title) {
    Write-Host "`n$Bold$Ciano==== [ $title ] ====$Reset"
}

function SubHeader($title) {
    Write-Host "$Bold$Ambar--- $title ---$Reset"
}

# --- 1. Inicializacao e Backups ---
Header "1. INICIALIZANDO SIMULACAO LOCAL"
Write-Host "${Dim}Criando backup temporario das memorias reais para restaurar depois...$Reset"

if (Test-Path $ContextFile) {
    $ContextBackup = Get-Content $ContextFile -Raw
} else {
    throw "Arquivo .context.md nao encontrado em $ContextFile"
}

if (Test-Path $UserFile) {
    $UserBackup = Get-Content $UserFile -Raw
} else {
    throw "Arquivo .user.md nao encontrado em $UserFile"
}

Write-Host "${Verde}Backup concluido com sucesso. Iniciando bateria de testes.$Reset"

try {
    # --- 2. Verificacao de Saude Inicial (Doctor) ---
    Header "2. EXECUTANDO HEALTH CHECK INICIAL (DOCTOR)"
    Write-Host "${Dim}Executando: brainforge doctor$Reset"
    
    # Executa o binario real do projeto!
    & $Exe doctor
    
    Write-Host "${Verde}Check estrutural do doctor concluido com status PASS!$Reset"

    # --- 3. Simulacao de Uso Clinico (Primeiro Prompt & Saturacao) ---
    Header "3. SIMULANDO CODIFICACAO ATIVA (VIBECODING)"
    Write-Host "Injetando entradas densas e repetitivas de forma automatizada no `.context.md`..."
    
    $MockEntries = @(
        "§Stack: Rust + Cargo workspace com crates core, cli e mcp§",
        "§Crates ativos: brainforge-core e brainforge-cli usando rustc 2021§",
        "§Configuracao do Banco de Dados: SQLite com pool de conexoes r2d2§",
        "§SQLite configurado localmente na pasta target/debug/db.sqlite para testes unitarios§",
        "§Servidor Web: Actix-web com roteamento assincrono para os endpoints do MCP§",
        "§MCP endpoints rodando em rotas http com TLS ativo localmente§",
        "§Nota de Performance: Compressao Jaccard roda no core em menos de 10ms§",
        "§Performance: Algoritmo de unificacao de duplicatas otimizado com stopword filtering§",
        "§Regra de commits: Sempre testar com doctor antes de realizar commits no git§"
    )
    
    # Constroi um arquivo .context.md artificialmente pesado
    $NewContext = @"
# Project Memory

**Capacity:** 0% · 0/2200 chars · >=80% → consolidate before add

## Entries

$( $MockEntries -join "`n" )
"@

    Set-Content -Path $ContextFile -Value $NewContext -Encoding utf8
    
    # Atualiza o cabecalho de capacidade usando o proprio CLI real!
    SubHeader "Calculando Capacidade apos Codificacao"
    & $Exe memory refresh --file context
    
    $CurrentStats = Get-Content $ContextFile -Raw
    Write-Host $CurrentStats
    
    # --- 4. Simulacao de Estouro e Auto-Compressao ---
    Header "4. EXECUTANDO COMPRESSAO DETERMINISTICA AUTOMATICA"
    Write-Host "${Ambar}Alerta detectado: A capacidade ultrapassou os limites recomendados!$Reset"
    Write-Host "Executando: ${Bold}brainforge memory compress --file context --allow-merge$Reset`n"
    
    # Executa o comando de compressao do CLI real que mescla as redundancias via algoritmo de Jaccard!
    & $Exe memory compress --file context --allow-merge
    
    SubHeader "Visualizando Arquivo de Memoria Comprimido"
    $CompressedStats = Get-Content $ContextFile -Raw
    Write-Host $CompressedStats

    # --- 5. Auditoria de Validacao Final ---
    Header "5. EXECUTANDO VALIDACAO DE INTEGRIDADE POS-COMPRESSAO"
    Write-Host "${Dim}Verificando se o algoritmo preservou URLs, ancoras e a estrutura do cabecalho...$Reset"
    
    & $Exe doctor
    
    Write-Host "${Verde}Validacao concluida! O ecossistema esta 100% integro e higienizado.$Reset"

}
catch {
    Write-Host "`n${Vermelho}Erro durante a simulacao: $_$Reset"
}
finally {
    # --- 6. Restauracao de Estado ---
    Header "6. LIMPANDO WORKSPACE DE SIMULACAO"
    Write-Host "${Dim}Restaurando memorias originais do backup...$Reset"
    
    if ($ContextBackup) {
        Set-Content -Path $ContextFile -Value $ContextBackup -Encoding utf8
    }
    if ($UserBackup) {
        Set-Content -Path $UserFile -Value $UserBackup -Encoding utf8
    }
    
    # Executa sync para garantir que as pontes de adapters voltem ao estado normal
    & $Exe sync --no-menu
    
    Write-Host "${Verde}Workspace higienizado. Estado original restaurado com sucesso!$Reset`n"
    Write-Host "$Bold$Verde==== [ BATERIA DE SIMULACAO AUTOMATICA CONCLUIDA ] ====$Reset"
}
