mod client;
mod server;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("invalid number");
        return;
    }
    match args[1].as_str() {
        "server" => server::main(),
        "client" => client::main(),
        _ => todo!(),
    }
}
