The rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own.

In the Cargo.toml file, everything that follows a header is part of that section that continues until another section starts.

 Cargo understands Semantic Versioning (sometimes called SemVer), which is a standard for writing version numbers. 
 The specifier 0.8.5 is actually shorthand for ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0.carg

 When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.