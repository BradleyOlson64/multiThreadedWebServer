# multiThreadedWebServer

This is a server based on the Rust Book final project. The intent of this project is to practice using web protocols (TCP, HTTP) as well as Rust threads.

Modifications vs the generic Rust Book version:
- Added error handling within the main thread, bubbling errors up to `main()`
- Added an alternative web server implementation using the `threads_pool` crate rather than implementing a thread pool from scratch.

TODO:
- Pool threads do not appropriately bubble errors up to `main()`. Add this advanced error handling, perhaps using a mpsc channel or the returned value on `join()`.
