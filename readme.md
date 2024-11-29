# Rust REPL

Rust REPL is a command-line application that provides an interactive Read-Eval-Print Loop (REPL) for the Rust programming language. It allows you to write and execute Rust code snippets directly from your terminal, making it easier to experiment with Rust code and quickly test ideas.

## Features

- **Interactive REPL**: Write and execute Rust code snippets interactively.
- **Error Handling**: Provides feedback on errors encountered during code execution.

## Installation

To use Rust REPL, you need to have Rust and Cargo installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Build from Source

1. Clone the Rust REPL repository:

   ```bash
   git clone https://github.com/abhishek6262/rust-repl
   ```

2. Navigate to the project directory:

   ```bash
   cd rust-repl
   ```

3. Install the project using Cargo:

   ```bash
   cargo install --path .
   ```

4. Run the REPL from anywhere on your system:

   ```bash
   rust-repl
   ```

## Usage

After building and running Rust REPL, you can use the following commands:

### Writing and Executing Code

Simply type your Rust code and press Enter to execute it. For example:

```rust
? : println!("Hello, world!");
Hello, world!
```

### Special Commands

- **.help**: Displays help information about special commands.

  ```rust
  ? : .help
  The REPL has some special commands, all starting with a dot. They are
  .help: shows the dot commands help
  .editor: enables editor mode, to write multiline Rust code with ease. Once you are in this mode, enter ctrl-D to run the code you wrote.
  .exit: exits the repl
  ```

- **.exit**: Exits the REPL.

  ```rust
  ? : .exit
  Goodbye!
  ```

## Contributing

Contributions to Rust REPL are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## License

Rust REPL is released under the GNU General Public License v3.0.
