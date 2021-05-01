use std::cmp::min;

pub fn to_csv(content: &str, eol: &str, delim: &str, rows: Option<&&String>) -> String {
    let data: Vec<Vec<&str>> = content
        .split_terminator(&eol)
        .map(|line| line.split(&delim).collect())
        .collect();

    let rows = min(data.len(),match rows {
        Some(s) if s.len() > 0 => {
            match str::parse(s) {
                Ok(n) => n,
                Err(_) => data.len()
            }
        }
        _ => {data.len()}
    });

    get_data_in_string(&data, rows)
}

fn get_data_in_string(data: &Vec<Vec<&str>>, rows: usize) -> String {
    let widths = get_col_widths(&data, rows);

    let total_width: usize = widths.iter().sum::<usize>() + widths.len() + 1;

    let mut result = String::new();
    result.push_str(get_separator(total_width).as_str());

    for i in 0..rows {
        for j in 0..widths.len() {
            let val = match data[i].get(j) {
                None => {""}
                Some(s) => {s}
            };
            result.push_str(format!("|{}", fit_to_width(val, widths[j])).as_str());
        }
        result.push_str("|\n");
    }

    result.push_str(get_separator(total_width).as_str());
    result
}

fn fit_to_width(s: &str, width: usize) -> String {
    let mut result = String::from(s);
    for _ in 0..(width - s.len()) {
        result.push_str(" ");
    }
    result
}

fn get_separator(width: usize) -> String {
    let mut result = String::from("-");
    for _i in 1..width {
        result.push_str("-");
    }
    result.push_str("\n");
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