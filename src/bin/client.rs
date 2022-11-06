use tokio::{
    io::{self, stdin, stdout, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let socket = TcpStream::connect("127.0.0.1:8080").await;
    let (mut reader, mut writer) = io::split(socket.unwrap());

    let input = tokio::spawn(async move {
        loop {
            let mut buffer = [0; 1024];
            let n = stdin().read(&mut buffer).await.unwrap();
            if n == 0 {
                break;
            }
            writer.write_all(&buffer).await.unwrap();
        }
    });

    tokio::spawn(async move {
        loop {
            let mut buffer = [0; 1024];
            reader.read(&mut buffer).await.unwrap();
            stdout().write_all(&buffer).await.unwrap();
        }
    });

    input.await.unwrap();
}
