# cargo-why

traces dependency paths to show why a crate was required

## installation

```
cargo install cargo-why
```

## usage

```
$ cargo why serde
cargo-why -> cargo_metadata -> semver -> serde
cargo-why -> cargo_metadata -> serde
cargo-why -> cargo_metadata -> serde_json -> serde
```
