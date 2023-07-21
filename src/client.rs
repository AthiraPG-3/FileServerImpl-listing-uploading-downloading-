use std::io::{self, Read, Write};
use std::net::TcpStream;

pub fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8081").expect("Failed to connect to server");

    loop {
        println!("Enter the option below:");
        println!("1. List files");
        println!("2. Upload file");
        println!("3. Download file");
        println!("4. Exit");

        let mut opt = String::new();
        io::stdin()
            .read_line(&mut opt)
            .expect("Failed to read input");

        match opt.trim().parse::<u32>() {
            Ok(choice) => match choice {
                1 => {
                    stream.write_all(b"list").expect("Failed to send request");
                    list_files(&mut stream);
                }
                2 => {
                    println!("Enter the file name to upload:");
                    let mut file_name = String::new();
                    io::stdin()
                        .read_line(&mut file_name)
                        .expect("Failed to read file name");
                    stream
                        .write_all(format!("upload{}", file_name.trim()).as_bytes())
                        .expect("Failed to send request");
                    upload_file(file_name.trim(), &mut stream);
                }
                3 => {
                    println!("Enter the file name to download:");
                    let mut file_name = String::new();
                    io::stdin()
                        .read_line(&mut file_name)
                        .expect("Failed to read file name");
                    stream
                        .write_all(format!("download{}", file_name.trim()).as_bytes())
                        .expect("Failed to send request");
                    download_file(file_name.trim(), &mut stream);
                }
                4 => {
                    println!("Exiting the client.");
                    return;
                }
                _ => {
                    println!("Invalid option. Try again.");
                }
            },
            Err(_) => {
                println!("Invalid input. Try again.");
            }
        }
    }
}

fn list_files(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read data");
    println!("Files on the server:\n{}", String::from_utf8_lossy(&buffer));
}

fn upload_file(filename: &str, stream: &mut TcpStream) {
    let mut file = match std::fs::File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            println!("File not found. Upload aborted.");
            return;
        }
    };

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
    println!("File uploaded successfully.");
}

fn download_file(filename: &str, stream: &mut TcpStream) {
    let mut file = match std::fs::File::create(filename) {
        Ok(file) => file,
        Err(_) => {
            println!("Failed to create the file for download.");
            return;
        }
    };

    let mut buffer = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buffer).expect("Failed to read data");
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])
            .expect("Failed to write to file");
    }
    println!("File downloaded successfully.");
}
