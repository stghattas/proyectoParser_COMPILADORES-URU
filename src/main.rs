mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;

fn main() {
    // 1. Recolectar los argumentos pasados por la terminal
    let args: Vec<String> = env::args().collect();

    // 2. Verificar que el usuario haya proporcionado un archivo
    if args.len() < 2 {
        eprintln!("Error: Falta el archivo de entrada.");
        eprintln!("Uso: cargo run -- <ruta/al/archivo.ni>");
        std::process::exit(1);
    }

    let ruta_archivo = &args[1];

    // 3. Leer el contenido del archivo .ni
    println!("Compilando archivo: {}", ruta_archivo);

    let codigo = match fs::read_to_string(ruta_archivo) {
        Ok(contenido) => contenido,
        Err(error) => {
            eprintln!("Error al leer el archivo '{}': {}", ruta_archivo, error);
            std::process::exit(1);
        }
    };

    // 4. Iniciar el proceso de Compilación (Fase 1 y 2)
    let mut lexer = Lexer::new(&codigo);
    let tokens = lexer.tokenize();

    // descomentar la siguiente línea si quiere ver la lista de tokens antes del AST
    // println!("Tokens generados: {:#?}", tokens);

    let mut parser = Parser::new(tokens);

    match parser.parse_programa() {
        Ok(arbol) => println!("AST Generado con éxito:\n{:#?}", arbol),
        Err(e) => println!("{}", e),
    }
}
