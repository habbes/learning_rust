use std::io;

fn main() {
    loop {
        println!("Enter value in fahrenheit to convert to celcius and value in celcius to convert in fahrenheit.");
        println!("Use suffix F for Fahrenheit (e.g. 30F) and C for Celcius (e.g. 30C)");
        println!("Press q to exit");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!.");
        
        let input = input.trim();

        if input.is_empty() {
            println!("Failed to read.");
            continue;
        }

        if input == "q" {
            break;
        }

        let value: f64 = match input.get(0..(input.len() - 1)) {
            Some(raw_val) => match raw_val.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Failed to parse value. Please enter a valid number.");
                    continue;
                }
            },
            None => {
                println!("Failed to read.");
                continue;
            }
        };

        let (result_value, original_unit, result_unit) = if input.ends_with('F') {
            (convert_fahr_to_celc(value), 'F', 'C')
        } else if input.ends_with('C') {
            (convert_celc_to_fahr(value), 'C', 'F')
        } else {
            println!("Unsupported unit");
            continue;
        };


        println!("{value}{original_unit} = {result_value}{result_unit}");
    }

}

fn convert_fahr_to_celc(fahr_value: f64) -> f64 {
    (fahr_value - 32.0) * (5.0 / 9.0)
}

fn convert_celc_to_fahr(celc_value: f64) -> f64 {
    celc_value * 9.0 / 5.0 + 32.0
}