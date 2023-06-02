mod decode;
mod encode;

use std::env;

fn main() {
    let args_count: usize = env::args().count();
    let args: Vec<String> = env::args().collect();
    if args_count < 2 {
        println!("Usage: cargo run <command> <input_file> <output_file> [message]");
        println!("Commands: encode, decode");
        return;
    }

    let command = &args[1];
    let input_file = &args[2];

    match command.as_str() {
        "encode" => {
            if args_count < 5 {
                println!("Usage: cargo run encode <input_file> <output_file> <message>");
                return;
            }
            let output_file = &args[3];
            let message = &args[4..].join(" ");
            match encode::encode_message(input_file, output_file, message) {
                Ok(_) => println!("Message encoded successfully."),
                Err(err) => eprintln!("Error: {}", err),
            }
        }
        "decode" => match decode::decode_message(input_file) {
            Ok(message) => println!("Decoded Message: {}", message),
            Err(err) => eprintln!("Error: {}", err),
        },
        _ => println!("Invalid command. Commands: encode, decode"),
    }
}
