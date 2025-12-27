

fn main() {
    
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let condition = false;
    let number = if condition {5} else {6};

    println!("The value of number is: {number}");

    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    println!("\n");


    // Labeled loop
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            
            if remaining == 9 {
                break; // we break the inner loop
            }

            if count == 2 {
                break 'counting_up; // we break the labeled loop
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count: {count}");


    println!("\n");


    let mut an_number = 3;

    while an_number != 0 {
        println!("{an_number}");

        an_number -= 1;
    }

    println!("LIFTOFF!!!");


    println!("\n");


    // looping throught a collection
    let arr = [1,2,3,4,5];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", arr[index]);

        index += 1;
    }


    println!("\n");


    let brr = [1,2,3,4,5];

    for element in brr {
        println!("The value is: {element}");
    }


    println!("\n");


    for number in (1..4).rev() {
        println!("value reversed: {number}!");
    }
    println!("LIFTOFF!!!");
}