mod content_reader;
mod formatter;

use std::collections::HashMap;
use std::process::exit;

pub fn run(opts: HashMap<String, Vec<&String>>) {
    let delim: String = get_opt_or_default(&opts, String::from("delimiter"), String::from(","));
    let eol: String = match get_bool_opt(&opts, "w", false) {
        true => String::from("\r\n"),
        false => String::from("\n")
    };

    let stdin: bool = !opts.contains_key("f");

    let content = get_content(stdin, &opts);

    let rows = match opts.get("t") {
        Some(s) => s.first(),
        _ => {None}
    };

    println!("{}", formatter::to_csv(&content, &eol, &delim, rows));
}

fn get_content(stdin: bool, opts: &HashMap<String, Vec<&String>>) -> String {
    match stdin {
        true => {content_reader::read_stdin()}
        false => {
            let file_path: &str = match opts.get("f").unwrap().first() {
                None => {
                    println!("Option -f was supplied without a file path");
                    exit(1);
                }
                Some(x) => {
                    x
                }
            };
            content_reader::read_file(file_path)
        }
    }
}

fn get_bool_opt(opts: &HashMap<String, Vec<&String>>, opt_name: &str, default: bool) -> bool {
    let eol_bool: String = get_opt_or_default(&opts, String::from(opt_name), default.to_string());
    match eol_bool.as_str() {
        "true" => true,
        _ => false
    }
}

fn get_opt_or_default(opts: &HashMap<String, Vec<&String>>, opt_name: String, default: String) -> String {
    match opts.get(&opt_name) {
        None => {default}
        Some(s) => {
            s.first().unwrap_or(&&default).to_string()
        }
    }
}