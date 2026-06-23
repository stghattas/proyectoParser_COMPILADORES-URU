#![allow(unused_variables)]

use crate::lexer::{Token, TokenType};
use crate::ast::{Expr, Stmt};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    // --- NUEVO: Bucle principal ---
    pub fn parse_programa(&mut self) -> Result<Vec<Stmt>, String> {
        let mut instrucciones = Vec::new();

        while let Some(token) = self.peek() {
            if token.token_type == TokenType::EOF {
                break;
            }
            // Ignoramos saltos de línea y puntuación extra por ahora en el bucle principal
            if token.value == "\n" || token.value == ";" {
                self.advance();
                continue;
            }

            let instruccion = self.parse_instruccion()?;
            instrucciones.push(instruccion);
        }

        Ok(instrucciones)
    }

    // --- Identificar qué tipo de instrucción es ---
    fn parse_instruccion(&mut self) -> Result<Stmt, String> {
        let token_actual = self.peek().cloned().ok_or("Fin de archivo inesperado")?;

        match &token_actual.token_type {
            TokenType::PalabraReservada(palabra) if palabra == "def" => {
                self.advance(); 
                self.advance(); 
                Err("Aún no hemos implementado el parsing de funciones (def)".to_string())
            }
            TokenType::Identificador(nombre) => {
                self.advance(); // Consumimos el identificador

                if let Some(siguiente) = self.peek() {
                    if let TokenType::Operador(op) = &siguiente.token_type {
                        if op == "=" {
                            self.advance(); // Consumimos el '='
                            let valor = self.parse_operacion_basica()?;
                            return Ok(Stmt::Asignacion {
                                nombre: nombre.clone(),
                                valor,
                            });
                        }
                    }
                }
                
                self.position -= 1;
                let expr = self.parse_operacion_basica()?;
                Ok(Stmt::Expresion(expr))
            }
            _ => {
                let expr = self.parse_operacion_basica()?;
                Ok(Stmt::Expresion(expr))
            }
        }
    }

    // --- La función que evalúa expresiones ---
    pub fn parse_operacion_basica(&mut self) -> Result<Expr, String> {
        let mut nodo_izquierdo = self.parse_primario()?;

        while let Some(token) = self.peek().cloned() {
            if let TokenType::Operador(op) = &token.token_type {
                if op == "+" || op == "-" || op == "*" || op == "/" {
                    self.advance(); 
                    let nodo_derecho = self.parse_primario()?;
                    nodo_izquierdo = Expr::OperacionBinaria {
                        izquierdo: Box::new(nodo_izquierdo),
                        operador: op.clone(),
                        derecho: Box::new(nodo_derecho),
                    };
                    continue; 
                }
            }
            break; 
        }
        Ok(nodo_izquierdo)
    }

    fn parse_primario(&mut self) -> Result<Expr, String> {
        let token = self.advance().cloned().ok_or("Fin de archivo inesperado")?;

        match token.token_type {
            TokenType::Integer(val) => Ok(Expr::LiteralInt(val)),
            TokenType::Float(val) => Ok(Expr::LiteralFloat(val)),
            TokenType::String(val) => Ok(Expr::LiteralString(val)),
            TokenType::Boolean(val) => Ok(Expr::LiteralBool(val)),
            TokenType::Identificador(nombre) => Ok(Expr::Identificador(nombre)),
            _ => Err(format!(
                "Error Sintáctico en la línea {}, columna {}: Se esperaba un valor primario, pero se encontró '{}'",
                token.line, token.column, token.value
            )),
        }
    }
}