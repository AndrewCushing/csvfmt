mod args_parser;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let opt_map = args_parser::parse_args(&args[1..]);

    if opt_map.contains_key("h") {
        print_usage();
        exit(0);
    }

    csvfmt::run(opt_map);
}

fn print_usage() {
    println!("Version {} of the csv formatter", env!("CARGO_PKG_VERSION"));
    println!("Usage:");
    println!("  {} [OPTIONS] FILEPATH", env!("CARGO_BIN_NAME"));
    println!();
    println!("Options:");
    println!("  Flag            Expected data type     Description");
    println!("  -d              string                 Specify the delimiter used. Default is the comma ','");
    println!("  -f              string                 Name of file to read instead of stdin");
    println!("  -h              none                   Print this help message. No other actions will be performed");
    println!("  -t              integer                Only print the top n lines");
    println!("  -w              none                   Set to true if the file uses Windows CRLF for line endings,");
    println!("                                         otherwise unix style LF line endings are assumed. Defaults ");
    println!("                                         to false");
}