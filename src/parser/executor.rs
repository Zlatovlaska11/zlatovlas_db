use crate::{content_manager::data_layout::data_layout::Data, data_engine::datastore::datastore};

use super::{ParseError, Query};

pub fn executor(query: Query, datastore: &mut datastore::DataStore) -> Result<(), ParseError> {
    match query.action {
        super::ActionType::Insert => todo!(),
        super::ActionType::Delete => todo!(),
        super::ActionType::Select => {
            //TODO: make a not fancy select data from table
            datastore.table_print(query.table, None);
        }
        super::ActionType::None => todo!(),
    }

    Ok(())
}

pub fn condition(
    query: Query,
    datastore: &mut datastore::DataStore,
) -> Result<Box<dyn Fn(&Vec<Data>, String) -> bool>, ParseError> {
    let (column, operator, _) = query.condition.ok_or(ParseError::InvalidArguments)?;

    if datastore.master_table.contains_key(&column) {
        return Err(ParseError::InvalidArguments);
    }

    let data = datastore.master_table.get(&query.table).ok_or(ParseError::InvalidArguments)?;

    let tbl = data.table_layout.iter().find(|x| *x.col_name == column).expect("error not found the shit");

    // feel like the indexing is that i need to get the col index and than need to cmp to this idk
    let filter: Box<dyn Fn(&Vec<Data>, String) -> bool> = match operator.as_str() {
        "=" => Box::new(move |data: &Vec<Data>, value: String| {
            // need to get the index of the row first
            data.get(col).map_or(false, |x| x == &value)
        }),
        "<" => Box::new(move |data: &Vec<Data>, value: String| {
            data.get(col).map_or(false, |x| x < &value)
        }),
        ">" => Box::new(move |data: &Vec<Data>, value: String| {
            data.get(col).map_or(false, |x| x > &value)
        }),
        "<=" => Box::new(move |data: &Vec<Data>, value: String| {
            data.get(col).map_or(false, |x| x <= &value)
        }),
        ">=" => Box::new(move |data: &Vec<Data>, value: String| {
            data.get(col).map_or(false, |x| x >= &value)
        }),
        "!=" => Box::new(move |data: &Vec<Data>, value: String| {
            data.get(col).map_or(false, |x| x != &value)
        }),
        _ => return Err(ParseError::InvalidArguments),
    };

    Ok(filter)
}
