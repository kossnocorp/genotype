install:
  cargo install cargo-watch
  cargo install cargo-nextest --locked

test:
  cargo nextest run

test-watch:
  cargo watch -s 'cargo nextest run'