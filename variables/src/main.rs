/*
fn main() {
    
    // let x = 5; // Las variables son inmutables por default
    // asi que declarar una variable mut con constantes de esa forma no es recomendable
    // puesto a que si se desea ser constante, debe ser mediante const unicamente

    let mut x = 5;
    println!("Te value of x is: {x}");

    x = 6;
    println!("Te value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Valor de la constante: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    // se puede declarar una nueva variable con el mismo nombre como la variable previa
    // con ello se dice que la primer variable es "opacada"(shadowed) por la segunda
    // lo cual sera lo que vera el compilador
    // Asi, podemos "opacar" una variable el mimo nombre de la variable y repetir el uso de let

    let y = 5; // Si, es inmutable por default
    // por ello vuelvo a declarar la variable y ocupo su valor previo y lo ocupo
    // le estoy haciendo shadowing
    let y = y + 1;

    { // Este scope solo ocurre aqui
        let y = y * 2;
        println!("El valor de la variable y en el scope: {y}");
    }

    println!("El valor de y fuera del scope: {y}");

    // Mediante shadowing podemos incluso cambiar el tipo de la variable
    let spaces = "    ";
    let spaces = spaces.len();

    // Al contrario, esto no es posible ante el uso de la palabra clave mut
    // que los establece como de tipo mutable
    // Esto arrojara un error
    //let mut spaces2 = "   ";
    // spaces2 = spaces2.len(); // no podemos cambiar el tipo de una variable mutable



    // Data types

    // Solo podemos inferir el tipo de dato basado en el valor y como este se usa
    // esto arrojara un error si quito el :u32
    let guess:u32 = "42".parse().expect("Not a number");

    // Valores escalares (un valor simple)
    // Enteros
    //      Signed   Unsigned
    /*
                i8      u8
                i16     u16
                i32     u32
                i64     u64
                i128    u128
dep. de arq.    isize   usize

    
    Number Literals     Example
        Decimal         98_222
        Hex             0xff
        Octal           0o77
        Binary          0b1111_0000
        Byte(u8 only)   b'A'
    */

    // Integer overflow
    // Cuando se desconoce hasta donde puede llegar a cubrir la declaracion y uso de la variable
    // este puede provocar un desbordamiento, como usar u8 y tratar de cargar un dato de u32
    // lo default es i32, el uso de isize y usize es mas que nada con colecciones
    
    // Se pueden utilizar wrappers para "soportar" inconvenientes sobre desbordamientos
    // mediante la biblioteca estandar para primitivos de tipo numericos
    // - Wrap in all modes with the wrapping_* methods, such as wrapping_add.
    // - Return the None value if there is overflow with the checked_* methods.
    // - Return the value and a Boolean indicating whether there was overflow with the overflowing_* methods.
    // - Saturate at the value’s minimum or maximum values with the saturating_* methods.

    

    // Float
    // Los tipos flotantes usan el standard EEE-754.
    let x = 2.0; // f64
    let y: f32 = 3.0 // f32

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // truncated = -1

    // remainder
    let remainder = 43 % 5;

    

    // Boolean
    let t = true;
    let f:bool = false; // Con anotacion explicita de tipo




    // Characters
    // Los chars literales son especificados mediante ' '
    // Las cadenas literales son mediante ""
    // los chars tienen un tipo de 4 bytes en tamaño y representa un valor Unicode escalar

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';



    // Tipos compuestos
    // agrupacion de multiples valores en un tipo

    // Tupla
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x,y,z) = tup; // Desestructuracion

    println!("El valor de y es: {y}");

    let obj = {
        x: 13,
        y: 21.7
    }
    
    let x:(i32, f64,u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;


    // Array
    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Estableciendo el tipo y numero de elementos
    let b = [3; 5]; // declarando un arreglo, que dato almacenara y el numero de elementos

    let c = [1,2,3,4,5];
    let valC_cero = c[0]// Aceso a valores del arreglo
    let valC_uno = c[1]
}
*/
use std::io;


fn main()
{
    let a = [1,2,3,4,5];

    println!("Please enter the index: ");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index:usize = index
        .trim()
        .parse()
        .expect("Index was not a number!");

    let element = a[index];
    println!("Index: {index}, number given index: {element}");

}