# 🚀 Compilador para Nivre (Parser)

Un analizador léxico y sintáctico construido desde cero en **Rust** para el lenguaje de programación personalizado **Nivre**.

Nivre está diseñado con una sintaxis inspirada en Python, utilizando la indentación para el control de alcance (scoping) y aplicando una jerarquía estricta para la evaluación de operaciones matemáticas y lógicas.

## ✨ Características Principales

* **Control de Alcance por Indentación:** Nivre utiliza un sistema de indentación de espaciado flexible para delimitar bloques lógicos (`if`, `else`, `def`), eliminando la necesidad de llaves `{}`.
* **Precedencia Matemática Estricta:** El analizador sintáctico (Parser) evalúa correctamente la jerarquía de operadores (multiplicación/división antes que suma/resta) y soporta el aislamiento de operaciones mediante paréntesis `()`.
* **Tipado Explícito en Declaraciones:** Soporte nativo para declaraciones de variables con verificación de tipo sintáctica (ej. `variable:int = 10`).
* **Control de Flujo:** Implementación completa de estructuras condicionales `if` y `else` anidadas.
* **Seguridad de Alcance Global:** Restricción arquitectónica que exige que todo el código ejecutable resida dentro de funciones declaradas (ej. `def main():`), prohibiendo el código suelto en la raíz del archivo.
* **Tipos de Datos Soportados:** Enteros (`i64`), Flotantes (`f64`), Cadenas de Texto (`String`) y Booleanos (`bool`).

## 🛠️ Arquitectura del Proyecto

El proyecto está dividido en tres módulos principales que interactúan en cascada:

1. **`lexer.rs`**: Convierte el código fuente de texto plano en una secuencia de `Tokens` estructurados, calculando dinámicamente los niveles de indentación.
2. **`parser.rs`**: Consume los tokens y aplica reglas gramaticales descendentes (Top-Down) para construir el Árbol Sintáctico Abstracto (AST).
3. **`ast.rs`**: Define las estructuras de datos recursivas (`Expr` y `Stmt`) en el *heap* de memoria usando `Box<T>`, e incluye un visualizador jerárquico para la consola.

## 🚀 Uso y Ejecución

Asegúrate de tener [Rust y Cargo](https://www.rust-lang.org/tools/install) instalados en tu sistema.

1. Clona el repositorio y navega a la carpeta del proyecto.
2. Crea un archivo de prueba con extensión `.ni` (por ejemplo, `NivreTest/prueba.ni`).
3. Ejecuta el compilador pasando la ruta del archivo como argumento:

```bash
cargo run -- NivreTest/prueba.ni
```

Hecho por:
- Samer Ghattas
- Diego Rojas
- Juan Garcia
- Diego Gonzalez