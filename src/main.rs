use std::io;

fn main() {
    println!("Fahrenheit-celsius converter.");
    println!("Which do you ipnut, Celsius or Fahrenheit?");

    println!("Input temperature(only number).");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line.");

    let temperature: f64 = temperature.trim().parse()
        .expect("Input a number.");

    println!("Please type unit(C or F).");

    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line.");

    let unit = unit.trim();    

    match unit {
        "C" | "c" => println!("{} °C is {} °F.", temperature, celsius_to_fahrenheit(temperature)),
        "F" | "f" => println!("{} °F is {} °C.", temperature, fahrenheit_to_celsius(temperature)),
        _ => println!("Invalid unit. Input C or F"),
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0  / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}