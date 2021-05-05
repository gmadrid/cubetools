use cubetools::ollrender::render;
use cubetools::ollspec::parse_desc;
use cubetools::{RenderOpts, Result};

#[derive(argh::FromArgs)]
/// Generate a little cubie diagram
struct Args {
    #[argh(positional)]
    input: String,

    #[argh(option, default = "25", short = 'w')]
    /// width of each cubie
    cubie_size: u32,
}

fn specs_from_args(args: &Args) -> RenderOpts {
    RenderOpts::with_cubie_size(args.cubie_size)
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let specs = specs_from_args(&args);
    let desc = parse_desc(&args.input)?;
    let svg = render(&desc, &specs);

    println!("{}", svg);

    Ok(())
}
