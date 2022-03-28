# mood-broadcast-api
API for the Mood Broadcast project

# Environment files

To run the project, you need to create : 
- `Rocket.toml` based on the example file with username/password/url for your database
- `src/secret.key` using `head -c16 /dev/urandom > src/secret.key`

# Run the project without Docker

We need to use nightly to be able to put this in main.rs : 
```
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]
```

You can use `rustup default nightly` but there is currently a [bug](https://github.com/rust-lang/rust/issues/95267) so you can use `rustup default nightly-2022-03-22` instead.
If you don't want to change global rustup, use `rustup override set nightly-2022-03-22` instead.

And then `cargo run` and you're good to go!

# Run the project using Docker

Run `docker-compose up` or build images yourself and run them :).
