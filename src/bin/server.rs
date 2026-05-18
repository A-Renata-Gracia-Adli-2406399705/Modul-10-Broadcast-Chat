use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {

    println!("{} connected", addr);

    let mut bcast_rx = bcast_tx.subscribe();

    loop {

        tokio::select! {

            incoming = ws_stream.next() => {

                match incoming {

                    Some(Ok(msg)) => {

                        if let Some(text) = msg.as_text() {

                            println!("Message from {}: {}", addr, text);

                            let broadcast_msg =
                                format!("{} says: {}", addr, text);

                            let _ = bcast_tx.send(broadcast_msg);
                        }
                    }

                    Some(Err(err)) => {
                        println!("Websocket error: {}", err);
                        return Err(Box::new(err));
                    }

                    None => {
                        println!("{} disconnected", addr);
                        break;
                    }
                }
            }

            broadcast_msg = bcast_rx.recv() => {

                match broadcast_msg {

                    Ok(msg) => {

                        ws_stream
                            .send(Message::text(msg))
                            .await?;
                    }

                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        break;
                    }

                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                        continue;
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("Server listening on port 2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr:?}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {

            let (_req, ws_stream) =
                ServerBuilder::new()
                    .accept(socket)
                    .await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}