//! CLI interface for running and benchmarking the Djikstra algorithm.
use clap::{Args, Parser, Subcommand};
use djikstra::djikstra::djikstra;
use djikstra::graph::Graph;
use std::str::FromStr;
use std::time::Instant;
use std::{fs, path::PathBuf};

/// CLI interface for running and benchmarking the Djikstra algorithm.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Verbose mode.
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Subcommands.
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the algorithm on the input graph.
    Run(RunArgs),

    /// Benchmarks the algorithm on the input graph.
    Benchmark(BenchmarkArgs),
}

/// Arguments for the run subcommand.
#[derive(Args)]
struct RunArgs {
    /// Input file that contains the graph.
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input_path: PathBuf,
}

/// Arguments for the benchmark subcommand.
#[derive(Args)]
struct BenchmarkArgs {
    /// Input file that contains the graph.
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input_path: PathBuf,
    /// Number of times to run the algorithm for benchmarking.
    #[arg(short, default_value_t = 1000)]
    n: usize,
}

fn main() {
    let args = Cli::parse();

    let verbosity = args.verbose;

    match &args.command {
        Commands::Run(cmd_args) => {
            run_command(cmd_args, verbosity);
        }
        Commands::Benchmark(cmd_args) => {
            benchmark_command(cmd_args, verbosity);
        }
    }
}

/// Run the Djikstra algorithm on the input graph.
fn run_command(args: &RunArgs, verbose: bool) {
    // djikstra run --input graph.txt --verbose

    let (start_vertex, graph) = match parse_input(&args.input_path) {
        Ok((start_vertex, graph)) => (start_vertex, graph),
        Err(e) => {
            eprintln!("Error parsing input: {0}", e.0);
            return;
        }
    };

    if verbose {
        println!("Read file {0:?} successfully.", &args.input_path);
        println!(
            "Running algorithm on graph with {0} vertices and start vertex {1}.\n",
            graph.n_vertices(),
            start_vertex
        );
    }

    // run the algorithm
    let start = Instant::now();
    let (paths_from_src, dists_from_src) = djikstra(&graph, start_vertex);
    let duration = start.elapsed();

    for idx in 0..graph.n_vertices() {
        if let Some(path) = &paths_from_src[idx] {
            print!("{idx} {} ", dists_from_src[idx]);
            print!("({}", path[0]);
            for vertex in path.iter().skip(1) {
                print!(" -> {}", vertex);
            }
            println!(")");
        } else {
            println!("{idx} inf");
        }
    }

    println!("Algorithm ran in {0}ns.", duration.as_nanos());
}

/// Benchmark the Djikstra algorithm on the input graph.
fn benchmark_command(args: &BenchmarkArgs, verbose: bool) {
    // djikstra benchmark --input graph.txt -n 1000

    let (start_vertex, graph) = match parse_input(&args.input_path) {
        Ok((start_vertex, graph)) => (start_vertex, graph),
        Err(e) => {
            eprintln!("Error parsing input: {0}", e.0);
            return;
        }
    };

    if verbose {
        println!(
            "Benchmarking {0:?} over {1:?} times.",
            &args.input_path, args.n
        );
        println!(
            "Algorithm will run on graph with {0} vertices and start vertex {1}.\n",
            graph.n_vertices(),
            start_vertex
        );
    }

    // benchmark the algorithm
    let mut results: Vec<u128> = vec![];

    for _ in 0..args.n {
        let start = Instant::now();
        let (_paths_from_src, _dists_from_src) = djikstra(&graph, start_vertex);
        let duration = start.elapsed();

        results.push(duration.as_nanos());
    }

    let avg_time = results.iter().sum::<u128>() / args.n as u128;
    println!("Average time: {0}ns", avg_time);
}

/// The error type returned when we run into any error when parsing
#[derive(Debug)]
struct InputError(String);

/// Parse the input file into a start vertex and a graph.
fn parse_input(input_path: &PathBuf) -> Result<(usize, Graph), InputError> {
    let contents = fs::read_to_string(input_path);
    let contents = contents.map_err(|e| InputError(format!("error reading file: {}", e)))?;

    let (start_vertex_str, graph_data) = contents
        .split_once('\n')
        .ok_or(InputError("cannot split on newline".to_string()))?;

    let start_vertex: usize = start_vertex_str
        .parse()
        .map_err(|e| InputError(format!("cannot parse start vertex: {}", e)))?;

    let graph = Graph::from_str(graph_data)
        .map_err(|e| InputError(format!("cannot parse graph: {}", e)))?;

    Ok((start_vertex, graph))
}
