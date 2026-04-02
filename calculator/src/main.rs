use std::io::{self, Write};

fn calculate(a: f64, op: &str, b: f64) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err("Error: tidak bisa dibagi dengan nol".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Error: operator '{}' tidak dikenal", op)),
    }
}

fn parse_number(input: &str) -> Result<f64, String> {
    input
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("Error: '{}' bukan angka yang valid", input.trim()))
}

fn main() {
    println!("=== Kalkulator Rust ===");
    println!("Operator yang tersedia: +  -  *  /");
    println!("Ketik 'keluar' untuk berhenti.\n");

    loop {
        print("Angka pertama: ");
        let input_a = read_line();
        if input_a.trim().eq_ignore_ascii_case("keluar") {
            break;
        }
        let a = match parse_number(&input_a) {
            Ok(n) => n,
            Err(e) => {
                println!("{}\n", e);
                continue;
            }
        };

        let op_input = loop {
            print("Operator (+, -, *, /): ");
            let input = read_line();
            let trimmed = input.trim().to_string();
            if trimmed.eq_ignore_ascii_case("keluar") {
                break trimmed;
            }
            if ["+", "-", "*", "/"].contains(&trimmed.as_str()) {
                break trimmed;
            }
            println!("Error: operator '{}' tidak dikenal, coba lagi.\n", trimmed);
        };
        let op = op_input.as_str();
        if op.eq_ignore_ascii_case("keluar") {
            break;
        }

        print("Angka kedua: ");
        let input_b = read_line();
        if input_b.trim().eq_ignore_ascii_case("keluar") {
            break;
        }
        let b = match parse_number(&input_b) {
            Ok(n) => n,
            Err(e) => {
                println!("{}\n", e);
                continue;
            }
        };

        match calculate(a, op, b) {
            Ok(result) => println!("Hasil: {} {} {} = {}\n", a, op, b, result),
            Err(e) => println!("{}\n", e),
        }
    }

    println!("Sampai jumpa!");
}

fn read_line() -> String {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn print(text: &str) {
    print!("{}", text);
}
