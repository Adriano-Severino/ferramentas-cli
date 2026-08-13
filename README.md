# Por do Sol SDK - Ferramenta CLI

Ferramenta de linha de comando para criação e gerenciamento de projetos na linguagem de programação "Por do Sol" - uma linguagem moderna escrita em português brasileiro.

## 📖 Sobre a Linguagem

A linguagem "Por do Sol" foi desenvolvida com foco acadêmico e educacional, visando democratizar o ensino de programação no Brasil através de uma sintaxe em português. Ela também é projetada para ser versátil o suficiente para desenvolvimento de aplicações com alta performance, graças à geração de código LLVM.

### 🎯 Objetivo

A ferramenta CLI (`pordosol`) serve como o ponto de entrada principal para desenvolvedores criarem, construírem, testarem e gerenciarem seus projetos Por do Sol de forma eficiente, similar ao `dotnet CLI`, `cargo` ou `npm`.

### 🚀 Recursos Principais

- **Criação de Projetos:** Gera novos projetos a partir de templates (console, web, biblioteca)
- **Compilação e Execução:** Compila código `.pr` e executa programas com múltiplos backends (LLVM, .NET, bytecode)
- **Gerenciamento de Dependências:** Sistema para gerenciar bibliotecas, incluindo o `sistema-padrao`
- **Diagnostics:** Ferramenta `doctor` para diagnosticar problemas de ambiente
- **Templates:** Templates pré-definidos para diferentes tipos de projetos

## 📋 Pré-requisitos

### Para Usuários Finais (Versão Beta)

- **Windows:** Windows 10 ou posterior (64-bit)
- **Linux:** Qualquer distribuição moderna (64-bit)
- **Espaço em disco:** 100MB
- **RAM:** 512MB

### Para Desenvolvedores (Modo Desenvolvimento)

- **Rust (versão 1.70+):** Necessário para construir o compilador
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
- **LLVM 16:** Para compilação LLVM (opcional)

## ⚙️ Instalação

### Instalação via Pacote (Recomendado para Usuários)

A versão beta inclui binários pré-compilados - você não precisa do Rust ou do código-fonte!

#### Windows

