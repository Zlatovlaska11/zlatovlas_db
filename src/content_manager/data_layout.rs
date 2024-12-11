pub mod data_layout {

    use core::panic;
    use std::fmt;

    use serde::{Deserialize, Serialize};
    use tabled::Tabled;

    #[derive(Serialize, Deserialize, Debug, Clone, Copy)]
    pub enum Type {
        Number,
        Text,
        Float,
        None,
    }

    impl Type {
        pub fn size(&self) -> usize {
            match self {
                Type::Number => 8,
                Type::Text => 64,
                Type::Float => 8,
                Type::None => panic!("bruh"),
            }
        }
    }

    // data structure serealiation
    // | 4b type | data | 1b sep |

    #[derive(Debug, Tabled)]
    pub struct PageHeader {
        pub page_id: usize,
        pub table_name: String,
        pub rows: u64,
        pub free_space_ptr: u64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct TableMetadata {
        pub pages: Vec<usize>,
        pub table_layout: Vec<ColData>,
        pub row_len: usize,
    }

    impl TableMetadata {
        pub fn new(pages: Vec<usize>, table_layout: Vec<ColData>) -> Self {
            Self {
                pages,
                row_len: table_layout.len(),
                table_layout,
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct ColData {
        pub col_type: Type,
        pub col_name: String,
    }

    impl ColData {
        pub fn new(col_type: Type, col_name: String) -> Self {
            Self { col_type, col_name }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Data {
        pub tp: Type,

        pub data: Vec<u8>,
    }

    #[derive(Debug)]
    pub struct PageData {
        pub header: PageHeader,
        pub data: Vec<Vec<Data>>,
    }

    impl fmt::Display for PageData {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "id: {}, table name: {}, number of rows: {}, free space ptr: {}\n",
                self.header.page_id,
                self.header.table_name,
                self.header.rows,
                self.header.free_space_ptr,
            )?;
            Ok(for x in self.data.clone() {
                write!(
                    f,
                    "[{}]",
                    x.iter().fold(String::new(), |acc, num| acc
                        + &String::from_utf8(num.data.clone()).unwrap()
                        + ", ")
                )?
            })
        }
    }

    impl fmt::Display for Data {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let tp = match self.tp {
                Type::Number => "num",
                Type::Text => "text",
                Type::Float => "float",
                Type::None => "null",
            };
            write!(
                f,
                "tp -> {}, data -> {}",
                tp,
                String::from_utf8(self.data.clone()).unwrap()
            )
        }
    }
    impl Data {
        pub fn new(tp: Type, data: &mut Vec<u8>) -> Self {
            let size = match tp {
                Type::Number => std::mem::size_of::<i32>(),
                Type::Text => 64,
                Type::Float => std::mem::size_of::<f32>(),
                Type::None => 0,
            };

            if data.len() != size {
                data.resize(size, 0u8);
            }

            Self {
                tp,
                data: data.to_vec(),
            }
        }
    }

    impl PageData {
        pub fn new(table_name: String, page_id: usize, data: Vec<Vec<Data>>) -> Self {
            let mut free_space_ptr = 88;
            for x in &data {
                for dta in x {
                    free_space_ptr += match dta.tp {
                        Type::Number => std::mem::size_of::<i32>() + 1,
                        Type::Text => 65,
                        Type::Float => std::mem::size_of::<f32>() + 1,
                        Type::None => 0,
                    };
                }
            }

            // if no data hos been added than dont write to the header but rather to the data part
            if free_space_ptr == 88 {
                free_space_ptr += 1;
            }

            Self {
                header: PageHeader::new(table_name, page_id, data.len(), free_space_ptr as u64)
                    .expect("something happened"),
                data,
            }
        }
    }

    impl PageHeader {
        pub fn new(
            table_name: String,
            page_id: usize,
            number_of_rows: usize,
            free_space_ptr: u64,
        ) -> Option<Self> {
            let bytes = table_name.as_str().as_bytes();

            if bytes.len() > 64 {
                return None;
            }
            let mut buffer = [0u8; 64]; // Initialize a buffer of size 64 filled with zeroes

            for i in 0..bytes.len() {
                buffer[i] = bytes[i];
            }

            let str = String::from_utf8(buffer.to_vec()).unwrap();

            Some(Self {
                page_id,
                table_name: str,
                rows: number_of_rows as u64,
                free_space_ptr,
            })
        }
    }
}
