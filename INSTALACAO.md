# Instalacao Do Por do Sol CLI

Este diretório contém instaladores para preparar um SDK local com a estrutura:

```text
<PORDOSOL_HOME>/
  bin/
    pordosol(.exe)
  tools/
    compilador(.exe)
    interpretador(.exe)
    stdlib/
  templates/
    console/
    web/
```

## Windows (PowerShell)

No diretório `ferramentas-cli`:

```powershell
.\install.ps1
```

Opções úteis:

```powershell
.\install.ps1 -InstallRoot "C:\pordosol" -SkipBuild
.\install.ps1 -NoPath
```

## Linux/macOS

No diretório `ferramentas-cli`:

```bash
chmod +x ./install.sh
./install.sh
```

Opções úteis:

```bash
./install.sh --install-root "$HOME/.pordosol" --skip-build
./install.sh --no-path
```

## Verificacao

Depois da instalação, reabra o terminal e execute:

```bash
pordosol doctor
pordosol new list
```
