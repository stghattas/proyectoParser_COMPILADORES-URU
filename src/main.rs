// 1. Declaramos los módulos (archivos) que existen en el proyecto
mod lexer;
mod ast;
mod parser;

// 2. Traemos las estructuras específicas que queremos usar
use lexer::Lexer;
use parser::Parser;

fn main() {
    // Tu código de prueba aquí...
    let codigo = "5 + 3";
    
    let mut lexer = Lexer::new(codigo);
    let tokens = lexer.tokenize();
    
    let mut parser = Parser::new(tokens);
    
    match parser.parse_operacion_basica() {
        Ok(arbol) => println!("AST Generado: {:#?}", arbol),
        Err(e) => println!("❌ {}", e),
    }
}