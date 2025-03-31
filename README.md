# To-Do List Manager

A simple command-line To-Do List application built with Rust.

## Features

- Add tasks
- List all tasks
- Remove tasks
- Data is stored in a JSON file

## Installation

1. Clone the repository:
   ```sh
   git clone <repository_url>
   cd <repository_name>
   ```
2. Install dependencies:
   ```sh
   cargo build
   ```

## Usage

Run the program with the following commands:

- **Add a task:**

  ```sh
  cargo run -- add "Complete Rust project"
  ```

- **List all tasks:**

  ```sh
  cargo run -- list
  ```

- **Remove a task by ID:**

  ```sh
  cargo run -- remove 1
  ```

## Dependencies

- [clap](https://crates.io/crates/clap) (for command-line argument parsing)
- [serde](https://crates.io/crates/serde) (for JSON serialization/deserialization)
- [serde\_json](https://crates.io/crates/serde_json) (for handling JSON data)


