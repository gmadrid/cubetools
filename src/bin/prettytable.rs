use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Debug, argh::FromArgs)]
/// Prettify the tables in a markdown file.
struct Args {
    #[argh(positional)]
    input_file: PathBuf,
}

fn prettify(input_file: &Path) -> Result<()> {
    let file = BufReader::new(File::open(input_file)?);

    let mut lines = file.lines().peekable();

    loop {
        if let Some(boring_line) = lines.next_if(|item| {
            if let Ok(line) = item {
                !line.starts_with('|')
            } else {
                true
            }
        }) {
            println!("{}", boring_line?);
        } else if lines.peek().is_none() {
            break;
        } else {
            prettify_table(&mut lines)?;
        }
    }
    Ok(())
}

fn prettify_table(
    lines: &mut Peekable<impl Iterator<Item = std::result::Result<String, std::io::Error>>>,
) -> Result<()> {
    let mut table_lines = vec![];

    // Collect the lines of the table into a Vec.
    loop {
        if let Some(line) = lines.next_if(|item| {
            if let Ok(line) = item {
                line.starts_with('|')
            } else {
                false
            }
        }) {
            table_lines.push(line?);
        } else {
            break;
        }
    }

    let segment_lists = table_lines
        .iter()
        .map(|line| line.split('|').map(|s| s.trim()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // unwrap: there will always be at least one element.
    let max_segments = segment_lists.iter().map(|lst| lst.len()).max().unwrap();
    let segment_lengths = vec![0; max_segments];

    let max_segment_lengths = segment_lists
        .iter()
        .fold(segment_lengths, |maxes, this_list| {
            copy_maxes(&maxes, this_list)
        });

    //dbg!(max_segment_lengths);
    for segment_list in segment_lists {
        let s = segment_list
            .iter()
            .zip(max_segment_lengths.iter())
            .map(|(segment, desired_len)| {
                if *desired_len == 0 {
                    segment.to_string()
                } else {
                    let mut str = " ".to_string();
                    str.push_str(segment);
                    // Subtract one to account for the space we just added at the beginning.
                    while str.len() - 1 < *desired_len {
                        str.push(' ');
                    }
                    str.push(' ');
                    str
                }
            })
            .collect::<Vec<_>>()
            .join("|");
        println!("{}", s);
        //        dbg!(segment_list);
    }

    Ok(())
}

fn copy_maxes(maxes: &Vec<usize>, segments: &Vec<&str>) -> Vec<usize> {
    maxes
        .iter()
        .zip(segments.iter())
        .map(|(max_len, segment)| std::cmp::max(*max_len, segment.trim().len()))
        .collect()
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    prettify(&args.input_file)?;

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn tes_fo() {
        let s = "| --- | --- | --- |";
        let ss = s.split("|").collect::<Vec<_>>();

        assert!(dbg!(ss).is_empty());
    }
}
