use std::fs;
use mini_lox_rust::executar;

fn main() {
    println!("Coé mundo!");
    let source = fs::read_to_string("leiaesse.crl")
        .expect("Erro ao ler esse crl!");

    // Mostra o conteúdo na tela
    println!("Conteúdo do arquivo:\n{}", source);
    executar(&source);
} 