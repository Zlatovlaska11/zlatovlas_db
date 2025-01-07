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
use content_manager::data_layout::data_layout::ColData;
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

pub static DATA_STORE: OnceCell<Arc<Mutex<DataStore>>> = OnceCell::new();

fn init_args() {
    let args = Args::parse();

    if args.file.is_none() {
        DATA_STORE
            .set(Arc::new(Mutex::new(DataStore::new(
                "./database.db".to_string(),
            ))))
            .unwrap();
    } else {
        println!("database atached");
        DATA_STORE
            .set(Arc::new(Mutex::new(DataStore::from_file(
                "./database.db".to_string(),
            ))))
            .unwrap();
    }
}

#[tokio::main]
async fn main() {


    // THE SERIALIZER NEEDS REWORK WITH THE BINCODE LIB BECAUSE IM TOO LAZY TO MAKE IT FROM SCRATCH

    init_args();

    {
        println!("{}", DATA_STORE
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .table_print("test".to_string(), None));
    }


    server::ws_server::ws_router().await;

    // need to work on the insert code and the query parsing the shit might be more complicated
    // than i thought but this is how it goes at the end of the day right?
}
