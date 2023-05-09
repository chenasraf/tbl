use std::{env,io};

fn main() {
    // get args
    let args: Vec<String> = env::args().collect();
    let mut args = &args[1..];

    if args.len() > 0 && (args[0] == "-h" || args[0] == "--help") {
        print_usage();
        return;
    }

    let mut input_str = String::new();
    let mut collect_str = String::new();

    // try to get from stdin
    while io::stdin().read_line(&mut input_str).unwrap() > 0 {
        collect_str.push_str(&input_str);
        input_str.clear();
    }

    // if no streamed input, and no args, print usage
    if collect_str.len() == 0 && args.len() < 1 {
        print_usage();
        return;
    }

    // if no streamed input, but args, use args
    if collect_str.len() == 0 {
        collect_str = args[0].clone();
        args = &args[1..];
    }

    let separator = if args.len() > 0 { &args[0] } else { " " };
    let out_separator = "  ";
    let out = format_as_table(&collect_str, separator, out_separator);
    println!("{}", out);

    return;
}

fn print_usage() {
    println!("tblf ----- Format input as a table");
    println!("");
    println!("Usage:");
    println!("");
    println!("  tbl <string> [separator]");
    println!("  <piped_output> | tbl [separator]");
}

fn format_as_table(input_str: &str, separator: &str, out_separator: &str) -> String {
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
                if spaces > 0 {
                    for _ in 0..(spaces) {
                        result.push_str(" ");
                    }
                }
                result.push_str(out_separator);
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
