use mini_lox_rust::executar;

#[test]
fn test_soma_simples() {
    let resultado = executar("2 + 3");
    assert_eq!(resultado, 5.0);
}

#[test]
fn test_operacoes_aritmeticas() {
    assert_eq!(executar("5 * (2 + 3)"), 25.0);
    assert_eq!(executar("10 / 2 - 1"), 4.0);
}

#[test]
fn test_numeros_validos() {
    assert_eq!(executar("1"), 1.0);
    assert_eq!(executar("2.72"), 2.72);
    assert_eq!(executar("3"), 3.0);
    assert_eq!(executar("42"), 42.0);
    assert_eq!(executar("5"), 5.0);
}

#[test]
fn test_operadores_unarios() {
    assert_eq!(executar("-5"), -5.0);
    assert_eq!(executar("!true"), 0.0); // !true -> false -> 0.0
    assert_eq!(executar("!false"), 1.0); // !false -> true -> 1.0
    assert_eq!(executar("!!true"), 1.0);
}

#[test]
fn test_operadores_logicos() {
    assert_eq!(executar("true and false"), 0.0);
    assert_eq!(executar("true or false"), 1.0);
    assert_eq!(executar("false or true"), 1.0);
    assert_eq!(executar("false and true"), 0.0);
}

#[test]
fn test_atribuicao_de_variaveis() {
    let codigo = "var x = 2\nx = 4\nx";
    assert_eq!(executar(codigo), 4.0);
}

#[test]
fn test_declaracao_de_variaveis() {
    let codigo_com_valor = "var a = 10\na";
    assert_eq!(executar(codigo_com_valor), 10.0);

    let codigo_multiplo = "var c = 1\nvar d = c + 2\nd";
    assert_eq!(executar(codigo_multiplo), 3.0);
}

#[test]
fn test_blocos() {
    let codigo = "{ var foo = 1; var bar = 2; bar }";
    assert_eq!(executar(codigo), 2.0);

    let codigo_fora = "{ var a = 1; } a";
    assert_eq!(executar(codigo_fora), 1.0);
}

#[test]
fn test_if_else() {
    let codigo_if = "if (true) 10";
    assert_eq!(executar(codigo_if), 10.0);

    let codigo_if_false = "if (false) 10";
    assert_eq!(executar(codigo_if_false), 0.0);

    let codigo_if_else_true = "if (1 < 2) 3 else 4";
    assert_eq!(executar(codigo_if_else_true), 3.0);

    let codigo_if_else_false = "if (1 > 2) 3 else 4";
    assert_eq!(executar(codigo_if_else_false), 4.0);
}

#[test]
fn test_while() {
    let codigo = "var i = 0\nwhile (i < 5) { i = i + 1; }\ni";
    assert_eq!(executar(codigo), 5.0);
} 