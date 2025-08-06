# Avro to JSON Converter

A Rust command-line tool to convert Apache Avro files to JSON format.

> **Note**: This project was developed with assistance from Warp AI, demonstrating the power of AI-assisted software development.

## Features

- Convert Avro files to JSON with full type support
- Output to file or stdout
- Pretty-print JSON output
- **Colored JSON output**: Optional syntax highlighting for improved readability
- Support for both JSON array and newline-delimited JSON (NDJSON) output formats
- **Comprehensive compression codec support**: Supports zstandard, snappy, bzip2, and xz codecs for reading compressed Avro files
- Handles all Avro data types including:
  - Primitive types (null, boolean, int, long, float, double, bytes, string)
  - Complex types (records, arrays, maps, unions, enums)
  - Logical types (dates, times, timestamps, decimals, UUIDs, durations)

## Installation

Make sure you have Rust installed, then build the project:

```bash
cargo build --release
```

The binary will be available at `target/release/avro-to-json`.

## Usage

### Basic Usage

Convert an Avro file to JSON (output to stdout):
```bash
cargo run -- -i input.avro
```

Convert an Avro file to a JSON file:
```bash
cargo run -- -i input.avro -o output.json
```

### Options

- `-i, --input <FILE>`: Input Avro file (required)
- `-o, --output <FILE>`: Output JSON file (optional, defaults to stdout)
- `-p, --pretty`: Pretty print JSON output
- `-c, --color`: Colorize JSON output
- `-a, --array`: Output as JSON array instead of newline-delimited JSON
- `-h, --help`: Show help information
- `-V, --version`: Show version information

### Examples

1. Convert with pretty printing:
```bash
cargo run -- -i data.avro -o output.json --pretty
```

2. Output as a single JSON array:
```bash
cargo run -- -i data.avro --array --pretty
```

3. Convert with colored output (in terminal):
```bash
cargo run -- -i data.avro --pretty --color
```

4. Pipe output to another command:
```bash
cargo run -- -i data.avro | jq '.field_name'
```

## Data Type Mapping

| Avro Type | JSON Representation |
|-----------|-------------------|
| null | null |
| boolean | boolean |
| int, long | number |
| float, double | number |
| bytes, fixed | base64-encoded string |
| string | string |
| enum | string (symbol name) |
| array | array |
| map | object |
| record | object |
| union | value of the union member |
| date | "days-since-epoch:{value}" |
| time-millis | "time-millis:{value}" |
| time-micros | "time-micros:{value}" |
| timestamp-millis | "timestamp-millis:{value}" |
| timestamp-micros | "timestamp-micros:{value}" |
| decimal | "decimal:{base64-encoded-bytes}" |
| uuid | standard UUID string |
| duration | "duration:{months}:{days}:{millis}" |

## Dependencies

- `apache-avro` (with multiple codec support): For reading Avro files including those compressed with zstandard, snappy, bzip2, and xz
- `serde_json`: For JSON serialization
- `clap`: For command-line argument parsing
- `anyhow`: For error handling
- `colored`: For colorizing JSON output

## Building and Testing

Build the project:
```bash
cargo build
```

Run tests:
```bash
cargo test
```

Build for release:
```bash
cargo build --release
```

## Example Avro File Creation

Here's how you might create a test Avro file using Python (for testing purposes):

```python
import avro.schema
import avro.io
import io

schema = avro.schema.parse("""
{
  "type": "record",
  "name": "User",
  "fields": [
    {"name": "id", "type": "int"},
    {"name": "name", "type": "string"},
    {"name": "email", "type": ["null", "string"], "default": null}
  ]
}
""")

writer = avro.io.DatumWriter(schema)
bytes_writer = io.BytesIO()
encoder = avro.io.BinaryEncoder(bytes_writer)

# Write some records
writer.write({"id": 1, "name": "Alice", "email": "alice@example.com"}, encoder)
writer.write({"id": 2, "name": "Bob", "email": None}, encoder)

# Save to file
with open("users.avro", "wb") as f:
    f.write(bytes_writer.getvalue())
```

## Project Structure

```
avro-to-json/
├── Cargo.toml              # Project configuration and dependencies
├── src/
│   ├── lib.rs             # Library functions for Avro-to-JSON conversion
│   └── main.rs            # Command-line interface
├── create_test_avro.py    # Python script to generate test Avro files
├── README.md              # This file
└── target/                # Build artifacts (not in version control)
```

## Development

### Running Tests

```bash
cargo test
```

### Building the Project

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Using as a Library

You can also use this as a Rust library in your own projects:

```rust
use avro_to_json::{convert_avro_to_json, avro_value_to_json};

// Convert entire file
convert_avro_to_json("input.avro", Some(&"output.json".to_string()), true, false)?;

// Convert individual Avro values
let json_value = avro_value_to_json(&avro_record)?;
```

## Testing the Project

The project includes comprehensive tests and example files:

1. **Generate test files**: `python3 create_test_avro.py`
2. **Run unit tests**: `cargo test`
3. **Test the CLI**:
   ```bash
   ./target/release/avro-to-json -i test_users.avro --pretty
   ./target/release/avro-to-json -i test_complex.avro --array --pretty
   ./target/release/avro-to-json -i test_products.avro -o output.json
   ```

## Acknowledgments

This project was developed with assistance from Warp AI, showcasing how AI can accelerate and enhance software development workflows.

## License

This project is licensed under the MIT License.
