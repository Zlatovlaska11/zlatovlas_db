use data_engine::datastore::datastore::DsTrait;

pub mod data_engine;

fn main() {
    let mut datastore = data_engine::datastore::datastore::DataStore::new("data.dat".to_string());
    datastore.allocate_page();
    datastore.write_into_page(0, 0, b"hello world").unwrap();
    let data = datastore.read_page(0).unwrap();
    datastore.flush_page(0).unwrap();

    print!("{}", data);
}
