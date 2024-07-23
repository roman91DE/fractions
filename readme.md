# Fraction Arithmetic in Rust

This project is an educational exercise to learn the Rust programming language by implementing a basic fraction arithmetic library. It includes functionalities to create fractions, add, subtract, multiply, divide, and simplify them.

## Overview

The purpose of this project is to serve as a learning tool for Rust, and it is not intended for production use. Contributions are welcome to help improve the code and learn together.

## Features

- Create fractions
- Add, subtract, multiply, and divide fractions
- Simplify fractions

## Example Usage

Here is a simple example of how to use the `Fraction` struct:

```rust
use fraction::Fraction;

fn main() {
    let frac1 = Fraction::new(1, 2);
    let frac2 = Fraction::new(3, 4);

    let sum = frac1.add(&frac2);
    let difference = frac1.subtract(&frac2);
    let product = frac1.multiply(&frac2);
    let quotient = frac1.divide(&frac2);

    println!("{} + {} = {}", frac1.to_string(), frac2.to_string(), sum.to_string());
    println!("{} - {} = {}", frac1.to_string(), frac2.to_string(), difference.to_string());
    println!("{} * {} = {}", frac1.to_string(), frac2.to_string(), product.to_string());
    println!("{} / {} = {}", frac1.to_string(), frac2.to_string(), quotient.to_string());
}
```

## Installation

To use this project, you need to have Rust installed on your system. If you don't have Rust installed, you can install it using `rustup`:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository and navigate to the project directory:

```sh
git clone https://github.com/roman91DE/fractions.git
cd fract-arithmetic
```

Build the project using Cargo:

```sh
cargo build
```

Run the tests to ensure everything is working correctly:

```sh
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue to discuss any changes or improvements. This project is intended for learning and collaborative development.

## License

This project is licensed under the GNU General Public License (GPL-3).

## Disclaimer

This project is purely educational and is not intended for production usage. The code may not be optimized or follow best practices for performance and security.

## Acknowledgments

- Thanks to the Rust community for their excellent documentation and support.

