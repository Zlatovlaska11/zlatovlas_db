# data\_engine

simple data store impl with some core functionality



## Page Allocator

{% @github-files/github-code-block url="https://github.com/Zlatovlaska11/zlatovlas_db/blob/main/src/data_engine/page_allocator.rs" %}

* The `Page Allocator` manages page allocation and deallocation in the `zlatovlas_db` project.
* It optimizes memory usage by maintaining a pool of available pages.
* Integrates with core functionalities to enhance performance and resource management.
* For implementation details, see the `page_allocator.rs` file.



## DataStore

{% @github-files/github-code-block url="https://github.com/Zlatovlaska11/zlatovlas_db/blob/main/src/data_engine/datastore.rs" %}



* Manages data storage in the `zlatovlas_db` project.
* Ensures efficient data retrieval and manipulation.
* Collaborates with the `Page Allocator` to optimize memory usage.
* Maintains database operation performance and integrity.
* Implementation details are available in the `datastore.rs` file.
