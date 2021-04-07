use std::env;
use std::collections::HashMap;
use csvfmt::run;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        exit(0);
    }

    let opt_map = match args.contains(&String::from("stdin")) {
        true => parse_opts(&args[1..(args.len()-1)]),
        false => parse_opts(&args[1..]),
    };

    run(opt_map, args.last().expect("").to_string());
}

fn print_usage() {
    println!("Version {} of the csv formatter", env!("CARGO_PKG_VERSION"));
    println!("Usage:");
    println!("  {} [OPTIONS] FILEPATH", env!("CARGO_BIN_NAME"));
    println!();
    println!("Options:");
    println!("  Option name     Expected data type     Description");
    println!("  --delimiter     string                 Specify the delimiter used. Default is the comma ','");
    println!("  --top           integer                Only print the top n lines");
    println!("  --crlf          boolean                Set to true if the file uses Windows CRLF for line endings, otherwise");
    println!("                                         unix style LF line endings are assumed. Defaults to false");
    println!("  --stdin         boolean                Set to true to read csv data from stdin. Defaults to false. If this is set to");
    println!("                                         true, there's no need to specify a file.");
}

fn parse_opts(args: &[String]) -> HashMap<String, Vec<&String>> {
    let mut i = 0;
    let mut opts = HashMap::new();
    let op_start = "--";
    while i < args.len() {
        let name: String;
        let mut vals: Vec<&String> = Vec::new();

        if args[i].starts_with(op_start) {
            name = args[i].trim_start_matches(op_start).to_string();
        } else {
            name = String::from(":Unknown:");
            vals.push(&args[i]);
        }

        for n in (i+1)..args.len() {
            if args[n].starts_with(op_start) {
                i = n - 1;
                break;
            } else {
                vals.push(&args[n]);
                if n == args.len() - 1 {
                    i = n;
                }
            }
        }
        opts.insert(name, vals);
        i += 1;
    }
    return opts;
}