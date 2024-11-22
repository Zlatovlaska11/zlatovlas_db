use data_engine::datastore::datastore::DsTrait;

pub mod data_engine;

fn main() {
    let mut datastore =
        data_engine::datastore::datastore::DataStore::from_file("./data.db".to_string());

    let data = datastore.read_page(0).unwrap();

    //println!("{:?}", datastore.pages);

    println!("{}", data)
}
