# RTK (único local)

RTK fica **somente** em `brainforge/tools/rtk/` — o sync **não** copia para `.cursor/`.

## Files

- `rtk.exe` — Windows binary
- `install-rtk-local.ps1` — download/update

## Install / update

```powershell
powershell -ExecutionPolicy Bypass -File .\brainforge\tools\rtk\install-rtk-local.ps1 -Force
```

## Run

```powershell
.\brainforge\tools\rtk\rtk.exe --version
.\brainforge\tools\rtk\rtk.exe gain
```

## Copy kit

Copie a pasta **`brainforge/`** inteira (com `tools/rtk/rtk.exe`).
