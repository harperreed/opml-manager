# OPML Manager 📑

Welcome to the **OPML Manager** repository! This project provides a tool for managing OPML feed lists, allowing you to analyze, deduplicate, validate, and generate reports for your feeds. 

## 🔍 Summary of Project
The **OPML Manager** is a Rust-based command-line application designed to handle OPML (Outline Processor Markup Language) files effectively. It offers functionality to:
- Analyze OPML files for duplicates and potential issues.
- Remove duplicate feeds while preserving categories.
- Validate feeds by checking if URLs are reachable and respond with proper RSS/Atom format.
- Generate detailed reports about the OPML file's feeds.

This project aims to equip users with a comprehensive set of tools for OPML file management, ensuring high-quality feed lists.

## 🚀 How to Use
To get started with the OPML Manager, follow the steps below:

### Prerequisites
1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
2. Clone the repository:
   ```bash
   git clone https://github.com/harperreed/opml-manager.git
   cd opml-manager
   ```

### Building the Project
To build the project, run:
```bash
cargo build --release
```

### Running the Application
You can run the OPML Manager using the following commands:

- **Analyze an OPML file:**
  ```bash
  cargo run --release -- analyze <input_file>
  ```
  
- **Deduplicate feeds:**
  ```bash
  cargo run --release -- dedupe <input_file> <output_file>
  ```

- **Validate feeds:**
  ```bash
  cargo run --release -- validate <input_file> --timeout <timeout_in_seconds>
  ```

- **Generate a report:**
  ```bash
  cargo run --release -- report <input_file> <output_file> --validate-feeds --timeout <timeout_in_seconds>
  ```

For more options, use:
```bash
cargo run --release -- --help
```

### Progress Bar for Feed Validation
The OPML Manager now includes a progress bar for feed validation to provide users with real-time updates on the validation process. This feature is especially useful for long-running operations.

## ⚙️ Tech Info
The OPML Manager is built using Rust and leverages several libraries:
- **Clap**: For parsing command-line arguments.
- **Tokio**: An asynchronous runtime for handling network requests.
- **Reqwest**: For making HTTP requests to validate feeds.
- **RoXMLTree**: For parsing and working with XML files.
- **Serde**: For serializing and deserializing data structures.

### Project Structure
The codebase is structured with distinct modules for organization:
- `cli.rs`: Command-line interface functionality.
- `error.rs`: Custom error types and result handling.
- `feed.rs`: Feed data model.
- `lib.rs`: Core library functionality.
- `opml.rs`: Parsing and generating OPML files.
- `report.rs`: Report generation functionality.
- `validation.rs`: Validation logic for feeds.
  
### Dependencies
Check the `Cargo.toml` file for a complete list of dependencies.

### Running Tests
To run the tests associated with this project, execute:
```bash
cargo test
```

### Continuous Integration
The project employs GitHub Actions for continuous integration, ensuring that tests pass on every push.

---

Happy coding! 🎉 Feel free to reach out for any queries or contributions!
