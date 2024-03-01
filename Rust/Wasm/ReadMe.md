# WebAssembly Table Project

This project is aimed at demonstrating how to use WebAssembly (Wasm) in a Rust project to process JSON data and perform table-related functionalities. The project utilizes Rust, Node.js, and WebAssembly to achieve this.


# Additional info

 - `rustup default nightly` - change it to nightly for smooth installation of wasm-pack in windows.

 - `cargo install wasm-pack` - install wasm-pack.

-  `rustup default stable-x86_64-pc-windows-gnu` - change it to stable version of rust.

-  `cargo new --lib wasm-table-project`  - create new project with library  type.

 -  in Cargo.toml file put this dependencies and library type.
 -- `[lib]`

 -- `crate-type = ["cdylib","rlib"]`

 -- `[dependencies]`

 -- `wasm-bindgen = "0.2.91"`

-  build pkg file with selected target. For example:
 `wasm-pack build --target nodejs`

-  create an index.js file  and pass the funcnalities through it and run it  `node index.js`.
