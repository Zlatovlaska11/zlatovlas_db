pub mod data_layout {
    use std::fmt;

    use tabled::Tabled;

    #[derive(Debug)]
    pub enum Type {
        Number,
        Text,
        Float,
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

    #[derive(Debug)]
    pub struct Data {
        pub tp: Type,

        pub data: Vec<u8>,
    }


    #[derive(Debug)]
    pub struct PageData {
        pub header: PageHeader,
        pub data: Vec<Data>,
    }

    impl Data {
        pub fn new(tp: Type, data: &mut Vec<u8>) -> Self {
            let size = match tp {
                Type::Number => std::mem::size_of::<i32>(),
                Type::Text => 64,
                Type::Float => std::mem::size_of::<f32>(),
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
        pub fn new(table_name: String, page_id: usize, data: Vec<Data>) -> Self {
            Self {
                header: PageHeader::new(table_name, page_id, data.len())
                    .expect("something happened"),
                data,
            }
        }
    }

    impl PageHeader {
        fn new(table_name: String, page_id: usize, number_of_rows: usize) -> Option<Self> {
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
                rows: 70,
                free_space_ptr: 74,
            })
        }
    }
}
