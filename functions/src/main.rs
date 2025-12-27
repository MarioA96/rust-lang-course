

fn main()
{
    println!("Hello World!");

    another_function(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("the value of y: {y}");

    let num = five();
    println!("five: {num}");
    println!("five_f: {:?}", five());

    let foo = |x: i32, y:i32| -> () {
        let sum = x + y;
        println!("Arrow function sum: {sum}");
    };

    foo(14, 15);

    let initial_value = 15;
    println!("initial_value: {initial_value}, plus one: {:?}", plus_one(initial_value));
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, unit_label: char)
{
    println!("values, x: {x}, char: {unit_label}");
}