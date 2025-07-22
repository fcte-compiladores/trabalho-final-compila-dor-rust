
#[derive(Debug, Clone, PartialEq)]
pub enum Simbolo {
    NumeroInteiro(i64),
    NumeroFloat(f64),
    String(String),
    Identificador(String),
    Ponto,
    AbreParenteses,
    FechaParenteses,
    AbreChaves,        // {
    FechaChaves,       // }
    Virgula,
    PontoVirgula,      // ;
    Negacao,           // !
    Soma,
    Subtracao,         // - (pode ser unário ou binário)
    Multiplicacao,
    Divisao,
    Modulo,            // %
    And,
    Or,
    Igual,             // =
    Maior,             // >
    Menor,             // <
    MaiorIgual,        // >=
    MenorIgual,        // <=
    IgualIgual,        // ==
    Diferente,         // !=
    Var,               // var
    If,                // if
    Else,              // else
    While,             // while
    Print,             // print
    Fim,
}

pub fn analisar(texto: &str) -> Vec<Simbolo> {
    let mut simbolos = Vec::new();
    let mut chars = texto.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '"' => {
                chars.next(); // consome a aspa inicial
                let mut string = String::new();
                while let Some(&d) = chars.peek() {
                    match d {
                        '"' => {
                            chars.next(); // consome a aspa final
                            break;
                        }
                        '\\' => {
                            chars.next(); // consome barra
                            if let Some(&esc) = chars.peek() {
                                let escaped = match esc {
                                    '"' => '"',
                                    '\\' => '\\',
                                    'n' => '\n',
                                    'r' => '\r',
                                    't' => '\t',
                                    _ => esc,
                                };
                                string.push(escaped);
                                chars.next();
                            }
                        }
                        _ => {
                            string.push(d);
                            chars.next();
                        }
                    }
                }
                simbolos.push(Simbolo::String(string));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identificador = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_alphanumeric() || d == '_' {
                        identificador.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match identificador.as_str() {
                    "and" => simbolos.push(Simbolo::And),
                    "or" => simbolos.push(Simbolo::Or),
                    "var" => simbolos.push(Simbolo::Var),
                    "if" => simbolos.push(Simbolo::If),
                    "else" => simbolos.push(Simbolo::Else),
                    "while" => simbolos.push(Simbolo::While),
                    "print" => simbolos.push(Simbolo::Print),
                    _ => simbolos.push(Simbolo::Identificador(identificador)),
                }
            }
            '.' => {
                chars.next();
                simbolos.push(Simbolo::Ponto);
            }
            '0'..='9' => {
                let mut numero_str = String::new();
                let mut is_float = false;

                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        numero_str.push(d);
                        chars.next();
                    } else if d == '.' && !is_float {
                        is_float = true;
                        numero_str.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if is_float {
                    simbolos.push(Simbolo::NumeroFloat(numero_str.parse().unwrap()));
                } else {
                    simbolos.push(Simbolo::NumeroInteiro(numero_str.parse().unwrap()));
                }
            }
            '+' => {
                chars.next();
                simbolos.push(Simbolo::Soma);
            }
            '-' => {
                chars.next();
                simbolos.push(Simbolo::Subtracao);
            }
            '*' => {
                chars.next();
                simbolos.push(Simbolo::Multiplicacao);
            }
            '/' => {
                chars.next();
                simbolos.push(Simbolo::Divisao);
            }
            '%' => {
                chars.next();
                simbolos.push(Simbolo::Modulo);
            }
            '(' => {
                chars.next();
                simbolos.push(Simbolo::AbreParenteses);
            }
            ')' => {
                chars.next();
                simbolos.push(Simbolo::FechaParenteses);
            }
            '{' => {
                chars.next();
                simbolos.push(Simbolo::AbreChaves);
            }
            '}' => {
                chars.next();
                simbolos.push(Simbolo::FechaChaves);
            }
            ',' => {
                chars.next();
                simbolos.push(Simbolo::Virgula);
            }
            ';' => {
                chars.next();
                simbolos.push(Simbolo::PontoVirgula);
            }
            '=' => {
                chars.next();
                // Verifica se é ==
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    simbolos.push(Simbolo::IgualIgual);
                } else {
                    simbolos.push(Simbolo::Igual);
                }
            }
            '>' => {
                chars.next();
                // Verifica se é >=
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    simbolos.push(Simbolo::MaiorIgual);
                } else {
                    simbolos.push(Simbolo::Maior);
                }
            }
            '<' => {
                chars.next();
                // Verifica se é <=
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    simbolos.push(Simbolo::MenorIgual);
                } else {
                    simbolos.push(Simbolo::Menor);
                }
            }
            '!' => {
                chars.next();
                // Verifica se é !=
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    simbolos.push(Simbolo::Diferente);
                } else {
                    simbolos.push(Simbolo::Negacao);
                }
            }
            ' ' | '\n' | '\t' => {
                chars.next(); // ignora espaços
            }
            outro => {
                panic!("Caractere inválido: {}", outro);
            }
        }
    }

    simbolos.push(Simbolo::Fim);
    simbolos
}