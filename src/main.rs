use std::env;

fn main() {
    // get args
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    if args.len() < 2 {
        // try get from stdin
        let mut input_str = String::new();
        let mut collect_str = String::new();
        while std::io::stdin().read_line(&mut input_str).unwrap() > 0 {
            collect_str.push_str(&input_str);
            input_str.clear();
        }
        if collect_str.len() == 0 {
            println!("Usage: {} <separator>", args[0]);
            return;
        }
        let separator = &args[0];
        let out = format_as_table(&collect_str, separator);
        println!("{}", out);
        return;
    }
    let separator = &args[1];
    let input_str = &args[0];
    let out = format_as_table(&input_str, separator);
    println!("{}", out);
}

fn format_as_table(input_str: &str, separator: &str) -> String {
    let mut result = String::new();
    let mut max_col_len = Vec::new();
    let mut rows = Vec::new();
    for line in input_str.lines() {
        let row = split_line(&line, &separator);
        rows.push(row);
    }
    for row in &rows {
        for (i, col) in row.iter().enumerate() {
            if i >= max_col_len.len() {
                max_col_len.push(col.len());
            } else if col.len() > max_col_len[i] {
                max_col_len[i] = col.len();
            }
        }
    }
    for row in &rows {
        for (i, col) in row.iter().enumerate() {
            result.push_str(col);
            if i < row.len() - 1 {
                let spaces = max_col_len[i] - col.len();
                for _ in 0..spaces {
                    result.push(' ');
                }
                result.push_str(separator);
            }
        }
        result.push('\n');
    }
    result
}

fn split_line<'a>(line: &'a str, separator: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut end = 0;
    while end < line.len() {
        if line[end..].starts_with(separator) {
            if start != end {
                result.push(&line[start..end]);
            }
            start = end + separator.len();
            end = start;
        } else {
            end += 1;
        }
    }
    if start != end {
        result.push(&line[start..end]);
    }
    result
}
