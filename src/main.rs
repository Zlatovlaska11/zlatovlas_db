#![feature(test)]
pub mod content_manager;
pub mod data_engine;
pub mod server;
pub mod parser;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use clap::Parser;
use data_engine::datastore::datastore::DataStore;
use once_cell::sync::OnceCell;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// file name of the database
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
    // ip addres of ws/http server
    //ip: Option<IpAddr>,
}

pub static DataStore: OnceCell<Arc<Mutex<DataStore>>> = OnceCell::new();

fn init_args() {
    let args = Args::parse();

    if args.file.is_none() {
        DataStore
            .set(Arc::new(Mutex::new(DataStore::new(
                "./database.db".to_string(),
            ))))
            .unwrap();
    } else {
        println!("database atached");
        DataStore
            .set(Arc::new(Mutex::new(DataStore::from_file(
                "./database.db".to_string(),
            ))))
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    init_args();

    server::ws_server::ws_router().await;

    // let mut datastore =
    //     data_engine::datastore::datastore::DataStore::from_file("./database.db".to_string());

    // datastore.create_table(
    //     "test".to_string(),
    //     vec![ColData::new(Type::Text, "username".to_string())],
    // );
    // datastore
    //     .write(
    //         "users".to_string(),
    //         content_manager::data_layout::data_layout::Data::new(
    //             Type::Text,
    //             &mut "test3".as_bytes().to_vec(),
    //         ),
    //     )
    //     .unwrap();

    // datastore
    //     .write(
    //         "test".to_string(),
    //         content_manager::data_layout::data_layout::Data::new(
    //             Type::Text,
    //             &mut "bruh2".as_bytes().to_vec(),
    //         ),
    //     )
    //     .unwrap();
    //
    // datastore
    //     .write(
    //         "test".to_string(),
    //         Data::new(Type::Text, &mut "my nigga".as_bytes().to_vec()),
    //     )
    //     .unwrap();

    //datastore.write_into_page(1, 153, b"thello there").unwrap();
    // let data = datastore.read_page(0).unwrap();
    // println!("{}", data);
    // datastore.shutdown();
}

// use warp::Filter;
// use tokio::sync::mpsc;
// use futures::{StreamExt, SinkExt};
//
// #[tokio::main]
// async fn main() {
//     // Define the WebSocket route
//     let ws_route = warp::path("ws")
//         .and(warp::ws())
//         .map(|ws: warp::ws::Ws| {
//             ws.on_upgrade(handle_connection)
//         });
//
//     // Run the server on localhost:3030
//     println!("WebSocket server is running at ws://127.0.0.1:3030/ws");
//     warp::serve(ws_route).run(([127, 0, 0, 1], 3030)).await;
// }
//
