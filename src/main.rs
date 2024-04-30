use std::time::{Duration, Instant};
use std::{fs, path::PathBuf, str::FromStr};

use clap::{Args, Parser, Subcommand};

mod graph;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Verbose mode.
    #[arg(short, long, global = true)]
    verbose: bool,

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

#[derive(Args)]
struct RunArgs {
    /// Input file that contains the graph.
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input_path: PathBuf,
}

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
    // TODO: ADD DJIKSTRA FUNCTION CALL
    let duration = start.elapsed();
    if verbose {
        println!("Algorithm ran in {0} ns.", duration.as_nanos());
    }

}

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
        println!("Benchmarking {0:?} over {1:?} times.", &args.input_path, args.n);
        println!(
            "Algorithm will run on graph with {0} vertices and start vertex {1}.\n",
            graph.n_vertices(),
            start_vertex
        );
    }

    // benchmark the algorithm
    // store the results in a vector
    let mut results: Vec<i64> = vec![0; args.n];

    for _ in 0..args.n {
        let start = Instant::now();
        // TODO: ADD DJIKSTRA FUNCTION CALL
        let duration = start.elapsed();

        results.push(duration.as_nanos() as i64);
    }

    let avg_time = results.iter().sum::<i64>() / results.len() as i64;

    println!("Average time: {0} ns", avg_time);

    if verbose {
        println!("Results: {0:?}", results);
    }
}

#[derive(Debug)]
struct InputError(String);

fn parse_input(input_path: &PathBuf) -> Result<(i64, graph::Graph), InputError> {
    let contents = fs::read_to_string(input_path);
    let contents = contents.map_err(|e| InputError(format!("error reading file: {}", e)))?;

    let (start_vertex_str, graph_data) = contents
        .split_once('\n')
        .ok_or(InputError("cannot split on newline".to_string()))?;

    let start_vertex: i64 = start_vertex_str
        .parse()
        .map_err(|e| InputError(format!("cannot parse start vertex: {}", e)))?;

    let graph = graph::Graph::from_str(graph_data)
        .map_err(|e| InputError(format!("cannot parse graph: {}", e)))?;

    Ok((start_vertex, graph))
}


mod test { 

    use super::*;

    #[test]
    fn test_parse_input() {
        let expected: (i64, graph::Graph) = (2, graph::Graph::new(vec![
            vec![(1, 3), (3, 4)],
            vec![(3, 4), (5, 2)],
            vec![(4, 2), (3, 2)],
            vec![(2, 2)],
            vec![(4, 2), (3, 2), (1, 4)],
        ]));
        
        // let input = "1\n4\n1,2 2,3\n3,4"; // IS THERE A WAY TO MOCK A FILE?
        // let fake_file = PathBuf::from("fake_file.txt"); // IS THERE A WAY TO MOCK A FILE?
        
        let path: PathBuf = ["data", "sample_input.txt"].iter().collect();
        let (start_vertex, graph) = parse_input(&path).unwrap();

        assert_eq!(start_vertex, expected.0);
        assert_eq!(graph, expected.1);
    }
}