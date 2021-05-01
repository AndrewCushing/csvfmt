use std::cmp::min;

pub fn to_csv(content: &str, eol: &str, delim: &str, rows: Option<&&String>) -> String {
    if content.len() == 0 {
        return String::new()
    }

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
    result.push_str(get_separator_line(total_width).as_str());

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

    result.push_str(get_separator_line(total_width).as_str());
    result
}

fn fit_to_width(s: &str, width: usize) -> String {
    let mut result = String::from(s);
    for _ in 0..(width - min(width, s.len())) {
        result.push_str(" ");
    }
    result
}

fn get_separator_line(width: usize) -> String {
    let mut result = String::new();
    for _i in 0..width {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_separator_line_zero() {
        assert_eq!(String::from("\n"), get_separator_line(0));
    }

    #[test]
    fn get_separator_line_ten() {
        assert_eq!(String::from("----------\n"), get_separator_line(10));
    }

    #[test]
    fn fit_to_width_string_less_than_width() {
        assert_eq!(String::from("bob  "), fit_to_width("bob", 5));
    }

    #[test]
    fn fit_to_width_string_equal_to_width() {
        assert_eq!(String::from("bob  "), fit_to_width("bob", 5));
    }

    #[test]
    fn fit_to_width_string_longer_than_width() {
        assert_eq!(String::from("silly me"), fit_to_width("silly me", 2));
    }

    #[test]
    fn fit_to_width_string_empty() {
        assert_eq!(String::from("        "), fit_to_width("", 8));
    }

    #[test]
    fn get_col_widths_some_zeros_all_rows() {
        let data = vec![
            vec!["", "", "0o0"],
            vec!["", "", "fdd"],
            vec!["", "", "ghuei"]
        ];

        assert_eq!(vec![0, 0, 5], get_col_widths(&data, data.len()));
    }

    #[test]
    fn get_col_widths_some_zeros_not_all_rows() {
        let data = vec![
            vec!["", "", "0o0"],
            vec!["", "55", "fdd"],
            vec!["", "", "ghuei"]
        ];

        assert_eq!(vec![0, 2, 3], get_col_widths(&data, 2));
    }

    #[test]
    fn get_col_widths_no_rows() {
        let data = vec![
            vec!["yum", "ok", "0o0"],
            vec!["654", "000", "fd654d"],
            vec!["", "ty", "ghuei"]
        ];

        assert_eq!(vec![3, 2, 3], get_col_widths(&data, 0));
    }


}