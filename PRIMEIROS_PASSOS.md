# Primeiros Passos com Por do Sol

Parabéns por instalar o Por do Sol SDK! Este guia vai ajudá-lo a criar e executar seu primeiro projeto na linguagem Por do Sol.

## Verificação da Instalação

Antes de começar, certifique-se de que o SDK está instalado corretamente:

```bash
pordosol doctor
```

Você deve ver uma mensagem indicando que o ambiente está pronto para uso.

## Criando Seu Primeiro Projeto

### 1. Criar um Projeto Console

Vamos criar um projeto console simples que imprime uma mensagem na tela:

```bash
pordosol new console MeuPrimeiroProjeto
```

Este comando cria uma nova pasta chamada `MeuPrimeiroProjeto` com a seguinte estrutura:

```
MeuPrimeiroProjeto/
├── src/
│   └── programa.pr          # Seu código fonte
├── pordosol.proj            # Arquivo de configuração do projeto
└── README.md                # Documentação do projeto
```

### 2. Entendendo a Estrutura do Projeto

**Arquivo `programa.pr`:**
Este é o arquivo principal do seu projeto. Vamos dar uma olhada:

```pr
// Namespace: MeuPrimeiroProjeto
função vazio Principal()
{
    imprima("Ola, MeuPrimeiroProjeto!");
    imprima("Seu projeto console ja esta pronto para evoluir.");
}
```

**Arquivo `pordosol.proj`:**
Este arquivo contém metadados e configurações do projeto em formato JSON:

```json
{
    "nome": "MeuPrimeiroProjeto",
    "tipo": "console",
    "versao": "1.0.0",
    "descricao": "Aplicacao console em Por do Sol",
    "autor": "",
    "dependencias": {},
    "configuracao": {
        "target_padrao": "bytecode",
        "otimizacao": false
    }
}
```

### 3. Modificando Seu Programa

Vamos modificar o programa para algo mais interessante. Abra o arquivo `src/programa.pr` e altere para:

```pr
// Namespace: MeuPrimeiroProjeto
função vazio Principal()
{
    imprima("=================================");
    imprima("   Bem-vindo ao Por do Sol!");
    imprima("=================================");
    imprima("");
    imprima("Este e seu primeiro projeto.");
    imprima("A linguagem Por do Sol e moderna e produtiva.");
    imprima("");
    imprima("Vamos programar em portugues!");
}
```

### 4. Compilando o Projeto

Navegue até a pasta do projeto e compile:

```bash
cd MeuPrimeiroProjeto
pordosol build
```

Este comando:
- Compila seu código fonte `.pr`
- Gera o bytecode correspondente `.pbc`
- Cria os arquivos na pasta `build/`

Você deve ver uma saída similar a:

```
Compilando MeuPrimeiroProjeto...
✓ programa.pr -> build/programa.pbc
Compilação concluída com sucesso.
```

### 5. Executando o Projeto

Agora vamos executar o programa compilado:

```bash
pordosol run
```

Você deve ver a saída:

```
=================================
   Bem-vindo ao Por do Sol!
=================================

Este e seu primeiro projeto.
A linguagem Por do Sol e moderna e produtiva.

Vamos programar em portugues!
```

## Comandos Básicos da CLI

### `pordosol new`
Cria um novo projeto a partir de um template.

```bash
pordosol new console NomeDoProjeto    # Cria projeto console
pordosol new web NomeDoProjeto        # Cria projeto web
pordosol new list                     # Lista templates disponíveis
```

### `pordosol build`
Compila o projeto atual.

```bash
pordosol build                         # Compila para bytecode (padrão)
pordosol build --target llvm-ir       # Compila para LLVM IR
pordosol build --target cil-bytecode  # Compila para CIL (.NET)
```

### `pordosol run`
Compila e executa o projeto.

```bash
pordosol run                           # Compila e executa
pordosol run --no-build               # Executa sem recompilar
pordosol run --force                  # Força recompilação
```

### `pordosol clean`
Limpa os arquivos de build.

```bash
pordosol clean
```

### `pordosol info`
Mostra informações sobre o projeto atual.

```bash
pordosol info
```

### `pordosol doctor`
Diagnostica o ambiente de desenvolvimento.

