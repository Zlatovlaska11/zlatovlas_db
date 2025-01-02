use serde::Serialize;

use crate::{content_manager::data_layout::data_layout::{ColData, Data}, data_engine::datastore::datastore::DataStore};

pub trait Formater {
    fn serialize(table_name: String, data: Vec<Vec<Data>>, datastore: &DataStore)-> String;
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
    pub fn new(data_layout: Vec<ColData>, data: Vec<Vec<Data>>) -> Self {
        let col_name: Vec<String> = data_layout.iter().map(|x| x.col_name.clone()).collect();
        let data: Vec<Vec<String>> = data
            .iter()
            .map(|x| x.iter().map(|x| String::from_utf8(x.data).unwrap()).collect())
            .collect();

        let mut celldata: Vec<CellData> = Vec::new();

        let mut counter = 0;
        for x in 0..data.len() {
            if counter == col_name.len() {
                counter = 0;
            }
            // add support for 2d array with looping over the data second dim. and arranging the
            // data with the col name one for and index the inner array and can both index them
            // with the second loop
            celldata.push(CellData {
                name: col_name[counter].clone(),
                data: data[x].clone(),
            });
        }

        return Row { data: celldata };
    }
}

impl Formater for JsonSer {
    fn serialize(table_name: String, data: Vec<Vec<Data>>, datastore: &DataStore)-> String {

        let layout = datastore.master_table.get(&table_name).unwrap().table_layout;

        let row = Row::new(layout, data);

    }
}
