use std::vec;

use content_manager::serializer::serializer::{deserializer, serialize};
use data_engine::datastore::datastore::DsTrait;
use tabled::Table;

pub mod data_engine;
pub mod content_manager;

fn main() {
    let mut datastore =
        data_engine::datastore::datastore::DataStore::from_file("./data.db".to_string());

    let data = datastore.read_page(0).unwrap();



    //println!("{:?}", datastore.pages);

    let ser: Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 0, 116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 116, 104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 116, 109, 121, 32, 110, 105, 103, 103, 97, 32, 98, 105, 116, 99, 104];



    let deser = deserializer(ser);
    let tbl = vec![deser];

    let table = Table::new(tbl).to_owned();

    println!("{}", table);

}
