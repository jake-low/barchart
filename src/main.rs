use std::error::Error;
use std::io::{self, Write};

use clap::Parser;
use nom::number::streaming::float;
use nom::IResult;
use unicode_width::UnicodeWidthStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'c', long, default_value_t = '■')]
    char: char,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let lines_and_nums: Vec<(String, Option<f32>)> = io::stdin()
        .lines()
        .flatten()
        .map(|line| {
            let res: IResult<&str, f32> = float(line.trim_start());
            let maybe_num = res.ok().map(|(_, num)| num);
            (line, maybe_num)
        })
        .collect();

    let max = lines_and_nums
        .iter()
        .filter_map(|&(_, num)| num)
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0.0);

    let prefix_len = lines_and_nums
        .iter()
        .filter_map(|(str, num)| num.map(|_| str))
        .map(|s| UnicodeWidthStr::width(&s[..]))
        .max()
        .unwrap_or(0);

    let columns = termsize::get().map(|size| size.cols).unwrap_or(80);
    let bar_length = columns as usize - (prefix_len + 1);

    let bar_character = args.char.to_string();

    for (line, num) in lines_and_nums {
        match num {
            Some(num) => {
                let count = (bar_length as f32 * num / max).ceil() as usize;
                writeln!(
                    io::stdout(),
                    "{: <align$} {}",
                    line,
                    bar_character.repeat(count),
                    align = prefix_len,
                )?;
            }
            None => writeln!(io::stdout(), "{}", line)?,
        }
    }

    Ok(())
}
