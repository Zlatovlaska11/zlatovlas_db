use warp::filters::query::query;

use crate::{content_manager::data_layout::data_layout::Data, data_engine::datastore::datastore};

use super::{ParseError, Query};

pub fn executor(
    query: Query,
    datastore: &mut datastore::DataStore,
) -> Result<Vec<Vec<String>>, ParseError> {
    match query.action {
        super::ActionType::Insert => todo!(),
        super::ActionType::Delete => todo!(),
        super::ActionType::Select => {
            //TODO: make a not fancy select data from table
            if query.condition.is_some() {
                let condition = condition(query.clone(), datastore).unwrap();
                let data = datastore.select(query.table, Some(condition));
                match data {
                    Some(data) => return Ok(data),
                    None => return Err(ParseError::InvalidQuery),
                }
            } else {
                let data = datastore.select(query.table, None);

                match data {
                    Some(data) => return Ok(data),
                    None => return Err(ParseError::InvalidQuery),
                }
            }
        }
        super::ActionType::None => todo!(),
    }
}

pub fn condition(
    query: Query,
    datastore: &mut datastore::DataStore,
) -> Result<Box<dyn Fn(&Vec<Data>) -> bool>, ParseError> {
    let (column, operator, comparator) = query.condition.ok_or(ParseError::InvalidArguments)?;

    if !datastore.master_table.contains_key(&column) {
        return Err(ParseError::InvalidArguments);
    }

    let data = datastore
        .master_table
        .get(&query.table)
        .ok_or(ParseError::InvalidArguments)?;

    let data_index = data
        .table_layout
        .iter()
        .position(|x| *x.col_name == column)
        .expect("error not found the shit");

    let filter: Box<dyn Fn(&Vec<Data>) -> bool> = match operator.as_str() {
        "=" => Box::new(move |data: &Vec<Data>| {
            data.get(data_index).map_or(false, |x| {
                String::from_utf8(x.data.clone()).unwrap() == comparator
            })
        }),
        "<" => Box::new(move |data: &Vec<Data>| {
            data.get(data_index).map_or(false, |x| {
                String::from_utf8(x.data.clone()).unwrap() < comparator
            })
        }),
        ">" => Box::new(move |data: &Vec<Data>| {
            data.get(data_index).map_or(false, |x| {
                String::from_utf8(x.data.clone()).unwrap() > comparator
            })
        }),
        "<=" => Box::new(move |data: &Vec<Data>| {
            data.get(data_index).map_or(false, |x| {
                String::from_utf8(x.data.clone()).unwrap() <= comparator
            })
        }),
        ">=" => Box::new(move |data: &Vec<Data>| {
            data.get(data_index).map_or(false, |x| {
                String::from_utf8(x.data.clone()).unwrap() >= comparator
            })
        }),
        "!=" => Box::new(move |data: &Vec<Data>| {
            data.get(data_index).map_or(false, |x| {
                String::from_utf8(x.data.clone()).unwrap() != comparator
            })
        }),
        _ => return Err(ParseError::InvalidArguments),
    };

    Ok(filter)
}
