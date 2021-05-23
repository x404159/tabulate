use clap::{AppSettings, Clap};
use std::fs::File;
use std::io::Write;

#[derive(Clap)]
#[clap(
    version = "0.1",
    author = "Abhinav <abhinavy14@gmail.com>",
    about = "prettify csv to table"
)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(value_name = "FILE")]
    input: String,
    #[clap(short, long, default_value = "\n", about = "csv delimiter")]
    delimiter: char,
    #[clap(short, long, default_value = "table.txt", about = "output file_name")]
    output: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let Opts {
        input,
        delimiter,
        output,
    } = opts;
    let data = std::fs::read_to_string(input).unwrap();
    let data = parse_csv(data, delimiter);
    println!("processing...");
    std::io::stdout().flush().unwrap();
    let table = process_string_to_table(&data, calc_max_len(&data));
    let mut f = File::create(&output).unwrap();
    f.write_all(table.as_bytes())
        .expect("unable to write to file");
    println!("\n{}\n\ntable saved to {}", &table, &output);

    println!("DONE!!!!!!!!!");
}

fn process_string_to_table(data: &[Vec<String>], col_len: Vec<usize>) -> String {
    let mut table = String::new();
    for row in 0..data.len() {
        table = format!(
            "{}{}{}",
            table,
            process_non_data_line(&col_len),
            process_data_line(row, data, &col_len)
        );
    }
    format!("{}{}", table, process_non_data_line(&col_len))
}

const PADDING: usize = 4;
const PURPLE: &str = "\x1b[38;5;57m";
const GREEN: &str = "\x1b[32;1m";
const RESET: &str = "\x1b[0m";

fn process_non_data_line(col_len_vec: &[usize]) -> String {
    let mut ret_string = String::new();
    for size in col_len_vec {
        ret_string = format!("{3}{2}+{:-<1$}", "-", size + PADDING, ret_string, PURPLE);
    }
    format!("{}{1}+{2}\n", ret_string, PURPLE, RESET)
}

fn process_data_line(row: usize, data: &[Vec<String>], col_len_vec: &[usize]) -> String {
    let mut ret_string = String::new();
    for (i, size) in col_len_vec.iter().enumerate() {
        ret_string = format!(
            "{2}{5}|{3}{:^1$}{4}",
            data[row][i],
            size + PADDING,
            ret_string,
            GREEN,
            RESET,
            PURPLE
        );
    }
    format!("{}{1}|{2}\n", ret_string, PURPLE, RESET)
}

fn calc_max_len(data: &[Vec<String>]) -> Vec<usize> {
    let mut length = vec![0; data[0].len()];
    for row_vec in data {
        for (j, s) in row_vec.iter().enumerate() {
            if length[j] < s.len() {
                length[j] = s.len();
            }
        }
    }
    length
}

fn parse_csv(data: String, delimiter: char) -> Vec<Vec<String>> {
    let mut rows = data
        .split(delimiter)
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    if rows.last().unwrap() == "" {
        rows.pop();
    }
    rows.iter()
        .map(move |row| row.split(',').map(|s| s.to_owned()).collect())
        .collect()
}