**Opção 1: Instalador MSI**
1. Baixe `pordosol-sdk-v0.1.0-windows-x64.msi` dos [releases](https://github.com/Adriano-Severino/Compilador/releases)
2. Execute o instalador e siga o assistente

**Opção 2: Script PowerShell**
1. Baixe e extraia `pordosol-sdk-v0.1.0-windows-x64.zip`
2. Execute: `.\install.ps1`

#### Linux

1. Baixe `pordosol-sdk-v0.1.0-linux-x64.tar.gz` dos [releases](https://github.com/Adriano-Severino/Compilador/releases)
2. Extraia: `tar -xzf pordosol-sdk-v0.1.0-linux-x64.tar.gz`
3. Execute: `./install.sh`

Para instruções detalhadas, consulte [INSTALACAO.md](INSTALACAO.md).

### Instalação via Código-Fonte (Desenvolvedores)

Se você deseja contribuir ou modificar a ferramenta:

```bash
# Clone o repositório
git clone https://github.com/Adriano-Severino/Compilador.git
cd Compilador/ferramentas-cli

# Execute o script de instalação em modo desenvolvimento
.\install.ps1  # Windows
./install.sh    # Linux
```

## 📝 Como Usar

### Verificar Instalação

```bash
pordosol doctor
```

### Criar um Novo Projeto

```bash
# Criar projeto console
pordosol new console MeuProjeto

# Criar projeto web
pordosol new web MeuProjetoWeb

# Listar templates disponíveis
pordosol new list
```

### Compilar e Executar

```bash
cd MeuProjeto

# Compilar
pordosol build

# Executar
pordosol run

# Compilar e executar em um comando
pordosol run --force
```

### Outros Comandos Úteis

```bash
pordosol info                    # Informações do projeto
pordosol clean                   # Limpar artefatos de build
pordosol list                    # Listar arquivos .pr
pordosol dep list                # Listar dependências
pordosol --versao               # Mostrar versão
```

## � Primeiros Passos

Após a instalação, siga o guia [PRIMEIROS_PASSOS.md](PRIMEIROS_PASSOS.md) para:
- Criar seu primeiro projeto
- Entender a estrutura de projetos
- Aprender os comandos básicos
- Explorar recursos da linguagem

## 🏗️ Estrutura do Projeto

```
ferramentas-cli/
├── src/                        # Código fonte da CLI
│   ├── main.rs                 # Ponto de entrada
│   ├── novo.rs                 # Comando new
│   ├── construir.rs            # Comando build
│   ├── executar.rs             # Comando run
│   └── toolchain.rs            # Detecção de ferramentas
├── templates/                  # Templates de projeto
│   ├── console/                # Template console
│   └── web/                    # Template web
├── scripts/                    # Scripts de build e empacotamento
│   ├── build-package.ps1       # Build pacote Windows
│   ├── build-package.sh        # Build pacote Linux
│   ├── build-installer.ps1     # Build instalador MSI
│   └── wix-config.xml          # Configuração WiX
├── install.ps1                 # Instalador Windows
├── install.sh                  # Instalador Linux
├── INSTALACAO.md               # Guia de instalação
├── PRIMEIROS_PASSOS.md         # Guia de primeiros passos
├── SOLUCAO_PROBLEMAS.md        # Solução de problemas
└── package-config.json          # Configuração do pacote
```

## 🧩 Extensões e Ferramentas para VS Code

- [Servidor de Linguagem Por do Sol (LSP)](https://github.com/Adriano-Severino/pordosol-language-server)
- [Extensão oficial VS Code: linguagem-portugues-por-do-sol](https://github.com/Adriano-Severino/linguagem-portugues-por-do-sol)

Essas extensões fornecem realce de sintaxe, auto-complete, diagnósticos e integração moderna para desenvolvimento com a linguagem Por do Sol no VS Code.

## 📚 Documentação Adicional

- [INSTALACAO.md](INSTALACAO.md) - Guia completo de instalação
- [PRIMEIROS_PASSOS.md](PRIMEIROS_PASSOS.md) - Tutorial de primeiros passos
- [SOLUCAO_PROBLEMAS.md](SOLUCAO_PROBLEMAS.md) - Solução de problemas comuns
- [Documentação do Compilador](../compilador-portugues/docs/) - Documentação técnica da linguagem
- [Exemplos de Código](../compilador-portugues/exemplos/) - Exemplos práticos

## 🤝 Contribuindo

Contribuições são muito bem-vindas! Para contribuir:

1. Faça um fork do repositório
2. Clone sua fork:
    ```bash
    git clone https://github.com/SeuUsuario/Compilador.git
    ```
3. Crie uma branch para sua feature:
    ```bash
    git checkout -b minha-nova-feature
    ```
4. Faça suas mudanças e adicione testes, se aplicável
5. Faça um commit das suas mudanças:
    ```bash
    git commit -m "Adiciona nova feature incrível"
    ```
6. Faça um push para sua fork:
    ```bash
    git push origin minha-nova-feature
    ```
7. Abra um Pull Request no repositório original

## Diretrizes de Contribuição

- Mantenha a sintaxe da linguagem e dos comentários em português brasileiro
- Adicione testes para novas funcionalidades ou correções de bugs
- Documente quaisquer mudanças significativas
- Siga o estilo de código existente
- Use boas práticas de engenharia de software

## 🐛 Reportando Problemas

Encontrou um bug ou tem alguma sugestão? Abra uma Issue [neste link](https://github.com/Adriano-Severino/Compilador/issues) com:

1. Descrição detalhada do problema ou sugestão
2. Passos para reproduzir o erro (se for um bug)
3. Informações do seu ambiente (sistema operacional, versão do SDK)
4. Saída do comando `pordosol doctor`
5. Se possível, um exemplo de código que reproduz o problema

## 📝 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo LICENSE para detalhes.

## Agradecimentos

- À comunidade Rust por suas ferramentas e ecossistema incríveis
- Ao projeto LLVM por fornecer uma infraestrutura de compilação robusta e poderosa
- Aos educadores e estudantes brasileiros que inspiram e podem se beneficiar deste projeto

⭐ Se este projeto foi útil, deixe uma estrela!

🌟 Ajude a democratizar a programação em português!