
//Basic Math Calculator

use std::io;
// use colored::Colorize;

fn main() {
        
    println!("Basic Math Calculator");
    println!("\nEnter a number to compute its factorial (or type 'fibo' for Fibonacci): ");

    let mut entry = String::new();
    io::stdin().read_line(&mut entry).expect("Couldn't read line");

    let entry = entry.trim();

    //* It could be easier/simpler to just ask the number and then proceed to ask the type of operation, but
    //* This a demonstration in order to try new operations such as parsing and limiting the option based on its type */

    if entry.parse::<i32>().is_ok(){
        //todo DONE!
        let entry: i32 = entry.parse().expect("Not a valid entry");
        let result: i32 = factorial(entry);

        println!("The result is: {}", result);
    } else {
        //todo We need to check the word, it must say 'fibo' otherwise it should throw an error and try again
        if word_checker_fibo(){
            //todo We ask then for how many numbers to generate
            println!("How many Fibonacci numbers to show?");
            let mut num_entry = String::new();

            io::stdin().read_line(&mut num_entry).expect("Something went wrong");
            //* Here I'll just throw a panic in order to no over extend complexity!
            let num_entry:i32 = num_entry.trim().parse().expect("Not a valid entry <Not a number!>");

            generate_sequence_fibo(num_entry);
        } else{ //todo Not the fibo word entered
            println!("Option entered is not valid <expected options: 'fibo'>");
        } 
    }

}


fn factorial(num: i32) -> i32{

    // n! = (n)(n-1)! -> 4! = 4*(4-1)! = 4*(3*(3-1)!) = 4*(3*(2*(2-1)!)) = 4*(3*(2*(1))) = 4*(3*(2)) = 4*(6) = 24
    // 1! = 1, 2! = 2, 3! = 3*(2)! = 6
    //         ^_______________^   
    let mut v_arr: Vec<i32> = vec![1,1,2];

    if num<0{
        return -1; //* Not a valid num to calculate its factorial */
    } else if num<=2{
        return v_arr[num as usize];
    } else {
        
        for i in 3..=num {
            v_arr.push( i * v_arr[(i-1) as usize] );
        }

        return v_arr[ num as usize ];
    }

}

fn word_checker_fibo() -> bool{

    return true;
}

fn generate_sequence_fibo(num: i32){

    println!("Printed sequence of {}", num);
}