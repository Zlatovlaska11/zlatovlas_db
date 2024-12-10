use std::{ops::Index, sync::Mutex};

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
    None,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Query {
    pub action: ActionType,
    pub columns: Vec<String>,
    pub table: String,
    pub condition: Option<(String, String, String)>,
}

impl Query {
    pub fn parse(stmt: String) -> Vec<Vec<TokenType>> {
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

        // match and assign to q
        let mut q = Query {
            action: ActionType::None,
            columns: Vec::new(),
            table: String::new(),
            condition: None,
        };

        let buffer: Mutex<Vec<String>> = Mutex::new(Vec::new());

        for x in &parts {
            for r in x[1..].to_vec() {
                match r {
                    TokenType::Identifier(val) => buffer.lock().unwrap().push(val),
                    TokenType::Condition(val) => buffer.lock().unwrap().push(val),
                    _ => panic!("what the hell"),
                }
            }

            // solve the copy problem

            match &x[0] {
                TokenType::Keyword(act) => {
                    q.action = act.clone();
                    q.columns = buffer.lock().unwrap().to_vec();
                }
                TokenType::Navigator() => q.table = buffer.lock().unwrap().index(0).to_string(),
                TokenType::Identifier(_) => panic!("wth"),
                TokenType::Condition(_) => {
                    let bfr = buffer.lock().unwrap();
                    q.condition = Some((
                        bfr.index(0).to_string(),
                        bfr.index(1).to_string(),
                        bfr.index(2).to_string(),
                    ))
                }
            }

            buffer.lock().unwrap().clear();
        }

        println!("{:?}", q);

        return parts;
    }
}
fn match_keyword(x: &str) -> TokenType {
    match x.to_uppercase().as_str() {
        "SELECT" => TokenType::Keyword(ActionType::Select),
        "INSERT" => TokenType::Keyword(ActionType::Insert),
        "DELETE" => TokenType::Keyword(ActionType::Delete),
        "FROM" => TokenType::Navigator(),
        "WHERE" => TokenType::Condition(x.to_string()),
        _ => TokenType::Identifier(x.to_string()),
    }
}

#[cfg(test)]
mod parse_test {
    use crate::parser::Query;
    extern crate test;

    #[bench]
    fn parser(b: &mut test::Bencher) {
        b.iter(|| Query::parse("SELECT * FROM test;".to_string()))
    }

    #[test]
    fn parse() {
        println!(
            "{:?}",
            Query::parse("SELECT * FROM test WHERE id < 20;".to_string())
        );
    }
}
