#![feature(test)]
pub mod content_manager;
pub mod data_engine;
pub mod parser;
pub mod server;

use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use clap::Parser;
use data_engine::datastore::{datastore::DataStore};
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

    {
        DataStore.get().unwrap().lock().unwrap().table_print("test".to_string(), None);
    }

    server::ws_server::ws_router().await;


    // for some reason the code works but only when selecting the username and when selecting
    // anything other than that it just gives you the usernames idk why need to work on this shit
    // but i hope that this will get better
    //
    // btw added some shitty code comments you'll laugh quite a bit

    //let mut datastore =
    //    data_engine::datastore::datastore::DataStore::from_file("./database.db".to_string());
    //
    // DataStore.set(Arc::new(Mutex::new(datastore))).unwrap();
    // let mut datastore = DataStore.get().unwrap().lock().unwrap();

    //datastore.create_table(
    //    "test".to_string(),
    //    vec![
    //        ColData::new(Type::Text, "username".to_string()),
    //        ColData::new(Type::Text, "password".to_string()),
    //    ],
    //);
    //
    //println!("succes");
    //
    //datastore
    //    .write(
    //        "test".to_string(),
    //        vec![Data::new(Type::Text, &mut "bruhpass".as_bytes().to_vec())],
    //    )
    //    .unwrap();
    //
    //datastore
    //    .write(
    //        "test".to_string(),
    //        vec![Data::new(Type::Text, &mut "bruhpass".as_bytes().to_vec())],
    //    )
    //    .unwrap();
    //
    //datastore
    //    .write(
    //        "test".to_string(),
    //        vec![
    //            Data::new(Type::Text, &mut "bruh2".as_bytes().to_vec()),
    //            Data::new(Type::Text, &mut "bruhpass".as_bytes().to_vec()),
    //        ],
    //    )
    //    .unwrap();
    //
    //println!("succes2");
    //
    //datastore
    //    .write(
    //        "test".to_string(),
    //        vec![
    //            Data::new(Type::Text, &mut "test".as_bytes().to_vec()),
    //            Data::new(Type::Text, &mut "testttt".as_bytes().to_vec()),
    //        ],
    //    )
    //    .unwrap();

    //datastore.write_into_page(1, 153, b"thello there").unwrap();
    // let data = datastore.read_page(0).unwrap();
    //datastore.table_print("test".to_string(), None);
    //datastore.shutdown();
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
