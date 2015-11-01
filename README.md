# rust-tinysegmenter

Rust implementation of [TinySegmenter](http://chasen.org/~taku/software/TinySegmenter/) library, which is a compact Japanese tokenizer.

## Install

Adding the following to the Cargo.toml in your project:

```toml
[dependencies]
tinysegmenter = "*"
```

and import using `extern crate`:

```rust
extern crate tinysegmenter;
```

## Usage

```rust
let tokens = tinysegmenter::tokenize("私の名前は中野です");
println!("{}", &tokens.join("|")); // 私|の|名前|は|中野|です
```

## License

Copyright (c) 2015 woxtu

Licensed under the MIT license.
