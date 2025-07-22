
# mini_lox_rust

Repositório original: [https://gitlab.com/siqueira-prog/mini-lox-rust](https://gitlab.com/siqueira-prog/mini-lox-rust)

Este projeto é um interpretador simples inspirado na linguagem Lox, escrito em
Rust. Ele lê expressões linha a linha do arquivo `leiaesse.crl`, constrói a
árvore sintática, avalia e imprime o resultado de cada expressão.

## Referências da Documentação Rust

- [`std::collections::HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html):
  Usado para armazenar variáveis e contexto de execução.
- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse):
  Conversão de string para número (`f64`, `i64`).
- [`f64`](https://doc.rust-lang.org/std/primitive.f64.html): Tipo de ponto
  flutuante usado para números.
- [`i64`](https://doc.rust-lang.org/std/primitive.i64.html): Tipo inteiro usado
  para números inteiros (se implementado).
- [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html): Usado para
  alocação dinâmica de nós na árvore sintática.
- [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html): Usado para
  valores opcionais (ex: ramo else, retorno de funções).
- [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html): Usado em
  parsing e manipulação de erros.

## Estrutura dos Arquivos

- **src/main.rs**: Ponto de entrada do programa. Lê o arquivo `leiaesse.crl`,
  mostra o conteúdo, e para cada linha:
  - Executa a análise léxica (tokenização)
  - Executa a análise sintática (gera a árvore sintática)
  - Avalia a expressão e imprime o resultado

- **src/analisador_lexico.rs**: Responsável por transformar o texto de entrada
  em uma lista de símbolos (tokens). Reconhece números, strings,
  identificadores, operadores, palavras-chave (`var`, `if`, `else`, `while`),
  delimitadores, etc. Se encontrar um caractere inválido, gera um erro.

- **src/analisador_sintatico.rs**: Implementa o parser recursivo descendente.
  Constrói a árvore sintática abstrata (AST) a partir dos símbolos gerados pelo
  léxico. Suporta blocos, expressões, atribuições, controle de fluxo (`if`,
  `while`), operações aritméticas e lógicas, etc.

- **src/arvore_sintatica_abstrata.rs**: Define a enumeração `Expr` que
  representa os diferentes tipos de nós da AST (números, strings, operações,
  blocos, if, while, etc). Implementa métodos para:
  - Avaliar a expressão (com contexto de variáveis)
  - Imprimir a árvore sintática de forma hierárquica

- **texto.crl**: Arquivo de entrada com exemplos de expressões e comandos para o
  interpretador executar.

- **exercicios/**: Pasta com exercícios e desafios para expandir ou testar o
  interpretador.

## Como Executar

1. Instale o Rust (https://rustup.rs/)
2. No terminal, execute:
   ```sh
   cargo run
   ```
3. O programa irá ler o arquivo `leiaesse.crl` e mostrar o resultado de cada
   linha.

## Sobre Recursão no Interpretador

O parser (`analisador_sintatico.rs`) utiliza **recursão** para analisar
expressões aninhadas e construir a árvore sintática. Por exemplo, ao analisar
uma expressão como `1 + 2 * (3 - 4)`, o parser chama funções recursivamente para
lidar com precedência de operadores e agrupamentos.

A avaliação das expressões (`arvore_sintatica_abstrata.rs`) também é feita de
forma recursiva: cada nó da árvore chama a avaliação de seus filhos, permitindo
calcular resultados de expressões complexas, blocos, condicionais e laços.

A recursão é fundamental para:

- Lidar com expressões aninhadas de profundidade arbitrária
- Implementar precedência e associatividade de operadores
- Avaliar blocos e estruturas de controle de fluxo

## Observações

- O interpretador ainda **não suporta** o operador `%` (módulo). Para adicionar,
  é necessário modificar o analisador léxico, sintático e a AST.
- Funções definidas pelo usuário ainda não são suportadas, apenas chamadas de
  funções nativas simuladas.

---

## Desafio: Calculando o GCD (Máximo Divisor Comum)

O objetivo deste projeto é, como exercício, calcular o GCD (Greatest Common
Divisor, ou Máximo Divisor Comum) entre dois números. Existem dois níveis de
desafio:

1. **Usando apenas while:**
   - Implemente o algoritmo de Euclides para GCD utilizando apenas variáveis,
     atribuições e o laço `while`.
   - Exemplo de lógica:
     ```
     var a = 48;
     var b = 18;
     while (b != 0) {
         var temp = b;
         b = a % b;
         a = temp;
     }
     a
     ```
   - _Obs: O interpretador precisa suportar o operador `%` para funcionar
     exatamente assim._

2. **Versão refinada (extra):**
   - Após implementar o suporte a funções definidas pelo usuário, escreva uma
     função `gcd(x, y)` e utilize-a para calcular o resultado de forma mais
     elegante:
     ```
     fun gcd(a, b) {
         while (b != 0) {
             var temp = b;
             b = a % b;
             a = temp;
         }
         return a;
     }
     gcd(48, 18)
     ```
   - _Este passo requer implementar suporte a funções no interpretador._

O desafio é uma ótima forma de praticar análise léxica, sintática, recursão e
controle de fluxo!

Sinta-se à vontade para modificar, expandir e experimentar com o interpretador!
