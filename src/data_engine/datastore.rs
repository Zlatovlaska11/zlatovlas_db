pub mod datastore {
    use std::{
        collections::HashMap,
        fs::File,
        io::{self, Read, Seek, Write},
        os::unix::fs::FileExt,
        u64,
    };

    use crate::data_engine::page_allocator::pager::{self, Page, PageImpl, PAGE_SIZE};

    pub struct DataStore {
        file: File,
        pub pages: HashMap<usize, pager::Page>,
        cur_id: usize,
    }

    pub trait DsTrait {
        fn evict_page(&mut self, page_id: usize) -> Result<(), String>;
        fn change_file(&mut self, filename: String) -> io::Result<()>;
        fn get_page_count(file_path: &str) -> io::Result<usize>;
        fn from_file(filename: String) -> DataStore;
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

        fn get_page_count(file_path: &str) -> io::Result<usize> {
            let file = File::open(file_path)?;
            let file_size = file.metadata()?.len(); // Get file size in bytes
            Ok((file_size as usize + PAGE_SIZE - 1) / PAGE_SIZE) // Round up to account for partial pages
        }

        fn from_file(filename: String) -> DataStore {
            let data = File::open(&filename).expect("file does not exists");

            let mut page: [u8; pager::PAGE_SIZE] = [0; pager::PAGE_SIZE];

            data.read_at(&mut page, 0).unwrap();

            let number_of_pages = DataStore::get_page_count(&filename).unwrap();

            let mut datastore = DataStore {
                file: data,
                pages: HashMap::new(),
                cur_id: 0,
            };

            for x in 0..number_of_pages {
                datastore.allocate_page();
                datastore.write_into_page(x, 0, &page).unwrap();

                datastore
                    .file
                    .read_at(&mut page, (datastore.cur_id * pager::PAGE_SIZE) as u64)
                    .unwrap();
            }

            datastore
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
                let mut page = Page::new(page_id);

                self.file
                    .read_exact_at(&mut page.data, (page_id * pager::PAGE_SIZE) as u64)
                    .map_err(|e| e.to_string())?;
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

        fn change_file(&mut self, filename: String) -> io::Result<()> {
            if let Ok(file) = File::open(&filename) {
                self.file = file;
            } else {
                self.file = File::create_new(filename)?;
            }

            Ok(())
        }

        fn evict_page(&mut self, page_id: usize) -> Result<(), String> {
            if let Some(page) = self.pages.get_mut(&page_id) {
                if page.modified {
                    self.flush_page(page_id)?;
                }
                self.pages.remove(&page_id).expect("could not remove page");
            }

            Ok(())
        }
    }
}
