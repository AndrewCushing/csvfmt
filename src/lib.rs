use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::cmp::min;
use std::process::exit;

pub fn run(opts: HashMap<String, Vec<&String>>, file_path: String) {
    let delim: String = get_opt_or_default(&opts, String::from("delimiter"), String::from(","));
    let eol: String = match get_bool_opt(&opts, "CRLF", false) {
        true => String::from("\r\n"),
        false => String::from("\n")
    };

    let stdin: bool = get_bool_opt(&opts, "stdin", false);

    let content = read_content(&file_path, stdin);

    let data: Vec<Vec<&str>> = content
        .split_terminator(&eol)
        .map(|line| line.split(&delim).collect())
        .collect();

    let rows = min(data.len(),match opts.get("top") {
        Some(s) if s.len() > 0 => {
            match str::parse(s[0]) {
            Ok(n) => n,
            Err(_) => data.len()
            }
        }
        _ => {data.len()}
    });

    print_data(&data, rows);
}

fn read_content(file_path: &String, stdin: bool) -> String {
    if stdin {
        let mut result: String = String::new();
        match std::io::stdin().read_to_string(&mut result) {
            Ok(_) => {}
            Err(_) => {
                println!("Unable to read contents from stdin");
                exit(1)
            }
        };
        result
    } else {
        match fs::read_to_string(file_path) {
            Ok(content) => {content}
            Err(_) => {
                println!("Unable to read contents of file");
                exit(1);
            }
        }
    }
}

fn print_data(data: &Vec<Vec<&str>>, rows: usize) {
    let widths = get_col_widths(&data, rows);

    let total_width: usize = widths.iter().sum::<usize>() + widths.len() + 1;

    println!("{}", get_table_top(total_width));

    for i in 0..rows {
        let mut line: String = String::new();
        for j in 0..widths.len() {
            let val = match data[i].get(j) {
                None => {""}
                Some(s) => {s}
            };
            line.push_str(format!("|{}", fit_to_width(val, widths[j])).as_str());
        }
        line.push_str("|");
        println!("{}", line);
    }

    println!("{}", get_table_end(total_width));
}

fn fit_to_width(s: &str, width: usize) -> String {
    let mut result = String::from(s);
    for _ in 0..(width - s.len()) {
        result.push_str(" ");
    }
    result
}

fn get_table_top(width: usize) -> String {
    let mut result = String::from("-");
    for _i in 1..width {
        result.push_str("-");
    }
    result
}

fn get_table_end(width: usize) -> String {
    let mut result = String::from("-");
    for _i in 1..width {
        result.push_str("-");
    }
    result
}

fn get_col_widths(data: &Vec<Vec<&str>>, rows: usize) -> Vec<usize> {
    let mut widths: Vec<usize> = data[0].iter().map(|s| s.len()).collect();

    for line_num in 1..rows {
        for val_num in 0..(min(data[line_num].len(), widths.len())) {
            if data[line_num][val_num].len() > widths[val_num] {
                widths.remove(val_num);
                widths.insert(val_num, data[line_num][val_num].len());
            }
        }
    }
    widths
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