use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").expect("Failed to bind");
    println!("Server listening on 127.0.0.1:8081");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read data");

    let request = String::from_utf8_lossy(&buffer[..]);
    let request = request.trim();

    if request == "list" {
        list_files(&mut stream);
    } else if request.starts_with("upload") {
        let filename = request.trim_start_matches("upload").trim();
        receive_file(filename, &mut stream);
    } else if request.starts_with("download") {
        let filename = request.trim_start_matches("download").trim();
        send_file(filename, &mut stream);
    } else {
        stream
            .write_all(b"Invalid command")
            .expect("Failed to send response");
    }
}

fn list_files(stream: &mut TcpStream) {
    let files = read_dir("./files/save").expect("Failed to read directory");
    let mut file_list = String::new();
    for file in files {
        if let Ok(entry) = file {
            if let Ok(file_name) = entry.file_name().into_string() {
                file_list.push_str(&file_name);
                file_list.push('\n');
            }
        }
    }
    stream
        .write_all(file_list.as_bytes())
        .expect("Failed to send file list");
}

fn receive_file(filename: &str, stream: &mut TcpStream) {
    let mut file =
        File::create(format!("./files/save/{}", filename)).expect("Failed to create file");
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).expect("Failed to read data");
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])
            .expect("Failed to write to file");
    }
    stream
        .write_all(b"File uploaded successfully")
        .expect("Failed to send response");
}

fn send_file(filename: &str, stream: &mut TcpStream) {
    let file_path = format!("./files/save/{}", filename);
    if let Ok(mut file) = File::open(&file_path) {
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = file.read(&mut buffer).expect("Failed to read file");
            if bytes_read == 0 {
                break;
            }
            stream
                .write_all(&buffer[..bytes_read])
                .expect("Failed to send file data");
        }
    } else {
        stream
            .write_all(b"File not found")
            .expect("Failed to send response");
    }
}
