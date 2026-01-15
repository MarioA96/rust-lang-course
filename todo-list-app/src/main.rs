use colored::Colorize;
use std::io::Write;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader},
    path::Path,
};

const FILE_NAME: &str = "TODOS.txt";
const PATH: &str = "./";

fn main() {
    println!(
        "{}",
        "\n\nTodo List App\nChecking for existing file...\n"
            .bold()
            .purple()
    );

    if !on_check_file() {
        println!("No {FILE_NAME} found!");

        on_create_new_file();

        println!("A new file: {FILE_NAME} has been made\n");
    }

    loop {
        println!("{}", "Seleccione una opcion:".bold().yellow());
        println!(
            "1.- Mostrar lista de TODOs\n2.- Agregar nuevo TODO\n3.- Eliminar un TODO\n4.- Salir"
        );

        let mut opcion = String::new();

        io::stdin()
            .read_line(&mut opcion)
            .expect("Couldn't read line");

        match opcion.trim() {
            "1" => {
                println!("{}", "\nDisplaying list of TODOs:".bold().blue());
                print_todos();
            }
            "2" => {
                if on_add_new_todo() {
                    println!("New TODO added succesfuly!\n");
                } else {
                    println!("Adding new TODO failed!\n");
                }
            }
            "3" => {
                let mut id_todo = String::new();

                println!("\nEnter the number of TODO to DELETE: ");

                io::stdin()
                    .read_line(&mut id_todo)
                    .expect("Failed to read line");
                let id_todo: i32 = id_todo
                    .trim()
                    .parse()
                    .expect("Failed to read entry <Not a number. Entry must be an integer>");

                println!("{}", "\nRemoving a TODO from the list...".bold().blue());

                if on_delete_todo(id_todo) {
                    println!("{}", "Deleted TODO succesfuly!\n".green());
                } else {
                    println!("Deleting TODO failed!\n");
                }
            }
            "4" => {
                println!("Exiting the app...");
                return;
            }
            _ => println!("Invalid operation!"),
        }
    }
}

fn on_check_file() -> bool {
    let path = PATH.to_string() + FILE_NAME;
    let path = path.as_str();

    if !Path::new(path).exists() {
        return false;
    } else {
        return true;
    }
}

fn on_create_new_file() {
    let path = PATH.to_string() + FILE_NAME;
    let path = path.as_str();

    fs::write(path, "").expect("Failed to write to file");
}

fn print_todos() {
    let path = PATH.to_string() + FILE_NAME;
    let path = path.as_str();

    let reader = BufReader::new(
        File::open(path)
            .expect("Couldn't READ on the file <file stream failed, moved or deleted file?>"),
    );

    for (index, line) in reader.lines().enumerate() {
        let i = index + (1 as usize);
        println!("{i}.- {}", line.expect("Failed while retrieving line"));
    }

    println!("\n");
}

fn get_number_lines_file() -> i32 {
    if !on_check_file() {
        eprintln!("Couldn't write on the file <file stream failed, moved or deleted file?>");

        return -1;
    } else {
        let path = PATH.to_string() + FILE_NAME;
        let path = path.as_str();

        let file = BufReader::new(
            File::open(path)
                .expect("Couldn't READ on the file <file stream failed, moved or deleted file?>"),
        );

        // println!("Number of lines: {}", file.lines().count());

        return file.lines().count() as i32;
    }
}

fn on_add_new_todo() -> bool {
    println!("\nSet the title of the new TODO");

    let mut title = String::new();

    io::stdin()
        .read_line(&mut title)
        .expect("Error while reading line!");

    let path = PATH.to_string() + FILE_NAME;
    let path = path.as_str();

    println!("\nAdding a new TODO to the list...");

    if !on_check_file() {
        eprintln!("Couldn't WRITE on the file <file stream failed, moved or deleted file?>");

        return false;
    } else {
        // let num_lines = (get_number_lines_file() + 1).to_string();
        // let new_title = num_lines + ".- " + title.as_str();
        let new_title = title.as_str();

        let mut file = OpenOptions::new()
            .append(true)
            .open(path)
            .expect("Error while trying to do operation over file <permission denied, not enough privileges?>");

        write!(file, "{}", new_title).expect("Failed to append to the file");

        return true;
    }
}

fn on_delete_todo(id: i32) -> bool {
    if !on_check_file() {
        eprintln!("Couldn't take action on the file <file stream failed, moved or deleted file?>");

        return false;
    } else {
        if id > get_number_lines_file() || id < 0 {
            eprintln!("The id entered doesnt exist in the list <is the id correct?>");

            return false;
        } else {
            let path = PATH.to_string() + FILE_NAME;
            let path = path.as_str();

            let reader =
                BufReader::new(File::open(path).expect(
                    "Couldn't READ on the file <file stream failed, moved or deleted file?>",
                ));

            let mut lines: Vec<String> = Vec::new();

            for line in reader.lines() {
                let line = line.expect("Couldnt retrieve line");

                lines.push(line);
            }

            let filtered_lines: Vec<String> = lines
                .into_iter()
                .enumerate()
                .filter(|&(index, _)| ((id - 1) as usize) != index)
                .map(|(_, element)| element)
                .collect();

            let filtered_lines = filtered_lines.join("\n");

            fs::write(path, filtered_lines + "\n").expect("Failed to write to file");

            return true;
        }
    }
}
