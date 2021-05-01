use std::collections::HashMap;

pub fn parse_args(args: &[String]) -> HashMap<String, Vec<&String>> {
    let mut i = 0;
    let mut opts = HashMap::new();
    let op_start = "-";
    while i < args.len() {
        let name: String;
        let mut vals: Vec<&String> = Vec::new();

        if args[i].starts_with(op_start) {
            name = args[i].trim_start_matches(op_start).to_string();
        } else {
            name = String::from(":Unknown:");
            vals.push(&args[i]);
        }

        for n in (i + 1)..args.len() {
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