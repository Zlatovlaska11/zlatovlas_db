pub mod serializer {

    use crate::{
        content_manager::data_layout::data_layout::{Data, PageData},
        DataStore,
    };

    pub fn serialize(page_data: PageData) -> Vec<u8> {
        let ser = bincode::serialize(&page_data).unwrap();

        ser
    }

    pub fn serialize_data(data: Vec<Data>, page_data: PageData) -> Vec<u8> {
        let mut pg_data: PageData = page_data;

        pg_data.data.push(data);

        let ser = serialize(pg_data);

        ser
    }

    /// deserializes serialized data according to its byte structure
    /// 88 bytes is the header
    /// 8 bytes for the type
    /// and by the type of the data the next len is either 64 or 8

    pub fn deserializer(data: Vec<u8>, dts: &DataStore) -> PageData {
        let deser: PageData = bincode::deserialize(&data).unwrap();

        deser
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
