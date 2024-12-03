# Advent of Code 2024 Day 2

Add to Cargo.toml https://doc.rust-lang.org/nightly/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html

```ini
edition = "2024"
```

Setup

```bash
rustup update nightly
cargo +nightly fix --edition
cargo add is_sorted
```

Execute code

```bash
cargo +nightly run -- data.txt
```
