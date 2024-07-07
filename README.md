# Screenshot capturer

A dead simple screenshot capturer script written in Rust. Run it, press PrtScr, and observe your screenshot in the `screens` directory (the directory will be created automatically if it does not exist).

## Dependencies

- [Chrono](https://crates.io/crates/chrono): A date and time library for Rust.
- [RDev](https://crates.io/crates/rdev): Rust library for grabbing input events from keyboards and mice.
- [xcap](https://crates.io/crates/xcap): Cross-platform library for capturing screenshots.

## Overriding directory name

To override the screenshots directory, simply provide the desired name when running the program:

```bash
rust-screenshots custom_dir
```

## Building
```bash
cargo build --release
```
