use crate::ollspec::Direction;
use crate::rendering::{big_square_size, render_big_square, render_square, row_or_col_start};
use crate::svg::{Path, Tag};
use crate::RenderOpts;

pub fn render(desc: &[Direction], specs: &RenderOpts) -> String {
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

fn render_small_squares(desc: &[Direction], specs: &RenderOpts) -> String {
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

fn render_stickers(desc: &[Direction], specs: &RenderOpts) -> String {
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

fn color_for_direction(dir: Direction) -> &'static str {
    use Direction::*;
    match dir {
        Face => "yellow",
        Empty => "gray",
        _ => "white",
    }
}
