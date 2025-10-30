

use std::io::{self, Write};

#[derive(Debug)]
enum ConversionType {
    Temperature,
    Length,
    Weight,
    Quit,
}

#[derive(Debug)]
enum TemperatureConversion {
    CToF,
    FToC,
}

#[derive(Debug)]
enum LengthConversion {
    KmToMi,
    MiToKm,
}

#[derive(Debug)]
enum WeightConversion {
    KgToLb,
    LbToKg,
}

fn main() {
    println!("=== Rust Unit Converter test ===");

    loop {
        println!("\nSelect conversion type:");
        println!("1) Temperature");
        println!("2) Length");
        println!("3) Weight");
        println!("4) Quit");

        let choice = read_menu_choice(4);
        println!("You have selected option {choice}")

        // match choice {
        //     1 => handle_temperature(),
        //     2 => handle_length(),
        //     3 => handle_weight(),
        //     4 => {
        //         println!("Goodbye!");
        //         break;
        //     }
        //     _ => unreachable!(), // read_menu_choice guarantees 1..=4
        // }
    }
}

// Handles Temperature flow (menu -> input -> conversion -> print)
// #region Handle Temperature
// fn handle_temperature() {
//     println!("\nTemperature conversions:");
//     println!("1) Celsius -> Fahrenheit");
//     println!("2) Fahrenheit -> Celsius");

//     let choice = read_menu_choice(2);
//     let conversion = match choice {
//         1 => TemperatureConversion::CToF,
//         2 => TemperatureConversion::FToC,
//         _ => unreachable!(),
//     };

//     let value = read_value_f64("Enter value to convert:");

//     let result = match conversion {
//         TemperatureConversion::CToF => c_to_f(value),
//         TemperatureConversion::FToC => f_to_c(value),
//     };

//     match conversion {
//         TemperatureConversion::CToF => {
//             println!("{:.2} °C = {:.2} °F", value, result);
//         }
//         TemperatureConversion::FToC => {
//             println!("{:.2} °F = {:.2} °C", value, result);
//         }
//     }
// }
// #endregion 

// Handles Length flow (km <-> mi)
// fn handle_length() {
//     println!("\nLength conversions:");
//     println!("1) Kilometers -> Miles");
//     println!("2) Miles -> Kilometers");

//     let choice = read_menu_choice(2);
//     let conversion = match choice {
//         1 => LengthConversion::KmToMi,
//         2 => LengthConversion::MiToKm,
//         _ => unreachable!(),
//     };

//     let value = read_value_f64("Enter value to convert:");

//     let result = match conversion {
//         LengthConversion::KmToMi => km_to_mi(value),
//         LengthConversion::MiToKm => mi_to_km(value),
//     };

//     match conversion {
//         LengthConversion::KmToMi => {
//             println!("{:.2} km = {:.2} mi", value, result);
//         }
//         LengthConversion::MiToKm => {
//             println!("{:.2} mi = {:.2} km", value, result);
//         }
//     }
// }

//  Handles Weight flow (kg <-> lb)
// fn handle_weight() {
//     println!("\nWeight conversions:");
//     println!("1) Kilograms -> Pounds");
//     println!("2) Pounds -> Kilograms");

//     let choice = read_menu_choice(2);
//     let conversion = match choice {
//         1 => WeightConversion::KgToLb,
//         2 => WeightConversion::LbToKg,
//         _ => unreachable!(),
//     };

//     let value = read_value_f64("Enter value to convert:");

//     let result = match conversion {
//         WeightConversion::KgToLb => kg_to_lb(value),
//         WeightConversion::LbToKg => lb_to_kg(value),
//     };

//     match conversion {
//         WeightConversion::KgToLb => {
//             println!("{:.2} kg = {:.2} lb", value, result);
//         }
//         WeightConversion::LbToKg => {
//             println!("{:.2} lb = {:.2} kg", value, result);
//         }
//     }
// }

// Read a menu choice (1..=max). Re-prompts until valid.
fn read_menu_choice(max: usize) -> usize {
    loop {
        print!("Enter your choice: ");
        io::stdout().flush().expect("flush failed");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input. Try again.");
            continue;
        }

        match input.trim().parse::<usize>() {
            Ok(n) if n >= 1 && n <= max => return n,
            _ => println!("Please enter a number between 1 and {}.", max),
        }
    }
}

//  Read a floating point value (f64). Re-prompts until valid.
// fn read_value_f64(prompt: &str) -> f64 {
//     loop {
//         println!("{}", prompt);
//         print!("> ");
//         io::stdout().flush().expect("flush failed");

//         let mut input = String::new();
//         if io::stdin().read_line(&mut input).is_err() {
//             println!("Failed to read input. Try again.");
//             continue;
//         }

//         match input.trim().parse::<f64>() {
//             Ok(v) => return v,
//             Err(_) => println!("Please enter a valid number (e.g., 12.5)."),
//         }
//     }
// }

/* ----- Conversion functions (pure, easy to test) ----- */

// fn c_to_f(c: f64) -> f64 {
//     (c * 9.0 / 5.0) + 32.0
// }

// fn f_to_c(f: f64) -> f64 {
//     (f - 32.0) * 5.0 / 9.0
// }

// fn km_to_mi(km: f64) -> f64 {
//     km * 0.621371
// }

// fn mi_to_km(mi: f64) -> f64 {
//     mi / 0.621371
// }

// fn kg_to_lb(kg: f64) -> f64 {
//     kg * 2.20462
// }

// fn lb_to_kg(lb: f64) -> f64 {
//     lb / 2.20462
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn temp_conversions() {
//         assert!((c_to_f(0.0) - 32.0).abs() < 1e-6);
//         assert!((f_to_c(32.0) - 0.0).abs() < 1e-6);
//     }

//     #[test]
//     fn length_conversions() {
//         assert!((km_to_mi(1.0) - 0.621371).abs() < 1e-6);
//         assert!((mi_to_km(1.0) - (1.0 / 0.621371)).abs() < 1e-6);
//     }

//     #[test]
//     fn weight_conversions() {
//         assert!((kg_to_lb(1.0) - 2.20462).abs() < 1e-6);
//         assert!((lb_to_kg(2.20462) - 1.0).abs() < 1e-6);
//     }
// }
