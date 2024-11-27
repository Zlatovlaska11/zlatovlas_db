use std::vec;

use content_manager::{
    data_layout::data_layout::{ColData, TableMetadata, Type},
    serializer::serializer::{deserializer, serialize},
};
use data_engine::datastore::datastore::DsTrait;

pub mod content_manager;
pub mod data_engine;

fn main() {
    let mut datastore =
        data_engine::datastore::datastore::DataStore::from_file("./database.db".to_string());

    let data = datastore.read_page(0).unwrap();

    //println!("{:?}", data);

    println!("{:?}", datastore.master_table.keys());

    println!("{:?}", deserializer(data.as_bytes().to_vec()));
}
