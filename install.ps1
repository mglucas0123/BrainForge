#Requires -Version 5.1
# Legacy URL — redirects to bf.ps1 (shorter).
$ErrorActionPreference = "Stop"
$urls = @(
    "https://mglucas0123.github.io/BrainForge/bf.ps1",
    "https://raw.githubusercontent.com/mglucas0123/BrainForge/v1.0.4/bf.ps1"
)
foreach ($u in $urls) {
    try {
        $script = (Invoke-WebRequest -Uri $u -UseBasicParsing).Content
        Invoke-Expression -Command $script
        exit $LASTEXITCODE
    } catch {
        Write-Warning "Falha em $u : $_"
    }
}
throw "Nao foi possivel baixar bf.ps1. Use: iex (irm https://mglucas0123.github.io/BrainForge/bf.ps1 -UseBasicParsing)"
