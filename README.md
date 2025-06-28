# Meu Compilador - Por do sol

Este é o repositório do projeto "ferramenta-cli", ferramenta de linha de comando para criação de projetos com para a linguagem de programação "Por do Sol" uma linguagem de programação moderna escrita em português brasileiro.

## 📖 Sobre a Linguagem

Esta linguagem "por do sol" foi desenvolvida com foco acadêmico e educacional, visando democratizar o ensino de programação no Brasil através de uma sintaxe em português. No entanto, ela também é projetada para ser versátil o suficiente para desenvolvimento de aplicações desktop nativas com alta performance, graças à geração de código LLVM.

### 🎯 Objetivo Principal



### 🚀 Recursos Principais



## 📋 Pré-requisitos

Antes de começar, certifique-se de que você tem os seguintes softwares instalados:

- **Rust (versão 1.70+):** Necessário para construir o compilador.
    - Para instalar o Rust, use o `rustup`:
      ```bash
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
- **LLVM 16:** A linguagem depende especificamente desta versão.
    - Para Ubuntu/Debian:
      ```bash
      
      ```
- 
## ⚙️ Instalação e Configuração

1. **Clone o repositório:**
    ```bash
    git clone https://github.com/Adriano-Severino/sistema-padrao.git
    cd sistema-padrao
  ```
3. **Construa o compilador:**
    ```bash
    cargo build --release
    ```
    Isso criará um executável chamado `sistema-padrao` na pasta `target/release`.

## 📝 Como Usar



### Estrutura Básica de um Programa



## Passos Manuais de compilação: (para Entender o Processo)
S


## 📚 Referência da Linguagem


## 💡 Exemplos de Código

## 🧩 Extensões e Ferramentas para VS Code

- [Servidor de Linguagem Por do Sol (LSP)](https://github.com/Adriano-Severino/pordosol-language-server)
- [Extensão oficial VS Code: linguagem-portugues-por-do-sol](https://github.com/Adriano-Severino/linguagem-portugues-por-do-sol)

Essas extensões fornecem realce de sintaxe, auto-complete, diagnósticos e integração moderna para desenvolvimento com a linguagem Por do Sol no VS Code.

## 🏗️ Estrutura do Projeto

```text
net-por-do-sol/
├── src/
│   ├── nucleo/               # Core do framework
│   │   ├── aplicacao.pr      # Similar a WebApplication
│   │   ├── servidor.pr       # HTTP Server
│   │   ├── middleware.pr     # Pipeline de middleware
│   │   └── rota.pr          # Sistema de rotas
│   ├── controladores/        # Controllers
│   │   ├── controlador_base.pr
│   │   └── api_controlador.pr
│   ├── http/                # HTTP abstractions
│   │   ├── requisicao.pr    # HttpRequest
│   │   ├── resposta.pr      # HttpResponse
│   │   └── contexto.pr      # HttpContext
│   └── dependencias/        # Dependency Injection
│       ├── container.pr
│       └── servicos.pr

```

## 🤝 Contribuindo

Contribuições são muito bem-vindas! Para contribuir:

1. Faça um fork do repositório
2. Clone sua fork:
    ```bash
    git clone https://github.com/Adriano-Severino/net-por-do-sol.git
    ```
3. Crie uma branch para sua feature:
    ```bash
    git checkout -b minha-nova-feature
    ```
4. Faça suas mudanças e adicione testes, se aplicável.
5. Faça um commit das suas mudanças:
    ```bash
    git commit -m "Adiciona nova feature incrível"
    ```
6. Faça um push para sua fork:
    ```bash
    git push origin minha-nova-feature
    ```
7. Abra um Pull Request no repositório original.

## Diretrizes para Contribuição

- Mantenha a sintaxe da linguagem e dos comentários em português brasileiro.
- Adicione testes para novas funcionalidades ou correções de bugs.
- Documente quaisquer mudanças significativas no README.md ou em comentários no código.
- Siga o estilo de código existente.

## 🐛 Reportando Problemas

Encontrou um bug ou tem alguma sugestão? Abra uma Issue [neste link](https://github.com/Adriano-Severino/net-por-do-sol) com:

1. Descrição detalhada do problema ou sugestão.
2. Passos para reproduzir o erro (se for um bug).
3. Informações do seu ambiente de desenvolvimento (versão do Rust, sistema operacional, etc).
4. Se possível, forneça um exemplo de código que reproduz o problema.
5. Saída esperada vs. saída atual.

## 📝 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo LICENSE para detalhes.

## Agradecimentos

- À comunidade Rust por suas ferramentas e ecossistema incríveis.
- Ao projeto LLVM por fornecer uma infraestrutura de compilação robusta e poderosa.
- Aos educadores e estudantes brasileiros que inspiram e podem se beneficiar deste projeto.

⭐ Se este projeto foi útil, deixe uma estrela!

🌟 Ajude a democratizar a programação em português!
