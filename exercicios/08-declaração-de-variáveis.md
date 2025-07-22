[medio]

Em Lox, devemos declarar variáveis antes do primeiro uso. Isso é feito com 
a sintaxe `var variavel = "valor";`, onde a expressão do lado direito da 
atribuição é opcional. Essa sintaxe está descrita como `varDecl` na [gramática do Lox](https://craftinginterpreters.com/appendix-i.html#expressions). 

Nesse exercício, vamos implementar essa funcionalidade no nosso interpretador.

Antes de começar nossa implementação, no entanto, recomendo ler a seção sobre
[variáveis globais](https://craftinginterpreters.com/statements-and-state.html#global-variables)
no Crafting Interpreters (e talvez a seção anterior sobre comandos/statements).

O livro aponta que precisamos modificar a nossa gramática para criar uma
hierarquia entre "declarações" de "comandos". Isso porque alguns comandos não 
podem ser incluídos como o corpo de um "if" ou um laço como "for" ou "while".

A nossa gramática possui uma declaração

    program : stmt*

Mas note que na gramática de Lox mudamos para

    program → declaration* EOF ;

E posteriormente
    
    declaration → ... | statement ;

O terminal EOF não precisa ser declarado no Lark, já que ele o inclui 
implicitamente na regra "start". Vemos portanto que um programa deve ser uma 
série de declarações e uma declaração pode posteriormente virar um `stmt`.

Reorganize a gramática com essa nova estrutura e em seguida inclua a regra de 
declaração de variáveis entre as possíveis declarações de acordo como está 
especificado na gramática.

Em todo caso, primeiro implemente a regra na gramática onde o valor inicial do 
lado direito é obrigatório. Você deve criar nós do tipo `VarDef` onde guardamos
o nome da variável declarada e o valor inicial no lado direito da mesma.

O método eval para `VarDef`'s deve seguir uma implementação similar à de
`Assign`: avaliamos a expressão que declara o valor do lado direito e salvamos
este valor no contexto de execução usando o nome da variável como chave.

Finalmente, para completar a implementação, devemos aceitar declarações sem
valor inicial, como em `var variavel;`. Lox trata estas declarações como se o 
usuário estivesse definido implicitamente `var variavel = nil;`. Podemos usar o 
`LoxTansformer` para tratar as declarações do primeiro tipo como se fossem do 
segundo: se o usuário não definiu o valor do lado direito, podemos criar um 
nó `VarDef` passando um objeto `Literal(None)` que seria criado se o usuário 
tivesse inicializado a variável explicitamente como `nil`. 

