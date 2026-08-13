# Solução de Problemas - Por do Sol SDK

Este guia ajuda a resolver problemas comuns que você pode encontrar ao usar o Por do Sol SDK.

## Diagnóstico Inicial

Se você está enfrentando problemas, o primeiro passo é executar o diagnóstico:

```bash
pordosol doctor
```

Este comando mostra o status de todos os componentes do SDK e ajuda a identificar problemas.

## Problemas Comuns

### 1. Comando não reconhecido

#### Sintoma
```
'pordosol' não é reconhecido como um comando interno ou externo
```

#### Causas Possíveis
- O SDK não foi instalado corretamente
- O PATH não foi configurado
- O terminal não foi reiniciado após a instalação

#### Soluções

**Solução 1: Reiniciar o terminal**
Feche e reabra o terminal. Isso é necessário para que as alterações no PATH tenham efeito.

**Solução 2: Verificar o PATH**
- **Windows (PowerShell):**
  ```powershell
  echo $env:PATH
  ```
- **Linux:**
  ```bash
  echo $PATH
  ```

Verifique se o diretório `bin` do Por do Sol está no PATH. Por padrão, deve ser:
- Windows: `C:\Users\SeuUsuario\.pordosol\bin` ou `C:\Program Files\PorDoSol\bin`
- Linux: `$HOME/.pordosol/bin`

**Solução 3: Adicionar manualmente ao PATH**

- **Windows:**
  1. Sistema > Configurações Avançadas do Sistema > Variáveis de Ambiente
  2. Em "Variáveis do usuário", edite `Path`
  3. Adicione o caminho do diretório `bin`

- **Linux:**
  Adicione ao seu `~/.bashrc` ou `~/.zshrc`:
  ```bash
  export PATH="$HOME/.pordosol/bin:$PATH"
  ```

**Solução 4: Reinstalar o SDK**
Se as soluções acima não funcionarem, reinstale o SDK seguindo o guia [INSTALACAO.md](INSTALACAO.md).

### 2. Erro de compilação

#### Sintoma
```
Erro ao compilar: arquivo não encontrado
```

#### Causas Possíveis
- O compilador não está instalado
- O arquivo `.pr` não existe
- Caminho incorreto do projeto

#### Soluções

**Solução 1: Verificar instalação do compilador**
```bash
pordosol doctor
```

Certifique-se de que o compilador aparece como "encontrado".

**Solução 2: Verificar estrutura do projeto**
Certifique-se de que seu projeto tem a estrutura correta:
```
MeuProjeto/
├── src/
│   └── programa.pr
└── pordosol.proj
```

**Solução 3: Especificar o caminho correto**
```bash
pordosol build --project caminho/para/projeto
```

### 3. Erro ao executar

#### Sintoma
```
Erro ao executar: interpretador não encontrado
```

#### Causas Possíveis
- O interpretador não está instalado
- O bytecode `.pbc` não foi gerado
- Permissões de arquivo incorretas

#### Soluções

**Solução 1: Verificar instalação do interpretador**
```bash
pordosol doctor
```

**Solução 2: Recompilar o projeto**
```bash
pordosol build --force
pordosol run
```

**Solução 3: Verificar permissões (Linux)**
```bash
chmod +x ~/.pordosol/tools/interpretador
chmod +x ~/.pordosol/bin/pordosol
```

### 4. Biblioteca padrão não encontrada

#### Sintoma
```
Erro: biblioteca padrão não encontrada
```

#### Causas Possíveis
- A biblioteca padrão não foi instalada
- Caminho incorreto da biblioteca
- Biblioteca não foi compilada

#### Soluções

**Solução 1: Verificar instalação da stdlib**
```bash
pordosol doctor
```

**Solução 2: Reinstalar o SDK**
A reinstalação deve incluir a biblioteca padrão.

**Solução 3: Compilar manualmente (modo desenvolvimento)**
Se você está em modo desenvolvimento:
```bash
cd sistema-padrao
../compilador-portugues/target/release/compilador --compilar-biblioteca=.
```

### 5. Erro de permissão (Linux/macOS)

#### Sintoma
```
Permission denied: './install.sh'
```

#### Causa
O script não tem permissão de execução.

#### Solução
```bash
chmod +x install.sh
./install.sh
```

### 6. Erro "cargo não encontrado"

#### Sintoma
```
cargo: command not found
```

#### Causa
Rust/Cargo não está instalado (apenas em modo desenvolvimento).

#### Solução
Se você está instalando a versão pré-compilada, não precisa do Cargo. Use o modo pacote:
```bash
./install.sh --package-mode
```

