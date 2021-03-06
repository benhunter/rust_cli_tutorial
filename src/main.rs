// Rust CLI Book Tutorial
// https://rust-cli.github.io/book/tutorial


use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    pattern: String, // The pattern to look for.
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,  // The path to the file to read. PathBuf provides cross-platform file paths.
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs:read_to_string(&args.path);
    //        .expect("could not read file");

    // for line in content.lines() {
    //    if line.contains(&args.pattern) {
    //        println!("{}", line);
    //    }
    //}
}
