//! TCP echo server.
//!
//! To send messages, do:
//!
//! ```sh
//! $ nc localhost 8080
//! ```

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

async fn process(stream: TcpStream) -> io::Result<()> {
    println!("Accepted from: {}", stream.peer_addr()?);

    let (reader, writer) = &mut (&stream, &stream);
    io::copy(reader, writer).await?;

    Ok(())
}

fn main() -> io::Result<()> {
    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:5555").await?;
        println!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                process(stream).await.unwrap();
            });
        }
        Ok(())
    })
}
