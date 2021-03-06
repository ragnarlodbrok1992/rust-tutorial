fn help() {
    println!("This is help function - TODO(implement)!")
}

fn unknown_command(s: &mut String) {
    println!("Unknown command: {}", s);
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn main() {
    use std::io::Write;

    println!("Rust tutorial - simple REPL engine.");
    let mut line = String::new();

    while line.to_lowercase() != "exit" {
        line.clear();
        print!("RustREPL ->: ");
        let _ = std::io::stdout().flush();
        let _bytes = std::io::stdin().read_line(&mut line).unwrap();
        trim_newline(&mut line);

        match line.as_str() {
            "help" => help(),
            _ => unknown_command(&mut line)
        }
    } 
}
