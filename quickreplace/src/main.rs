use std::env;

#[derive(Debug)]
struct Arguments {
    target: String,
    filename: String,
    replacement: String,
    output: String,
}
fn main() {
    println!("Hello, world!");
    let args = parse_arguments();
    println!("{:?}", args);
}

fn parse_arguments() -> Arguments {
    let arguments: Vec<String> = env::args().skip(1).collect();

    if arguments.len() != 4 {
        print_usage();
        std::process::exit(1);
    }

    Arguments {
        target: arguments[0].clone(),
        filename: arguments[1].clone(),
        replacement: arguments[2].clone(),
        output: arguments[3].clone()
    }
}

fn print_usage() {
    eprintln!("Usage: quickreplace <target> <replacement> <input file> <output file>");
}
