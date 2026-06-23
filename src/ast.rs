#[derive(Debug, Clone)]
pub enum Expr {
    LiteralInt(i64),
    LiteralFloat(f64),
    LiteralString(String),
    LiteralBool(bool),
    Identificador(String),
    // Para operaciones como: a + b
    OperacionBinaria {
        izquierdo: Box<Expr>,
        operador: String,
        derecho: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    // Para: variable = expresion
    Asignacion {
        nombre: String,
        valor: Expr,
    },
    // Para: if condicion: bloque
    If {
        condicion: Expr,
        bloque_true: Vec<Stmt>,
        bloque_else: Option<Vec<Stmt>>, 
    },
    // Para: while condicion: bloque
    While {
        condicion: Expr,
        bloque: Vec<Stmt>,
    },
    // A veces una expresión por sí sola es una instrucción válida (ej. llamar a una función)
    Expresion(Expr), 
}