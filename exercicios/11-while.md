[facil]

O [laço while](https://craftinginterpreters.com/control-flow.html#while-loops)
em Lox funciona como em quase todas linguagens de programação. Inclua o suporte
ao while como mostrado na gramática e depois implemente o método eval
correspondente.

Na implementação do método `While.eval` usamos o fato que a semância do `while`
é a mesma em Python e em Lox. Por isso, podemos implementar o `while` do Lox
usando um laço `while` em Python que avalia o corpo do laço toda vez que a
condição avaliar como verdadeira.
