# sqlite-hashes

[![GitHub](https://img.shields.io/badge/github-nyurik/sqlite--hashes-8da0cb?logo=github)](https://github.com/nyurik/sqlite-hashes)
[![crates.io version](https://img.shields.io/crates/v/sqlite-hashes.svg)](https://crates.io/crates/sqlite-hashes)
[![docs.rs docs](https://docs.rs/sqlite-hashes/badge.svg)](https://docs.rs/sqlite-hashes)
[![crates.io version](https://img.shields.io/crates/l/sqlite-hashes.svg)](https://github.com/nyurik/sqlite-hashes/blob/main/LICENSE-APACHE)
[![CI build](https://github.com/nyurik/sqlite-hashes/workflows/CI/badge.svg)](https://github.com/nyurik/sqlite-hashes/actions)


Use this crate to add various hash functions to SQLite, including MD5, SHA1, SHA256, and SHA512.

This crate uses [rusqlite](https://crates.io/crates/rusqlite) to add user-defined scalar functions using static linking. Eventually it would be good to build dynamically loadable extension binaries usable from other languages (PRs welcome).

## Usage

```rust
use sqlite_hashes::{register_sha256_function, rusqlite::Connection};

fn main() {
  let db = Connection::open_in_memory().unwrap();
  register_sha256_function(&db).unwrap();

  let sql = "SELECT hex(sha256('password'))";
  let hash: String = db.query_row_and_then(&sql, [], |r| r.get(0)).unwrap();
  assert_eq!(hash, "5E884898DA28047151D0E56F8DC6292773603D0D6AABBDD62A11EF721D1542D8");
}
```

## Features
By default, this crate will compile with all hash functions. You can enable just the ones you need to reduce compile time.

```toml
[dependencies]
sqlite-hashes = { version = "0.1", default-features = false, features = ["sha256"] }
``` 

## Development
* This project is easier to develop with [just](https://github.com/casey/just#readme), a modern alternative to `make`. Install it with `cargo install just`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.
* On `git push`, it will run a few validations, including `cargo fmt`, `cargo clippy`, and `cargo test`.  Use `git push --no-verify` to skip these checks.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
