# Expresiones condicionales

Las expresiones condicionales te permiten tomar decisiones en tu codigo.

## Tipos

Hay tres tipos de expresiones condicionales en Rust:
- [if](#if)
- [if let](#if-let)
- [match](#match)

### if
Una expresion if es una expresion que evalua una condicion y ejecuta un bloque de codigo si la condicion es verdadera.

```rust
if condicion {
    // Codigo a ejecutar si la condicion es verdadera
}
```

Se puede añadir un else a la expresion if para ejecutar un bloque de codigo si la condicion es falsa.

```rust
if condicion {
    // Codigo a ejecutar si la condicion es verdadera
} else {
    // Codigo a ejecutar si la condicion es falsa
}
```

Se puede añadir un else if a la expresion if para ejecutar un bloque de codigo si la condicion es falsa y otra condicion es verdadera.

```rust
if condicion {
    // Codigo a ejecutar si la condicion es verdadera
} else if otra_condicion {
    // Codigo a ejecutar si la condicion es falsa y otra_condicion es verdadera
} else {
    // Codigo a ejecutar si la condicion es falsa y otra_condicion es falsa
}
```

Se pueden encadenar tantos else if como se quiera y se pueden anidar un if dentro de otro if.

```rust
if condicion {
    if otra_condicion {
        // Codigo a ejecutar si la condicion es verdadera y otra_condicion es verdadera
    }
}
```

#### if abreviado
Se puede usar un if abreviado para asignar un valor a una variable si la condicion es verdadera.

```rust
let variable = if condicion {
    // Valor a asignar a la variable si la condicion es verdadera
} else {
    // Valor a asignar a la variable si la condicion es falsa
};
```



### if let

Una expresion if let es una expresion que evalua un patron y ejecuta un bloque de codigo si el patron coincide con el valor.

```rust
valores = ("valor1", "valor2", "valor3");
if let ("valor1", "valor2", "valor3") = valores {
    // Codigo a ejecutar si el patron coincide con el valor
}else{
    // Codigo a ejecutar si el patron no coincide con el valor
}
```

Si el patron coincide en uno de los primeros valores se puede adivinar el tercero.


```rust
valores = ("valor1", "valor2", "valor3");
if let ("valor1", "valor2", c) = valores {
    println!("c: {}", c); // c: "valor3"
    // Codigo a ejecutar si el patron coincide con el valor
}else{
    // Codigo a ejecutar si el patron no coincide con el valor
}
```
Si el patron coincide un valor se puede adivinar el resto.

```rust
valores = ("valor1", "valor2", "valor3");
if let (a, "valor2", c) = valores {
    println!("a: {}, c: {}", a, c); // a: "valor1", c: "valor3"
    // Codigo a ejecutar si el patron coincide con el valor
}else{
    // Codigo a ejecutar si el patron no coincide con el valor
}
```

Tambien se puede cambiar el patron por una `_` para ignorar el valor. Pero es una tonteria porque hara que siempre se ejecute el codigo del `if let`. Y el compilador soltara una advertencia.



```rust
if let _ = 10 {
    // Este codigo se ejecutara siempre
}
```

### match
La expresion match es una expresion que evalua si un valor esta en una lista de valores. La expresion match es similar a un switch en otros lenguajes.

#### Metodo 1

Si solo quieres comparar el valor de la variable y no asignarle ningun valor resultante puedes usar el metodo 1.


```rust
let valor = 10;
match valor {
    1 => println!("El valor es 1"),
    2 => println!("El valor es 2"),
    3 => println!("El valor es 3"),
    4 => println!("El valor es 4"),
    5 => println!("El valor es 5"),
    _ => println!("El valor no es 1, 2, 3, 4 o 5"),
}
```

#### Metodo 2
Si aparte de comparar el valor de la variable quieres asignarle un valor resultante puedes usar el metodo 2.

```rust
let valor = 10;
let valor_resultante = match valor {
    1 => "El valor es 1",
    2 => "El valor es 2",
    3 => "El valor es 3",
    4 => "El valor es 4",
    5 => "El valor es 5",
    _ => "El valor no es 1, 2, 3, 4 o 5",
};
println!("{}", valor_resultante);
```