use std::collections::HashMap;
use std::fs;

pub fn run(opts: HashMap<String, Vec<&String>>, file_path: String) {
    println!("{:?}", opts);

    let delim: String = get_delim(&opts);
    let eol: String = get_eol(&opts);

    let content = fs::read_to_string(&file_path).expect("Unable to read content of file");

    let data: Vec<Vec<&str>> = content
        .split_terminator(&eol)
        .map(|line| line.split(&delim).collect())
        .collect();

    let widths = get_col_widths(data);



    println!("Column widths are: {:?}", widths);
}

fn get_col_widths(data: Vec<Vec<&str>>) -> Vec<usize> {
    let mut widths: Vec<usize> = data[0].iter().map(|s| s.len()).collect();

    for line_num in 1..data.len() {
        for val_num in 0..data[line_num].len() {
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