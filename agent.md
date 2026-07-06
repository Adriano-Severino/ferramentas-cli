# Agent.md - Ferramentas CLI

## Objetivo do Projeto

O projeto `ferramentas-cli` tem como objetivo fornecer uma ferramenta de linha de comando (CLI) para simplificar e automatizar o desenvolvimento de projetos na linguagem "Por do Sol". A CLI serve como o ponto de entrada principal para que desenvolvedores possam criar, construir, testar e gerenciar seus projetos de forma eficiente.

## Visão e Funcionalidades

A visão é criar uma experiência de desenvolvedor (DX) moderna e produtiva, similar a outras ferramentas de CLI como `dotnet CLI`, `cargo` ou `npm`. As funcionalidades planejadas incluem:

- **Criação de Projetos:** Comandos para gerar novos projetos a partir de templates pré-definidos (ex: console, biblioteca, projeto web).
- **Compilação e Execução:** Comandos para compilar o código-fonte `.pr` e executar os programas resultantes, abstraindo os detalhes dos diferentes backends do compilador (LLVM, .NET, bytecode).
- **Gerenciamento de Dependências:** No futuro, poderá incluir funcionalidades para gerenciar dependências de bibliotecas, incluindo o `sistema-padrao` ou outras bibliotecas da comunidade.
- **Testes:** Integração com um framework de testes para executar testes de unidade e integração.
- **Empacotamento:** Comandos para empacotar bibliotecas e aplicações para distribuição.

## Estado Atual

O projeto está em desenvolvimento. O foco atual é na implementação dos comandos básicos para criação e compilação de projetos simples.

## Relação com Outros Projetos

- **compilador-portugues:** A `ferramentas-cli` utiliza o `compilador-portugues` nos bastidores para realizar a compilação do código-fonte. Ela atua como uma fachada, simplificando a interação do usuário com o compilador.
- **sistema-padrao:** A CLI será responsável por garantir que novos projetos sejam criados com a referência correta ao `sistema-padrao`, facilitando o uso da biblioteca padrão desde o início.
- **net-por-do-sol:** Quando o usuário optar por compilar para .NET, a CLI invocará a lógica do `net-por-do-sol` para gerar o assembly CIL.
- **pordosol-language-server:** A CLI e o Language Server são as duas principais ferramentas que compõem a experiência de desenvolvimento da linguagem "Por do Sol".
