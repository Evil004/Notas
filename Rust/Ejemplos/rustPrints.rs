fn main() {
    println!("Hello, World!") // Imprimir una cadena de caracteres en este caso "Hello, World"

    println!("{}, {}", "Hello", "World"); // Imrime un String que se le han pasado los argumentos por valores

    println!("{1}, {0}, {0}", "Hello", "World");// Imrime un String que se le han pasado los argumentos por valores indicando su posicion
        
    println!("{greeting}, {name}", greeting="Hello", name="World");// Imrime un String que se le han pasado los argumentos por valores y se le ha indicado el nombre del valor

    println!("Numero: 10 en binario:{:b}, hexadecimal:{:x}, octal:{:o}", 10, 10, 10); // convierte el argumento a binario, hexadecimal y octal

    println!("{:?}", ("Hello", "World")); // Imprime varios valores a la vez

    //Tipos de print

    print!("Hello, World"); // imprime por consola sin saltar de linea.
    println!("Hello, World"); // imprime por consola y salta de linea.
    eprint!("Hello, World"); // imprime por consola un error sin saltar de linea.

    eprintln!("Hello, World");// imprime por consola un error y salta de linea.


    // Comentario de linea

    /*
    * Esto es un comentario en bloque
    * Esto es el mismo comentario en bloque
    */
    

}