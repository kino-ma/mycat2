use std::io::{self, Write, Read};

fn main() -> io::Result<()> {
    use std::env;
    use std::fs;

    let args: Vec<String> = env::args().collect();

    let mut stdout = io::stdout();

    let mut buffer: Vec<u8> = Vec::new();

    for file in &args[1..] {
        let mut f = fs::File::open(file)?;
        f.read_to_end(&mut buffer)?;

        stdout.write_all(&buffer)?;
    }
    Ok(())
}
