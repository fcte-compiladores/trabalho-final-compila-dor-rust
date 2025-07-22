[medio]

Lox aceita atribuições como expressões. Isso significa que, se houver uma 
variável declarada em um escopo, podemos modificar seu valor em qualquer 
lugar em uma expressão: pode ser dentro do argumento de uma função, em um 
dos operandos de um operador, etc. 

Desta forma, o programa abaixo é um Lox válido

```lox
var x = 2;
print sqrt(x = 4);  // imprime 2
print x;            // imprime 4
```

Veja como está definido na [gramática do Lox](https://craftinginterpreters.com/appendix-i.html#expressions). 
Note que a regra de "assignment" aceita expressões relativamente complexas
do lado esquerdo, como chamada de funções, valores literais, etc, desde que 
tenham um acesso a atributo no final. Vamos ignorar esses casos patológicos 
por enquanto.

O programa abaixo, por exemplo, é gramaticalmente correto, ainda que
falhe durante a execução

```lox
var x = 2;
print sqrt(x = 4);  // imprime 2
print x;            // imprime 4
```

Neste exercício, você deve implementar uma parte da regra de "assignment"
da gramática, ignorando a parte que lida com atribuição de atributos. 
Considere então que, ao invés de,

    assignment  → ( call "." )? IDENTIFIER "=" assignment
                | logic_or ;

vamos implementar 

    assignment  → IDENTIFIER "=" assignment
                | logic_or ;

Implemente a atribuição criando nós do tipo Assign na AST. Lembre-se que na 
implementação de eval(), vamos salvar o valor do lado direito da atribuição
no dicionário de contexto usando o identificador escolhido como chave.