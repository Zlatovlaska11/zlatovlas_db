pub mod pager {

    pub const PAGE_SIZE: usize = 4096; // 4 KB page size

    #[derive(Debug)]
    pub struct Page {
        pub id: usize,
        pub data: [u8; PAGE_SIZE],
        pub modified: bool,
    }

    pub trait PageImpl {
        fn new(id: usize) -> Self;
        fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), String>;
        fn read(&self) -> Result<[u8; PAGE_SIZE], String>;
    }

    impl PageImpl for Page {
        fn new(id: usize) -> Self {
            let pg = Page {
                id,
                modified: false,
                data: [0; PAGE_SIZE],
            };
            return pg;
        }

        fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), String> {
            if offset + data.len() > PAGE_SIZE {
                return Err("not enough space".to_string());
            }

            self.data[offset..offset + data.len()].copy_from_slice(data);
            self.modified = true;
            Ok(())
        }

        fn read(&self) -> Result<[u8; PAGE_SIZE], String> {
            return Ok(self.data);
        }
    }
}
