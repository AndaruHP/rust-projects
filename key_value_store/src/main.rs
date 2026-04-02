use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

const HOST: &str = "127.0.0.1:7878";

type Store = Arc<Mutex<HashMap<String, String>>>;

fn parse_request(raw: &str) -> (String, String, String) {
    let mut lines = raw.lines();

    let first_line = lines.next().unwrap_or_default();
    let parts: Vec<&str> = first_line.split_whitespace().collect();

    let method = parts.get(0).unwrap_or(&"").to_string();
    let path = parts.get(1).unwrap_or(&"").to_string();
    
    let body = if let Some(pos) = raw.find("\r\n\r\n") {
        raw[pos + 4..]
            .trim_matches(char::from(0))
            .trim()
            .to_string()
    } else {
        String::new()
    };

    (method, path, body)
}

fn send_response(stream: &mut TcpStream, status_code: u16, status_text: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        status_code,
        status_text,
        body.len(),
        body
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Gagal kirim response: {}", e);
    }
}

fn handle_connection(mut stream: TcpStream, store: Store) {
    let mut buffer = [0u8; 2048];
    let _n = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Gagal baca request: {}", e);
            return;
        }
    };

    let raw = String::from_utf8_lossy(&buffer).to_string();
    let (method, path, _body) = parse_request(&raw);

    println!("{} {}", method, path);

    let segments: Vec<&str> = path
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();

    let mut map = store.lock().unwrap();

    match (method.as_str(), segments.as_slice()) {
        ("GET", ["get", key]) => {
            match map.get(*key) {
                Some(value) => send_response(&mut stream, 200, "OK", value),
                None => send_response(&mut stream, 404, "Not Found", "Key tidak ditemukan."),
            }
        }

        ("POST", ["set", key, value]) => {
            map.insert(key.to_string(), value.to_string());
            send_response(&mut stream, 200, "OK", &format!("OK: {} = {}", key, value));
        }
 
        ("DELETE", ["delete", key]) => {
            match map.remove(*key) {
                Some(_) => send_response(&mut stream, 200, "OK", &format!("Deleted: {}", key)),
                None => send_response(&mut stream, 404, "Not Found", "Key tidak ditemukan."),
            }
        }
 
        ("GET", ["list"]) => {
            if map.is_empty() {
                send_response(&mut stream, 200, "OK", "Store kosong.");
            } else {
                let list = map
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, v))
                    .collect::<Vec<String>>()
                    .join("\n");
                send_response(&mut stream, 200, "OK", &list);
            }
        }
 
        _ => {
            send_response(&mut stream, 404, "Not Found", "Endpoint tidak ditemukan.\n\nEndpoint tersedia:\n  GET    /get/{key}\n  POST   /set/{key}/{value}\n  DELETE /delete/{key}\n  GET    /list");
        }
    }
}

fn main() {
    let listener = TcpListener::bind(HOST).unwrap_or_else(|e| {
        eprintln!("Gagal bind: {}", e);
        std::process::exit(1);
    });

    let store: Store = Arc::new(Mutex::new(HashMap::new()));
 
    println!("Key-Value Store berjalan di http://{}", HOST);
    println!("\nContoh penggunaan (curl):");
    println!("  curl -X POST http://{}/set/nama/Andaru", HOST);
    println!("  curl http://{}/get/nama", HOST);
    println!("  curl http://{}/list", HOST);
    println!("  curl -X DELETE http://{}/delete/nama", HOST);
    println!("\nTekan Ctrl+C untuk berhenti.\n");
 
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = Arc::clone(&store);
 
                thread::spawn(move || {
                    handle_connection(stream, store);
                });
            }
            Err(e) => eprintln!("Koneksi gagal: {}", e),
        }
    }
}