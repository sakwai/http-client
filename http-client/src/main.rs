use std::io::{self, Write, stdin, stdout};

fn main() -> io::Result<()>{
    print!("Enter domain: ");
    stdout().flush()?;

    let mut domain = String::new();
    stdin().read_line(&mut domain)?;

    Ok(())
}
