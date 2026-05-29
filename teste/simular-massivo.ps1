# simular-massivo.ps1 — Teste de Carga e Estresse Massivo (50+ Prompts Otimizado)
# Executa 100% local, estressando o binario real do BrainForge de forma ultra rapida!

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
$ContextFile = ".\.brainforge\memory\.context.md"
$UserFile = ".\.brainforge\memory\.user.md"

# Backups
$ContextBackup = Get-Content $ContextFile -Raw
$UserBackup = Get-Content $UserFile -Raw

function Header($title) {
    Write-Host "`n$Bold$Ciano==== [ $title ] ====$Reset"
}

function SubHeader($title) {
    Write-Host "$Bold$Ambar--- $title ---$Reset"
}

# --- Base de dados de 50 chaves tecnicas complexas do ContaCerta ---
$MockDatabase = @(
    "§Stack: Node.js + Express + Postgres com pool de conexoes pg-pool§",
    "§Modulo PDV: abertura e fechamento de caixa com saldo inicial estruturado§",
    "§Modulo PDV: suporte a sangria e reforco de caixa em tempo real§",
    "§Modulo PDV: emissao de cupom nao fiscal para impressora termica de 80mm§",
    "§Modulo Mesas: comandas eletronicas vinculadas ao garcom pelo ID do terminal§",
    "§Modulo Mesas: divisao automatica de conta entre clientes da mesma mesa§",
    "§Modulo Mesas: suporte a taxa de servico opcional de 10% parametrizada§",
    "§Cardapio Digital: QR Code dinamico impresso por mesa gerado pelo front§",
    "§Cardapio Digital: sincronizacao instantanea de alteracoes de precos via websocket§",
    "§Cardapio Digital: carrinho de compras local com persistencia em LocalStorage§",
    "§Integracao NFe: comunicacao assincrona com API do SEFAZ com fallback§",
    "§Integracao NFe: transmissao de XML estruturado e geracao de DANFE in PDF§",
    "§Integracao NFe: consulta automatica de status de lote enviado em segundo plano§",
    "§Integracao Sat: suporte a transmissao de cupom fiscal em ambiente de contingencia§",
    "§Modulo Financeiro: fluxo de caixa com contas a pagar e receber integradas§",
    "§Modulo Financeiro: conciliacao bancaria automatica via leitura de arquivos OFX§",
    "§Modulo Financeiro: relatorios de DRE simplificados gerados mensalmente§",
    "§Integracao Pagamento: checkout integrado com maquininha Stone via API local§",
    "§Integracao Pagamento: suporte a pagamento via PIX copia e cola com QR Code§",
    "§Integracao Pagamento: webhook para captura automatica de status de pagamento Stone§",
    "§Modulo Delivery: integracao bidirecional com API do iFood para pedidos§",
    "§Modulo Delivery: controle de status de entrega iFood (despachado, entregue)§",
    "§Modulo Delivery: gestao de frota de motoboys terceirizados com calculo de taxa§",
    "§Estoque: controle de insumos com alerta de estoque minimo parametrizavel§",
    "§Estoque: ficha tecnica de pratos com baixa automatica de ingredientes no PDV§",
    "§Estoque: entrada de mercadorias via importacao de XML de nota fiscal de compra§",
    "§Usuario: autenticacao baseada em JWT com expiracao de 12 horas§",
    "§Usuario: niveis de permissao definidos (administrador, gerente, caixa, garcom)§",
    "§Seguranca: criptografia de senhas usando bcrypt com salt de 12 rounds§",
    "§Seguranca: rate limiting nas rotas sensiveis de login para evitar brute force§",
    "§Modulo Cozinha: tela KDS (Kitchen Display System) para visualizacao de pedidos§",
    "§Modulo Cozinha: alerta sonoro de novos pedidos pendentes no painel de preparo§",
    "§Modulo Cozinha: tempo medio de preparo registrado por prato no banco de dados§",
    "§Relatorios: exportacao de planilhas Excel (XLSX) de vendas por periodo§",
    "§Relatorios: painel de controle (dashboard) com graficos de faturamento HSL§",
    "§Relatorios: ranking de pratos mais vendidos gerado automaticamente§",
    "§Infraestrutura: deploy automatizado via Docker compose no VPS Linux§",
    "§Infraestrutura: backup diario do PostgreSQL configurado com cron job no S3§",
    "§Infraestrutura: monitoramento de erros em producao utilizando Sentry§",
    "§Modulo Clientes: cadastro de fidelidade com acumulo de pontos por compra§",
    "§Modulo Clientes: suporte a cupons de desconto gerados para campanhas de marketing§",
    "§Modulo Clientes: historico de consumo individual para recomendacoes personalizadas§",
    "§API: endpoints REST documentados com Swagger para integradores externos§",
    "§API: suporte a CORS configurado estritamente para subdominios confiaveis§",
    "§Concorrencia: travas otimistas no banco para evitar conflito de mesas duplicadas§",
    "§Notificacoes: mensageria via Telegram bot para avisos de fechamento de caixa§",
    "§Configuracao: fuso horario fixado em America/Sao_Paulo no runtime§",
    "§Configuracao: cache de consultas pesadas de estoque utilizando Redis local§",
    "§Validador: check do doctor roda antes de commits estruturais no Git§",
    "§RTK local: habilitado para capturar outputs densos de compilacao sem saturar§"
)

