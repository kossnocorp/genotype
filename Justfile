test:
  cargo nextest run

test-watch:
  bacon nextest

test-run-cli:
  cargo run --bin gt -- build ${TEST_RUN_CLI_PROJECT}

build:
  cargo build

build-watch:
  cargo watch -x build

build-json-types:
  cargo run --bin gt -- build json/types

build-json-types-watch:
  while sleep 0.1; do ls json/schema/src/*.type | entr -d just build-json-types; done

version version:
  cargo release version {{version}} --exclude litty --exclude litty_macro --exclude genotype_runtime --exclude genotype_json_types --execute
  cd lsp && just version {{version}}
  # [TODO] Include into the release process?
  # cd vscode && just version {{version}}

publish:
  cargo release publish --exclude litty --exclude litty_macro --exclude genotype_runtime --exclude genotype_json_types --execute

publish-litty:
  cargo release publish --package litty_macro --execute
  cargo release publish --package litty --execute