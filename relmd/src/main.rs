use tokio::net::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::Path;
use std::fs;

const SOCKET_PATH: &str = "/tmp/relmd.sock";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if Path::new(SOCKET_PATH).exists() {
        fs::remove_file(SOCKET_PATH)?;
    }
    let listener = UnixListener::bind(SOCKET_PATH)?;
    println!("relmd listening on {}", SOCKET_PATH);
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(mut stream: UnixStream) -> std::io::Result<()> {
    let mut buffer = vec![0; 1024];
    let _n = stream.read(&mut buffer).await?;
    let response = r#"{"status":"ok"}"#;
    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}