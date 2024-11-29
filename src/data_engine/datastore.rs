pub mod datastore {
    extern crate bincode;
    use bincode::deserialize;

    use crate::{
        content_manager::{
            data_layout::data_layout::{ColData, Data, PageData, TableMetadata},
            serializer::{
                self,
                serializer::{deserializer, serialize, serialize_data},
            },
        },
        data_engine::page_allocator::pager::{self, Page, PageImpl, PAGE_SIZE},
    };
    use std::{
        collections::HashMap,
        fs::File,
        io::{self, Read, Seek, Write},
        os::unix::fs::FileExt,
        sync::{Arc, Mutex},
        usize,
    };

    pub static MAX_PAGES: usize = 10;

    pub struct DataStore {
        file: File,
        pub pages: HashMap<usize, pager::Page>,
        pub master_table: HashMap<String, TableMetadata>,
        cur_id: usize,
    }

    // remake this to trait only important things not the organs of this shit

    impl DataStore {
        pub fn new(filename: String) -> DataStore {
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

        pub fn from_file(filename: String) -> DataStore {
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

                    let k = dta.header.table_name;

                    if datastore.master_table.contains_key(&k) {
                        datastore.master_table.get_mut(&k).unwrap().pages.push(x);
                    } else {
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

                println!("making new page");

                self.file
                    .read_exact_at(&mut page.data, (page_id * pager::PAGE_SIZE) as u64)
                    .map_err(|e| e.to_string())?;
            }

            return Ok(page.unwrap());
        }

        pub fn read_page(&mut self, page_id: usize) -> Result<PageData, String> {
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

        pub fn shutdown(&mut self) {
            let mut pages: Vec<usize> = Vec::new();

            for (id, page) in &self.pages {
                if page.modified {
                    pages.push(*id);
                }
            }

            for id in pages {
                self.flush_page(id).unwrap();
            }
        }

        pub fn write(&mut self, table_name: String, data: Data) -> Result<(), ()> {
            let pgd = self.master_table.get(&table_name);

            if pgd.is_none() {
                return Err(());
            }

            let pages = &pgd.unwrap().pages;

            let mut free_space_ptr = u64::MAX;
            let mut page_id = -1;

            let mut buffer: [u8; 8] = [0u8; 8];

            for x in pages {
                let bytes = &self.pages.get(x).unwrap().data[80..88];

                buffer.copy_from_slice(bytes);

                let free_space = u64::from_ne_bytes(buffer);

                if free_space as usize >= data.tp.size() + 1 {
                    println!("{:?}", free_space);
                    free_space_ptr = free_space;
                    page_id = *x as i32;
                    break;
                }
            }

            // not writing new data into table
            // !!!
            // really why

            self.write_into_page(
                page_id as usize,
                free_space_ptr as usize,
                &serialize_data(vec![data]).as_slice(),
            )
            .expect("error writing new data");

            return Ok(());
        }

        pub fn create_table(&mut self, table_name: String, table_layout: Vec<ColData>) {
            let id = self.cur_id;

            self.allocate_page();

            let header = crate::content_manager::data_layout::data_layout::PageData::new(
                table_name.clone(),
                id,
                vec![],
            );

            self.write_into_page(id, 0, &serialize(header))
                .expect("could not write to the page idk why");

            self.master_table
                .insert(table_name, TableMetadata::new(vec![id], table_layout));
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
