fn main() {
    use std::env;
    use std::io::{self, Write};
    use std::fs;

    let args: Vec<String> = env::args().collect();

    let mut stdout = io::stdout();

    for file in &args[1..] {
        let content = match fs::read_to_string(file) {
            Ok(content) => content,
            Err(error) => {
                println!("read {} failed: {}", file, error);
                continue;
            }
        };

        stdout.write_all(&content.into_bytes()).unwrap();
    }
}
