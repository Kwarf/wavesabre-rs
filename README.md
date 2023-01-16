# wavesabre-rs

This is a small wrapper library to be able to play/render songs made for
[WaveSabre](https://github.com/logicomacorp/WaveSabre) from Rust.

This library is `no_std` so the usual tricks for
[Minimizing Rust Binary Size](https://github.com/johnthagen/min-sized-rust) can be used.

## Instructions

Due to the limited use case of this library as well as the fact that it only works on Windows I've not uploaded it to
crates.io, so you will have to point directly to the repository.

```toml
[dependencies]
wavesabre-rs = { git = "https://github.com/Kwarf/wavesabre-rs.git" }
```
