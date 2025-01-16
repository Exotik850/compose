# Compose

A Rust macro for function composition, enabling you to combine multiple functions into a single function effortlessly. This crate also serves as an educational resource for understanding Rust's procedural macros.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Table of Contents

- [Compose](#compose)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Key Features](#key-features)
  - [Understanding Proc-Macros](#understanding-proc-macros)
  - [Configuration](#configuration)
  - [Contributing](#contributing)
  - [License](#license)
  - [Support](#support)

## Installation

To use the `compose` macro in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
compose-macro = "0.1.0"
```

Ensure you have Rust installed. If not, you can install it from [rustup.rs](https://rustup.rs/).

## Usage

The `compose!` macro allows you to chain functions together. Here's a basic example:

```rust
use compose_macro::compose;

fn h(x: i32) -> i32 {
    x * 2
}

fn g(x: i32) -> i32 {
    x * 3
}

fn k(x: i32) -> i32 {
    x * 4
}

fn main() {
    let composed_function = compose!(h -> g -> k);
    let result = composed_function(2);
    println!("Result: {}", result); // Output: Result: 48
}
```

## Key Features

- **Function Composition**: Easily combine multiple functions into a single function.
- **Macro Syntax**: Simple and intuitive macro syntax for chaining functions.
- **Type Safety**: Leverages Rust's strong type system to ensure correctness.

## Understanding Proc-Macros

This crate is not only a tool for function composition but also a practical example of how to create and use procedural macros in Rust. Procedural macros allow you to operate on Rust code at compile time, enabling powerful code generation and transformation capabilities.

The `compose!` macro is implemented as a procedural macro, which parses the input tokens, constructs a new function composition, and generates the corresponding Rust code. This makes it an excellent resource for developers looking to understand and experiment with Rust's macro system.

Here's a brief overview of how the procedural macro works:

1. **Parsing**: The macro parses the input tokens to identify the functions to be composed.
2. **Code Generation**: It generates a new function that represents the composition of the input functions.
3. **Output**: The generated code is inserted into the caller's code, allowing seamless use of the composed function.

For more details, you can explore the source code in the `compose-macro` directory.

## Configuration

The `compose!` macro does not require any configuration. It works out of the box with any functions that match the expected input and output types.

## Contributing

We welcome contributions! Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Ensure your code adheres to the Rust coding standards.
4. Submit a pull request with a detailed description of your changes.

For coding standards, please refer to the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Support

For support, please open an issue on the [GitHub repository](https://github.com/exotik850/compose/issues).