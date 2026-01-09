use std::{io, fs};


const PATH: &str = "./TODOS.txt";

fn main() {
    
    println!("Todo List App");
    println!("Checking for existing file...");


    
    loop {

        println!("Seleccione una opcion:");
        println!("1.- Mostrar lista de TODOs, 2.- Agregar nuevo TODO, 3.- Eliminar un TODO, 4.- Salir");

        let mut opcion = String::new();

        io::stdin()
            .read_line(&mut opcion)
            .expect("Couldn't read line");

        match opcion.trim() {
            "1" => {
                println!("Displaying list of TODOs...");
                get_todos();
                //todo ask_to_continue fn -> not needed since the options are inside a loop for continous operations
            },
            "2" => println!("Adding a new TODO to the list..."),
            "3" => println!("Removing a TODO from the list..."),
            "4" => {
                println!("Exiting the app...");
                return;
            },
            _ => println!("Invalid operation!"),
        }

    }

}


fn get_todos() {

    let data = fs::read(PATH).expect("Should be able to read file");
    println!("{}", data.len());
    
}