# Instalação do Por do Sol SDK

Este guia descreve como instalar o Por do Sol SDK em seu computador. O SDK inclui a ferramenta CLI, o compilador, o interpretador e a biblioteca padrão, tudo pronto para uso sem precisar compilar código-fonte.

## Requisitos do Sistema

### Windows
- Windows 10 ou posterior (64-bit)
- 100MB de espaço em disco
- 512MB de RAM

### Linux
- Qualquer distribuição Linux moderna (64-bit)
- 100MB de espaço em disco
- 512MB de RAM

## Métodos de Instalação

### Windows

#### Opção 1: Instalador MSI (Recomendado)

1. Baixe o arquivo `pordosol-sdk-v0.1.0-windows-x64.msi` dos [releases do GitHub](https://github.com/Adriano-Severino/Compilador/releases)
2. Execute o instalador: dê duplo clique no arquivo `.msi`
3. Siga o assistente de instalação:
   - Aceite os termos de licença
   - Escolha o diretório de instalação (padrão: `C:\Program Files\PorDoSol`)
   - Clique em "Install"
4. Aguarde a conclusão da instalação
5. Reabra o terminal para que as alterações no PATH tenham efeito

#### Opção 2: Script PowerShell

1. Baixe o arquivo `pordosol-sdk-v0.1.0-windows-x64.zip` dos [releases do GitHub](https://github.com/Adriano-Severino/Compilador/releases)
2. Extraia o arquivo `.zip` em uma pasta temporária
3. Abra o PowerShell como Administrador
4. Navegue até a pasta extraída:
   ```powershell
   cd Caminho\Para\Pasta\Extraida
   ```
5. Execute o script de instalação:
   ```powershell
   .\install.ps1
   ```
6. Reabra o terminal para que as alterações no PATH tenham efeito

### Linux

#### Script Bash

1. Baixe o arquivo `pordosol-sdk-v0.1.0-linux-x64.tar.gz` dos [releases do GitHub](https://github.com/Adriano-Severino/Compilador/releases)
2. Extraia o arquivo:
   ```bash
   tar -xzf pordosol-sdk-v0.1.0-linux-x64.tar.gz
   ```
3. Navegue até a pasta extraída:
   ```bash
   cd pordosol-sdk-v0.1.0-linux-x64
   ```
4. Torne o script executável:
   ```bash
   chmod +x install.sh
   ```
5. Execute o script de instalação:
   ```bash
   ./install.sh
   ```
6. Reabra o terminal para que as alterações no PATH tenham efeito

## Verificação da Instalação

Após a instalação, verifique se tudo está funcionando corretamente:

```bash
pordosol doctor
```

Este comando deve mostrar:
- ✓ Compilador: encontrado
- ✓ Interpretador: encontrado
- ✓ Biblioteca padrão: encontrada
- Resultado: ambiente pronto para uso

## Estrutura de Instalação

O SDK é instalado na seguinte estrutura:

```
PORDOSOL_HOME/
├── bin/
│   └── pordosol           # Ferramenta CLI
├── tools/
│   ├── compilador         # Compilador
│   ├── interpretador      # Interpretador de bytecode
│   └── stdlib/           # Biblioteca padrão
└── templates/
    ├── console/          # Template para aplicação console
    └── web/              # Template para aplicação web
```

## Variáveis de Ambiente

A instalação configura as seguintes variáveis de ambiente:

- `PORDOSOL_HOME`: Diretório de instalação do SDK
- `PATH`: Adiciona `PORDOSOL_HOME/bin` ao PATH do sistema

## Opções de Instalação Avançadas

### Diretório de Instalação Personalizado

**Windows (PowerShell):**
```powershell
.\install.ps1 -InstallRoot "C:\MeuPordosol"
```

**Linux (Bash):**
```bash
./install.sh --install-root "$HOME/meu-pordosol"
```

### Não Modificar o PATH

Se você preferir não modificar o PATH automaticamente:

**Windows (PowerShell):**
```powershell
.\install.ps1 -NoPath
```

**Linux (Bash):**
```bash
./install.sh --no-path
```

Neste caso, você precisará adicionar manualmente o diretório `bin` ao seu PATH ou usar o caminho completo ao executar o comando.

## Desinstalação

### Windows (Instalador MSI)

1. Abra "Painel de Controle" > "Programas e Recursos"
2. Encontre "Por do Sol SDK" na lista
3. Clique em "Desinstalar"
4. Siga o assistente de desinstalação

### Windows (Script PowerShell)

1. Abra o PowerShell como Administrador
2. Execute:
   ```powershell
   $env:PORDOSOL_HOME = "C:\Users\SeuUsuario\.pordosol"  # ou seu diretório de instalação
   Remove-Item -Recurse -Force $env:PORDOSOL_HOME
   ```
3. Remova manualmente o diretório do PATH:
   - Sistema > Configurações Avançadas do Sistema > Variáveis de Ambiente
   - Edite a variável `Path` e remova a entrada do Por do Sol

### Linux

1. Remova o diretório de instalação:
   ```bash
   rm -rf ~/.pordosol
   ```
2. Edite seu arquivo de profile (`~/.bashrc`, `~/.zshrc`, etc.) e remova as linhas:
   ```bash
   # >>> pordosol cli >>>
   export PORDOSOL_HOME="$HOME/.pordosol"
   export PATH="$PORDOSOL_HOME/bin:$PATH"
   # <<< pordosol cli <<<
   ```

## Atualização

Para atualizar para uma nova versão:

1. Baixe a nova versão dos [releases do GitHub](https://github.com/Adriano-Severino/Compilador/releases)
2. Desinstale a versão anterior (veja seção acima)
3. Instale a nova versão seguindo os passos de instalação

Ou use o comando de atualização (se disponível):
```bash
pordosol update
```

## Solução de Problemas

### "pordosol não é reconhecido como comando interno"

**Causa:** O PATH não foi configurado corretamente.

**Solução:**
1. Feche e reabra o terminal
2. Se o problema persistir, verifique se `PORDOSOL_HOME/bin` está no PATH:
   - **Windows:** `echo $env:PATH`
   - **Linux:** `echo $PATH`
3. Adicione manualmente se necessário

### "Erro: compilador não encontrado"

**Causa:** O compilador não foi instalado corretamente.

**Solução:**
1. Execute `pordosol doctor` para diagnosticar o problema
2. Verifique se o arquivo `compilador.exe` (Windows) ou `compilador` (Linux) existe em `PORDOSOL_HOME/tools/`
3. Reinstale o SDK se necessário

### "Erro de permissão" no Linux

**Causa:** Os scripts não têm permissão de execução.

**Solução:**
```bash
chmod +x install.sh
chmod +x ~/.pordosol/bin/pordosol
chmod +x ~/.pordosol/tools/compilador
chmod +x ~/.pordosol/tools/interpretador
```

### Espaço insuficiente em disco

**Causa:** Não há espaço suficiente para a instalação.

**Solução:**
- Libere espaço em disco ou escolha um diretório de instalação em outra partição com mais espaço

## Suporte

Se você encontrar problemas não documentados aqui:

1. Verifique o guia [SOLUCAO_PROBLEMAS.md](SOLUCAO_PROBLEMAS.md) para problemas comuns
2. Abra uma issue no [GitHub](https://github.com/Adriano-Severino/Compilador/issues)
3. Forneça detalhes do seu sistema operacional e versão do SDK

## Próximos Passos

Após a instalação bem-sucedida, consulte o guia [PRIMEIROS_PASSOS.md](PRIMEIROS_PASSOS.md) para criar seu primeiro projeto em Por do Sol.