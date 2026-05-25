use futures_util::StreamExt;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

pub async fn start_websocket_server() {
    let listener = TcpListener::bind("0.0.0.0:9001")
        .await
        .expect("Failed to bind WebSocket server");

    println!("WebSocket server running on ws://0.0.0.0:9001");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("New connection: {}", addr);

        tokio::spawn(async move {
            let ws_stream = accept_async(stream)
                .await
                .expect("WebSocket handshake failed");

            let (_write, mut read) = ws_stream.split();

            while let Some(message) = read.next().await {
                match message {
                    Ok(msg) => {
                        println!("Received: {:?}", msg);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }
        });
    }
}