use std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    NumeroInteiro(i64),
    NumeroFloat(f64),
    String(String),
    Identificador(String),
    Assign(String, Box<Expr>),
    VarDef(String, Box<Expr>),   // var variavel = valor
    Block(Vec<Expr>),            // { declarações }
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>), // if (condição) then else
    While(Box<Expr>, Box<Expr>), // while (condição) corpo
    GetAttr(Box<Expr>, String), // obj.atributo
    Call(Box<Expr>, Vec<Expr>), // funcao(argumentos)
    Negacao(Box<Expr>),          // !value
    NegacaoAritmetica(Box<Expr>), // -value (criado pelo parser quando - está em contexto unário)
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Soma(Box<Expr>, Box<Expr>),
    Subtracao(Box<Expr>, Box<Expr>),
    Multiplicacao(Box<Expr>, Box<Expr>),
    Divisao(Box<Expr>, Box<Expr>),
    Modulo(Box<Expr>, Box<Expr>),
    Maior(Box<Expr>, Box<Expr>),
    Menor(Box<Expr>, Box<Expr>),
    MaiorIgual(Box<Expr>, Box<Expr>),
    MenorIgual(Box<Expr>, Box<Expr>),
    IgualIgual(Box<Expr>, Box<Expr>),
    Diferente(Box<Expr>, Box<Expr>),
    Print(Box<Expr>), // Para a instrução print
    // Para contexto/escopo HashTable RC(Reference Count) - Celula
}

impl Expr {
    pub fn avaliar_com_contexto(&self, ctx: &mut HashMap<String, f64>) -> f64 {
        match self {
            Expr::NumeroInteiro(n) => *n as f64,
            Expr::NumeroFloat(n) => *n,
            Expr::String(_) => 0.0,
            Expr::Identificador(nome) => {
                match nome.as_str() {
                    "true" => 1.0,
                    "false" => 0.0,
                    _ => *ctx.get(nome).unwrap_or(&0.0),
                }
            }
            Expr::Assign(nome, expr) => {
                let valor = expr.avaliar_com_contexto(ctx);
                ctx.insert(nome.clone(), valor);
                valor
            }
            Expr::VarDef(nome, expr) => {
                let valor = expr.avaliar_com_contexto(ctx);
                ctx.insert(nome.clone(), valor);
                valor
            }
            Expr::Block(declaracoes) => {
                let mut resultado = 0.0;
                for declaracao in declaracoes {
                    resultado = declaracao.avaliar_com_contexto(ctx);
                }
                resultado
            }
            Expr::If(condicao, then_expr, else_expr) => {
                let valor_condicao = condicao.avaliar_com_contexto(ctx);
                if valor_condicao != 0.0 {
                    // condição é verdadeira
                    then_expr.avaliar_com_contexto(ctx)
                } else {
                    // condição é falsa
                    match else_expr {
                        Some(expr) => expr.avaliar_com_contexto(ctx),
                        None => 0.0, // nil
                    }
                }
            }
            Expr::While(condicao, corpo) => {
                let mut resultado = 0.0;
                while condicao.avaliar_com_contexto(ctx) != 0.0 {
                    resultado = corpo.avaliar_com_contexto(ctx);
                }
                resultado
            }
            Expr::GetAttr(obj, attr) => {
                println!("Acessando atributo '{}' do objeto", attr);
                0.0
            }
            Expr::Call(callee, args) => {
                println!("Chamando função com {} argumentos", args.len());
                for (i, arg) in args.iter().enumerate() {
                    println!("  Argumento {}: {}", i, arg.avaliar_com_contexto(ctx));
                }
                0.0
            }
            Expr::Negacao(expr) => {
                let valor = expr.avaliar_com_contexto(ctx);
                if valor == 0.0 { 1.0 } else { 0.0 }
            }
            Expr::NegacaoAritmetica(expr) => -expr.avaliar_com_contexto(ctx),
            Expr::And(esq, dir) => {
                if esq.avaliar_com_contexto(ctx) == 0.0 {
                    0.0
                } else {
                    dir.avaliar_com_contexto(ctx)
                }
            }
            Expr::Or(esq, dir) => {
                if esq.avaliar_com_contexto(ctx) != 0.0 {
                    1.0
                } else {
                    if dir.avaliar_com_contexto(ctx) != 0.0 { 1.0 } else { 0.0 }
                }
            }
            Expr::Soma(esq, dir) => esq.avaliar_com_contexto(ctx) + dir.avaliar_com_contexto(ctx),
            Expr::Subtracao(esq, dir) => esq.avaliar_com_contexto(ctx) - dir.avaliar_com_contexto(ctx),
            Expr::Multiplicacao(esq, dir) => esq.avaliar_com_contexto(ctx) * dir.avaliar_com_contexto(ctx),
            Expr::Divisao(esq, dir) => esq.avaliar_com_contexto(ctx) / dir.avaliar_com_contexto(ctx),
            Expr::Modulo(esq, dir) => esq.avaliar_com_contexto(ctx) % dir.avaliar_com_contexto(ctx),
            Expr::Maior(esq, dir) => {
                if esq.avaliar_com_contexto(ctx) > dir.avaliar_com_contexto(ctx) { 1.0 } else { 0.0 }
            }
            Expr::Menor(esq, dir) => {
                if esq.avaliar_com_contexto(ctx) < dir.avaliar_com_contexto(ctx) { 1.0 } else { 0.0 }
            }
            Expr::MaiorIgual(esq, dir) => {
                if esq.avaliar_com_contexto(ctx) >= dir.avaliar_com_contexto(ctx) { 1.0 } else { 0.0 }
            }
            Expr::MenorIgual(esq, dir) => {
                if esq.avaliar_com_contexto(ctx) <= dir.avaliar_com_contexto(ctx) { 1.0 } else { 0.0 }
            }
            Expr::IgualIgual(esq, dir) => {
                if esq.avaliar_com_contexto(ctx) == dir.avaliar_com_contexto(ctx) { 1.0 } else { 0.0 }
            }
            Expr::Diferente(esq, dir) => {
                if esq.avaliar_com_contexto(ctx) != dir.avaliar_com_contexto(ctx) { 1.0 } else { 0.0 }
            }
            Expr::Print(expr) => {
                let valor = expr.avaliar_com_contexto(ctx);
                println!("{}", valor);
                valor
            }
        }
    }

