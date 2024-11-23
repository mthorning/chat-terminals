use std::io::Write;
use tokio::{
    io::{self, stdin, stdout, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let name = get_user_name()?;

    let socket = TcpStream::connect("127.0.0.1:8080").await;
    let (mut reader, mut writer) = io::split(socket?);

    writer
        .write_all(format!("{} has joined the chat.", &name).as_bytes())
        .await?;

    let input = tokio::spawn(async move {
        loop {
            let mut buffer = [0; 1024];
            let Ok(n) = stdin().read(&mut buffer).await else {
                return;
            };

            if n == 0 {
                break;
            }
            let Ok(_) = writer.write_all(format!("[{}]: {}", &name, String::from_utf8_lossy(&buffer)).as_bytes()).await else {
                return;
            };
        }
    });

    tokio::spawn(async move {
        loop {
            let mut buffer = [0; 1024];
            let Ok(_) = reader.read(&mut buffer).await else {
                return;
            };
            let Ok(_) = stdout().write_all(&buffer).await else {
                return;
            };
        }
    });

    input.await?;

    Ok(())
}

fn get_user_name() -> Result<String, io::Error> {
    std::io::stdout().write_all(b"Enter your name: ")?;
    std::io::stdout().flush()?;
    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;
    Ok(String::from(name.trim()))
}
