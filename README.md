<div align="center">
    <h1>unsaferef</h1>
    An unsafe reference without explicit lifetime
</div>

<div align="right">
    <a href="https://github.com/ohkami-rs/unsaferef/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/crates/l/unsaferef.svg" /></a>
    <a href="https://github.com/ohkami-rs/unsaferef/actions"><img alt="CI status" src="https://github.com/ohkami-rs/unsaferef/actions/workflows/CI.yml/badge.svg"/></a>
    <a href="https://crates.io/crates/unsaferef"><img alt="crates.io" src="https://img.shields.io/crates/v/unsaferef" /></a>
</div>

## Example

```rust
use unsaferef::UnsafeRef;

fn main() {
    let data = String::from("Hello, world!");

    // SAFETY: `data` is alive as long as `r` is in use
    let r: UnsafeRef<String> = unsafe {
        UnsafeRef::new(&data)
    };

    assert_eq!(*r, "Hello, world!");

    drop(data)
}
```

## no-std and cow

`UnsafeRef` supports `no-std::no-alloc`.

On `alloc` feature ( default ), unsaferef provides `UnsafeCow`, an unsafe Clone-on-Write container without explicit lifetime.

## License

unsaferef is licensed under MIT LICENSE ( [LICENSE](https://github.com/ohkami-rs/unsaferef/blob/main/LICENSE) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT) ).