```bash
pordosol doctor
```

## Explorando a Linguagem

### Variáveis e Tipos

```pr
função vazio Principal()
{
    inteiro idade = 25;
    texto nome = "Maria";
    booleano ativo = verdadeiro;
    flutuante altura = 1.75;
    
    imprima("Nome: " + nome);
    imprima("Idade: " + idade);
    imprima("Altura: " + altura);
}
```

### Condicionais

```pr
função vazio Principal()
{
    inteiro idade = 18;
    
    se (idade >= 18) {
        imprima("Voce e maior de idade.");
    } senao {
        imprima("Voce e menor de idade.");
    }
}
```

### Loops

```pr
função vazio Principal()
{
    para (inteiro i = 0; i < 5; i = i + 1) {
        imprima("Contagem: " + i);
    }
    
    inteiro contador = 0;
    enquanto (contador < 3) {
        imprima("Enquanto: " + contador);
        contador = contador + 1;
    }
}
```

### Funções

```pr
função inteiro Somar(inteiro a, inteiro b)
{
    retorne a + b;
}

função vazio Principal()
{
    inteiro resultado = Somar(5, 3);
    imprima("Resultado: " + resultado);
}
```

## Usando a Biblioteca Padrão

A biblioteca padrão do Por do Sol (`sistema-padrao`) fornece funcionalidades comuns. Para usá-la, adicione a diretiva `usando` no topo do seu arquivo:

```pr
usando Sistema.IO;

função vazio Principal()
{
    // Exemplo usando a biblioteca padrão (quando disponível)
    imprima("Usando a biblioteca padrao");
}
```

## Próximos Passos

Agora que você criou seu primeiro projeto, você pode:

1. **Explorar mais recursos da linguagem:**
   - Classes e orientação a objetos
   - Interfaces
   - Herança
   - Enumerações
   - Genéricos

2. **Consultar a documentação completa:**
   - [Documentação da Linguagem](../compilador-portugues/docs/)
   - [Exemplos de Código](../compilador-portugues/exemplos/)

3. **Criar projetos mais complexos:**
   - Experimente o template web: `pordosol new web MeuProjetoWeb`
   - Adicione múltiplos arquivos `.pr` ao seu projeto
   - Organize seu código em namespaces

4. **Contribuir com o projeto:**
   - Reporte bugs no [GitHub Issues](https://github.com/Adriano-Severino/Compilador/issues)
   - Contribua com melhorias e novos recursos

## Recursos Adicionais

- **GitHub Repository:** [https://github.com/Adriano-Severino/Compilador](https://github.com/Adriano-Severino/Compilador)
- **Documentação de Instalação:** [INSTALACAO.md](INSTALACAO.md)
- **Solução de Problemas:** [SOLUCAO_PROBLEMAS.md](SOLUCAO_PROBLEMAS.md)

## Exemplo Completo

Aqui está um exemplo mais completo que combina vários conceitos:

```pr
usando Sistema;

função vazio ImprimirMensagem(texto mensagem)
{
    imprima("Mensagem: " + mensagem);
}

função inteiro CalcularFatorial(inteiro n)
{
    se (n <= 1) {
        retorne 1;
    }
    retorne n * CalcularFatorial(n - 1);
}

função vazio Principal()
{
    imprima("=== Programa de Exemplo ===");
    imprima("");
    
    // Variáveis
    inteiro numero = 5;
    texto nome = "Programador";
    
    // Chamada de função
    ImprimirMensagem("Ola, " + nome + "!");
    
    // Cálculo
    inteiro fatorial = CalcularFatorial(numero);
    imprima("Fatorial de " + numero + " e: " + fatorial);
    
    // Loop
    imprima("");
    imprima("Contagem regressiva:");
    para (inteiro i = 10; i >= 1; i = i - 1) {
        imprima(i);
    }
    imprima("Lancar!");
    
    // Condicional
    imprima("");
    se (fatorial > 100) {
        imprima("O fatorial e maior que 100");
    } senao {
        imprima("O fatorial e menor ou igual a 100");
    }
    
    imprima("");
    imprima("=== Fim do Programa ===");
}
```

Compile e execute este exemplo para ver todos os conceitos em ação!

Boa programação em Por do Sol! 🚀