try {
    Header "SIMULACAO DE CARGA MASSIVA OTIMIZADA - 50 PROMPTS EM LOTE"
    Write-Host "Grava as 50 entradas tecnicas no disco de forma instantanea para evitar gargalos"
    Write-Host "de inicializacao de processos no Windows."
    
    # 1. Configura estado inicial limpo do ContaCerta
    $InitialContext = @'
# Project Memory

**Capacity:** 0% * 0/2200 chars * >=80% -> consolidate before add

## Entries

§Stack: Node.js + Express + PostgreSQL§
'@
    Set-Content -Path $ContextFile -Value $InitialContext -Encoding utf8
    
    # 2. Concatena todos os 50 prompts de uma vez!
    $CurrentContent = Get-Content $ContextFile -Raw
    $NewContent = $CurrentContent.Replace("## Entries", "## Entries`n`n$( $MockDatabase -join "`n" )")
    Set-Content -Path $ContextFile -Value $NewContent -Encoding utf8
    
    # 3. Calcula capacidade inicial massiva acumulada
    Header "3. CALCULANDO CAPACIDADE INICIAL DA CARGA MASSIVA"
    & $Exe memory refresh --file context
    
    SubHeader "Estatisticas da Memoria Inundada no Disco"
    & $Exe memory read --file context
    
    # 4. Dispara compressao deterministica em lote unico
    Header "4. EXECUTANDO COMPRESSAO MASSIVA JACCARD"
    Write-Host "Executando: ${Bold}brainforge memory compress --file context --allow-merge$Reset`n"
    
    # Roda a compressao do binario real
    & $Exe memory compress --file context --allow-merge
    
    SubHeader "Visualizando Arquivo de Memoria Apos Compressao"
    & $Exe memory read --file context

    # 5. Auditoria Doctor de Estresse
    Header "5. AUDITORIA FINAL DOCTOR"
    & $Exe doctor

}
catch {
    Write-Host "${Vermelho}Falha durante teste massivo: $_$Reset"
}
finally {
    # Restaura Workspace
    Set-Content -Path $ContextFile -Value $ContextBackup -Encoding utf8
    Set-Content -Path $UserFile -Value $UserBackup -Encoding utf8
    & $Exe sync --no-menu > $null
    Write-Host "`n${Dim}Workspace restaurado e limpo.$Reset"
    Write-Host "$Bold$Verde==== [ BATERIA DE SIMULACAO MASSIVA CONCLUIDA ] ====$Reset"
}
