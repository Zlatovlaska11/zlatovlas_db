pub mod datastore {
    use std::{
        collections::HashMap,
        fs::File,
        io::{Seek, Write},
    };

    use crate::data_engine::page_allocator::pager::{self, Page, PageImpl};

    pub struct DataStore {
        file: File,
        pages: HashMap<usize, pager::Page>,
        cur_id: usize,
    }

    pub trait DsTrait {
        fn write_into_page(
            &mut self,
            page_id: usize,
            offset: usize,
            data: &[u8],
        ) -> Result<(), String>;
        fn read_page(&mut self, page_id: usize) -> Result<String, String>;
        fn new(filename: String) -> DataStore;
        fn flush_page(&mut self, page_id: usize) -> Result<(), String>;
        fn allocate_page(&mut self);
        fn get_page(&mut self, page_id: usize) -> Result<&mut Page, String>;
    }

    impl DsTrait for DataStore {
        fn new(filename: String) -> DataStore {
            let file = File::create(filename).expect("Failed to create file");

            DataStore {
                file,
                pages: HashMap::new(),
                cur_id: 0,
            }
        }

        fn flush_page(&mut self, page_id: usize) -> Result<(), String> {
            let page = self.pages.get_mut(&page_id).expect("page not found");

            self.file
                .seek(std::io::SeekFrom::Start(
                    (page_id * pager::PAGE_SIZE) as u64,
                ))
                .map_err(|e| e.to_string())?;

            self.file.write_all(&page.data).map_err(|e| e.to_string())?;
            page.modified = false;

            Ok(())
        }

        fn allocate_page(&mut self) {
            let page = pager::Page::new(self.cur_id);
            self.pages.insert(self.cur_id, page);
            self.cur_id += 1;
        }

        fn get_page(&mut self, page_id: usize) -> Result<&mut Page, String> {
            let page = self.pages.get_mut(&page_id);

            if page.is_none() {
                return Err("page not found".to_string());
            }

            return Ok(page.unwrap());
        }

        fn read_page(&mut self, page_id: usize) -> Result<String, String> {
            let data = self.get_page(page_id)?.read()?;
            let data = String::from_utf8(data.to_vec());

            if data.is_ok() {
                Ok(data.unwrap())
            } else {
                return Err("Error converting".to_string());
            }
        }
        fn write_into_page(
            &mut self,
            page_id: usize,
            offset: usize,
            data: &[u8],
        ) -> Result<(), String> {
            self.get_page(page_id)?.write(offset, data)
        }
    }
}
