# transactions-test

This is a toy implementation of a banks transactional system.
A real system would of-course utilize a database and handle multiple transactions at once. This could easily be expanded upon and replace the CSV parsing with a multithreaded async web server listening for incoming transactions and performing the updates to a database instead to the in memory hashmaps.

The internal transaction system have a set of unit test that verifies some of the most critical parts of the code. It also utilizes rust type system and pattern matching to make sure we are only allowing valid transactions to be processed.

Usage:  

```cargo run -- input.csv > output.csv```

It's utilizing the following crates:

* [serde](https://crates.io/crates/serde) - for serialization and deserialization
* [csv](https://crates.io/crates/csv) - for reading and writing csv data
* [clap](https://crates.io/crates/clap) - for handling command line argument
* [rust_decimal](https://crates.io/crates/rust_decimal) - for making sure we do not get floating point precision issues we could potentially run into with f32 and f64 [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)