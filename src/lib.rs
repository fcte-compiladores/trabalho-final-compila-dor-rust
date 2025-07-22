pub mod analisador_lexico;
pub mod arvore_sintatica_abstrata;
pub mod analisador_sintatico;
use analisador_lexico::analisar;
use crate::analisador_sintatico::Analisador;

// use lexer::tokenize;

pub fn executar(source: &str) -> f64 {
    let mut resultado_final = 0.0;
    let mut contexto = std::collections::HashMap::new();

    for (i, linha) in source.lines().enumerate() {
        if linha.trim().is_empty() { continue; }
        println!("\nExpressão linha {}: {}", i + 1, linha);
        let simbolos = analisar(linha);
        let mut analisador = Analisador::new(simbolos.clone());
        match analisador.analisar_expressao() {
            Some(ast) => {
                ast.imprimir(0);
                println!("Símbolos encontrados: {:?}", simbolos);
                resultado_final = ast.avaliar_com_contexto(&mut contexto);
                println!("Resultado da expressão: {}", resultado_final);
            },
            None => {
                println!("Erro na análise sintática da linha {}", i + 1);
            }
        }
    }
    resultado_final
}
