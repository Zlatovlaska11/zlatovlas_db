use crate::data_engine::datastore::datastore;

use super::{ParseError, Query};

pub fn executor(query: Query, datastore: &mut datastore::DataStore) -> Result<(), ParseError> {
    match query.action {
        super::ActionType::Insert => todo!(),
        super::ActionType::Delete => todo!(),
        super::ActionType::Select => {
            datastore.table_print(query.table);
        }
        super::ActionType::None => todo!(),
    }

    Ok(())
}
