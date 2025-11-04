use std::io;

pub fn conversion() {
    println!("\nWhich unit to convert from?\n(1) km/h  (2) m/s\n(3) mi/h  (4) ft/s  (5) knots");
    let mut unit1 = String::new();
    let _ = io::stdin().read_line(&mut unit1);
    let unit1: u8 = unit1.trim().parse().unwrap_or(0);

    println!("\nInput your unit:");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let mut input: f64 = input.trim().parse().expect("Invalid input!");

    println!("\nWhich unit to convert to?\n(1) km/h  (2) m/s\n(3) mi/h  (4) ft/s  (5) knots");
    let mut unit2 = String::new();
    let _ = io::stdin().read_line(&mut unit2);
    let unit2: u8 = unit2.trim().parse().unwrap_or(0);

    match unit1 {
        1 => input /= 3.6,
        2 => input = input,
        3 => input *= 0.44704,
        4 => input *= 0.3048,
        5 => input /= 1.943844,
        _ => {
            println!("Invalid input!");
            return;
        }
    };

    match unit2 {
        1 => input *= 3.6,
        2 => input = input,
        3 => input /= 0.44704,
        4 => input /= 0.3048,
        5 => input *= 1.943844,
        _ => {
            println!("Invalid Input");
            return;
        }
    };

    let mut unit0 = "";

    match unit2 {
        1 => unit0 = "km/h",
        2 => unit0 = "m/s",
        3 => unit0 = "mi/h",
        4 => unit0 = "ft/s",
        5 => unit0 = "kn",
        _ => {
            println!("Invalid Input");
            return;
        }
    }
    println!("the conversion is {input}{unit0}");
}
