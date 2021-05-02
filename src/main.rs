/*
    Input a string like: U U R L F F D D F

    And get a diagram.
*/

use anyhow::anyhow;
use once_cell::sync::Lazy;
use std::str::FromStr;

const BORDER_WIDTH: u32 = 2;
const CELL_SIZE: u32 = 50;
const GUTTER_SIZE: u32 = 5;
const STICKER_WIDTH: u32 = 10;

#[derive(argh::FromArgs)]
/// Generate a little cubie diagram
struct Args {
    #[argh(positional)]
    input: String,

    #[argh(option, default = "50", short = 'w')]
    /// width of each cubie
    cubie_size: u32,
}

struct Specs {
    cubie_size: u32,
    border_width: u32,
    gutter_size: u32,
    sticker_width: u32,
}

static CUBE: Lazy<Vec<Cubie>> = Lazy::new(|| {
    use Direction::*;
    vec![
        vec![Left, Up],
        vec![Up],
        vec![Right, Up],
        vec![Left],
        vec![],
        vec![Right],
        vec![Left, Down],
        vec![Down],
        vec![Right, Down],
    ]
});

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Face,
    Empty,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;

        match s.chars().next() {
            None => Err(anyhow!("Cannot parse Direction from empty string")),
            Some(ch) => match ch {
                'U' => Ok(Up),
                'D' => Ok(Down),
                'L' => Ok(Left),
                'R' => Ok(Right),
                '=' | 'F' => Ok(Face),
                '.' | 'E' | 'X' | 'x' => Ok(Empty),
                _ => Err(anyhow!(format!("Unknown char, '{}', for Direction", ch))),
            },
        }
    }
}

type Cubie = Vec<Direction>;

fn parse_desc(input: &str) -> Vec<Direction> {
    use Direction::*;
    let mut dirs = vec![];
    let mut idx = 0;
    for ch in input.chars() {
        if ch.is_whitespace() {
            continue;
        }

        let direction = ch.to_string().parse::<Direction>().unwrap();

        let cubie = CUBE.get(idx).unwrap();
        match direction {
            Face => dirs.push(Face),
            Empty => dirs.push(Empty),
            _ => {
                if cubie.contains(&direction) {
                    dirs.push(direction);
                } else {
                    panic!("XXXYYY");
                }
            }
        }
        idx += 1;
    }
    dirs
}

fn render_square(x: u32, y: u32, width: u32, fill: &str) -> String {
    format!(
        "<path fill=\"{}\" border-width=\"0\" d=\"M {},{} h {} v {} h -{} v -{}\" />\n",
        fill, x, y, width, width, width, width
    )
}

fn render_rect(x: u32, y: u32, width: u32, height: u32, fill: &str) -> String {
    format!(
        "<path fill=\"{}\" stroke=\"black\" stroke-width=\"2\" d=\"M {},{} h {} v {} h -{} v -{}\" />\n",
        fill, x, y, width, height, width, height
    )
}

fn big_square_size() -> u32 {
    BORDER_WIDTH * 2 + GUTTER_SIZE * 4 + CELL_SIZE * 3
}

fn render_big_square() -> String {
    /*
      border * 2 + gutter * 4 + cell * 3
    */
    let big_square_size = big_square_size();
    render_square(
        STICKER_WIDTH + GUTTER_SIZE,
        STICKER_WIDTH + GUTTER_SIZE,
        big_square_size,
        "black",
    )
}

fn color_for_direction(dir: Direction) -> &'static str {
    use Direction::*;
    match dir {
        Face => "yellow",
        Empty => "gray",
        _ => "white",
    }
}

fn row_or_col_start(idx: u32) -> u32 {
    STICKER_WIDTH + GUTTER_SIZE * (2 + idx) + BORDER_WIDTH + CELL_SIZE * idx
}

fn render_small_squares(desc: &[Direction]) -> String {
    let mut result = String::default();
    for idx in 0..9 {
        let row = idx / 3;
        let col = idx % 3;

        let x = row_or_col_start(col);
        let y = row_or_col_start(row);

        let color = color_for_direction(desc[idx as usize]);
        result.push_str(&render_square(x, y, CELL_SIZE, color))
    }
    result
}

fn render_stickers(desc: &[Direction]) -> String {
    let mut result = String::default();

    for (idx, dir) in desc.iter().enumerate() {
        match *dir {
            Direction::Up => {
                let x = row_or_col_start(idx as u32 % 3);
                result.push_str(&render_rect(x, 0, CELL_SIZE, STICKER_WIDTH, "yellow"));
            }
            Direction::Left => {
                let y = row_or_col_start(idx as u32 / 3);
                result.push_str(&render_rect(0, y, STICKER_WIDTH, CELL_SIZE, "yellow"));
            }
            Direction::Right => {
                let x = big_square_size() + STICKER_WIDTH + GUTTER_SIZE * 2;
                let y = row_or_col_start(idx as u32 / 3);
                result.push_str(&render_rect(x, y, STICKER_WIDTH, CELL_SIZE, "yellow"));
            }
            Direction::Down => {
                let x = row_or_col_start(idx as u32 / 3);
                let y = big_square_size() + STICKER_WIDTH + GUTTER_SIZE * 2;
                result.push_str(&render_rect(x, y, CELL_SIZE, STICKER_WIDTH, "yellow"));
            }
            _ => {}
        }
    }

    result
}

fn render(desc: &[Direction]) -> String {
    let mut svg = String::new();
    //    svg.push_str("<?xml version=\"1.0\"?>");
    svg.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\">");

    svg.push_str(&render_big_square());
    svg.push_str(&render_small_squares(desc));
    svg.push_str(&render_stickers(desc));

    svg.push_str("</svg>");
    svg
}

fn main() {
    let args: Args = argh::from_env();
    let desc = parse_desc(&args.input);
    let svg = render(&desc);
    println!("{}", svg);
}