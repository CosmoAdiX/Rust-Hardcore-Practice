<div align="center">

# 🦀 Rust Speed Run

**A comprehensive Rust learning project covering core concepts with colorful terminal output**

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-0.1.0-green.svg)
![Status](https://img.shields.io/badge/status-learning-yellow.svg)

[Features](#-features) • [Installation](#-installation) • [Usage](#-usage) • [Topics Covered](#-topics-covered)

</div>

---

## 📋 Table of Contents

- [About](#-about)
- [Features](#-features)
- [Topics Covered](#-topics-covered)
- [Installation](#-installation)
- [Usage](#-usage)
- [Project Structure](#-project-structure)
- [Dependencies](#-dependencies)
- [Learning Path](#-learning-path)

---

## 📖 About

This project is a **Rust learning journey** that demonstrates fundamental Rust concepts through practical examples. Each module focuses on a specific topic, with colorful terminal output to make learning more engaging and visual.

### Why This Project?

- ✅ **Hands-on Learning** - Learn by doing, not just reading
- ✅ **Visual Output** - Colorful terminal output using the `colored` crate
- ✅ **Well Organized** - Each concept in its own module
- ✅ **Progressive** - Builds from basics to advanced topics
- ✅ **Real Examples** - Practical code you can run and modify

---

## ✨ Features

- 🎨 **Colorful Terminal Output** - Uses `colored` crate for beautiful console output
- 📚 **Modular Structure** - Each concept in separate files
- 🔄 **Loops** - Demonstrates `loop`, `while`, and `for` loops
- 🎯 **Ownership** - Deep dive into Rust's ownership system
- 🔗 **References** - Understanding borrowing and references
- ✂️ **Slices** - Working with string and array slices
- 📝 **Expressions** - Rust expressions and statements

---

## 📚 Topics Covered

### 1. Expressions & Statements
- Understanding the difference between expressions and statements
- Return values and implicit returns

### 2. Loops
- `loop` - Infinite loops with break
- `while` - Conditional loops
- `for` - Iterating over collections and ranges

### 3. Ownership
- Ownership rules in Rust
- Move semantics
- Stack vs Heap

### 4. References & Borrowing
- Immutable references (`&`)
- Mutable references (`&mut`)
- Dereferencing

### 5. Slices
- String slices
- Array slices
- Working with byte arrays

---

## 🛠️ Installation

### Prerequisites

- **Rust** (1.70 or later)
- **Cargo** (comes with Rust)
- **Visual Studio Build Tools** (Windows) or **GCC** (Linux/Mac)

### Steps

1. **Clone the repository:**
```bash
git clone https://github.com/yourusername/SpeedrunRust.git
cd SpeedrunRust/RustSpeedRun-V1
```

2. **Build the project:**
```bash
cargo build
```

3. **Run the project:**
```bash
cargo run
```

---

## 💻 Usage

### Running the Full Demo

Simply run:
```bash
cargo run
```

This will execute all modules and demonstrate:
- Expressions and statements
- Different types of loops
- Ownership concepts
- References and borrowing
- String and array slicing

### Running Specific Modules

You can modify `main.rs` to run specific modules:

```rust
// Comment out modules you don't want to run
// loops::loops();
ownership::ownership();
```

### Example Output

The program produces colorful terminal output like:

```
======================== This is loops in rust!!! ========================
[Colorful loop demonstrations]

======================== Ownership ========================
[Ownership examples with colored output]
```

---

## 📁 Project Structure

```
RustSpeedRun-V1/
├── src/
│   ├── main.rs                    # Main entry point
│   ├── expressions.rs             # Expressions and statements
│   ├── loops.rs                   # Loop demonstrations
│   ├── ownership.rs               # Ownership concepts
│   ├── take_ownership.rs          # Ownership transfer examples
│   ├── referencing_dereferencing.rs # References and borrowing
│   └── slice_type.rs              # String and array slices
├── Cargo.toml                     # Project dependencies
└── README.md                       # This file
```

---

## 📦 Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| `colored` | 2.1 | Terminal text coloring |

### Adding Dependencies

To add a new dependency, edit `Cargo.toml`:

```toml
[dependencies]
colored = "2.1"
your-crate = "version"
```

Then run:
```bash
cargo build
```

---

## 🎓 Learning Path

This project follows a logical learning progression:

1. **Start Here** → `expressions.rs` - Basic Rust syntax
2. **Control Flow** → `loops.rs` - Iteration and loops
3. **Memory Management** → `ownership.rs` - Understanding ownership
4. **Advanced Ownership** → `take_ownership.rs` - Ownership transfer
5. **References** → `referencing_dereferencing.rs` - Borrowing
6. **Slices** → `slice_type.rs` - Working with slices

---

## 🎨 Color Output

This project uses the `colored` crate for beautiful terminal output:

```rust
use colored::*;

println!("{}", "Hello".green());
println!("{}", "World".red().bold());
println!("{}", "Rust".cyan().underline());
```

**Available Colors:**
- Basic: `red()`, `green()`, `blue()`, `yellow()`, `cyan()`, `magenta()`
- Bright: `bright_red()`, `bright_green()`, etc.
- Styles: `bold()`, `underline()`, `italic()`, `dimmed()`

---

## 📝 Notes

- This is a **learning project** - code is organized for educational purposes
- Some functions return placeholder values for demonstration
- Comments explain concepts throughout the code
- Feel free to modify and experiment!

---

## 🤝 Contributing

This is a personal learning project, but suggestions and improvements are welcome!

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

---

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

---

## 👤 Author

**Your Name**

- Learning Rust one concept at a time 🦀
- Building projects to understand the language better

---

## 🔗 Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

---

<div align="center">

**Made with ❤️ and Rust 🦀**

⭐ Star this repo if you found it helpful!

**Happy Learning! 🚀**

</div>

