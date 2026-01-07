use std::io;


fn main(){

    println!("Basic Calculator!");

    loop {
        // We keep these inside because of the buffer, once the variable is created we need to store the std input to buffer
        // so, if keep it outside the stdin would place it in u32 type instead of the String type
        let mut n_1 = String::new();
        let mut n_2 = String::new();
        let mut operation_sym = String::new();

        println!("Enter the first number: ");
        io::stdin().read_line(&mut n_1).expect("Error while reading input");

        println!("Enter the second number: ");
        io::stdin().read_line(&mut n_2).expect("Error while reading input");

        let n_1: u32 = n_1.trim().parse().expect("Please type a number!");
        let n_2: u32 = n_2.trim().parse().expect("Please type a number!");

        println!("Enter the desired operation (symbol: +, -, *, /)");
        io::stdin().read_line(&mut operation_sym).expect("Error while reading input");

        match operation_sym.trim() {
            "+" => println!("Result: {}", n_1 + n_2),
            "-" => println!("Result: {}", n_1 - n_2),
            "*" => println!("Result: {}", n_1 * n_2),
            "/" => println!("Result: {}", n_1 / n_2),
            _ => println!("Invalid operation!"),
        }

        // Inner loop for continue prompt
        loop {
            let mut continue_sym = String::new();

            println!("Do you wish to continue? (y/n)");
            io::stdin().read_line(&mut continue_sym).expect("Error while reading input");
            
            let cont = continue_sym.trim();

            if cont == "y" {
                break; // break inner loop, continue outer loop
            } else if cont == "n" {
                return; // exit the program
            } else {
                println!("Not a valid option <nor 'y/n' entered as input>");
                println!("Try again");
            }
        }

    }

}











// fn main() {
    
//     println!("Simple calculator");
//     println!("Valid operations: +, -, *, /");

//     loop {
//         let mut input = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Error while reading input");

//         let i_len: usize = input.len();
//         let symbol = input.chars().nth(i_len - 1).unwrap_or('\n');

//         if symbol == '\n'{
//             break;
//         }
//     }

//     println!("End");
// }
