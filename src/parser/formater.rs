use serde::Serialize;
use serde_json::Value;

use crate::{
    content_manager::data_layout::data_layout::{ColData, Data},
    data_engine::datastore::datastore::DataStore,
};

pub trait Formater {
    fn serialize(table_name: String, data: Vec<Vec<String>>, datastore: &DataStore) -> serde_json::Value;
}

pub struct JsonSer {
    data: Vec<Vec<Data>>,
    table_name: String,
}

#[derive(Serialize)]
struct CellData {
    name: String,
    data: String,
}

#[derive(Serialize)]
pub struct Row {
    data: Vec<CellData>,
}

impl Row {
    // remake this for Vec<Vec<Data>>
    pub fn new(data_layout: Vec<ColData>, data: Vec<Vec<String>>) -> Self {
        let col_name: Vec<String> = data_layout.iter().map(|x| x.col_name.clone()).collect();

        let mut celldata: Vec<CellData> = Vec::new();

        for x in 0..data.len() {
            // add support for 2d array with looping over the data second dim. and arranging the
            // data with the col name one for and index the inner array and can both index them
            // with the second loop
            for y in 0..col_name.len() {
                celldata.push(CellData {
                    name: col_name[y].clone(),
                    data: data[x][y].clone().to_string(),
                });
            }
        }

        return Row { data: celldata };
    }
}

impl Formater for JsonSer {
    fn serialize(table_name: String, data: Vec<Vec<String>>, datastore: &DataStore) -> Value {
        let layout = &datastore
            .master_table
            .get(&table_name)
            .unwrap()
            .table_layout;

        let row = Row::new(layout.to_vec(), data);

        println!("{}", serde_json::to_string(&row).unwrap());
        return serde_json::json!(&row)


    }
}
