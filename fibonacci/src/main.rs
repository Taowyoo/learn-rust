use std::io;

fn main() {
    println!("Please enter the number of first nth Fibonacci you want to generate:");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read the line");
    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please input a positive integer");
            0
        }
    };

    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for _ in 1..=num {
        print!("{} ", a);
        b = a + b;
        std::mem::swap(&mut a, &mut b);
    }
    println!();
}
