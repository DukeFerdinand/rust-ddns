## rust-ddns

A super simple DDNS updater script using `reqwest` and https://ipify.org.

## How to run

- Step 1, have a [rustup](https://rustup.rs/) installation
- Step 2, clone this repo
- Step 3, populate `config.toml` based on the following, filling data as appropriate:

```toml
[[domains]]
domain = "example.com"
# subdomain is an optional field
subdomain = "@"
domain_type = "Namecheap"
password = "password"

[[domains]]
domain = "example.org"
domain_type = "Google"
username = "username"
password = "password"
```

- Step 4 (optional), run `cargo test` to test the core parsers in this app
- Step 5, run `cargo run` to run locally
- Step 6, when you're done, run `cargo build --release`

## Tests

There are only a few tests in this app right now, but as I run into more DDNS needs that might go up. Please feel free to open a PR if you'd like to see more support in this project.
