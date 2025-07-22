[facil]

Em Lox (e em quase todas as linguagens que usam chaves para delimitar blocos),
um bloco de código define um novo escopo de execução. Como em 

```lox
var foo = 1;
{
    var bar = 2;
    print foo;
    print bar;
}
print foo;
print bar; 
```

No código acima, a variável `bar` deveria estar disponível apenas dentro das 
chaves. Deste modo, o segundo comando `print bar` deveria causar erro pois bar
não estaria disponível nessa linha.

O controle preciso do escopo de variáveis é um detalhe importante e sutil de
um interpretador. Nesse exercícios nos **não vamos** nos preocupar com isso: é 
oficialmente um problema pro futuro! Vamos tratar o bloco simplesmente como uma 
sintaxe de agrupar comandos. 

Acrescente a regra de "block" ao conjunto de possíveis "stmt". Para eval, 
repetimos a mesma estratégia de `Program`: avaliamos cada declaração na lista
de declarações e em seguida retornamos None.

Recomendo a discussão sobre [escopo](https://craftinginterpreters.com/statements-and-state.html#scope) 
no Crafting Interpreters para entender melhor a semântica de blocos e o motivo
para eles existirem, ainda que nesse exercício vamos fazer apenas uma implementação 
parcial. A seção [Block Syntax e Semantics](https://craftinginterpreters.com/statements-and-state.html#block-syntax-and-semantics) 
é especialmente útil.