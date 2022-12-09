# Rust Website

## Requirements
- postgresql `apt-get install postgresql libpq`
- diesel_cli `cargo install diesel_cli --no-default-features --features postgres`

## Build & Run
``` bash
cargo watch -x run
```

## Migrate database
``` bash
# First time
diesel migration run
# Rerun
diesel migration redo
```

## Thanks for the great tutorial
[dev.to/krowemoh](https://dev.to/krowemoh) - https://dev.to/krowemoh/a-web-app-in-rust-11-user-profiles-44g3
