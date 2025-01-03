use serde::Serialize;
use serde_json::Value;

use crate::{
    content_manager::data_layout::data_layout::{ColData, Data},
    data_engine::datastore::datastore::DataStore,
};

pub trait Formater {
    fn serialize(
        table_name: String,
        data: Vec<Vec<String>>,
        datastore: Vec<String>,
    ) -> serde_json::Value;
}

pub struct JsonSer {
    data: Vec<Vec<Data>>,
    table_name: String,
}

#[derive(Serialize)]
struct CellData {
    column_name: String,
    Value: String,
}

#[derive(Serialize)]
pub struct Row {
    Data: Vec<CellData>,
}

impl Row {
    // remake this for Vec<Vec<Data>>
    pub fn new(
        data_layout: Vec<String>,
        data: Vec<Vec<String>>,
    ) -> Self {

        let mut celldata: Vec<CellData> = Vec::new();

        for x in 0..data.len() {
            for y in 0..data_layout.len() {
                if y < data[x].len() {
                    celldata.push(CellData {
                        column_name: data_layout[y].clone(),
                        Value: data[x][y].clone(),
                    });
                }
            }
        }

        return Row { Data: celldata };
    }
}

impl Formater for JsonSer {
    fn serialize(table_name: String, data: Vec<Vec<String>>, layout: Vec<String>) -> Value {
        let row = Row::new(layout.to_vec(), data);

        println!("{}", serde_json::to_string(&row).unwrap());
        return serde_json::json!(&row);
    }
}
