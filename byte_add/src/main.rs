use byte_add::{add_u8_checked, add_u8_saturating, add_u8_wrapping};
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 1 {
        eprintln!("Expected at most one argument!");
        process::exit(1);
    }

    let mode: &str = if !args.is_empty() {
        &args[0]
    } else {
        "checked"
    };

    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<u8> = input
        .split_whitespace()
        .take(2)
        .map(|s| s.parse::<u8>().expect("Please enter valid numbers (0-255)") as u8)
        .collect();

    let sum: u8 = match mode {
        "checked" => match add_u8_checked(numbers[0], numbers[1]) {
            Some(r) => r,
            None => {
                eprintln!("Overflow!");
                process::exit(1);
            }
        },
        "wrapped" => {
            let r = add_u8_wrapping(numbers[0], numbers[1]);
            r
        }
        "saturated" => {
            let r = add_u8_saturating(numbers[0], numbers[1]);
            r
        }
        _ => {
            eprintln!("Unknown mode: {mode}");
            process::exit(1);
        }
    };

    println!("{}", sum);
}
