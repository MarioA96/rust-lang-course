//Basic Math Calculator

use std::io;
// use colored::Colorize;

fn main() {
    println!("Basic Math Calculator");

    loop {
        calc_operations();

        if !should_continue(){
            break;
        }
    }
}

fn factorial(num: i32) -> i32 {
    // n! = (n)(n-1)! -> 4! = 4*(4-1)! = 4*(3*(3-1)!) = 4*(3*(2*(2-1)!)) = 4*(3*(2*(1))) = 4*(3*(2)) = 4*(6) = 24
    // 1! = 1, 2! = 2, 3! = 3*(2)! = 6
    //         ^_______________^
    let mut v_arr: Vec<i32> = vec![1, 1, 2];

    if num < 0 {
        return -1; //* Not a valid num to calculate its factorial */
    } else if num <= 2 {
        return v_arr[num as usize];
    } else {
        for i in 3..=num {
            v_arr.push(i * v_arr[(i - 1) as usize]);
        }

        return v_arr[num as usize];
    }
}

fn word_checker_fibo(entry: &str) -> bool {
    if entry == "fibo" {
        return true;
    } else {
        return false;
    }
}

fn generate_sequence_fibo(num: i32) -> i32 {
    let mut buffer: Vec<i32> = vec![1, 1];
    // [1,1,2,3,5,8,13,21,34,55,89,144,233]
    // [0,1,2]

    if num < 0 {
        return -1;
    } else if num < 2 {
        return 1;
    } else {
        // for i=0; i<num; i++
        for i in 2..(num + 1) {
            buffer.push(buffer[(i - 1) as usize] + buffer[(i - 2) as usize]);
        }

        return buffer[(num - 1) as usize];
    }
}

fn calc_operations() {
    println!("\nEnter a number to compute its factorial (or type 'fibo' for Fibonacci): ");

    let mut entry = String::new();
    io::stdin()
        .read_line(&mut entry)
        .expect("Couldn't read line");

    let entry = entry.trim();

    //* It could be easier/simpler to just ask the number and then proceed to ask the type of operation, but
    //* This a demonstration in order to try new operations such as parsing and limiting the option based on its type */
    if entry.parse::<i32>().is_ok() {
        //todo DONE!
        let entry: i32 = entry.parse().expect("Not a valid entry");
        let result: i32 = factorial(entry);

        println!("{}! = {}", entry, result);
    } else {
        //todo We need to check the word, it must say 'fibo' otherwise it should throw an error and try again
        if word_checker_fibo(entry) {
            //todo We ask then for how many numbers to generate
            println!("How many Fibonacci numbers to show?");
            let mut num_entry = String::new();

            io::stdin()
                .read_line(&mut num_entry)
                .expect("Something went wrong");
            //* Here I'll just throw a panic in order to no over extend complexity!
            let num_entry: i32 = num_entry
                .trim()
                .parse()
                .expect("Not a valid entry <Not a number!>");

            let result = generate_sequence_fibo(num_entry);
            println!("Fib({}) = {}", num_entry, result);
        } else {
            //todo Not the fibo word entered
            println!("Option entered is not valid <expected options: 'fibo'>");
        }
    }
}

fn should_continue() -> bool {
    loop {
        println!("Do you wish to continue? <Y/N>");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Couldn't read line");

        let opt = option.trim();

        if opt == "y" {
            return true; // break inner loop, continue outer loop
        } else if opt == "n" {
            return false; // exit the program
        } else {
            println!("Not a valid option <nor 'y/n' entered as input>");
            println!("Try again");
        }
    }
}
