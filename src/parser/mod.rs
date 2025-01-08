pub mod executor;
pub mod formater;

use std::{
    fmt,
    ops::Index,
    sync::{Mutex, MutexGuard},
};

use serde_json::to_string;

use crate::{
    content_manager::data_layout::data_layout::{ColData, Type},
    data_engine::datastore::datastore::DataStore,
    DATA_STORE,
};

#[derive(Debug)]
pub enum ParseError {
    InvalidQuery,
    SemicolonNotFound,
    InvalidArguments,
}

impl ParseError {
    fn get_val(&self) -> String {
        match self {
            ParseError::InvalidQuery => "query is not valid".to_string(),
            ParseError::SemicolonNotFound => {
                "syntax error at the end of the query there need's to be a semicolon".to_string()
            }
            ParseError::InvalidArguments => {
                "the arguments you provided are not valid like wtf bro".to_string()
            }
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.get_val())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword(ActionType),
    Navigator(),
    Identifier(String),
    Condition(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ActionType {
    Insert,
    Delete,
    Select,
    Create,
    None,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Query {
    pub action: ActionType,
    pub columns: Vec<String>,
    pub table: String,
    pub condition: Option<(String, String, String)>,
    pub values: Option<Vec<String>>,

    pub query: String,
}

impl Query {
    pub fn parse(
        mut stmt: String,
        data_store: Option<&mut DataStore>,
    ) -> Result<Query, ParseError> {
        let mut token_chain: Vec<TokenType> = Vec::new();

        for x in stmt.split(' ') {
            token_chain.push(match_keyword(x));
        }

        let boundaries = vec![
            TokenType::Navigator(),
            TokenType::Keyword(ActionType::Insert),
            TokenType::Keyword(ActionType::Select),
            TokenType::Keyword(ActionType::Delete),
            TokenType::Condition("WHERE".to_string()),
            TokenType::Keyword(ActionType::Create),
        ];

        let mut parts: Vec<Vec<TokenType>> = Vec::new();
        let mut cur: Vec<TokenType> = Vec::new();

        token_chain.into_iter().for_each(|x| {
            if boundaries.contains(&x) {
                if !cur.is_empty() {
                    parts.push(cur.clone());
                }

                cur.clear();
                cur.push(x);
            } else {
                cur.push(x);
            }
        });

        if !cur.is_empty() {
            parts.push(cur);
        }

        let mut q = Query {
            action: ActionType::Create,
            columns: Vec::new(),
            table: String::new(),
            condition: None,
            values: None,
            query: stmt.clone(),
        };

        let buffer: Mutex<Vec<String>> = Mutex::new(Vec::new());
        println!("{:?}", parts);

        //let tokens: Vec<Vec<TokenType>> = parts.into_iter().map(|x| {
        //    x.into_iter().filter(|x| *x != TokenType::Identifier(String::new().to_string())).collect::<Vec<TokenType>>()
        //}).collect();
        let tokens = parts;
        if *tokens.first().unwrap().first().unwrap() == TokenType::Keyword(ActionType::Insert) {
            return Query::parse_insert(tokens);
        }

        if *tokens.first().unwrap().first().unwrap() == TokenType::Keyword(ActionType::Create) {
            println!("here");
            Query::parse_create(
                tokens.clone(),
                &mut stmt,
                //&mut DATA_STORE.clone().get_mut().unwrap().lock().unwrap(),
                data_store.unwrap(),
            )?;
            return Ok(q);
        }

        for x in &tokens {
            for r in x[1..].to_vec() {
                match r {
                    TokenType::Identifier(val) => buffer.lock().unwrap().push(val),
                    TokenType::Condition(val) => buffer.lock().unwrap().push(val),
                    _ => return Err(ParseError::InvalidQuery),
                }
            }

            // solve the copy problem

            match &x[0] {
                TokenType::Keyword(act) => {
                    q.action = act.clone();
                    q.columns = buffer.lock().unwrap().to_vec();
                }
                TokenType::Navigator() => q.table = buffer.lock().unwrap().index(0).to_string(),
                TokenType::Identifier(_) => return Err(ParseError::InvalidQuery),
                TokenType::Condition(_) => {
                    let bfr = buffer.lock().unwrap();
                    if bfr.len() < 3 {
                        return Err(ParseError::InvalidArguments);
                    }
                    q.condition = Some((
                        bfr.index(0).to_string(),
                        bfr.index(1).to_string(),
                        bfr.index(2).to_string(),
                    ))
                }
            }

            buffer.lock().unwrap().clear();
        }

        return Ok(q);
    }

    pub fn parse_create(
        tokens: Vec<Vec<TokenType>>,
        raw_query: &mut String,
        data_store: &mut DataStore,
    ) -> Result<(), ParseError> {
        let tail: &String = &raw_query
            .split_off(raw_query.find('(').unwrap())
            .replace("(", "")
            .replace(")", "")
            .replace(";", "")
            .replace('\n', "");

        let mut cols = vec![];

        // split this bitch up to columns [col_name, type]
        for x in tail.split(',') {
            cols.push(x.to_string());
        }

        println!("here1");

        let cols: Vec<&str> = cols.iter().map(|x| x.trim_start_matches(' ')).collect();

        println!("{:?}", cols);

        let cls: Vec<Vec<&str>> = cols
            .iter()
            .map(|x| x.split(' ').collect::<Vec<&str>>())
            .collect();

        println!("\n{:?}\n", cls);

        let cols: Vec<ColData> = cls
            .into_iter()
            .map(|f| ColData::new(Query::match_type(f[1].to_string()), f[0].to_string()))
            .collect();

        let table_name = match &tokens[0][2] {
            TokenType::Identifier(table_name) => table_name,
            _ => return Err(ParseError::InvalidArguments),
        };

        data_store.create_table(table_name.to_string(), cols);

        Ok(())
    }

    fn match_type(data: String) -> Type {
        match data.to_lowercase().as_str() {
            "string" => return Type::Text,
            "number" => return Type::Number,
            "float" => return Type::Float,
            _ => return Type::None,
        }
    }

    pub fn parse_insert(tokens: Vec<Vec<TokenType>>) -> Result<Query, ParseError> {
        if tokens.is_empty() || tokens[0].is_empty() {
            return Err(ParseError::InvalidQuery);
        }

        match &tokens[0][0] {
            TokenType::Keyword(ActionType::Insert) => (),
            _ => return Err(ParseError::InvalidQuery),
        }

        let table = match &tokens[1][1] {
            TokenType::Identifier(name) => name,
            _ => return Err(ParseError::InvalidArguments),
        };

        let mut cols = vec![];

        for x in &tokens[1][2..] {
            match x {
                TokenType::Identifier(col) => {
                    cols.push(col.to_string().trim_matches(['(', ')', ',']).to_string())
                }
                _ => return Err(ParseError::InvalidArguments),
            }
        }

        let vals = &tokens[2][1..];

        let mut vls = vec![];

        for x in vals {
            match x {
                TokenType::Identifier(val) => {
                    vls.push(val.to_string().trim_end_matches([';', '\n']).to_string())
                }
                _ => return Err(ParseError::InvalidArguments),
            }
        }

        let query = Query {
            action: ActionType::Insert,
            columns: cols.to_vec(),
            table: table.to_string(),
            condition: None,
            values: Some(vls),
            query: String::new(),
        };

        Ok(query)

        // Parse the table and columns
    }
}

fn match_keyword(x: &str) -> TokenType {
    match x.to_uppercase().as_str() {
        "SELECT" => TokenType::Keyword(ActionType::Select),
        "INSERT" => TokenType::Keyword(ActionType::Insert),
        "INTO" => TokenType::Navigator(),
        "VALUES" => TokenType::Navigator(),
        "DELETE" => TokenType::Keyword(ActionType::Delete),
        "FROM" => TokenType::Navigator(),
        "WHERE" => TokenType::Condition(x.to_string()),
        "CREATE" => TokenType::Keyword(ActionType::Create),
        _ => TokenType::Identifier(x.to_string()),
    }
}

#[cfg(test)]
mod parse_test {

    use crate::parser::Query;
    extern crate test;

    #[bench]
    fn parser(b: &mut test::Bencher) {
        b.iter(|| Query::parse("SELECT * FROM test;".to_string(), None))
    }

    #[test]
    fn insert() {
        println!(
            "{:?}",
            Query::parse(
                "INSERT INTO test ('username', 'password') VALUES ('test', 'test1')"
                    .to_string(),
                None
            )
        );
    }

    #[test]
    #[should_panic]
    fn parse() {
        let query = Query::parse("this is some bullshit".to_string(), None);

        match query {
            Ok(q) => println!("{:?}", q),
            Err(_) => panic!("i know"),
        }
    }
}
