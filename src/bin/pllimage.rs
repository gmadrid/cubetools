use anyhow::anyhow;
use cubetools::rendering::{big_square_size, render_big_square, render_square};
use cubetools::svg::{Path, Tag};
use cubetools::Specs;

type Result<T> = std::result::Result<T, anyhow::Error>;

/// Output an image representing a PLL.
#[derive(argh::FromArgs)]
struct Args {
    #[argh(positional)]
    input: String,

    #[argh(option, default = "25", short = 'w')]
    /// width of each cubie
    cubie_size: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Arrow {
    start_cubie: u8,
    end_cubie: u8,
    start_head: bool,
    end_head: bool,
}

/*
    Simple grammar:

    image => statements
    statements => statement statements
               =>
    statement => cubie op cubie
    cubie => [1-9]
    op => '<'
       => '>'
       => '<>'
*/

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Program {
    statements: Statements,
}

fn parse_program(input: &str) -> Result<Program> {
    let (statements, _) = parse_statements(input)?;

    Ok(Program { statements })
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Statements {
    statements: Vec<Statement>,
}

fn parse_statements(input: &str) -> Result<(Statements, &str)> {
    let mut statements = vec![];

    let mut tail = input;
    while !tail.is_empty() {
        let (statement, tail_) = parse_statement(tail)?;
        tail = tail_;
        statements.push(statement);
    }

    Ok((Statements { statements }, input))
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Statement {
    start: Cubie,
    end: Cubie,
    op: Operator,
}

fn parse_statement(input: &str) -> Result<(Statement, &str)> {
    let input = input.trim();
    let (start, input) = parse_cubie(input)?;
    let input = input.trim();
    let (op, input) = parse_operator(input)?;
    let input = input.trim();
    let (end, input) = parse_cubie(input)?;

    Ok((Statement { start, op, end }, input))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    StartHead,
    EndHead,
    BothHead,
}

fn parse_operator(input: &str) -> Result<(Operator, &str)> {
    let input = input.trim();
    if input.starts_with("<>") {
        Ok((Operator::BothHead, &input[2..]))
    } else if input.starts_with("<") {
        Ok((Operator::StartHead, &input[1..]))
    } else if input.starts_with(">") {
        Ok((Operator::EndHead, &input[1..]))
    } else {
        Err(anyhow!(
            "Unexpected string, {}, found while parsing Operator",
            &input[..1]
        ))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cubie {
    idx: u8, // 0-indexed, 0-8.
}

fn parse_cubie(input: &str) -> Result<(Cubie, &str)> {
    let input = input.trim();
    let cubie_index = input[0..1].parse::<u8>()?;

    if cubie_index < 1 || cubie_index > 9 {
        return Err(anyhow!(
            "Out-of-range value, {}, found while parsing Cubie",
            cubie_index
        ));
    }

    Ok((
        Cubie {
            idx: cubie_index - 1,
        },
        &input[1..],
    ))
}

fn row_or_col_start(idx: u32, specs: &Specs) -> u32 {
    specs.sticker_width
        + specs.gutter_size * (2 + idx)
        + specs.border_width
        + specs.cubie_size * idx
}

fn render_small_squares(specs: &Specs) -> String {
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

fn render_stmt(stmt: &Statement, specs: &Specs) -> String {
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

fn render(program: Program, specs: &Specs) -> String {
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

fn specs_from_args(args: &Args) -> Specs {
    Specs::with_cubie_size(args.cubie_size)
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let specs = specs_from_args(&args);

    let program = parse_program(&args.input)?;

    let svg = render(program, &specs);

    println!("{}", svg);

    Ok(())
}
