use clap::Parser;
use libmemtester::MemoryTests;
use regex::Regex;
use std::io::{stdout, Write};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of cycles to run
    #[clap(short, long, default_value_t = 1)]
    cycles: usize,

    /// The memory size to allocate (default in bytes) : eg. 1G, 32M, 128K, ...
    size: String,
}

fn flush() {
    stdout().flush().expect("Failed to flush stdout");
}

fn main() {
    let args = Args::parse();

    let size_regex = Regex::new(r"^(\d+)(\w)$").unwrap();
    let cap = size_regex.captures(&args.size);
    if cap.is_none() {
        println!("Invalid size format");
        return;
    }
    let cap = cap.unwrap();
    let size = match cap[1].parse::<usize>() {
        Ok(size) => size,
        Err(_) => {
            println!("Invalid size format");
            return;
        }
    };
    let unit = &cap[2];
    let allocation_size = match unit {
        "K" => size * 1024,
        "M" => size * 1024 * 1024,
        "G" => size * 1024 * 1024 * 1024,
        _ => {
            println!("Invalid size format");
            return;
        }
    };

    let mem_tests_res = MemoryTests::new(allocation_size, false);
    if let Err(e) = mem_tests_res {
        println!("Error: {}", e);
        return;
    }
    let mut mem_tests = mem_tests_res.unwrap();

    println!(
        "Running {} cycles of memory tests on {} bytes",
        args.cycles, allocation_size
    );

    let mut error_count = 0;
    for cycle in 0..args.cycles {
        println!("\nCycle {}/{}", cycle + 1, args.cycles);
        let mut test_iter = mem_tests.get_iterator();

        print!("Test {}: running...", test_iter.next_test_name().unwrap());
        flush();
        while let Some((name, errors)) = test_iter.next() {
            println!("\r\x1B[2KTest {}: {} errors", name, errors);
            error_count += errors;

            if let Some(next_test_name) = test_iter.next_test_name() {
                print!("Test {}: running...", next_test_name);
                flush();
            }
        }
    }

    println!("\nAll tests done");

    println!("Total errors: {}", error_count);

    for (name, errors) in mem_tests.get_errors() {
        println!("Address {}: {} errors", name, errors);
    }
}
