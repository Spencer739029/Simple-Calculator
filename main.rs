use std::io;
use std::io::Write;

fn main() {
    println!("Enter number: ");
    let str1 = get_input();

    println!("Enter second number: ");
    let str2 = get_input();

    println!("Enter function (+, -, *, /): ");
    let function = get_input();
    match function.as_str() {
        "+" => {
            let num1 = str1.parse::<u64>().unwrap();
            let num2 = str2.parse::<u64>().unwrap();
            let answer = num1 + num2;
            println!("Sum: {}", answer);
        },
        "-" => {
            let num1 = str1.parse::<u64>().unwrap();
            let num2 = str2.parse::<u64>().unwrap();
            let answer = num1 - num2;
            println!("Sum: {}", answer);
        },
        "*" => {
            let num1 = str1.parse::<u64>().unwrap();
            let num2 = str2.parse::<u64>().unwrap();
            let answer = num1 * num2;
            println!("Sum: {}", answer);
        },
        "/" => {
            let num1 = str1.parse::<u64>().unwrap();
            let num2 = str2.parse::<u64>().unwrap();
            let answer = num1 / num2;
            println!("Sum: {}", answer);
        },
        _ => {
            println!("Invalid operation!");
        }
    }
}

fn get_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}