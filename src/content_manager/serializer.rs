pub mod serializer {

    use crate::{
        content_manager::data_layout::{
            self,
            data_layout::{Data, PageData, Type},
        },
        DataStore,
    };

    pub fn serialize(page_data: PageData) -> Vec<u8> {
        let mut ser: Vec<u8> = Vec::new();

        // Serialize header (as before)
        let header = page_data.header;

        ser.extend_from_slice(&header.page_id.to_ne_bytes());
        ser.extend(header.table_name.bytes());
        ser.extend_from_slice(&header.rows.to_ne_bytes());
        ser.extend_from_slice(&header.free_space_ptr.to_ne_bytes());

        // Serialize each row (Vec<Data>) with length prefix
        page_data.data.into_iter().for_each(|row| {
            let row_data = serialize_data(row); // Serialize individual row
            let row_length = row_data.len() as u32;
            ser.extend_from_slice(&row_length.to_ne_bytes()); // Prefix row length
            ser.extend(row_data); // Add serialized row data
        });

        ser
    }

    pub fn serialize_data(data: Vec<Data>) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();

        data.into_iter().for_each(|data| {
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
                res.push(size as u8);

                buffer.iter().for_each(|x| res.push(*x));
            } else {
                let dt = data.data.as_slice();
                res.push(size as u8);

                dt.iter().for_each(|x| res.push(*x));
            }
        });

        res
    }

    /// deserializes serialized data according to its byte structure
    /// 88 bytes is the header
    /// 8 bytes for the type
    /// and by the type of the data the next len is either 64 or 8

    pub fn deserializer(data: Vec<u8>, dts: &DataStore) -> PageData {
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

        let data_packs = &dta[88..];
        let tbl_name = String::from_utf8(table_name.to_vec()).unwrap().to_string();

        let x = dts.master_table.get("test").unwrap().row_len;

        //println!("test");

        let in_one_row = x;

        if !data_packs[1] as char == 't' {
            return PageData::new(tbl_name, page_id, vec![]);
        } else {
            let mut jump = 0;

            let mut data: Vec<Vec<Data>> = Vec::new();

            let mut dta = parse_data(&data_packs[1..], &mut jump);

            // multiple clones idk why

            while dta.is_some() {
                let mut buffer: Vec<Data> = Vec::new();
                while buffer.len() < in_one_row  && dta.is_some(){
                    buffer.push(dta.unwrap());
                    dta = parse_data(&data_packs[jump..], &mut jump);
                }
                data.push(buffer.clone());
                buffer.clear();
            }

            //println!("{:?}", data);

            let pagedata = PageData::new(
                String::from_utf8(table_name.to_vec())
                    .unwrap()
                    .trim_end_matches("\0")
                    .to_string(),
                page_id,
                data,
            );

            pagedata
        }
    }
    fn parse_data(data: &[u8], jump: &mut usize) -> Option<Data> {
        let tp = data[0] as char;
        //println!("type -> {}", tp);

        let mut tps: Type = Type::Text;

        let jmp: usize = match tp {
            't' => {
                tps = Type::Text;
                64 + 1
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
            //println!("usize max");
            return None;
        }

        let data = &data[..jmp];

        //println!("{:?}", data);

        *jump += jmp;

        //println!("{:?}", data);

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
            vec![data],
        );

        let ser = crate::content_manager::serializer::serializer::serialize(page_data);

        //println!("{:?}", ser[88] as char);

        //let deser = crate::content_manager::serializer::serializer::deserializer(data);

        //println!("{:?}", deser);
    }
}
