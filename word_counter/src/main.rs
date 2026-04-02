use std::fs;
use std::io::{self, Write};

struct Stats {
    lines: usize,
    words: usize,
    chars: usize,
    chars_no_space: usize,
}

fn count(content: &str) -> Stats {
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    let chars_no_space = content.chars().filter(|c| !c.is_whitespace()).count();

    Stats {
        lines,
        words,
        chars,
        chars_no_space,
    }
}

fn print(text: &str) {
    print!("{}", text);
    io::stdout().flush().unwrap();
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    println!("=== File Word Counter ===");
    println!("Ketik 'keluar' untuk berhenti.\n");

    loop {
        print("Masukkan path file: ");
        let input = read_line();
        let path = input.trim();

        if path.eq_ignore_ascii_case("keluar") {
            break;
        }

        if path.is_empty() {
            println!("Error: path tidak boleh kosong.\n");
            continue;
        }

        match fs::read_to_string(path) {
            Ok(content) => {
                let stats = count(&content);
                println!("\n--- Hasil untuk '{}' ---", path);
                println!("  Baris                : {}", stats.lines);
                println!("  Kata                 : {}", stats.words);
                println!("  Karakter             : {}", stats.chars);
                println!("  Karakter tanpa spasi : {}", stats.chars_no_space);
                println!();
            }
            Err(e) => {
                println!("Error: tidak bisa membaca file — {}\n", e);
            }
        }
    }

    println!("Sampai jumpa!");
}
