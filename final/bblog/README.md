## BBlog: A full stack blog app in Rust by Blake Boris

#### Preparations:

1. If you want your own database, delete `BBlog.db` and make your own sqlite database named `BBlog.db`
2. Then, run `sqlx database create` and `sqlx migrate run` to rebuild an empty database

#### Steps to build:

1. Install cargo-leptos with `cargo install cargo-leptos` or, if necessary, `cargo install --git https://github.com/leptos-rs/cargo-leptos --locked cargo-leptos` for bleeding-edge features and deps
2. Run `cargo leptos watch` from the bblog directory

#### Steps to deploy:

1. `cargo leptos build --release`
