standard:
    @cargo test
    @cargo bench
    @cargo fmt -- --check
    @cargo clippy -- -D warnings

run:
    @cargo run
