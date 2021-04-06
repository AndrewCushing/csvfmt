use std::collections::HashMap;
use std::fs;

pub fn run(opts: HashMap<String, Vec<&String>>, file_path: String) {
    let delim: String = get_delim(&opts);
    let eol: String = get_eol(&opts);

    let content = fs::read_to_string(&file_path).expect("Unable to read content of file");

    let data: Vec<Vec<&str>> = content
        .split_terminator(&eol)
        .map(|line| line.split(&delim).collect())
        .collect();

    let widths = get_col_widths(&data);

    let total_width: usize = widths.iter().sum::<usize>() + widths.len() + 1;

    println!("{}", get_table_top(total_width));

    for i in 0..data.len() {
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

    println!("{}", get_table_top(total_width));
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
    for i in 1..width {
        result.push_str("-");
    }
    result
}

fn get_table_end(width: usize) -> String {
    let mut result = String::from("-");
    for i in 1..width {
        result.push_str("-");
    }
    result
}

fn get_col_widths(data: &Vec<Vec<&str>>) -> Vec<usize> {
    let mut widths: Vec<usize> = data[0].iter().map(|s| s.len()).collect();

    for line_num in 1..data.len() {
        for val_num in 0..widths.len() {
            if data[line_num][val_num].len() > widths[val_num] {
                widths.remove(val_num);
                widths.insert(val_num, data[line_num][val_num].len());
            }
        }
    }
    widths
}

fn get_delim(opts: &HashMap<String, Vec<&String>>) -> String {
    get_opt_or_default(&opts, String::from("delimiter"), String::from(","))
}

fn get_eol(opts: &HashMap<String, Vec<&String>>) -> String {
    let eol_bool: String = get_opt_or_default(&opts, String::from("CRLF"), String::from("false"));
    match eol_bool.as_str() {
        "true" => {String::from("\r\n")}
        _ => {String::from("\n")}
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