use std::{io, fs, path::Path};


const FILE_NAME: &str = "TODOS.txt";
const PATH: &str = "./";

fn main() {
    
    println!("Todo List App");
    println!("Checking for existing file...\n");

    if !on_check_file() {
        println!("No {FILE_NAME} found!");

        on_create_new_file();

        println!("A new file: {FILE_NAME} has been made\n");
    }
    
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
                print_todos();
            },
            "2" => {
                if on_add_new_todo(){
                    println!("New TODO added succesfuly!\n");
                } else {
                    println!("Adding new TODO failed!\n");
                }
            },
            "3" => println!("Removing a TODO from the list..."),
            "4" => {
                println!("Exiting the app...");
                return;
            },
            _ => println!("Invalid operation!"),
        }

    }

}


fn on_check_file() -> bool {
    let path = PATH.to_string()+FILE_NAME;
    let path = path.as_str();

    if !Path::new(path).exists() {
        return false;
    } else {
        return true;
    }
}

fn on_create_new_file() {
    let path = PATH.to_string()+FILE_NAME;
    let path = path.as_str();

    fs::write(path, "").expect("Failed to write to file");
}

fn print_todos() {

    let data = fs::read(PATH.to_string()+FILE_NAME).expect("Should be able to read file");

    if data.len() == 0 {
        println!("No TODOS to display <List is empty>\n");
    } else {
        match String::from_utf8(data) {
            Ok(text) => println!("{}", text),
            Err(_) => println!("Error: Could not decode file contents as UTF-8"),
        }
    }

}

fn get_number_lines_file() -> i32 {
    return 1;
}

fn on_add_new_todo() -> bool{

    println!("\nSet the title of the new TODO");

    let mut title = String::new();

    io::stdin().read_line(&mut title)
        .expect("Error while reading line!");


    let path = PATH.to_string()+FILE_NAME;
    let path = path.as_str();

    println!("\nAdding a new TODO to the list...");

    if !on_check_file() {
        eprintln!("Couldn't write on the file <file stream failed, moved or deleted file?>");

        return false;
    } else {
        let num_lines = (get_number_lines_file() + 1).to_string();
        let new_title = "\n".to_string()+num_lines.as_str()+".- "+title.as_str();

        fs::write(path, new_title).expect("Failed to write to file");
        return true;
    }
}