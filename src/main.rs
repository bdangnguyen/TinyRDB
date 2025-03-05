use std::{io::Write, os::macos::raw::stat};

enum MetaCommand {
    Exit,
    Unknown(String)
}

impl From<&str> for MetaCommand {
    fn from(input_buffer: &str) -> Self {
        match input_buffer.trim() {
            ".exit" => Self::Exit,
            _ => Self::Unknown(input_buffer.to_string())
        }
    } 
}

enum StatementType {
    Insert,
    Select,
    Unknown(String)
}

struct Statement {
    statement_type: StatementType
}

impl From<&str> for Statement {
    fn from(input_buffer: &str) -> Self {
        let statement_type = match input_buffer {
            input_buffer if input_buffer.starts_with("insert") => StatementType::Insert,
            input_buffer if input_buffer.starts_with("select") => StatementType::Select,
            _ => StatementType::Unknown(input_buffer.to_string())
        };

        return Statement {
            statement_type
        }
    }
}

fn main() {
    loop {
        print_prompt();
        let input_buffer: String = read_input();

        if input_buffer.chars().next().eq(&Some('.')) {
            let meta_command: MetaCommand = MetaCommand::from(input_buffer.as_str());
            match meta_command {
                MetaCommand::Exit => std::process::exit(0),
                MetaCommand::Unknown(command) => {
                    print!("Unrecognized command: {}", command);
                    continue;
                }
            };
        }

        let statement = handle_statement(input_buffer.as_str());
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

fn handle_statement(input_buffer: &str) {
    let statement: Statement = Statement::from(input_buffer);

    match statement.statement_type {
        StatementType::Insert =>  println!("Handling Insert"),
        StatementType::Select => println!("Handling Select"),
        StatementType::Unknown(statement) => print!("Unrecognized keyword at start of: {}", statement)
    }
}
