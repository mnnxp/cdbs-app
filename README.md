CADBase Frontend
=====
The platform for publishing and sharing information on drawings and manufacturers.

## How to run it
- Make sure you have Rust Nightly enabled for the project.
- You will need Trunk, an amazing light weight bundler (no need for webpack madness): `cargo install trunk`
- Create a `.env` file in root and add a `DATABASE_URL` variable with your local psql db.
- In backend dir: `cargo run`, in frontend dir: `trunk serve`
- That's it!

## Collection of major crates used in CADBase Frontend
- yew - [link](https://yew.rs/)
- wasm-bindgen - [link](https://docs.rs/wasm-bindgen/)
- yew-router - [link](https://docs.rs/yew-router/)
- yewtil - [link](https://docs.rs/yewtil/)
- wasm-logger - [link](https://docs.rs/wasm-logger/)
- instant - [link](https://docs.rs/instant/)
- lipsum - [link](https://docs.rs/lipsum/)
- log - [link](https://docs.rs/log/)
- getrandom - [link](https://docs.serde.rs/getrandom/)
- rand - [link](https://github.com/bryant/rand)
- chrono - [link](https://docs.rs/chrono)
- dotenv_codegen - [link](https://github.com/dtolnay/dotenv_codegen)
- lazy_static - [link](https://github.com/dtolnay/lazy_static)
- parking_lot - [link](https://docs.rs/parking_lot/)
- pulldown-cmark - [link](https://docs.rs/pulldown-cmark/)
- serde - [link](https://docs.rs/serde/)
- regex - [link](https://docs.rs/regex/)
- serde_json - [link](https://docs.rs/serde_json/)
- thiserror - [link](https://docs.rs/thiserror/)
- graphql_client - [link](https://docs.rs/graphql_client/)
