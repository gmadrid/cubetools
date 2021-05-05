use crate::Result;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use std::str::FromStr;

/// For each of the nine positions on a cube face, list the legal positions for the sticker.
/// (Face and Empty are always valid.)
///
/// The cube face is numbered like this:
///
///    0 1 2
///    3 4 5
///    6 7 8
///
static CUBE: Lazy<Vec<Vec<Direction>>> = Lazy::new(|| {
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
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Face,
    Empty,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
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

pub fn parse_desc(input: &str) -> Result<Vec<Direction>> {
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
    Ok(dirs)
}
