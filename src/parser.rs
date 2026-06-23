// parser.rs
use crate::ast::{Expr, Stmt};
use crate::lexer::{Token, TokenType}; // Asumiendo que los declaraste en ast.rs

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    // Devuelve el token actual sin avanzar
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    // Devuelve el token actual y avanza la posición
    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    fn parse_primario(&mut self) -> Result<Expr, String> {
        // Clonamos el token para no pelear con el borrow checker de Rust
        let token = self.advance().cloned().ok_or("Fin de archivo inesperado")?;

        match token.token_type {
            TokenType::Integer(val) => Ok(Expr::LiteralInt(val)),
            TokenType::Float(val) => Ok(Expr::LiteralFloat(val)),
            TokenType::Identificador(nombre) => Ok(Expr::Identificador(nombre)),
            _ => Err(format!(
                "Error Sintáctico en la línea {}, columna {}: Se esperaba un número o identificador, pero se encontró '{}'",
                token.line, token.column, token.value
            )),
        }
    }

    pub fn parse_operacion_basica(&mut self) -> Result<Expr, String> {
        // 1. Obtenemos el valor izquierdo (ej. el '5')
        let mut nodo_izquierdo = self.parse_primario()?;

        // 2. Revisamos si el token que sigue es un operador
        while let Some(token) = self.peek().cloned() {
            if let TokenType::Operador(op) = &token.token_type {
                
                // Si es un operador matemático, procedemos
                if op == "+" || op == "-" || op == "*" || op == "/" {
                    self.advance(); // Consumimos el token del operador

                    // 3. Obtenemos el valor derecho (ej. el '3')
                    let nodo_derecho = self.parse_primario()?;

                    // 4. Transformamos nuestro nodo izquierdo en una Operación Binaria
                    // que engloba tanto al izquierdo original como al nuevo derecho.
                    nodo_izquierdo = Expr::OperacionBinaria {
                        izquierdo: Box::new(nodo_izquierdo),
                        operador: op.clone(),
                        derecho: Box::new(nodo_derecho),
                    };
                    
                    // El 'continue' permite encadenar operaciones como '5 + 3 + 2'
                    continue; 
                }
            }
            // Si no es un operador (ej. es un salto de línea), terminamos la expresión
            break; 
        }

        Ok(nodo_izquierdo)
    }

    // Consume el token si coincide con el tipo esperado, si no, retorna error
    fn expect_operator(&mut self, op_esperado: &str) -> Result<(), String> {
        if let Some(token) = self.advance() {
            if let TokenType::Operador(ref op) = token.token_type {
                if op == op_esperado {
                    return Ok(());
                }
            }
            Err(format!(
                "Error Sintáctico en la línea {}, columna {}: Se esperaba el operador '{}', pero se encontró '{}'", 
                token.line, token.column, op_esperado, token.value
            ))
        } else {
            Err("Error Sintáctico: Fin de archivo inesperado".to_string())
        }
    }
}
