# Operadores

Los operadores son símbolos que le indican la operacion que debe realizar entre valores o variables.

## Typos de operadores
Hay diferentes tipos de operadores en Rust se categorizan en:
- Operadores unitarios
    - [Borrow expresions](#expresiones-borrow)
    - [Dereference expressions](#derreferenciacion)

- Operadores binarios
    - [Expresiones de aritmeticas](#expresiones-aritmeticas)
    - [Expresiones logicas](#expresiones-logicas)
    - [Expresiones de comparacion](#expresiones-de-comparacion)
    - [Expresiones de bit a bit](#expresiones-de-bit-a-bit)
    - [Expresiones de asignacion](#expresiones-de-asignacion--asignacion-compuesto)
    - [Expresiones de asignacion compuesto](#expresiones-de-asignacion--asignacion-compuesto)
    - [Expresiones typecast](#expresiones-typecast)

### Expresiones aritmeticas
Los operadores aritmeticos son los operadores que se usan para realizar operaciones aritmeticas entre valores o variables.

| Operador | Nombre | Descripcion |
|----------|-------------| ----------- |
| +        | Suma        | Suma dos valores o variables |
| -        | Resta       | Resta dos valores o variables |
| *        | Multiplicacion | Multiplica dos valores o variables |
| /        | Division    |  Divide dos valores o variables |
| %        | Modulo      | Obtiene el residuo de una division |


```rust
let a = 10;
let b = 5;
let c = a + b; // c = 15
let d = a - b; // d = 5
let e = a * b; // e = 50
let f = a / b; // f = 2
let g = a % b; // g = 0
```

### Expresiones logicas
Los operadores logicos son los operadores que se usan para realizar operaciones logicas.

| Operador | Nombre | Descripcion |
|----------|-------------| ----------- |
| &&       | AND         | Evalua si dos expresiones son verdaderas |
| ||       | OR          | Evalua si una de las expresiones es verdadera |
| !        | NOT         | Invierte el valor de una expresion |

```rust
let a = true;
let b = false;
let c = a && b; // c = false
let d = a || b; // d = true
let e = !a; // e = false
```


### Expresiones de comparacion
Los operadores de comparacion son los operadores que se usan para comparar valores o variables.

| Operador | Nombre | Descripcion |
|----------|-------------| ----------- |
| ==       | Igual       | Evalua si dos valores o variables son iguales |
| !=       | Diferente   | Evalua si dos valores o variables son diferentes |
| >        | Mayor que   | Evalua si un valor o variable es mayor que otro |
| <        | Menor que   | Evalua si un valor o variable es menor que otro |
| >=       | Mayor o igual que | Evalua si un valor o variable es mayor o igual que otro |
| <=       | Menor o igual que | Evalua si un valor o variable es menor o igual que otro |

```rust
let a = 10;
let b = 5;
let c = a == b; // c = false
let d = a != b; // d = true
let e = a > b; // e = true
let f = a < b; // f = false
let g = a >= b; // g = true
let h = a <= b; // h = false
```

### Expresiones de bit a bit
Los operadores de bit a bit son los operadores que se usan para realizar operaciones a nivel de bits, es decir transforman el numero en bits y hacen la operacion. 

| Operador | Nombre | Descripcion |
|----------|-------------| ----------- |
| &        | AND         | Evalua si dos valores o variables son iguales |
| \|       | OR          | Evalua si una de las expresiones es verdadera |
| ^        | XOR         | Evalua si una de las expresiones es verdadera |
| !        | NOT         | Invierte el valor de una expresion |
| <<       | Desplazamiento a la izquierda | Desplaza los bits de un valor o variable a la izquierda |
| >>       | Desplazamiento a la derecha | Desplaza los bits de un valor o variable a la derecha |

```rust
let a = 10; // 1010
let b = 6; // 0110
let c = a & b; // c = 2  1010 & 0110 = 0010
let d = a | b; // d = 14 1010 | 0110 = 1110
let e = a ^ b; // e = 12 1010 ^ 0110 = 1100
let f = !a; // f = -11 1010 = 0101
let g = a << 1; // g = 20 1010 << 1 = 10100
let h = a >> 1; // h = 5 1010 >> 1 = 0101
```

### Expresiones de asignacion & asignacion compuesto
Los operadores de asignacion son los operadores que se usan para asignar valores a variables.

| Operador | Nombre | Descripcion |
|----------|-------------| ----------- |
| =        | Asignacion  | Asigna un valor a una variable |
| +=       | Asignacion compuesta suma | Asigna un valor a una variable sumandole el valor de otra variable |
| -=       | Asignacion compuesta resta | Asigna un valor a una variable restandole el valor de otra variable |
| *=       | Asignacion compuesta multiplicacion | Asigna un valor a una variable multiplicandole el valor de otra variable |
| /=       | Asignacion compuesta division | Asigna un valor a una variable dividiendole el valor de otra variable |
| %=       | Asignacion compuesta modulo | Asigna un valor a una variable obteniendo el residuo de la division entre el valor de otra variable |

```rust
let mut a = 10;
let b = 5;
a = b; // a = 5
a += b; // a = 10
a -= b; // a = 5
a *= b; // a = 25
a /= b; // a = 5
a %= b; // a = 0
```

### Expresiones typecast
Los operadores typecast son los operadores que se usan para convertir un valor de un tipo a otro.

| Operador | Nombre | Descripcion |
|----------|-------------| ----------- |
| as       | Typecast    | Convierte un valor de un tipo a otro |

```rust
let a = 10;
let b = a as f32; // b = 10.0
```
_Nota: Los typecast solo se pueden hacer entre tipos numericos_

### Expresiones Borrow
Los operadores borrow son los operadores que se usan para obtener y almacenar la direccion de memoria de una variable.

| Operador | Nombre | Descripcion |
|----------|-------------| ----------- |
| &        | Shared Borrow      |Puede leer datos de la direccion de memoria |
| &mut     | Mutable Borrow  | Puede leer y alterar datos de la direccion |

```rust
let a = 10;
let b = &a; // b = 10
let mut c = 10;
let d = &mut c; // d = 10
```

### Derreferenciacion
Los operadores derreferenciacion sirven para cambiar el valor de una variable a traves de su direccion de memoria.

| Operador | Nombre | Descripcion |
|----------|-------------| ----------- |
| *        | Dereference      | Cambia el valor de una variable a traves de su direccion de memoria |

```rust
let mut a = 10;
let b = &mut a; // b = 10
*b = 5; // a = 5
```

