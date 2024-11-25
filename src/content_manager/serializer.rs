pub mod serializer {

    use std::{fmt::write, io::Read, usize};

    use prettytable::{row, Table};
    use tabled::Tabled;

    use crate::content_manager::{
        self,
        data_layout::{
            self,
            data_layout::{Data, PageData, PageHeader, Type},
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
    }

    /// deserializes serialized data according to its byte structure
    /// 81 bytes is the header
    /// 1 byte for the type
    /// and by the type of the data the next len is either 64 or 32
    pub fn deserializer(data: Vec<u8>) -> PageData {
        let dta: &[u8] = &data;
        let page_id = &data[0..8];

        let mut buffer: [u8; 8] = [0; 8];

        for i in 0..8 {
            buffer[i] = page_id[i];
        }

        let page_id = buffer;

        let page_id = usize::from_ne_bytes(page_id);

        let tbl = &dta[8..65 + 8];

        let mut buffer: [u8; 64] = [0; 64];

        for i in 0..64 {
            buffer[i] = tbl[i];
        }

        let table_name = buffer;

        let number_of_rows = &dta[73..81];

        let mut buffer: [u8; 8] = [0; 8];

        for i in 0..8 {
            buffer[i] = number_of_rows[i];
        }

        let free_space_ptr = &dta[80..88];
        let mut buffer: [u8; 8] = [0; 8];

        for i in 0..8 {
            buffer[i] = free_space_ptr[i];
        }

        let data_packs = &dta[88..];

        let mut jump = 0;

        if data_packs.len() < 90 {
            let page_data = PageData::new(
                String::from_utf8(table_name.to_vec()).unwrap(),
                page_id,
                Vec::new(),
            );

            return page_data;
        }

        // later make a check that checks if there is acctually any data
        let mut data = vec![parse_data(&dta[88..], &mut jump).unwrap()];

        let mut dta = parse_data(&data_packs[jump + 1..], &mut jump);

        while dta.is_some() {
            data.push(dta.unwrap());
            dta = parse_data(&data_packs[jump..], &mut jump);
        }

        let page_header = PageData::new(
            String::from_utf8(table_name.to_vec()).unwrap(),
            page_id,
            data,
        );

        return page_header;
    }

    /// put only trimmed data not with header
    fn parse_data(data: &[u8], jump: &mut usize) -> Option<Data> {
        let tp = data[0] as char;

        let mut tps: Type = Type::Text;

        let jmp: usize = match tp {
            't' => {
                tps = Type::Text;
                64
            }
            'n' => {
                tps = Type::Number;
                std::mem::size_of::<i32>()
            }
            'f' => {
                tps = Type::Float;
                std::mem::size_of::<f32>()
            }

            _ => usize::MAX,
        };

        if jmp == usize::MAX {
            return None;
        }

        let data = &data[..jmp];

        *jump += jmp;

        Some(Data::new(tps, &mut data.to_vec()))
    }
}

mod test {

    #[test]
    fn serialization_test() {
        let mut data: Vec<crate::content_manager::data_layout::data_layout::Data> = Vec::new();

        let dt = crate::content_manager::data_layout::data_layout::Data::new(
            crate::content_manager::data_layout::data_layout::Type::Text,
            &mut b"hello world".to_vec(),
        );
        let dt1 = crate::content_manager::data_layout::data_layout::Data::new(
            crate::content_manager::data_layout::data_layout::Type::Text,
            &mut b"my nigga bitch".to_vec(),
        );

        data.push(dt);
        data.push(dt1);

        let page_data = crate::content_manager::data_layout::data_layout::PageData::new(
            "test".to_string(),
            1,
            data,
        );

        let ser = crate::content_manager::serializer::serializer::serialize(page_data);

        let deser = crate::content_manager::serializer::serializer::deserializer(ser);

        println!("{:?}", deser.data);
    }
}
