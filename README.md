# Preface 
I needed to create this to prove to myself that it is possible (it is) so if this helps then I'm glad. As you might expect, a library 
wraps a bindings library which contains the C/C++/CMake project: 

```
.
├── Cargo.lock
├── Cargo.toml
├── hello_lib
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── libhello_bindings
│   │   ├── Cargo.lock
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── libhello
│   │   │   ├── CMakeLists.txt
│   │   │   ├── hello.c
│   │   │   └── proto.h
│   │   └── src
│   │       └── lib.rs
│   └── src
│       └── lib.rs
└── src
    └── main.rs
```

the file `build.rs` is responsible for first executing the CMake build as well as generating the Rust FFI bindings which are used (and tested with `cargo test`) 
in the `hello_lib` sub-crate.


# Info 
- Cargo build scripts: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/cargo/reference/build-scripts.html
- cmake crate: https://docs.rs/cmake/latest/cmake/
- bindgen crate: https://crates.io/crates/bindgen/0.19.2
- bindgen documentation https://rust-lang.github.io/rust-bindgen/
