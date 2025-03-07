# Memory Manager

This is a Rust implementation of a memory manager using a buddy allocation strategy. The project demonstrates a custom memory management system with the following key features:

## Features

- Memory allocation using best-fit approach
- Buddy memory allocation and deallocation
- Command-line interface for memory operations
- Support for inserting, deleting, reading, and updating memory blocks

## Usage

Run the program by providing a command file:

```bash
cargo run -- commands.cmmd
```

### Command File Format

The command file supports the following operations:

- `INSERT <size> <data>`: Allocate a new memory block
- `DELETE <id>`: Delete a memory block
- `READ <id>`: Read the contents of a memory block
- `UPDATE <id> <new_data>`: Update a memory block
- `DUMP`: Print all allocated and free memory blocks

## Implementation Details

- Total memory buffer: 65,535 bytes
- Best-fit memory allocation
- Buddy memory allocation for deallocation
- Automatic memory defragmentation

## Testing

Run the test suite with:

```bash
cargo test
```

## License

[Add your license information here]
