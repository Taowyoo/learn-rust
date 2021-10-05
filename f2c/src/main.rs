use ctrlc;
use std::io;

fn main() {
    ctrlc::set_handler(||std::process::exit(0)).expect("Error setting Ctrl-C handler");
    
    println!("Convert temperatures between Fahrenheit and Celsius.");
    loop {
        println!("Please direction to convert:");
        println!("1. From Fahrenheit to Celsius.");
        println!("2. From Celsius to Fahrenheit.");
        println!("Please input 1 or 2 for the direction you want to convert:");

        let mut choose = String::new();

        io::stdin()
            .read_line(&mut choose)
            .expect("Failed to read the line");

        let choose: u32 = match choose.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not number, Please input 1 or 2:");
                continue;
            }
        };

        match choose {
            1 => println!("You choose 1. From Fahrenheit to Celsius."),
            2 => println!("You choose 2. From Celsius to Fahrenheit."),
            _ => {
                println!("Invalid number {}, Please input 1 or 2:", choose);
                continue;
            }
        }

        let mut degree = String::new();
        println!("Please input the the degree value:");
        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read the line");

        let degree: f32 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not number, Please input again:");
                continue;
            }
        };

        match choose {
            1 => println!("{} F = {} C", degree, (degree - 32.0) * 5.0 / 9.0),
            2 => println!("{} C = {} F", degree, degree * 9.0 / 5.0 + 32.0),
            _ => {}
        }
        println!();
    }
}
