set shell := ["bash", "-c"]

default:
  @just --choose


commit: 
    cargo run --package tools --bin commit
setup:
    cargo run --package tools --bin init -- setup

explain-diff:
    lumen explain --diff

git:
    serie

docmdgen:
    mkdir -p docs/libs
    echo "" > target/all-docs.md
    #RUSTDOCFLAGS="-Z unstable-options --output-format json" cargo doc --no-deps
    RUSTDOCFLAGS="-Z unstable-options --output-format json" cargo doc 
    for f in target/doc/*.json; do \
        echo "$f"; \
        rustdoc-md --path "$f" --output "docs/libs/$(basename $f).md"; \
        cat "docs/libs/$(basename $f).md" >> target/all-docs.md ; \
    done