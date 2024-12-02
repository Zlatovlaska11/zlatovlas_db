pub mod datastore {
    extern crate bincode;
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
        usize,
    };

    pub static MAX_PAGES: usize = 10;

    pub struct DataStore {
        file: File,
        pub pages: HashMap<usize, pager::Page>,
        pub master_table: HashMap<String, TableMetadata>,
        cur_id: usize,
        from_file: bool,
    }

    // remake this to trait only important things not the organs of this shit

    impl DataStore {
        pub fn new(filename: String) -> DataStore {
            let file = File::create(filename).expect("Failed to create file");

            DataStore {
                from_file: false,
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
            // Open the main data file in read-write mode.
            let file = File::options()
                .read(true)
                .write(true)
                .open(&filename)
                .expect("Failed to open file");

            // Calculate the total number of pages in the file.
            let number_of_pages = DataStore::get_page_count(&filename)
                .expect("Failed to determine the number of pages in the file");

            // Initialize an empty DataStore.
            let mut datastore = DataStore {
                from_file: true,
                file,
                pages: HashMap::new(),
                cur_id: 0,
                master_table: HashMap::new(),
            };

            // Load table metadata from "schemes.dat".
            let mut table_metadata: Vec<_> = vec![];
            if let Ok(mut metadata_file) = File::open("schemes.dat") {
                let mut buffer = String::new();
                metadata_file
                    .read_to_string(&mut buffer)
                    .expect("Failed to read table metadata");

                table_metadata = bincode::deserialize(buffer.as_bytes()).unwrap_or_default();
            }

            // cannot find in the hashmap due to the name problem with not striping the null
            // characters shit ass bithch problem

            // Iterate through all pages in the file.
            for page_index in 0..number_of_pages {
                // Allocate a new page in the DataStore.
                datastore.allocate_page();

                // Create a buffer to read the page data.
                let mut page_data = [0u8; PAGE_SIZE];

                // Read the page data from the file.
                datastore
                    .file
                    .read_exact_at(&mut page_data, (page_index * PAGE_SIZE) as u64)
                    .expect("Failed to read page data");

                // Deserialize the page header and table data.
                let page_content = serializer::serializer::deserializer(page_data.to_vec());

                // it didn't work because of this shit
                // cmon mf
                datastore
                    .write_into_page(page_index, 0, &page_data)
                    .unwrap();

                // Update the master table with page references and metadata.
                let table_name = &page_content.header.table_name;

                if datastore.master_table.contains_key(table_name) {
                    datastore
                        .master_table
                        .get_mut(table_name)
                        .unwrap()
                        .pages
                        .push(page_index);
                } else {
                    let layout = table_metadata
                        .get(page_index)
                        .map(|metadata: &TableMetadata| metadata.table_layout.clone())
                        .unwrap_or_else(Vec::new);

                    let table_name =
                        String::from_utf8(page_content.header.table_name.to_string().into())
                            .unwrap()
                            .trim_end_matches('\0')
                            .to_string();

                    datastore.master_table.insert(
                        table_name.clone(),
                        TableMetadata::new(vec![page_index], layout),
                    );
                }
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

        pub fn write(&mut self, table_name: String, data: Data) -> Result<(), String> {
            let pgd = self.master_table.get(&table_name);

            if pgd.is_none() {
                return Err("no table found".to_string());
            }

            let pages = &pgd.unwrap().pages;

            let mut free_space_ptr = u64::MAX;
            let mut page_id = -1;

            let mut buffer: [u8; 8] = [0u8; 8];

            for x in pages {
                let bytes = &self.pages.get(x).unwrap().data[80..88];

                buffer.copy_from_slice(bytes);

                let free_space = u64::from_ne_bytes(buffer);

                if free_space as usize >= data.tp.size() {
                    free_space_ptr = if self.from_file {
                        free_space + 1
                    } else {
                        free_space
                    };
                    println!("{}", free_space);
                    page_id = *x as i32;
                    break;
                }
            }

            // not writing new data into table
            // !!!
            // really why

            let ser_dta = serialize_data(vec![data]);

            self.write_into_page(page_id as usize, free_space_ptr as usize, &ser_dta)
                .expect("error writing");

            self.update_free_space_ptr(page_id as usize, free_space_ptr as usize + 64);

            return Ok(());
        }

        fn update_free_space_ptr(&mut self, page_id: usize, new: usize) {
            let new = new.to_ne_bytes();

            let mut buffer: [u8; 8] = [0u8; 8];

            for x in 0..buffer.len() {
                buffer[x] = new[x];
            }

            self.write_into_page(page_id, 80, &buffer).unwrap();
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

            println!("{}", table_name);
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
