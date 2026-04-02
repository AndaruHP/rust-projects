use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const HOST: &str = "127.0.0.1:7878";

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];

    let _bytes_read = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Gagal membaca request: {}", e);
            return;
        }
    };

    let request = String::from_utf8_lossy(&buffer);
    let first_line = request.lines().next().unwrap_or_default();

    println!("Request masuk: {}", first_line);

    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() < 2 {
        send_response(&mut stream, 400, "Bad Request", "Request tidak valid.");
        return;
    }

    let method = parts[0];
    let path = parts[1];

    match (method, path) {
        ("GET", "/") => {
            send_response(&mut stream, 200, "OK", "Selamat datang di server Rust");
        }
        ("GET", "/hello") => {
            send_response(&mut stream, 200, "OK", "Halo dari Rust");
        }
        ("GET", "/ping") => {
            send_response(&mut stream, 200, "OK", "pong");
        }
        _ => send_response(&mut stream, 404, "Not Found", "Halaman tidak ditemukan"),
    }
}

fn send_response(stream: &mut TcpStream, status_code: u16, status_text: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContentType: text/plain\r\n\r\n{}",
        status_code,
        status_text,
        body.len(),
        body
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Gagal mengirim response: {}", e);
    }
}

fn main() {
    let listener = TcpListener::bind(HOST).unwrap_or_else(|e| {
        eprintln!("Gagal bind ke {}: {}", HOST, e);
        std::process::exit(1);
    });

    println!("Server berjalan di http://{}", HOST);
    println!("Coba buka browser ke:");
    println!("  http://{}/", HOST);
    println!("  http://{}/hello", HOST);
    println!("  http://{}/ping", HOST);
    println!("\nTekan Ctrl+C untuk berhenti.\n");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Koneksi gagal: {}", e);
            }
        }
    }
}
