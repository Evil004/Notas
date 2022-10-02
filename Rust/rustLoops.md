# Loops

Los loops son un tipo de estructura de control que se usa para repetir un bloque de codigo hasta que se cumpla una condicion.

Hay 2 tipos de loops en Rust:

-   finitos
    -   for
-   infinitos
    -   while
    -   loop

## for

Un bucle for es un bucle finito que se usa para iterar sobre una cantidad de veces determinada.

La sintaxis de un bucle for es la siguiente:
`rust for variable in range {Codigo}`

El siguient programa imprime los numeros del 0 al 9:

```rust
for i in 0..10 {
    println!("El valor de i es: {}", i);
}
```

_Nota: El rango inferior es incluido pero el superior no, por eso el rango va de 0 a 9._

En rust existe la funcion `enumerate()` que dice cuantas veces se hae ejecutado el loop.

La sintaaxis es la siguiente: `for (count, variable) in range.enumerate() {Codigo}`

El siguiente programa imprime los numeros del 5 al 11 y dice cuantas veces se ha ejecutado el loop:

```rust
for (count, i) in (5..12).enumerate() {
    println!("El valor de i es: {} y se ha ejecutado {} veces", i, count);
}
```

## while

El buclee while es un bucle infinito que se ejecuta mientras la condicion indicada sea verdadera.

La sintaxis de un bucle while es la siguiente: `while condicion {Codigo}`
El siguiente programa imprime los numeros del 0 al 9 y al llegar a 9 se sale del loop:

```rust
let mut i = 0;
let mut found = false;
while !found {
    println!("El valor de i es: {}", i);
    if i == 9 {
        found = true;
    }
    i += 1;
}
```

## loop

El bucle loop es un bucle infinito que no para de ejecutarse.

La sintaxis de un bucle loop es la siguiente: `loop {Codigo}`

El siguiente codigo imprime numeros del 1 al infinito:

```rust
let mut i = 1;
loop {
    println!("El valor de i es: {}", i);
    i += 1;
}
```

## break

El break es una palabra reservada que se usa para salir de un bucle de manera forzada. Es decir que aunque la condicion siga siendo verdadera, el bucle se sale.

Se suele usar dentro de condicionales dentro del bucle.

La sintaxis de un break es usar la palabra reservada `break` dentro del bucle.



### break en for

El siguiente codigo imprimiria los numeros del 0 al 9, pero al llegar a 5 se ejecuta el break y se sale del loop.

```rust
for i in 0..10 {
    println!("i:{}", i);
    if i == 5 {
      break;
    }
  }
```

### break en while

En el siguiente codigo se imprime los numeros del 1 al 10, pero al llegar a 5 se ejecuta el break y se sale del loop.

```rust
let mut i = 1;
  let found = false;

    while !found {
    println!("i:{}", i);
    if i == 5 {
      break;
    }
    if i == 10 {
      found = true;
    }

    i = i + 1;
  }
```

### break en loop

En el siguiente codigo se imprimirian numeros infinitos, pero al llegar a 5 se ejecuta el break y se sale del loop.
```rust
let mut i = 1;
loop {
    println!("El valor de i es: {}", i);
    if i == 5 {
        break;
    }
    i += 1;
}
```

## continue
El continue es una palabra reservada que se usa para saltar una iteracion de un bucle. Es decir que en el momento en el que se ejecuta el continue, se salta la iteracion actual y se ejecuta la siguiente.

La sintaxis de un continue es usar la palabra reservada `continue` dentro del bucle.
### continue en for

El siguiente codigo imprime los numeros del 0 al 4 y del 6 al 9, porque al llegar a 5 se ejecuta el continue y se salta la iteracion actual y se ejecuta la siguiente.

```rust
for i in 0..10 {
    if i == 5 {
      continue;
    }
    println!("i:{}", i);
  }
```

### continue en while

El siguiente codigo imprime los numeros del 1 al 4 y del 6 al 10, porque al llegar a 5 se ejecuta el continue y se salta la iteracion actual y se ejecuta la siguiente.

```rust
let mut i = 1;
  let found = false;

    while !found {
    if i == 5 {
      continue;
    }
    println!("i:{}", i);
    if i == 10 {
      found = true;
    }

    i = i + 1;
  }
```

### continue en loop

El siguiente codigo imprime los numeros del 1 al 4 y del 6 al infinito, porque al llegar a 5 se ejecuta el continue y se salta la iteracion actual y se ejecuta la siguiente.

```rust
let mut i = 1;
loop {
    if i == 5 {
        continue;
    }
    println!("El valor de i es: {}", i);
    i += 1;
}
```

## Etiquetas de los loops
En rust se pueden poner etiquetas a los loops para identificarlos en las declaraciones `break` y `continue`.

La sintaxis de una etiqueta es la siguiente: `'etiqueta: loop {Codigo}`

El siguiente codigo imprime la tabla del 1, 2 y 4, pero no imprime la tabla del 3 porque al llegar a 3 se ejecuta el continue y se salta la iteracion actual y se ejecuta la siguiente. Y en las tablas de multiplicar no imprime el 2 porque al llegar a 2 se ejecuta el continue y se salta la iteracion actual y se ejecuta la siguiente.

```rust
'outer:for i in 1..5 { 
    println!("Muliplication Table : {}", i);
   'inner:for j in 1..5 { 
        if i == 3 { continue 'outer; } 
        if j == 2 { continue 'inner; } 
        println!("{} * {} = {}", i, j, i * j);
   }
 }
```