    // Mantém o método antigo para compatibilidade
    pub fn avaliar(&self) -> f64 {
        let mut ctx = HashMap::new();
        self.avaliar_com_contexto(&mut ctx)
    }
}

impl Expr {
    pub fn imprimir(&self, nivel: usize) {
        let indent = "   ".repeat(nivel);
        match self {
            Expr::NumeroInteiro(n) => println!("{}Número Inteiro: {}", indent, n),
            Expr::NumeroFloat(n) => println!("{}Número Float: {}", indent, n),
            Expr::String(s) => println!("{}String: \"{}\"", indent, s),
            Expr::Identificador(id) => println!("{}Identificador: {}", indent, id),
            Expr::Assign(nome, expr) => {
                println!("{}Assign: {} =", indent, nome);
                expr.imprimir(nivel + 1);
            }
            Expr::VarDef(nome, expr) => {
                println!("{}VarDef: var {} =", indent, nome);
                expr.imprimir(nivel + 1);
            }
            Expr::Block(declaracoes) => {
                println!("{}Block:", indent);
                for declaracao in declaracoes {
                    declaracao.imprimir(nivel + 1);
                }
            }
            Expr::If(condicao, then_expr, else_expr) => {
                println!("{}If:", indent);
                condicao.imprimir(nivel + 1);
                println!("{}Then:", indent);
                then_expr.imprimir(nivel + 1);
                if let Some(expr) = else_expr {
                    println!("{}Else:", indent);
                    expr.imprimir(nivel + 1);
                }
            }
            Expr::While(condicao, corpo) => {
                println!("{}While:", indent);
                condicao.imprimir(nivel + 1);
                println!("{}Corpo:", indent);
                corpo.imprimir(nivel + 1);
            }
            Expr::GetAttr(obj, attr) => {
                println!("{}GetAttr: {}", indent, attr);
                obj.imprimir(nivel + 1);
            }
            Expr::Call(callee, args) => {
                println!("{}Call:", indent);
                callee.imprimir(nivel + 1);
                for (i, arg) in args.iter().enumerate() {
                    println!("{}Argumento {}:", indent, i);
                    arg.imprimir(nivel + 1);
                }
            }
            Expr::Negacao(expr) => {
                println!("{}Negação (!):", indent);
                expr.imprimir(nivel + 1);
            }
            Expr::NegacaoAritmetica(expr) => {
                println!("{}Negação Aritmética (-):", indent);
                expr.imprimir(nivel + 1);
            }
            Expr::And(esq, dir) => {
                println!("{}And:", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::Or(esq, dir) => {
                println!("{}Or:", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::Soma(esq, dir) => {
                println!("{}Soma:", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::Subtracao(esq, dir) => {
                println!("{}Subtração:", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1); // <- estava faltando argumento
            }
            Expr::Multiplicacao(esq, dir) => {
                println!("{}Multiplicação:", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::Divisao(esq, dir) => {
                println!("{}Divisão:", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::Modulo(esq, dir) => {
                println!("{}Modulo:", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::Maior(esq, dir) => {
                println!("{}Maior (>):", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::Menor(esq, dir) => {
                println!("{}Menor (<):", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::MaiorIgual(esq, dir) => {
                println!("{}Maior Igual (>=):", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::MenorIgual(esq, dir) => {
                println!("{}Menor Igual (<=):", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::IgualIgual(esq, dir) => {
                println!("{}Igual (==):", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::Diferente(esq, dir) => {
                println!("{}Diferente (!=):", indent);
                esq.imprimir(nivel + 1);
                dir.imprimir(nivel + 1);
            }
            Expr::Print(expr) => {
                println!("{}Print:", indent);
                expr.imprimir(nivel + 1);
            }
        }
    }
}

