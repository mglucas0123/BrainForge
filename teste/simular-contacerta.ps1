# simular-contacerta.ps1 — Simulacao de Estresse do Projeto ContaCerta (Comercio Grande)
# Executa 100% local usando o brainforge.exe real com dados do ContaCerta!

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

# Caminhos
$Exe = ".\.brainforge\brainforge.exe"
$ContextFile = ".\.brainforge\memory\.context.md"
$UserFile = ".\.brainforge\memory\.user.md"

# Backups
$ContextBackup = Get-Content $ContextFile -Raw
$UserBackup = Get-Content $UserFile -Raw

function Header($title) {
    Write-Host "`n$Bold$Ciano==== [ $title ] ====$Reset"
}

function PromptSim($user, $msg) {
    Write-Host "`n$Bold$Verde[USUARIO - $user]:$Reset $msg"
}

function AgentSim($thought, $reply) {
    Write-Host "$Dim[PENSAMENTO DA IA]: $thought$Reset"
    Write-Host "$Bold$Ciano[AGENTE ANTIGRAVITY]:$Reset $reply"
}

try {
    Header "SIMULANDO PROJETO CONTACERTA (6 MESES DE VIBECODING)"
    Write-Host "Stack: Node.js + Postgres. Modulos: PDV, Mesas, Cardapio, NFe."
    Write-Host "Uso de IA: 100x prompts/dia. Memoria inicial saturada (75% de capacidade)."

    # 1. Configura estado inicial denso do ContaCerta
    $InitialContext = @"
# Project Memory

**Capacity:** 75% · 1650/2200 chars · >=80% → consolidate before add

## Entries

§Stack: Node.js + Express + PostgreSQL§
§Modulo PDV: controle de caixa, fluxo de vendas e fechamento diario§
§Modulo Mesas: mapeamento de consumo, abertura e fechamento de comandas§
§Modulo Cardapio Digital: QR Code nas mesas, pedidos integrados com a cozinha§
§Integracao NFe: emissao de nota fiscal via API externa e envio de XML§
§Regra: manter calculos de comissao de garcons centralizados no modulo de mesas§
§Regra de seguranca: rotas de pagamento criptografadas com JWT no Express§
"@
    Set-Content -Path $ContextFile -Value $InitialContext -Encoding utf8
    & $Exe memory refresh --file context > $null

    # --- SIMULACAO DE INTERACOES DO VIBECODER ---

    # PROMPT 1
    PromptSim "Vibecoder" "cara faz o modulo de nfe agora com envio de lote e gera o xml na pasta temp"
    
    # IA processa e adiciona na memoria
    $Entry1 = "Integracao NFe: geracao de arquivos XML em lote salvos em temp/ e integrados com a API local de contingencia"
    $CurrentMemory = Get-Content $ContextFile -Raw
    $NewMemory = $CurrentMemory.Replace("## Entries", "## Entries`n`n§$Entry1§")
    Set-Content -Path $ContextFile -Value $NewMemory -Encoding utf8
    & $Exe memory refresh --file context > $null
    
    AgentSim "Usuario pedindo envio de lote na NFe. Gravando entrada. Capacidade subindo." "Entendido. Criar modulo de envio em lote para NFe. Gravar XML na pasta temp. Feito."

    # PROMPT 2
    PromptSim "Vibecoder" "agora refina o pdv pra calcular troco na tela rapido sem dar delay"
    
    # IA processa e adiciona
    $Entry2 = "Modulo PDV: calculo rapido de troco no front-end em JavaScript, sem delay na requisição assincrona"
    $CurrentMemory = Get-Content $ContextFile -Raw
    $NewMemory = $CurrentMemory.Replace("## Entries", "## Entries`n`n§$Entry2§")
    Set-Content -Path $ContextFile -Value $NewMemory -Encoding utf8
    & $Exe memory refresh --file context > $null

    AgentSim "Usuario quer PDV rapido com troco. Adicionando ao contexto." "Refinar calculo de troco no PDV. Sem delay na renderizacao do caixa. Código ajustado."

    # PROMPT 3 (Estouro do Limite de 80%)
    PromptSim "Vibecoder" "cara poe mesa integrada no cardapio digital e valida se o cliente pagou antes de pedir mais"
    
    # IA processa e adiciona
    $Entry3 = "Modulo Cardapio Digital: integracao de comandas de mesa para validacao de pagamento previo antes de novos pedidos"
    $CurrentMemory = Get-Content $ContextFile -Raw
    $NewMemory = $CurrentMemory.Replace("## Entries", "## Entries`n`n§$Entry3§")
    Set-Content -Path $ContextFile -Value $NewMemory -Encoding utf8
    
    Header "DISPARANDO MOTOR DE COMPRESSAO DO BRAINFORGE"
    Write-Host "${Ambar}Alerta de estouro detectado. Capacidade real ultrapassou 80%!$Reset"
    Write-Host "Executando compressao Jaccard para fundir as redundancias de 6 meses..."
    
    # Roda a compressao deterministica real do CLI!
    & $Exe memory compress --file context --allow-merge

    # Mostra os resultados reais de fusao do ContaCerta
    Write-Host "`n$Bold$Verde=== ARQUIVO COMPRIMIDO E HIGIENIZADO NO DISCO ===$Reset"
    $FinalStats = Get-Content $ContextFile -Raw
    Write-Host $FinalStats

}
catch {
    Write-Host "${Vermelho}Erro: $_$Reset"
}
finally {
    # Restaura Workspace
    Set-Content -Path $ContextFile -Value $ContextBackup -Encoding utf8
    Set-Content -Path $UserFile -Value $UserBackup -Encoding utf8
    & $Exe sync --no-menu > $null
    Write-Host "`n${Dim}Workspace restaurado e limpo.$Reset"
}
