
#[derive(Debug, Clone, PartialEq)]
pub enum Simbolo {
    Numero(f64),
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
                    _ => simbolos.push(Simbolo::Identificador(identificador)),
                }
            }
            '.' => {
                chars.next();
                simbolos.push(Simbolo::Ponto);
            }
            '0'..='9' => {
                let mut numero = String::new();
                let primeiro_digito = c;
                // Regra do Lox: se começa com 0, só pode ser 0 ou 0.xxx
                if primeiro_digito == '0' {
                    numero.push(chars.next().unwrap());
                    if let Some(&proximo) = chars.peek() {
                        if proximo == '.' {
                            numero.push(chars.next().unwrap());
                            while let Some(&d) = chars.peek() {
                                if d.is_ascii_digit() {
                                    numero.push(d);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    while let Some(&d) = chars.peek() {
                        if d.is_ascii_digit() || d == '.' {
                            numero.push(d);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
                simbolos.push(Simbolo::Numero(numero.parse().unwrap()));
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