# Development notes

## Preparation
```sh
cargo install cargo-watch
cargo install cross --git https://github.com/cross-rs/cross
cargo install just
```

## Development
```sh
cargo watch -s "just devl"
cargo watch -s "just devr"
```

## RFID

Crate: https://docs.rs/mfrc522/latest/mfrc522/
Documentation: https://gitlab.com/jspngh/rfid-rs
