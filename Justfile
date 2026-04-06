test:
  cargo nextest run

test-watch:
  bacon nextest

test-run-cli:
  cargo run --bin gt -- build ${TEST_RUN_CLI_PROJECT}

build:
  cargo build

build-watch:
  bacon

check:
  cargo check

check-watch:
  bacon check

build-json-types:
  cargo run --bin gt -- build pkgs/npm-genotype-json-types

build-json-types-watch:
  while sleep 0.1; do ls json/schema/src/*.type | entr -d just build-json-types; done

version version:
  cargo release version {{version}} \
    --exclude litty \
    --exclude litty_macro \
    --exclude litty_macro_tests \
    --exclude toml_ext \
    --exclude genotype_runtime \
    --exclude genotype_json_types \
    --execute
  cd pkgs/npm-genotype-lsp && just version {{version}}
  # [TODO] Include into the release process?
  # cd pkgs/npm-vscode-genotype && just version {{version}}

publish:
  cargo release publish \
    --exclude litty \
    --exclude litty_macro \
    --exclude toml_ext \
    --exclude genotype_runtime \
    --exclude genotype_json_types \
    --execute

version-litty version:
  cargo release version {{version}} --package litty_macro --execute
  cargo release version {{version}} --package litty --execute

publish-litty:
  cargo release publish --package litty_macro --execute
  cargo release publish --package litty --execute

publish-json-types:
  cargo release publish --package genotype_json_types --execute
