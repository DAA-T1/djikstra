# Djikstra

Rusty Djikstra

## The Question

> Implement Djikstra's Algorithm in Rust.

## Style Conventions

- Use `cargo fmt` to format your code. Make sure `cargo clippy` says your code is clean.

## Building

- Build only: `cargo build`

## Running

- Build and Run: `cargo run -- <arguments>`.
- Or, see your `/target` folder.

## Tests

- sample present in `graph.rs`.

## Input

Adjacency List of Length N, in the form:

```txt
Number_of_Vertices 
Vertex,Weight Vertex,Weight
Vertex,Weight
```

Each line represents a vertex and its edges.

The files will only have the input for the graph and the source vertex could then be passed as a separate command line option.

## Output

```txt
0 Distance (Path)
1 Distance (Path)
2 Distance (Path)
...
```

Example:

```txt
0 1 (1 -> 2 -> 3)
1 2 (1 -> 3)
```
