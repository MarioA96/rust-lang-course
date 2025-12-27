use std::io;
use std::vec;

fn main() {

    println!("Enter the number of the nth fibonacci value");
    println!("The entered value should be greater or equal to 0");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Couldnt read line");

    let index:i32 = index
        .trim()
        .parse()
        .expect("Not a number entered");

    if index < 0 {
        eprintln!("Not a valid number entered <expected number greater or equal to 0>");
    }

    let mut buffer = vec![1,1,2];

    if index == 0 { 
        println!("{index}th fibonacci: {}", buffer[0]); 
    } else if index == 1 { 
        println!("{index}st fibonacci 1: {}", buffer[1]); 
    } else if index == 2 { 
        println!("{index}nd fibonacci: {}", buffer[2]); 
    } else if index > 2 {

        let mut i = 3;
        while i <= index {
            //println!("{}", buffer[(i as usize)-1]);
            //println!("{}", buffer[i]);
            buffer.push( buffer[(i as usize)-1]+buffer[(i as usize)-2] );

            i += 1;
        }
        println!("{index}th fibonacci: {}", buffer[index as usize]);
    }

}
