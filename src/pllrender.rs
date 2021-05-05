use crate::pllspec::{Operator, Program, Statement};
use crate::rendering::{big_square_size, render_big_square, render_square, row_or_col_start};
use crate::svg::{Path, Tag};
use crate::RenderOpts;

pub fn render(program: &Program, specs: &RenderOpts) -> String {
    let mut svg = String::default();

    let size = big_square_size(specs) + specs.gutter_size * 2 + specs.sticker_width * 2;
    let tag = Tag::new("svg")
        .attr("xmlns", "http://www.w3.org/2000/svg")
        .attr("height", &size.to_string())
        .attr("width", &size.to_string());

    svg.push_str(&tag.open());

    svg.push_str(&render_defs());
    svg.push_str(&render_big_square(&specs));
    svg.push_str(&render_small_squares(&specs));

    for stmt in &program.statements.statements {
        svg.push_str(&render_stmt(stmt, specs));
    }

    svg.push_str(&tag.close());

    svg
}

fn render_defs() -> String {
    let mut output = String::default();

    let defs = Tag::new("defs");
    let marker = Tag::new("marker")
        .attr("id", "arrow")
        .attr("viewBox", "0 0 10 10")
        .attr("refX", "5")
        .attr("refY", "5")
        .attr("markerWidth", "3")
        .attr("markerHeight", "3")
        .attr("orient", "auto-start-reverse");

    let path = Path::new().M(0, 0).L(10, 5).L(0, 10).z();
    let path_tag = Tag::new("path")
        .attr("d", path.output())
        .attr("fill", "red");

    output.push_str(&defs.open());
    output.push_str(&marker.open());
    output.push_str(&path_tag.open());
    output.push_str(&path_tag.close());
    output.push_str(&marker.close());
    output.push_str(&defs.close());

    output
}

fn render_stmt(stmt: &Statement, specs: &RenderOpts) -> String {
    let start_row = stmt.start.idx / 3;
    let start_col = stmt.start.idx % 3;
    let end_row = stmt.end.idx / 3;
    let end_col = stmt.end.idx % 3;

    let start_x = row_or_col_start(start_col as u32, specs) + specs.cubie_size / 2;
    let start_y = row_or_col_start(start_row as u32, specs) + specs.cubie_size / 2;
    let end_x = row_or_col_start(end_col as u32, specs) + specs.cubie_size / 2;
    let end_y = row_or_col_start(end_row as u32, specs) + specs.cubie_size / 2;

    let tag = Tag::new("line")
        .attr("x1", &start_x.to_string())
        .attr("y1", &start_y.to_string())
        .attr("x2", &end_x.to_string())
        .attr("y2", &end_y.to_string())
        .attr("stroke-width", "4")
        .attr("stroke", "red");

    let tag = match stmt.op {
        Operator::StartHead => tag.attr("marker-start", "url(#arrow)"),
        Operator::EndHead => tag.attr("marker-end", "url(#arrow)"),
        Operator::BothHead => tag
            .attr("marker-start", "url(#arrow)")
            .attr("marker-end", "url(#arrow)"),
    };

    let mut output = String::default();

    output.push_str(&tag.open());
    output.push_str(&tag.close());

    output
}

fn render_small_squares(specs: &RenderOpts) -> String {
    let mut result = String::default();
    for idx in 0..9 {
        let row = idx / 3;
        let col = idx % 3;

        let x = row_or_col_start(col, specs);
        let y = row_or_col_start(row, specs);

        result.push_str(&render_square(x, y, specs.cubie_size, "yellow"))
    }
    result
}
