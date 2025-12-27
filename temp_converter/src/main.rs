use std::io;

fn main() {
    println!("Temperture converter");

    println!("Enter the value of temperature...");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value:f32 = value
        .trim()
        .parse()
        .expect("Not a number entered!");
    
    println!("Enter f-c for conversion from Fahrenheit to Celcius");
    println!("Enter c-f for conversion from Celcius to Fahrenheit");

    let mut label = String::new();

    io::stdin()
        .read_line(&mut label)
        .expect("Failed to read line");

    let label = label
        .trim()
        .to_lowercase();

    if label != "c-f" {
        if label == "f-c" {
            fahrenheit_to_celcius(value);
        } else if label != "f-c" {
            println!("expected 1 valid String entered, options: <c-f or f-c>");
        }
    } else {
        celcius_to_fahrenheit(value);
    }
}


fn fahrenheit_to_celcius(value: f32){
    let float_value = value as f32;
    let conversion: f32 = (5.0/9.0)*(float_value-32.0);

    println!("Fahrenheit to celcius: {conversion}");
}

fn celcius_to_fahrenheit(value: f32){
    println!("Entered c-f");

    let float_value = value as f32;
    let conversion: f32 = ((9.0/5.0)*float_value)+32.0;

    println!("Fahrenheit to celcius: {conversion}");
}