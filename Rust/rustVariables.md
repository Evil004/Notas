# Variables & tipos de datos en Rust

## Variables

Las variables se declaran con la palabra reservada `let` seguida del nombre de la variable.

```rust
let edad;
```

_NOTA: En rust la notacion es sake_case, es decir todo minuscula y separado por barras bajas (\_)_

### Inicializacion de variables

Las variables se inicializan asignando un valor con el operador `=`.

```rust
let name = "Oscar";
```

### Variables mutables

En rust las variables no son mutables por defecto para hacer una variable mutable se usa la palabra reservada `mut` antes de la declaracion de la variable. Ahora el valor de la variable puede ser cambiado.

```rust
let mut edad = 20;
```

### Asignar multiples variables

Se puede asignar multiples variables en una sola linea. Para esto se ponen los nombres de variables dentro de parentesis y se separan con comas. Lo mismo con los valores.

```rust
let (name, age) = ("Oscar", 18);
```

### Scope de las variables

El scope de una variable es la visibilidad de la variable o desde que partes del codigo se puede acceder a la variable.

Depende donde fue declarada, si fue declarada dentro de unos brackets `{}` solo se vera dentro de esos brackets, si no sera global.

#### Variables locales

Las variables declaradas dentro de un bloque de codigo son locales y solo se pueden usar dentro de ese bloque de codigo. Al cerrar el bloque la variable se livera de memoria.

```rust
{
    let name = "Oscar"; // La variable es declarada dentro del bloque
    println!("{}", name); // Al intentar acceder a la variable desde dentro del bloque imprimira el valor de la variable.

}

 println!("{}", name); // Al intentar acceder a la variable desde fuera del bloque dara error
```

#### Variables globales

Las variables declaradas fuera de un bloque de codigo son globales y se pueden usar en cualquier parte del codigo.

```rust
let name = "Oscar"; // La variable es declarada fuera del bloque

{
    println!("{}", name); // Al intentar acceder a la variable desde dentro del bloque imprimira el valor de la variable.

}

 println!("{}", name); // Al intentar acceder a la variable desde fuera del bloque imprimira el valor de la variable.
```

#### Variables ocultas

Las variables declaradas dentro de un bloque de codigo con el mismo nombre que una variable global se ocultan y solo se pueden usar dentro de ese bloque de codigo. Al cerrar el bloque la variable se livera de memoria.

```rust
let name = "Oscar"; // La variable es declarada fuera del bloque

{
    let name = "Juan"; // La variable es declarada dentro del bloque con el mismo nombre que la variable global

    println!("{}", name); // Al intentar acceder a la variable desde dentro del bloque imprimira el valor de la declarada dentro del bloque.

}

 println!("{}", name); // Al intentar acceder a la variable desde fuera del bloque imprimira el valor de la variable declarada fuera del bloque.
```

### Tipos de datos

Los tipos de datos en rust son estaticos, es decir que una vez que se declara el tipo de dato de una variable no se puede cambiar.

Rust tiene definicion de datos implicita, es decir que no es necesario declarar el tipo de dato de una variable o explicita que le indicas al compilador que tipo de variable es.

#### Definicion de datos implicita

La sintaaxis para definir una variable con definicion de datos implicita es `let nombre_variable = valor;`

```rust
let name = "Oscar"; // La variable adquiere el tipo de dato String
```

#### Definicion de datos explicita

La sintaaxis para definir una variable con definicion de datos explicita es `let nombre_variable: tipo_dato = valor;`

```rust
let name: &str = "Oscar"; // La variable se declara con el tipo de dato String que hemos indicado
```

#### Tipos de datos primitivos

-   Tipos escalares

    -   [Integer](#Integer)
    -   [Float](#Float)

    -   [Boolean](#Boolean)
    -   [Character](#Character)

-   Tipos compuestos
    -   [Tuplas](#Tuplas)
    -   [Arrays](#Arrays)

#### Integer
Las variables de tipo integer almacenan numeros enteros. Rust tiene 8 tipos de integer en rust dependiendo del tamaño del tamaño en bits que ocupa en memoria, se agupan en 2 tipos:

-   Signed(`i`): Pueden ser positivos o negativos
-   Unsigned(`u`): Solo pueden ser positivos

| Tipo | Tamaño en bits | Rango |
| :--- | :--- | :--- |
| i8 | 8 | -128 a 127 |
| i16 | 16 | -32,768 a 32,767 |
| i32 | 32 | -2,147,483,648 a 2,147,483,647 |
| i64 | 64 | -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807 |
| u8 | 8 | 0 a 255 |
| u16 | 16 | 0 a 65,535 |
| u32 | 32 | 0 a 4,294,967,295 |
| u64 | 64 | 0 a 18,446,744,073,709,551,615 |

Existen otros tipo de integer dependiendo de la arquitectura del procesador, isize y usize.

```rust
let num: i32 = 10;
let num: u32 = -10; 
```

#### Float
Las variables de tipo float almacenan numeros decimales. Rust tiene 2 tipos de float en rust dependiendo del tamaño del tamaño en bits que ocupa en memoria, Se declaran:

-   `f32`: ocupa 32 bits en memoria.
-   `f64`: ocupa 64 bits en memoria y es mas preciso.

```rust
let x: f64 = 2.0; // f64
let y: f32 = 3.0; // f32
```

#### Boolean
Las variables de tipo boolean almacenan valores booleanos. Es decir puede ser `true` o `false`. Se declaran con la palabra reservada `bool`.

```rust:
let is_active:bool = true;
```


#### Character
Las variables de tipo character almacenan caracteres unicode. Se declaran con comillas simples. Ocupan 4 bytes en memoria porque pueden almacenar mas que caracteres ASCII.


```rust
let name = 'G';
```

#### String
Las variables de tipo string almacenan cadenas de caracteres cerradas entre comillas dobles `""`. Se declaran con la palabra reservada `String`.

```rust
let name:&str = "Oscar";
```

#### Arrays

Los arrays son una coleccion de datos del mismo tipo.Para definir un array se declara con el typo de dato  y el numero de elementos que contendra el array dentro de corchetes `[]` separdo por `;`.

```rust
let array: [i32; 5] = [1, 2, 3, 4, 5]; // de manera explicita
let array = [1,2]; // de manera implicita
let array = [3; 5]; // de manera implicita con el mismo valor en todos los elementos
```

De los arrays se pueed obtener el numero de elementos con la funcion `len()`.

```rust
let array = [1,2,3,4,5];
println!("{}", array.len()); // Imprime 5
```

De los arrays se puede obtener una porcion de los elementos apuntando a la direccion de memoria del array y indicando entre corchetes `[]` el indice del primer elemento y el indice del ultimo elemento.

```rust
let array = [1,2,3,4,5];
let array_slice = &array[0..2]; // Obtiene los elementos desde el indice 0 hasta el indice 2
```

#### Tuplas

Las tuplas son una coleccion de datos de diferentes tipos. Para definir una tupla de manera explicita se declara entre parentesis `()` y cada tipo de dato en la posicion que concuerde.

```rust
let tupla: (i32, f64, u8) = (500, 6.4, 1); // de manera explicita
let tupla = (500, 6.4, 1); // de manera implicita
```
#### Constantes
Las constantes son variables que no pueden ser modificadas una vez que se les asigna un valor. Para declarar una constante se utiliza la palabra reservada `const` y se declara con el tipo de dato y el valor.

```rust
const MAX_POINTS: u32 = 100;
```