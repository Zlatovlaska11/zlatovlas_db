# ws\_server

{% @github-files/github-code-block url="https://github.com/Zlatovlaska11/zlatovlas_db/blob/main/src/server/ws_server/mod.rs" %}

## WebSocket Server

Simple WebSocket server using the Warp web framework, Tokio for asynchronous runtime, and `futures` for handling asynchronous streams and sinks. Below is a detailed explanation of the code structure, functionality, and purpose.

***

### Modules and Imports

#### External Dependencies:

* **`futures::{SinkExt, StreamExt}`**: Provides extensions for handling asynchronous streams and sinks.
* **`tokio::sync::mpsc`**: A multi-producer, single-consumer channel for message passing.
* **`warp::Filter`**: The core abstraction in `warp` for defining filters and composing routes.

#### Internal Dependency:

* **`crate::data_engine::datastore::datastore::DataStore`**: A placeholder for an external `DataStore` module. Although unused in this snippet, it might be utilized in other parts of the application.

***

### Key Functions

#### `handle_connection(ws: warp::ws::WebSocket)`

This function manages the lifecycle of an individual WebSocket connection.

1. **WebSocket Handling**:
   * The WebSocket is split into a sender (`ws_sender`) and a receiver (`ws_receiver`) using `ws.split()`.
2. **Message Forwarding**:
   * A channel (`mpsc::unbounded_channel`) is created to handle communication between the server logic and the WebSocket sender.
   * A background task (`tokio::spawn`) forwards messages from the `rx` channel to the client via the WebSocket connection.
3. **Message Processing**:
   * Listens to incoming WebSocket messages using `ws_receiver.next()`.
   * Processes text messages and echoes them back to the client with the prefix `Echo:`.
   * Handles errors and terminates the connection on failure.
4. **Error Handling**:
   * Errors during message reception or WebSocket communication are logged and result in the termination of the connection.

**Example Workflow:**

1. Client sends a text message to the WebSocket server.
2. The server logs the received message and echoes it back to the client.
3. If any error occurs, the connection is gracefully terminated.

***

#### `ws_router()`

This function sets up and runs the WebSocket server.

1. **Route Definition**:
   * A WebSocket route is defined at the path `/ws` using `warp::path` and `warp::ws()`.
   * The `on_upgrade` method ensures the WebSocket handshake upgrades HTTP requests to WebSocket connections, calling `handle_connection` for each new connection.
2. **Server Setup**:
   * The server listens on `127.0.0.1:3030`.
   * A graceful shutdown mechanism is implemented, triggered by a `Ctrl+C` signal (`tokio::signal::ctrl_c()`).
3. **Server Execution**:
   * The `warp::serve` method starts the server with the specified route and shutdown logic.
   * Upon shutdown, a message ("shutting down") is printed.

**Example Workflow:**

1. Run the server.
2. Clients connect to `ws://127.0.0.1:3030/ws`.
3. WebSocket connections are managed by the `handle_connection` function.

***

### Key Notes

* **Graceful Shutdown**:
  * The server shuts down cleanly when a `Ctrl+C` signal is detected, ensuring no abrupt termination.
* **Scalability**:
  * The `tokio` runtime and `mpsc` channels allow handling multiple WebSocket connections concurrently without blocking.
* **Extensibility**:
  * This implementation can be extended to include authentication, advanced message handling, or integration with the `DataStore` module.
