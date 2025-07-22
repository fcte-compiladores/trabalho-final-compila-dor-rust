[medio]

Lox suporta operadores lógicos do tipo `x and y` ou `x or y`. 
Use a [gramática do Lox](https://craftinginterpreters.com/appendix-i.html#expressions) 
como referência e implemente suporte a esses dois operadores na linguagem.

Uma observação importante sobre a semântica destes operadores. O `and` e
o `or` do Lox são operadores "curto-circuito". Isso significa que eles não
avaliam necessariamente ambos argumentos. 

No caso do `and`, por exemplo, avaliamos o primeiro argumento e, caso seja 
verdadeiro, avaliamos o segundo. Isso porque se o primeiro argumento de uma 
expressão and for falso, não tem porque avaliar o segundo já que o
resultado certamente é verdadeiro. 

O operador `or`, por outro lado, avalia o segundo argumento somente se o
primeiro for falso.