Se você precisa do Cargo para desenvolvimento:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 7. Erro de espaço em disco

#### Sintoma
```
Erro: espaço insuficiente em disco
```

#### Causa
Não há espaço suficiente para a instalação ou compilação.

#### Solução
- Libere espaço em disco
- Instale em outro diretório com mais espaço:
  ```bash
  ./install.sh --install-root /caminho/com/mais/espaco
  ```

### 8. Problemas com WiX (Windows)

#### Sintoma
```
Erro: WiX Toolset não encontrado
```

#### Causa
WiX Toolset não está instalado (necessário apenas para criar instalador MSI).

#### Solução
1. Baixe o WiX Toolset em: https://wixtoolset.org/releases/
2. Instale e adicione ao PATH
3. Reexecute o script de build do instalador

Ou use o pacote .zip em vez do instalador MSI.

### 9. Projeto não compila após atualização

#### Sintoma
```
Erro de sintaxe após atualizar o SDK
```

#### Causa
Mudanças na linguagem entre versões.

#### Soluções

**Solução 1: Verificar documentação**
Consulte a documentação da versão mais recente para mudanças na sintaxe.

**Solução 2: Limpar e recompilar**
```bash
pordosol clean
pordosol build --force
```

**Solução 3: Adaptar código**
Se houve mudanças na linguagem, adapte seu código conforme a nova sintaxe.

### 10. Problemas com templates

#### Sintoma
```
Erro ao criar projeto: template não encontrado
```

#### Causa
Templates não foram instalados corretamente.

#### Solução
```bash
# Verificar se templates existem
ls ~/.pordosol/templates  # Linux
dir %PORDOSOL_HOME%\templates  # Windows

# Reinstalar o SDK se necessário
```

## Problemas Específicos por Plataforma

### Windows

#### PowerShell Execution Policy
**Sintoma:**
```
execução de scripts desabilitada neste sistema
```

**Solução:**
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

#### Antivírus bloqueando executáveis
**Sintoma:**
Antivírus bloqueia `pordosol.exe` ou outros binários.

**Solução:**
- Adicione exceção para o diretório do Por do Sol no antivírus
- Ou desative temporariamente o antivírus durante a instalação

### Linux

#### Dependências faltando
**Sintoma:**
Erro ao executar binários devido a bibliotecas faltando.

**Solução:**
```bash
# Debian/Ubuntu
sudo apt-get update
sudo apt-get install build-essential

# Fedora
sudo dnf install build-essential
```

#### Problemas com perfil
**Sintoma:**
Comando `pordosol` funciona apenas no terminal atual.

**Solução:**
Verifique se adicionou ao arquivo de profile correto:
```bash
# Para bash
echo 'source ~/.bashrc' >> ~/.bash_profile

# Para zsh
echo 'source ~/.zshrc' >> ~/.zprofile
```

## Obtendo Ajuda Adicional

Se você não conseguiu resolver seu problema com este guia:

### 1. Verificar a documentação
- [INSTALACAO.md](INSTALACAO.md) - Guia de instalação
- [PRIMEIROS_PASSOS.md](PRIMEIROS_PASSOS.md) - Guia de primeiros passos
- [Documentação do Compilador](../compilador-portugues/docs/) - Documentação técnica

### 2. Executar diagnóstico detalhado
```bash
pordosol doctor --verbose
```

### 3. Verificar logs
Se houver logs de erro, eles podem estar em:
- Windows: `%TEMP%` ou pasta do projeto
- Linux: `/tmp` ou pasta do projeto

### 4. Reportar o problema
Abra uma issue no [GitHub](https://github.com/Adriano-Severino/Compilador/issues) com:
- Descrição detalhada do problema
- Passos para reproduzir
- Saída do comando `pordosol doctor`
- Seu sistema operacional e versão
- Versão do SDK (`pordosol --versao`)

### 5. Comunicar-se com a comunidade
- Verifique issues existentes no GitHub
- Participe de discussões no repositório

## Dicas Gerais

### Manter o SDK atualizado
```bash
pordosol update
```

### Limpar cache periodicamente
```bash
pordosol clean
```

### Verificar versões
```bash
pordosol --versao
```

### Usar caminhos absolutos em caso de problemas
```bash
pordosol build --project /caminho/completo/para/projeto
```

### Reinstalar como último recurso
Se nada mais funcionar, reinstale o SDK completamente seguindo o guia de instalação.

Esperamos que este guia ajude a resolver seus problemas! Se você encontrar um problema não documentado aqui, considere reportá-lo para ajudar a melhorar o SDK.