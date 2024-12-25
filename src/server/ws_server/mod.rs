pub use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc::{self, unbounded_channel};
use warp::Filter;

pub async fn handle_connection(ws: warp::ws::WebSocket) {
    let (mut ws_sender, mut ws_receiver) = ws.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if ws_sender
                .send(warp::ws::Message::text(message))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    while let Some(message) = ws_receiver.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() {
                    let text = msg.to_str().unwrap();
                    println!("recived message: {}", text);

                    if text == "show\n" {
                        println!("recived a show");
                        if tx
                            .send(format!(
                                "{}",
                                crate::DataStore
                                    .get()
                                    .unwrap()
                                    .lock()
                                    .unwrap()
                                    .table_print("test".to_string(), None)
                            ))
                            .is_err()
                        {
                            break;
                        }
                    } else {
                        if tx.send(format!("Echo: {}", text)).is_err() {
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("user probably disconnected but anyway: {}", e);
                break;
            }
        }
    }
}

pub async fn ws_router() {
    // Define the WebSocket route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(handle_connection));

    println!("WebSocket server is running at ws://127.0.0.1:3030/ws");

    let (_addr, fut) =
        warp::serve(ws_route).bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async move {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to listen to shutdown signal");
        });

    fut.await;

    println!("shutting down");
}
