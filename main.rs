use std::io::{self, Write};

fn main() {
    println!("Welcome to the calculator app!");
    loop {
        println!("=== Calculator ===");
        println!("1: Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Square Root");
        println!("6. Power To");
        println!("7. Quit");
        
        let choice: String = get_input();
        match choice.as_str() {
            "1" | "addition" => addition(),
            "2" | "subtraction" => subtraction(),
            "3" | "multiplication" => multiplication(),
            "4" | "division" => division(),
            "5" | "square root" => square_root(),
            "6" | "power to" => power_to(),
            "7" | "quit" => break,
            _ => println!("You can't do that."),
        }
    }
}

fn addition() {
    println!("Enter first number:");
    let num1: f64 = get_input().parse().unwrap_or(0.0);

    println!("Enter another number:");
    let num2: f64 = get_input().parse().unwrap_or(0.0);

    let ans = num1 + num2;
    let ans = rounding(ans);
    println!("Answer: {}", ans);
}

fn subtraction() {
    println!("Enter first number:");
    let num1: f64 = get_input().parse().unwrap_or(0.0);

    println!("Enter another number:");
    let num2: f64 = get_input().parse().unwrap_or(0.0);

    let ans = num1 - num2;
    let ans = rounding(ans);
    println!("Answer: {}", ans);
}

fn multiplication() {
    println!("Enter first number:");
    let num1: f64 = get_input().parse().unwrap_or(0.0);

    println!("Enter another number:");
    let num2: f64 = get_input().parse().unwrap_or(0.0);

    let ans = num1 * num2;
    let ans = rounding(ans);
    println!("Answer: {}", ans);
}

fn division() {
    println!("Enter first number:");
    let num1: f64 = get_input().parse().unwrap_or(0.0);

    println!("Enter another number:");
    let num2: f64 = get_input().parse().unwrap_or(0.0);

    if num2 == 0.0 {
        println!("Error: Cannot divide by zero!");
        return;
    }

    let ans = num1 / num2;
    let ans = rounding(ans);
    println!("Answer: {}", ans);
}

fn square_root() {
    println!("Enter number:");
    let num1: f64 = get_input().parse().unwrap_or(0.0);
    let root = num1.sqrt();

    let root = rounding(root);
    println!("Square root: {}", root);
}

fn power_to() {
    println!("Enter base number:");
    let base: f64 = get_input().parse().unwrap_or(0.0);

    println!("Enter exponent:");
    let exponent: f64 = get_input().parse().unwrap_or(0.0);

    let result = base.powf(exponent);
    let result = rounding(result);
    println!("{} to the power of {} is {}", base, exponent, result);
}

fn rounding(ans: f64) -> f64 {
    println!("Round up?");
    let choice = get_input();
    if choice == "yes" {
        return ans.ceil();
    } else if choice == "no" {
        println!("Round down?");
        let choice = get_input();
        if choice == "yes" {
            return ans.floor();
        }
    }
    ans
}

fn get_input() -> String {
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_lowercase()
}