use content_manager::data_layout::data_layout::{ColData, Data, Type};
use tabled::{builder::Builder, settings::Style};

pub mod content_manager;
pub mod data_engine;

fn main() {
    let mut datastore =
        data_engine::datastore::datastore::DataStore::from_file("./database.db".to_string());

    // datastore.create_table(
    //     "names".to_string(),
    //     vec![ColData::new(Type::Text, "username".to_string())],
    // );

    // datastore
    //     .write(
    //         "names".to_string(),
    //         content_manager::data_layout::data_layout::Data::new(
    //             Type::Text,
    //             &mut "Zdenda".as_bytes().to_vec(),
    //         ),
    //     )
    //     .unwrap();
    // //
    // datastore
    //     .write(
    //         "test".to_string(),
    //         content_manager::data_layout::data_layout::Data::new(
    //             Type::Text,
    //             &mut "bruh3".as_bytes().to_vec(),
    //         ),
    //     )
    //     .unwrap();
    //
    // datastore
    //     .write(
    //         "test".to_string(),
    //         Data::new(Type::Text, &mut "helo kello".as_bytes().to_vec()),
    //     )
    //     .unwrap();
    //
    let data = datastore.read_page(1).unwrap();

    println!("{}", data);

    println!("{:?}", datastore.master_table);
    datastore.shutdown();
}
