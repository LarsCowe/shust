use std::io::{self, Write};

fn main() {
    let mut input: String = String::new();

    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();

        let bytes_read = io::stdin().read_line(&mut input).unwrap();

        if bytes_read == 0 {
            println!();
            break;
        }

        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            continue;
        }
        println!("{trimmed_input}");
    }
}
