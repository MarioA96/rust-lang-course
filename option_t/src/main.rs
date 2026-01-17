// Esto es por default por parte de Rust, sin necesidad de declarar de forma explicita
// enum Option<T>{
//     None,
//     Some(T),
// }

struct Usuario {
    nombre: String,
    edad: u8,
}

fn main() {
    let valor = Some(42);
    let texto = Some(String::from("Mario"));

    match valor {
        Some(numero) => println!("El numero es: {}", numero),
        None => println!("No hay numero"),
    }

    match texto {
        Some(nombre) => println!("Tu nombre es: {}", nombre),
        None => println!("No hay nada"),
    }

    let usuario = Some(Usuario {
        nombre: String::from("Mario"),
        edad: 28,
    });

    match usuario {
        Some(us) => println!("El usuario es: {} y su edad es: {}", us.nombre, us.edad),
        None => println!("Ni se ingreso nada"),
    }
}
