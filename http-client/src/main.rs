use std::io::{Write, stdin, stdout};
use anyhow::Result;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()>{
    print!("Enter domain: ");
    stdout().flush()?;

    let mut domain = String::new();
    stdin().read_line(&mut domain)?;
    let domain = domain.trim();

    let mut stream = TcpStream::connect(format!("{domain}:80")).await?;

    let request = format!(
        "GET / HTTP/1.1\r\n\
        Host: {domain}\r\n\
        Connection: close\r\n\
        \r\n"
    );

    stream.write_all(request.as_bytes()).await?;

    let mut reader = BufReader::new(stream);
    let mut server_status = String::new();
    reader.read_line(&mut server_status).await?;

    print!("Status: {server_status}");
    Ok(())
}
