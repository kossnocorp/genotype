install:
  cargo install cargo-watch
  cargo install cargo-nextest --locked
  cargo install cargo-release

test:
  cargo nextest run

test-watch:
  cargo watch -s 'cargo nextest run'

version version:
  cargo release version {{version}} --exclude literals --exclude genotype_runtime --execute

publish:
  cargo release publish --exclude literals --exclude genotype_runtime --execute