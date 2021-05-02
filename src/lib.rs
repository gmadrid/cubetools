mod path;
mod tags;

pub mod svg {
    pub use crate::path::Path;
    pub use crate::tags::Tag;
}

pub struct Specs {
    pub cubie_size: u32,
    pub border_width: u32,
    pub gutter_size: u32,
    pub sticker_width: u32,
}

impl Specs {
    pub fn with_cubie_size(cubie_size: u32) -> Self {
        Self {
            cubie_size,
            border_width: 2,
            gutter_size: cubie_size / 10,
            sticker_width: cubie_size / 5,
        }
    }
}

pub mod rendering {
    use super::*;
    use svg::{Path, Tag};

    pub fn render_big_square(specs: &Specs) -> String {
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

    pub fn render_square(x: u32, y: u32, width: u32, fill: &str) -> String {
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

    pub fn big_square_size(specs: &Specs) -> u32 {
        specs.border_width * 2 + specs.gutter_size * 4 + specs.cubie_size * 3
    }
}
