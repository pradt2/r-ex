# r-ex 

Zero-bloat Rust core library extensions that improve the experience 
of using the Rust's type system in everyday tasks.

## Examples

### Copy between arrays

With r-ex (code never panics):

```rust
let mut buf = [0u8; 4];

buf
    .carved_mut()?          // automatically select range
    .copy_from(&[1, 2]);    // type-safe & size-safe copy never panics
                            // buf = [1, 2, 0, 0]
```

Without r-ex (can panic):
```rust
let mut buf = [0u8; 4];

buf
    .get_mut(0..2)?             // explicitly specify range
    .copy_from_slice(&[1, 2]);  // copy_from_slice panics if sizes mismatch
                                // buf = [1, 2, 0, 0]
```

### Write a big-endian value to a buffer at offset

With `r-ex`:
```rust
fn write_big_endian(dest: &mut [u8], offset: usize, val: u32) {
    dest.carve_mut(offset)? // automatically select range
        .set_be(val);       // supports big-endian and little-endian values; never panics
}
```

Without `r-ex`:
```rust
fn write_big_endian(dest: &mut [u8], offset: usize, val: u32) {
    dest.get_mut(offset..offset + core::mem::size_of_val(&val))?    // explicitly specify range
        .copy_from_slice(&val.to_be_bytes())                        // copy_from_slice can panic
}
```


### Motivation
The Rust core library is awesome. It's clear, concise, non-bloated.
This is what we like it for, and we'd all like to keep it this way.
And yet, we often find ourselves writing `utils` or `misc` modules with a handful of small,
general-purpose extensions to the core library. Sometime these few dozen lines make for
the most genius, powerful, or useful code in the whole project.
To think that these moments of brilliance and ingenuity sit buried down in some internal module,
unexposed to the public, is saddening.

This is where the `r-ex` project comes in. It is home to these little but precious core library
extensions that would otherwise remain unnoticed.

### Principles
 
 - **Zero bloat.** Each extension sits behind its own feature flag,
   and no features are enabled by default.


 - **Zero risk.** Any feature that may panic or uses unsafe code has
   `-unsafe` added to its feature name.


 - **Zero feature interdependencies.** Each extension is standalone, 
   and compatible with all the others, so that they can be freely mixed and matched.


 - **Zero external dependencies.** 


 - **No-std.** This project focuses exclusively on the _core_ library.
   Would you like to see a similar project for the _standard_ library? [Let me know.](https://github.com/pradt2/r-ex/issues/new)


 - **Stable Rust only.** 


 - **100% code coverage.**


### Contributor guidelines

Pull requests are welcome. Please make sure your contribution adheres to the [Principles](#Principles) section above.

