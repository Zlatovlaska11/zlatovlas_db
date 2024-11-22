pub mod data_layout {
    use tabled::Tabled;


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

    pub struct Data {
        pub tp: Type,

        pub data: Vec<u8>,
    }

    pub struct PageData {
        pub header: PageHeader,
        pub data: Vec<Data>,
    }

    impl Data {
        pub fn new(tp: Type, data: Vec<u8>) -> Self {
            Self { tp, data }
        }
    }

    impl PageData {
        pub fn new(table_name: String, page_id: usize, data: Vec<Data>) -> Self {
            Self {
                header: PageHeader::new(table_name, page_id, data.len()).expect("something happened"),
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

            Some(Self {
                page_id,
                table_name,
                rows: 70,
                free_space_ptr: 74,
            })
        }
    }
}
