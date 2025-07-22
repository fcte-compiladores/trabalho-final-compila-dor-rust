[facil]

Nossa gramática ainda não suporta acesso a atributos. Modifique a gramática para
que seja possível acessar atributos usando a notação `obj.atributo.sub_atributo` 

Modifique a classe lox.ast.Gettattr com os parâmetros adequados e implemente o
seu método eval. Lembre-se que em Python é possível selecionar um atributo 
dinamicamente a partir do valor de uma string usando a função getattr.

```python
method = "lower"  # ou talvez upper, ou title, etc.
some_string = "hello world"
func = getattr(some_string, method)
result = func()
```

Usa o acesso de atributo do Python na implementação do método eval.