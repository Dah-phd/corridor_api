# Quoridor Game

* This is an API able to connect via Event Stream player in order to share game of [Quoridor](https://en.wikipedia.org/wiki/Quoridor).
* Also includes primitive AI player for practice/testing.
* You can view the API in practice [here](). It might take a minute to load up due to herroku due to herroku platform.


# HOW TO RUN:
==========================
* You need to create src/auth/secret_key.rs with pub const KEY: &[u8] = b"secret_key_here"
* You need diesel cli:
    * create db.sqlite3 <= database
    * run "diesel migration run --database-url db.sqlite3" from corridor_api location