# RUST

## Inicio

### Que es Rust?

Rust es un lenguaje de programación que es seguro y concurrente.
Rust está inspuirado en C y C++ en tema de sintaxis y rendimiento y en tema de sintaxis en Haskell y ML.

### Para que es usado?

-   Para crear poderas aplicaciones web.
-   Para crear servicios online.
-   Para tener soporte en CLI multiplataforma.
-   Etc.

## Lo basico

### Hello world

Este porograma nos imprime por consola "Hello world".

```rust
fn main() {
    println!("Hello, world!");
}
```

### Macros
Las macros son "funciones" que genera codigo en tiempo de compilacion. Es decir que no se conviete en codigo ejecutable directamente si no que se expande en el codigo de la funcion. Una macro se define con el simbolo de admiracion "!" despues del nombre de la funcion.

Un ejemplo de macro es el println!() que se usa para imprimir por consola.
```rust
println!("Hello, world!");
```



### Imprimir valores

En rust se usa el simbolo `{}` para indicar que se va a imprimir una variable o valor y seprado por comas se pone el nombre de la variable o valor (si no se dejan los `{}` vacios se tiene que introducir la misma cantidad de valores que de `{}`).

Por ejemplo el siguiente codigo imprime "Hello, World":

```rust
println!("{}, {}", "Hello", "World");
```

#### Argumentos posicionales

Especifican la posicion del valor en la frase.
Cada numero se asigna basado en el indice de las bariables. EL siguiente codigo imprime "World, Hello, Hello":

```rust
println!("{1}, {0}, {0}", "Hello", "World");
```

#### Argumentos nombrados

Se especifica el nombre de la variable y se pone entre llaves. El siguiente codigo imprime "Hello, World":

```rust
println!("{greeting}, {name}", greeting="Hello", name="World");
```

#### Cambiar la base de un numero

Puedes cambiar un numero de base con los placeholders `{:b}` para binario, `{:o}` para octal, `{:x}` para hexadecimal. El siguiente codigo imprime "Numero: 10 en binario: 1010, Hexadecimal: A, Octal:12".

```rust
println!("Numero: 10 en binario:{:b}, hexadecimal:{:x}, octal:{:o}", 10, 10, 10);
```

#### Imprimir varios valores
Se pueden imprimir varios valores en una sola linea con el placeholder `{:?}`. El siguiente codigo imprime "("Hello", "World")":

```rust
println!("{:?}", ("Hello", "World"));
```


### Estilos de impresion

#### print!()
La macro de print!() imprime por consola sin saltar de linea.

```rust
print!("Hello, World");
```

#### println!()
La macro de println!() imprime por consola y salta de linea.

```rust
println!("Hello, World");
```

#### eprint!()
La macro de eprint!() imprime por consola un error sin saltar de linea.

```rust
eprint!("Hello, World");
```

#### eprintln!()

La macro de eprintln!() imprime por consola un error y salta de linea.

```rust
eprintln!("Hello, World");
```

### Comentarios
Hay dos tipos de comentarios en rust:

#### Comentarios de linea
Los comentarios de linea se hacen con `//` y se usa para comentar una sola linea.

```rust
// Esto es un comentario de linea
```

#### Comentarios de bloque
Los comentarios de bloque se hacen con `/*` y `*/` y se usa para comentar varias lineas.

```rust
/*
Esto es un comentario de bloque
*/
```

### Comentarios de documentacion
Los comentarios de documentacion se hacen con `///` y se usa para documentar funciones, modulos, clases, etc.

```rust
/// Esto es un comentario de documentacion
```

Tambien se pueden poner comentarios de documentacion dentro de funciones, modulos, clases, etc. Con `//!` y se usa para documentar funciones, modulos, clases, etc.

```rust
//! Esto es un comentario de documentacion dentro de una funcion
```
### [Ejemplo de lo anterior en codigo](./Ejemplos/rustPrints.rs)

### [Variables & data types](./rustVariables.md)

### [Operadores](./rustOperators.md)