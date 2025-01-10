install:
  cargo install cargo-watch
  cargo install cargo-nextest --locked
  cargo install cargo-release

test:
  cargo nextest run

test-watch:
  cargo watch -s 'cargo nextest run'

build:
  cargo build

build-watch:
  cargo watch -x build

version version:
  cargo release version {{version}} --exclude literals --exclude genotype_runtime --execute
  cd lsp && just version {{version}}
  # [TODO] Include into the release process?
  # cd vscode && just version {{version}}

publish:
  cargo release publish --exclude literals --exclude genotype_runtime --execute