pub mod serializer {

    use std::{fmt::write, io::Read};

    use prettytable::{row, Table};
    use tabled::Tabled;

    use crate::content_manager::{
        self,
        data_layout::{
            self,
            data_layout::{PageData, PageHeader},
        },
    };

    pub fn serialize(page_data: PageData) -> Vec<u8> {
        let mut ser: Vec<u8> = Vec::new();

        let header = page_data.header;

        let bytes = header.page_id.to_ne_bytes();

        bytes.iter().for_each(|x| ser.push(*x));

        header
            .table_name
            .bytes()
            .into_iter()
            .for_each(|x| ser.push(x));

        let bytes = header.rows.to_ne_bytes();

        bytes.iter().for_each(|x| ser.push(*x));

        let bytes = header.free_space_ptr.to_ne_bytes();

        bytes.iter().for_each(|x| ser.push(*x));

        page_data.data.into_iter().for_each(|data| {
            let size: char = match data.tp {
                data_layout::data_layout::Type::Number => 'n',
                data_layout::data_layout::Type::Text => 't',
                data_layout::data_layout::Type::Float => 'f',
            };

            if size == 't' {
                let mut buffer: [u8; 64] = [0u8; 64];

                if data.data.len() > buffer.len() {
                    panic!("not a text buffer as text");
                }

                for x in 0..data.data.len() {
                    buffer[x] = data.data[x];
                }
                ser.push(size as u8);

                buffer.iter().for_each(|x| ser.push(*x));
            } else {
                let dt = data.data.as_slice();
                ser.push(size as u8);

                dt.iter().for_each(|x| ser.push(*x));
            }
        });

        ser

        // get size and than create aray of bytes to than convert to vec to than back to array and
        // to write into page if the destined table
    }

    /// deserializes serialized data according to its byte structure
    pub fn deserializer(data: Vec<u8>) -> PageHeader {
        let dta: &[u8] = &data;
        let page_id = data[0];

        let tbl = &dta[1..65];

        let mut buffer: [u8; 64] = [0; 64];

        for i in 0..64 {
            buffer[i] = tbl[i];
        }

        let table_name = buffer;

        let number_of_rows = &dta[65..73];

        let mut buffer: [u8; 8] = [0; 8];

        for i in 0..8 {
            buffer[i] = number_of_rows[i];
        }

        let number_of_rows = u64::from_be_bytes(buffer);

        let free_space_ptr = &dta[73..81];
        let mut buffer: [u8; 8] = [0; 8];

        for i in 0..8 {
            buffer[i] = free_space_ptr[i];
        }

        // 81 is first type identifier 
        // by the identifier count the next data type off set 
        // ref. ./data_layout.md

        println!("{}", String::from_utf8_lossy(&dta[81..]));
        println!("{:?}", dta);

        //println!("{:?}", String::from_utf8_lossy(&dta[81..81+64].to_vec()));

        //println!("{}", len);

        let free_space_pt = u64::from_be_bytes(buffer);

        let page_header = PageHeader {
            page_id: page_id as usize,
            table_name: String::from_utf8_lossy(&table_name.to_vec()).to_string(),
            rows: number_of_rows,
            free_space_ptr: free_space_pt,
        };

        return page_header;
    }
}

mod test {
    use crate::content_manager::data_layout::data_layout::Data;
    use crate::content_manager::data_layout::data_layout::PageData;
    use crate::content_manager::serializer::serializer::deserializer;

    use super::serializer::serialize;

    #[test]
    fn serialization_test() {
        let mut data: Vec<crate::content_manager::data_layout::data_layout::Data> = Vec::new();

        let dt = Data::new(
            crate::content_manager::data_layout::data_layout::Type::Text,
            &mut b"hello world".to_vec(),
        );
        let dt1 = Data::new(
            crate::content_manager::data_layout::data_layout::Type::Text,
            &mut b"my nigga bitch".to_vec(),
        );

        data.push(dt);
        data.push(dt1);

        let page_data = PageData::new("test".to_string(), 1, data);

        let ser = serialize(page_data);

        //println!("{:?}", ser);

        //println!("{}", String::from_utf8_lossy(&ser));

        println!("{:?}", deserializer(ser));
    }
}
