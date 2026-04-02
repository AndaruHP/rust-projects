use std::fs;
use std::io::{self, Write};

use serde::{Deserialize, Serialize};

const SAVE_FILE: &str = "todos.json";

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    task: String,
    done: bool
}

impl Todo {
    fn new(id: usize, task: String) -> Todo {
        Todo {id, task, done: false}
    }
}

fn load_todos() -> Vec<Todo> {
    match fs::read_to_string(SAVE_FILE) {
        Ok(content) => {
            serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
        }
        Err(_) => Vec::new()
    }
}

fn save_todos(todos: &Vec<Todo>) {
    let json = serde_json::to_string_pretty(todos).unwrap();
    fs::write(SAVE_FILE, json).unwrap();
}

fn list_todos(todos: &Vec<Todo>) {
    if todos.is_empty() {
        println!("Belum ada task.\n");
        return;
    }

    println!("\n{:<5} {:<8} {}", "ID", "Status", "Task");
    println!("{}", "-".repeat(40));

    for todo in todos.iter() {
        let status = if todo.done { "✓ selesai" } else { "○ belum" };
        println!("{:<5} {:<8} {}", todo.id, status, todo.task);
    }

    println!();
}

fn add_todo(todos: &mut Vec<Todo>, task: String) {
    let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    let todo = Todo::new(next_id, task);
    todos.push(todo);
    save_todos(todos);
    println!("Task ditambahkan dengan ID {}.\n", next_id);
}

fn complete_todo(todos: &mut Vec<Todo>, id: usize) {
    for todo in todos.iter_mut() {
        if todo.id == id {
            if todo.done {
                println!("Task {} sudah selesai sebelumnya.\n", id);
            } else {
                todo.done = true;
                save_todos(todos);
                println!("Task {} ditandai selesai.\n", id)
            }
            return;
        }
    }
    println!("Task dengan ID {} tidak ditemukan.\n", id);
}

fn delete_todo(todos: &mut Vec<Todo>, id: usize) {
    let sebelum = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() < sebelum {
        save_todos(todos);
        println!("Task {} dihapus.\n", id);
    } else {
        println!("Task dengan ID {} tidak ditemukan.\n", id);
    }
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_id(input: &str) -> Option<usize> {
    input.trim().parse::<usize>().ok()
}


fn main() {
    let mut todos = load_todos();

    loop {
        println!("Pilih aksi:");
        println!(" 1. Lihat semua task");
        println!(" 2. Tambah task");
        println!(" 3. Tandai selesai");
        println!(" 4. Hapus task");
        println!(" 0. Keluar");

        let pilihan = read_line("\nPilihan: ");

        match pilihan.as_str() {
            "1" => list_todos(&todos),
            "2" => {
                let task = read_line("Nama task: ");
                if task.is_empty() {
                    println!("Task tidak boleh kosong.\n");
                } else {
                    add_todo(&mut todos, task);
                }
            }
            "3" => {
                list_todos(&todos);
                let input = read_line("Masukkan ID task yang selesai: ");
                match parse_id(&input) {
                    Some(id) => complete_todo(&mut todos, id),
                    None => println!("ID tidak valid.\n")
                }
            }
            "4" => {
                list_todos(&todos);
                let input = read_line("Masukkan ID task yang dihapus: ");
                match parse_id(&input) {
                    Some(id) => delete_todo(&mut todos, id),
                    None => println!("ID tidak valid.\n")
                }
            }
            "0" => {
                println!("Sampai jumpa");
                break;
            }
            _ => println!("Pilihan tidak valid.\n")
        }
    }
}
