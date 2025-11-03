use std::io;

pub fn conversion() {
    println!("\nWhich unit to convert from?\n(1) meters  (2) kilometers  (3)  centimeters\n(4) feet  (5) miles  (6) inches");
    let mut unit1 = String::new();
    let _ = io::stdin().read_line(&mut unit1);
    let unit1: u8 = unit1.trim().parse().unwrap_or(0);

    println!("\nInput your unit:");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let input: f64 = input.trim().parse().expect("Invalid input!");

    println!("\nWhich unit to convert to?\n(1) meters  (2) kilometers  (3)  centimeters\n(4) feet  (5) miles  (6) inches");
    let mut unit2 = String::new();
    let _ = io::stdin().read_line(&mut unit2);
    let unit2: u8 = unit2.trim().parse().unwrap_or(0);

    match unit1 {
        1 => Some(if unit2 == 1 {
            println!("The conversion is {input}m");
        } else if unit2 == 2 {
            let input = input / 1000.0;
            println!("The conversion is {input}km");
        } else if unit2 == 3 {
            let input = input * 100.0;
            println!("The conversion is {input}cm");
        } else if unit2 == 4 {
            let input = input * 3.280839895;
            println!("The conversion is {input}ft");
        } else if unit2 == 5 {
            let input = input * 0.00062137;
            println!("The conversion is {input}mi");
        } else if unit2 == 6 {
            let input = input * 39.3700787402;
            println!("The conversion is {input}in");
        } else {
            println!("Invalid Input!");
            return;
        }),
        2 => Some(if unit2 == 1 {
            let input = input * 1000.0;
            println!("The conversion is {input}m");
        } else if unit2 == 2 {
            println!("The conversion is {input}km");
        } else if unit2 == 3 {
            let input = input * 100_000.0;
            println!("The conversion is {input}cm");
        } else if unit2 == 4 {
            let input = input * 3280.839895;
            println!("The conversion is {input}ft");
        } else if unit2 == 5 {
            let input = input * 0.62137;
            println!("The conversion is {input}mi");
        } else if unit2 == 6 {
            let input = input * 39_370.0787402;
            println!("The conversion is {input}in");
        } else {
            println!("Invalid Input!");
            return;
        }),
        3 => Some(if unit2 == 1 {
            let input = input / 100.0;
            println!("The conversion is {input}m");
        } else if unit2 == 2 {
            let input = input / 100000.0;
            println!("The conversion is {input}km");
        } else if unit2 == 3 {
            let input = input * 100.0;
            println!("The conversion is {input}cm");
        } else if unit2 == 4 {
            let input = input * 0.03280839895;
            println!("The conversion is {input}ft");
        } else if unit2 == 5 {
            let input = input * 0.0000062137;
            println!("The conversion is {input}mi");
        } else if unit2 == 6 {
            let input = input * 0.393700787402;
            println!("The conversion is {input}in");
        } else {
            println!("Invalid Input!");
            return;
        }),
        4 => Some(if unit2 == 1 {
            let input = input * 0.3048;
            println!("The conversion is {input}m");
        } else if unit2 == 2 {
            let input = input / 0.0003048;
            println!("The conversion is {input}km");
        } else if unit2 == 3 {
            let input = input * 30.48;
            println!("The conversion is {input}cm");
        } else if unit2 == 4 {
            println!("The conversion is {input}ft");
        } else if unit2 == 5 {
            let input = input * 0.000189393939;
            println!("The conversion is {input}mi");
        } else if unit2 == 6 {
            let input = input * 12.0;
            println!("The conversion is {input}in");
        } else {
            println!("Invalid Input!");
            return;
        }),
        5 => Some(if unit2 == 1 {
            let input = input * 1609.34;
            println!("The conversion is {input}m");
        } else if unit2 == 2 {
            let input = input * 1.60934;
            println!("The conversion is {input}km");
        } else if unit2 == 3 {
            let input = input * 30.48;
            println!("The conversion is {input}cm");
        } else if unit2 == 4 {
            let input = input * 5280.0;
            println!("The conversion is {input}ft");
        } else if unit2 == 5 {
            println!("The conversion is {input}mi");
        } else if unit2 == 6 {
            let input = input * 63360.0;
            println!("The conversion is {input}in");
        } else {
            println!("Invalid Input!");
            return;
        }),
        6 => Some(if unit2 == 1 {
            let input = input * 0.0254;
            println!("The conversion is {input}m");
        } else if unit2 == 2 {
            let input = input * 0.0000254;
            println!("The conversion is {input}km");
        } else if unit2 == 3 {
            let input = input * 2.54;
            println!("The conversion is {input}cm");
        } else if unit2 == 4 {
            let input = input / 12.0;
            println!("The conversion is {input}ft");
        } else if unit2 == 5 {
            let input = input * 0.00001578;
            println!("The conversion is {input}mi");
        } else if unit2 == 6 {
            println!("The conversion is {input}in");
        } else {
            println!("Invalid Input!");
            return;
        }),
        _ => None,
    }
    .expect("Invalid input!")
}
