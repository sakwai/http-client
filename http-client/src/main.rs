use std::io::{Write, stdin, stdout};
use std::sync::Arc;
use rustls_pki_types::ServerName;
use anyhow::Result;
use tokio::net::TcpStream;
use tokio_rustls::rustls::{ClientConfig, RootCertStore};
use tokio_rustls::TlsConnector;
use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()>{
    print!("Enter domain: ");
    stdout().flush()?;

    let mut domain = String::new();
    stdin().read_line(&mut domain)?;
    let domain = domain.trim();

    let mut root_cert_store = RootCertStore::empty();
    root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    let dnsname = ServerName::try_from(domain.to_owned())?;

    let stream = TcpStream::connect(format!("{domain}:443")).await?;
    let mut stream = connector.connect(dnsname, stream).await?;

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
