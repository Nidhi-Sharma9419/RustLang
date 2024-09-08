The compiler `rustc`

Rust has a single compiler that supports multiple Editions, which can be considered different versions of the Rust language. Packages written in one edition can depend on packages written in another, preventing the split in the ecosystem. This allows software maintainers to independently update the source code to a modern edition, using language improvements.

Alternative implementations
- mrustc aims to bootstrap the Rust compiler
- gccrs is a gcc front-end

Rust has two main release channels : stable and nightly