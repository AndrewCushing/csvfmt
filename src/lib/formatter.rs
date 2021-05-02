use std::cmp::min;
use unicode_segmentation::UnicodeSegmentation;

pub fn to_csv(content: &str, eol: &str, delim: &str, rows: Option<&&String>) -> String {
    let trimmed = content.trim();

    if trimmed.len() == 0 {
        return String::from("\n");
    }

    let data: Vec<Vec<&str>> = trimmed
        .split_terminator(&eol)
        .map(|line| line.split(&delim).collect())
        .collect();

    let rows = min(data.len(), match rows {
        Some(s) if s.len() > 0 => {
            match str::parse(s) {
                Ok(n) => n,
                Err(_) => data.len()
            }
        }
        _ => { data.len() }
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
                None => { "" }
                Some(s) => { s }
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
            if get_len(data[line_num][val_num]) > widths[val_num] {
                widths.remove(val_num);
                widths.insert(val_num, get_len(data[line_num][val_num]));
            }
        }
    }
    widths
}

fn get_len(s: &str) -> usize {
    s.graphemes(true).collect::<Vec<&str>>().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_len_normal_chars() {
        assert_eq!(3, get_len("abc"));
    }

    #[test]
    fn get_len_no_chars() {
        assert_eq!(0, get_len(""));
    }

    #[test]
    fn get_len_fancy_chars() {
        assert_eq!(15, get_len("Владимир Петков"));
    }

    #[test]
    fn get_separator_line_zero() {
        assert_eq!("\n", get_separator_line(0));
    }

    #[test]
    fn get_separator_line_ten() {
        assert_eq!("----------\n", get_separator_line(10));
    }

    #[test]
    fn fit_to_width_string_less_than_width() {
        assert_eq!("bob  ", fit_to_width("bob", 5));
    }

    #[test]
    fn fit_to_width_string_equal_to_width() {
        assert_eq!("bob  ", fit_to_width("bob", 5));
    }

    #[test]
    fn fit_to_width_string_longer_than_width() {
        assert_eq!("silly me", fit_to_width("silly me", 2));
    }

    #[test]
    fn fit_to_width_string_empty() {
        assert_eq!("        ", fit_to_width("", 8));
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

    #[test]
    fn get_data_in_string_starts_and_ends_with_separator() {
        let data = vec![
            vec!["yum", "ok", "0o0", "kldg", "lskdgh"],
            vec!["654", "000", "fd654d", "lkjhg", "iuf9u0"],
            vec!["", "ty", "ghuei"],
            vec!["dfg", "gdfsdf", "sdffg", "gg", ""]
        ];

        let actual_result = get_data_in_string(&data, 4);

        assert_eq!(true, actual_result.starts_with(&get_separator_line(32)));
        assert_eq!(true, actual_result.ends_with(&get_separator_line(32)));
    }

    #[test]
    fn get_data_in_string_has_correct_number_of_lines() {
        let data = vec![
            vec!["yum", "ok", "", "kldg", "lskdgh"],
            vec!["", "", "", "", ""],
            vec!["", "", ""],
            vec!["", "", "", "", ""],
            vec!["", "", "", "", ""],
        ];

        let actual_result = get_data_in_string(&data, 5);

        assert_eq!(8, actual_result.split("\n").fold(0, |i, _| i + 1));
    }

    #[test]
    fn to_csv_no_content() {
        assert_eq!("\n",
                   to_csv("", "\n", ",", Some(&&String::from("5"))));
    }

    #[test]
    fn to_csv_no_delimiters_one_line() {
        assert_eq!("-------\n|hello|\n-------\n",
                   to_csv("hello", "\n", ",", Some(&&String::from("5"))));
    }

    #[test]
    fn to_csv_no_delimiters_two_lines() {
        assert_eq!("-------\n|hello|\n|world|\n-------\n",
                   to_csv("hello\nworld", "\n", ",", Some(&&String::from("5"))));
    }

    #[test]
    fn to_csv_normal_content() {
        assert_eq!("-------\n|1|2|3|\n|4|5|6|\n-------\n",
                   to_csv("1,2,3\n4,5,6", "\n", ",", Some(&&String::from("5"))));
    }

    #[test]
    fn to_csv_blank_first_and_last_line() {
        assert_eq!("-------\n|1|2|3|\n|4|5|6|\n-------\n",
                   to_csv("\n1,2,3\n4,5,6\n", "\n", ",", Some(&&String::from("5"))));
    }

    #[test]
    fn to_csv_extra_data_on_lines_after_first() {
        assert_eq!("--------\n|1|2 |3|\n|4|5 |6|\n|1|66|4|\n--------\n",
                   to_csv("\n1,2,3\n4,5,6,7\n1,66,4\n", "\n", ",", Some(&&String::from("5"))));
    }

    #[test]
    fn to_csv_missing_data_on_lines_after_first() {
        assert_eq!("--------\n|1|2 |3|\n|4|5 | |\n|1|66|4|\n--------\n",
                   to_csv("\n1,2,3\n4,5\n1,66,4\n", "\n", ",", Some(&&String::from("5"))));
    }

    #[test]
    fn to_csv_only_print_1_row() {
        assert_eq!("-------\n|1|2|3|\n-------\n",
                   to_csv("\n1,2,3\n4,5,6,7\n", "\n", ",", Some(&&String::from("1"))));
    }

    #[test]
    fn to_csv_only_print_2_rows() {
        assert_eq!("-------\n|1|2|3|\n|4|5|6|\n-------\n",
                   to_csv("\n1,2,3\n4,5,6,7\n", "\n", ",", Some(&&String::from("2"))));
    }

    #[test]
    fn to_csv_print_0_rows() {
        assert_eq!("-------\n-------\n",
                   to_csv("\n1,2,3\n4,5,6,7\n", "\n", ",", Some(&&String::from("0"))));
    }

    #[test]
    fn to_csv_spaces_in_content() {
        let mut expected = String::new();
        expected.push_str("------------------------------------\n");
        expected.push_str("|hi there|I'm bob|what's your name?|\n");
        expected.push_str("|4       |5      |6                |\n");
        expected.push_str("------------------------------------\n");
        assert_eq!(expected,
                   to_csv("\nhi there,I'm bob,what's your name?\n4,5,6,7\n",
                          "\n", ",", Some(&&String::from("2"))));
    }

    #[test]
    fn to_csv_non_comma_delimiter() {
        let mut expected = String::new();
        expected.push_str("------------------------------------\n");
        expected.push_str("|hi there|I'm bob|what's your name?|\n");
        expected.push_str("|4       |5      |6                |\n");
        expected.push_str("------------------------------------\n");
        assert_eq!(expected,
                   to_csv("\nhi there|I'm bob|what's your name?\n4|5|6|7\n",
                          "\n", "|", Some(&&String::from("2"))));
    }
}