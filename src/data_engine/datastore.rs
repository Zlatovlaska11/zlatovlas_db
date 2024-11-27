pub mod datastore {
    extern crate bincode;
    use bincode::deserialize;

    use crate::{
        content_manager::{
            data_layout::data_layout::{PageData, TableMetadata},
            serializer::{
                self,
                serializer::{deserializer},
            },
        },
        data_engine::page_allocator::pager::{self, Page, PageImpl, PAGE_SIZE},
    };
    use std::{
        borrow::{Borrow, BorrowMut},
        collections::HashMap,
        fs::{File, OpenOptions},
        io::{self, Read, Seek, Write},
        os::unix::fs::FileExt,
        sync::{Arc, Mutex},
        u64, usize,
    };

    pub static MAX_PAGES: usize = 10;

    pub struct DataStore {
        file: File,
        pub pages: HashMap<usize, pager::Page>,
        pub master_table: HashMap<String, TableMetadata>,
        cur_id: usize,
    }

    // remake this to trait only important things not the organs of this shit
    pub trait DsTrait {
        fn get_page_count(file_path: &str) -> io::Result<usize>;
        fn from_file(filename: String) -> DataStore;
        fn write_into_page(
            &mut self,
            page_id: usize,
            offset: usize,
            data: &[u8],
        ) -> Result<(), String>;
        fn read_page(&mut self, page_id: usize) -> Result<PageData, String>;
        fn new(filename: String) -> DataStore;
        fn flush_page(&mut self, page_id: usize) -> Result<(), String>;
        fn allocate_page(&mut self);
        fn get_page(&mut self, page_id: usize) -> Result<&mut Page, String>;
        fn evict_page(&mut self, page_id: usize) -> Result<(), String>;
        fn change_file(&mut self, filename: String) -> io::Result<()>;
        //fn add_page(&mut self, filename: String) -> io::Result<()>;
    }

    impl DsTrait for DataStore {
        fn new(filename: String) -> DataStore {
            let file = File::create(filename).expect("Failed to create file");

            DataStore {
                master_table: HashMap::new(),
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
            let data = File::open(&filename).unwrap();
            let mut page: [u8; pager::PAGE_SIZE] = [0; pager::PAGE_SIZE];

            data.read_exact_at(&mut page, 0).unwrap();

            let number_of_pages = DataStore::get_page_count(&filename).unwrap();

            let mut datastore = DataStore {
                file: data,
                pages: HashMap::new(),
                cur_id: 0,
                master_table: HashMap::new(),
            };
            let mut buffer: String = String::new();
            let mut file = File::open("schemes.dat").unwrap();
            file.read_to_string(&mut buffer).unwrap();

            let table_metadata: Arc<Mutex<Vec<TableMetadata>>> = Arc::new(Mutex::new(
                deserialize(&buffer.as_bytes())
                    .unwrap_or(vec![(TableMetadata::new(vec![], vec![]))]),
            ));

            //println!("{}", number_of_pages);

            for x in 0..number_of_pages {
                datastore.allocate_page();

                datastore.write_into_page(x, 0, &page).unwrap();

                if buffer.len() != 0 {
                    let dta = serializer::serializer::deserializer(page.to_vec());

                    //println!("{:?}", dta.header);
                    let k = dta.header.table_name;

                    if datastore.master_table.contains_key(&k) {
                        datastore.master_table.get_mut(&k).unwrap().pages.push(x);
                    } else {
                        // load table

                        datastore.master_table.insert(
                            k,
                            TableMetadata::new(
                                vec![x],
                                table_metadata
                                    .lock()
                                    .unwrap()
                                    .get(x)
                                    .expect("there is no table scheme")
                                    .table_layout
                                    .clone(),
                            ),
                        );
                    }
                }

                datastore
                    .file
                    .read_at(&mut page, (datastore.cur_id * pager::PAGE_SIZE) as u64)
                    .expect("this shit");
            }

            datastore
        }

        fn flush_page(&mut self, page_id: usize) -> Result<(), String> {
            let page = self.pages.get_mut(&page_id).expect("page not found");

            if !page.modified {
                return Ok(());
            }

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
            // not loading pages err
            let page = self.pages.get_mut(&page_id);

            if page.is_none() {
                let mut page = Page::new(page_id);

                self.file
                    .read_exact_at(&mut page.data, (page_id * pager::PAGE_SIZE) as u64)
                    .map_err(|e| e.to_string())?;
            }

            return Ok(page.unwrap());
        }

        fn read_page(&mut self, page_id: usize) -> Result<PageData, String> {
            let data = self.get_page(page_id)?.read()?;
            let data = deserializer(data.to_vec());

            Ok(data)
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
