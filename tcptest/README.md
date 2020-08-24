# Simple Rust TCP Demo

This is just a simple demonstration of a Rust TCP server. It gives an idea of how the server and client should be setup. It also demonstrates serialising and deserialising Rust objects, using `Bincode`.

TODO:
-----

* Fix client code, to work more like server code (to prevent thread leaks).
* Fix thread leaks in server and client.
* Maybe move to Tokio + async / add examples uisng Tokio library.