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

impl Stmt {
    pub fn imprimir_arbol(&self, nivel: usize) {
        let sangria = "    ".repeat(nivel); // 4 espacios por nivel

        match self {
            Stmt::Asignacion { nombre, valor } => {
                println!("{}└── Asignación: {}", sangria, nombre);
                valor.imprimir_arbol(nivel + 1);
            }
            Stmt::Expresion(expr) => {
                println!("{}└── Expresión Suelta", sangria);
                expr.imprimir_arbol(nivel + 1);
            }
            Stmt::If { condicion, bloque_true, bloque_else } => {
                println!("{}└── If", sangria);
                condicion.imprimir_arbol(nivel + 1);
                println!("{}    └── Bloque True", sangria);
                for stmt in bloque_true {
                    stmt.imprimir_arbol(nivel + 2);
                }
                if let Some(else_block) = bloque_else {
                    println!("{}    └── Bloque Else", sangria);
                    for stmt in else_block {
                        stmt.imprimir_arbol(nivel + 2);
                    }
                }
            }
            Stmt::While { condicion, bloque } => {
                println!("{}└── While", sangria);
                condicion.imprimir_arbol(nivel + 1);
                println!("{}    └── Bloque", sangria);
                for stmt in bloque {
                    stmt.imprimir_arbol(nivel + 2);
                }
            }
        }
    }
}

impl Expr {
    pub fn imprimir_arbol(&self, nivel: usize) {
        let sangria = "    ".repeat(nivel);

        match self {
            Expr::LiteralInt(val) => println!("{}├── Entero: {}", sangria, val),
            Expr::LiteralFloat(val) => println!("{}├── Flotante: {}", sangria, val),
            Expr::LiteralString(val) => println!("{}├── String: \"{}\"", sangria, val),
            Expr::LiteralBool(val) => println!("{}├── Booleano: {}", sangria, val),
            Expr::Identificador(nombre) => println!("{}├── Id: {}", sangria, nombre),
            Expr::OperacionBinaria { izquierdo, operador, derecho } => {
                println!("{}├── Operación: [{}]", sangria, operador);
                izquierdo.imprimir_arbol(nivel + 1);
                derecho.imprimir_arbol(nivel + 1);
            }
        }
    }
}