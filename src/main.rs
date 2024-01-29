use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use redis_protocol::resp2::prelude::*;
use bytes::{Bytes, BytesMut};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on localhost:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = BytesMut::new();
            loop {
                match socket.read_buf(&mut buf).await {
                    Ok(_) => {
                        if let Some((frame, _)) = decode(&Bytes::copy_from_slice(&buf)).unwrap() {
                            println!("Received frame: {:?}", frame);

                            // Encode the frame into bytes and print it
                            let mut encoded_buf = BytesMut::new();
                            let len = encode_bytes(&mut encoded_buf, &frame).unwrap();
                            println!("Encoded {} bytes into buffer with contents {:?}", len, encoded_buf);

                            // Handle the frame here...
                        }
                        buf.clear(); // Clear buffer after processing
                    },
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        break;
                    }
                }
            }
        });
    }
}
