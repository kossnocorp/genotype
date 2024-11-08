test:
  cargo nextest run

test-watch:
  cargo watch -s 'cargo nextest run'