use std::io::Write;

fn main() {
    loop {
        print_prompt();
        let input_buffer = read_input();

        match input_buffer.as_str().trim() {
            ".exit" => std::process::exit(0),
            _ => println!("Unrecognized command: {}", input_buffer)
        }
    }
}

fn print_prompt() {
    print!("rdb > ");
    std::io::stdout().flush().unwrap();
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    return input;
}
