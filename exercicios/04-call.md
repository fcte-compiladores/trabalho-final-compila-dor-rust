[medio]

Nossa gramática implementa chamada de funções de zero a muitos argumentos usando
as seguintes regras:

    call       : VAR "(" params ")"
    params     : [ expr ("," expr )* ]

Isso permite escrever expressões lox como `clock()`, `sqrt(9)`, `f(x, y)`, etc.
Apesar de suportar estes casos válidos, a regra acima não suporta uma variante importante: métodos e funções que retornam funções. 

Por exemplo em `obj.method()`, quem cumpre o papel do objeto a ser chamado é a 
expressão `obj.method` e não um terminal `VAR`. De forma similar, uma função pode retornar outras funções e podemos querer executá-la imediatamente. Como`em 
`create_incrementer(2)(40)`. Neste caso, `create_incrementer(2)` cumpre o papel
do objeto a ser chamado, que também não é um VAR.

Conserte a gramática e possivelmente a implementação do método eval dos nós de
`Call` para suportar estas novas funcionalidades.

DICA: Talvez seja necessário mudar a declaração de átomos. A gramática oficial do
[Lox](https://craftinginterpreters.com/appendix-i.html#expressions) mostra uma 
maneira de fazer isto.