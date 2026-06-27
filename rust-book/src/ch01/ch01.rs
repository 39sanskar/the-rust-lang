// ====================== CHAPTER 1 ======================

// Chapter 1 is all about Rust installation and Cargo,
// which is Rust's build system and package manager.

// Important Cargo Commands:

// Create a new project:
// cargo new project_name

// Initialize Cargo in an existing folder:
// cargo init

// Build a project:
// cargo build

// Build and run a project:
// cargo run

// Check for compilation errors without creating binaries:
// cargo check

// Build an optimized release version:
// cargo build --release

// Format code automatically:
// cargo fmt

// Run the linter:
// cargo clippy

// Run tests:
// cargo test

// Generate documentation:
// cargo doc --open

// Remove build artifacts:
// cargo clean

// Cargo.toml contains project metadata and dependencies.
// Cargo.lock stores exact dependency versions.

/*
 
Project structure:
project/
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── main.rs
└── target/

*/

pub fn main() {
    println!("Hello World, This is Rust Chapter 1");
}
