use anyhow::anyhow;
use once_cell::sync::Lazy;
use path::Path;
use std::str::FromStr;
use tags::Tag;

mod path;
mod tags;

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

impl Specs {
    fn with_cubie_size(cubie_size: u32) -> Self {
        Self {
            cubie_size,
            border_width: 2,
            gutter_size: cubie_size / 10,
            sticker_width: cubie_size / 5,
        }
    }
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
    let path = Path::new()
        .M(x as i32, y as i32)
        .h(width as i32)
        .v(width as i32)
        .h(-(width as i32))
        .v(-(width as i32));

    let tag = Tag::new("path")
        .attr("fill", fill)
        .attr("border-width", "0")
        .attr("d", path.output());

    let mut str = tag.open();
    str.push_str(&tag.close());

    str
}

fn render_rect(x: u32, y: u32, width: u32, height: u32, fill: &str) -> String {
    let path = Path::new()
        .M(x as i32, y as i32)
        .h(width as i32)
        .v(height as i32)
        .h(-(width as i32))
        .v(-(height as i32));

    let tag = Tag::new("path")
        .attr("fill", fill)
        .attr("stroke", "black")
        .attr("stroke-width", &2.to_string())
        .attr("d", path.output());

    let mut str = tag.open();
    str.push_str(&tag.close());

    str
}

fn big_square_size(specs: &Specs) -> u32 {
    specs.border_width * 2 + specs.gutter_size * 4 + specs.cubie_size * 3
}

fn render_big_square(specs: &Specs) -> String {
    /*
      border * 2 + gutter * 4 + cell * 3
    */
    let big_square_size = big_square_size(specs);
    render_square(
        specs.sticker_width + specs.gutter_size,
        specs.sticker_width + specs.gutter_size,
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

fn row_or_col_start(idx: u32, specs: &Specs) -> u32 {
    specs.sticker_width
        + specs.gutter_size * (2 + idx)
        + specs.border_width
        + specs.cubie_size * idx
}

fn render_small_squares(desc: &[Direction], specs: &Specs) -> String {
    let mut result = String::default();
    for idx in 0..9 {
        let row = idx / 3;
        let col = idx % 3;

        let x = row_or_col_start(col, specs);
        let y = row_or_col_start(row, specs);

        let color = color_for_direction(desc[idx as usize]);
        result.push_str(&render_square(x, y, specs.cubie_size, color))
    }
    result
}

fn render_stickers(desc: &[Direction], specs: &Specs) -> String {
    let mut result = String::default();

    for (idx, dir) in desc.iter().enumerate() {
        match *dir {
            Direction::Up => {
                let x = row_or_col_start(idx as u32 % 3, specs);
                result.push_str(&render_rect(
                    x,
                    0,
                    specs.cubie_size,
                    specs.sticker_width,
                    "yellow",
                ));
            }
            Direction::Left => {
                let y = row_or_col_start(idx as u32 / 3, specs);
                result.push_str(&render_rect(
                    0,
                    y,
                    specs.sticker_width,
                    specs.cubie_size,
                    "yellow",
                ));
            }
            Direction::Right => {
                let x = big_square_size(specs) + specs.sticker_width + specs.gutter_size * 2;
                let y = row_or_col_start(idx as u32 / 3, specs);
                result.push_str(&render_rect(
                    x,
                    y,
                    specs.sticker_width,
                    specs.cubie_size,
                    "yellow",
                ));
            }
            Direction::Down => {
                let x = row_or_col_start(idx as u32 % 3, specs);
                let y = big_square_size(specs) + specs.sticker_width + specs.gutter_size * 2;
                result.push_str(&render_rect(
                    x,
                    y,
                    specs.cubie_size,
                    specs.sticker_width,
                    "yellow",
                ));
            }
            _ => {}
        }
    }

    result
}

fn render(desc: &[Direction], specs: &Specs) -> String {
    let mut svg = String::new();

    let size = big_square_size(specs) + specs.gutter_size * 2 + specs.sticker_width * 2;

    let tag = Tag::new("svg")
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr("height", &size.to_string())
        .attr("width", &size.to_string());

    svg.push_str(&tag.open());

    svg.push_str(&render_big_square(&specs));
    svg.push_str(&render_small_squares(desc, &specs));
    svg.push_str(&render_stickers(desc, &specs));

    svg.push_str(&tag.close());

    svg
}

fn specs_from_args(args: &Args) -> Specs {
    Specs::with_cubie_size(args.cubie_size)
}

fn main() {
    let args: Args = argh::from_env();
    let specs = specs_from_args(&args);
    let desc = parse_desc(&args.input);
    let svg = render(&desc, &specs);
    println!("{}", svg);
}
