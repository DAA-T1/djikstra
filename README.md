# Djikstra

Blazingly Fast Djikstra's Path Finding Algorithm implemented in Rust.

## The Question

> Implement Djikstra's Algorithm in Rust.

## Building

- Build only: `cargo build --release`. You can then find the binary in `/target/release`.

## Running

- Build and Run: `cargo run --release -- <arguments>`.
- Help Menu: `cargo run --release -- --help`. Use this to get a list of all available options.
- Example: `cargo run --release -- run --input input.txt`.
- Example: `cargo run --release -- benchmark --input input.txt -n 100000`.

## Tests for Correctness

Run testcases to check for correctness using `cargo test`.

## Advanced Tests

- We have testcases (and their expected output) in the `tests` directory. The expected output is generated using the `networkX` library in Python.
- Run the files using `cargo run --release -- run --input data/<filename>.txt` and compare the output with the expected output using the `diff` command.
- All the testcases are correct and have been verified using the above approach.

## Documentation

Use `cargo doc --open` to browse the documentation in your browser.


## Input

### Format

Adjacency List of length `N`, in the form:

```txt
Source_Vertex
Number_of_Vertices (N)
Vertex,Weight Vertex,Weight
Vertex,Weight
```

Each line represents a vertex and its edges (with weights) to other vertices.

### Example

```txt
0
3
1,3 2,3
2,2 0,3
1,2 0,3
```

## Output

### Format

```txt
0 Distance (Path)
1 Distance (Path)
2 Distance (Path)
...
```

### Example

```txt
0 0 (0)
1 2 (1 -> 2 -> 3)
2 1 (1 -> 3)
```

## Contributing

### Style Guide

- Use `cargo fmt` to format your code. Make sure `cargo clippy` says your code is clean.
- Write tests for your code.
- Write documentation for your code.

