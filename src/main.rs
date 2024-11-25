use std::vec;

use content_manager::serializer::serializer::{deserializer, serialize};
use data_engine::datastore::datastore::DsTrait;
use tabled::Table;

pub mod content_manager;
pub mod data_engine;

fn main() {
    let mut datastore =
        data_engine::datastore::datastore::DataStore::from_file("./data.db".to_string());

    let data = datastore.read_page(0).unwrap();

    //println!("{:?}", datastore.pages);

    let res = vec![
        1, 0, 0, 0, 0, 0, 0, 0, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0,
        116, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    println!("{:?}", deserializer(res));
}
