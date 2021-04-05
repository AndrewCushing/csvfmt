use std::env;
use std::collections::HashMap;
use csvfmt::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
    }

    let delim: char;
    if args.len() == 2 {
        delim = ',';
    } else {
        let opt_map = parse_opts(&args[1..=(args.len()-1)]);
        run(opt_map);
    }
}

fn print_usage() {
    println!("Version {} of the csv formatter", env!("CARGO_PKG_VERSION"));
    println!("Usage:");
    println!("  {} [OPTIONS] FILEPATH", env!("CARGO_BIN_NAME"));
    println!();
    println!("Options:");
    println!("  --delimiter     Specify the delimiter used. Default is the comma (,)");
}

fn parse_opts(args: &[String]) -> HashMap<String, Vec<&String>> {
    let mut i = 0;
    let mut opts = HashMap::new();
    let op_start = "--";
    while i < args.len() {
        println!("start");
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