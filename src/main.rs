use content_manager::data_layout::data_layout::{ColData, Type};

pub mod content_manager;
pub mod data_engine;

fn main() {
    let mut datastore =
        data_engine::datastore::datastore::DataStore::from_file("./database.db".to_string());

    //datastore.create_table("new_table".to_string(), vec![ColData::new(Type::Text, "username".to_string())]);

    //datastore.write("new_table".to_string(), content_manager::data_layout::data_layout::Data::new(Type::Text, &mut "test".as_bytes().to_vec())).unwrap();

    let data = datastore.read_page(0).unwrap();

    datastore.shutdown();

    println!("{:?}", data.data);


}
