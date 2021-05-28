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

### "Why?" For a Specific Version

```
$ cargo why "libc 0.2.66"
cargo-why -> failure -> backtrace -> backtrace-sys -> libc 0.2.66
cargo-why -> failure -> backtrace -> libc 0.2.66
```

Or, even more specifically:

```
$ cargo why "libc 0.2.66 (registry+https://github.com/rust-lang/crates.io-index)"
```
