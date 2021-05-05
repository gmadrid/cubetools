use cubetools::pllrender::render;
use cubetools::pllspec::parse_program;
use cubetools::RenderOpts;

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

fn specs_from_args(args: &Args) -> RenderOpts {
    RenderOpts::with_cubie_size(args.cubie_size)
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let specs = specs_from_args(&args);

    let program = parse_program(&args.input)?;

    let svg = render(&program, &specs);

    println!("{}", svg);

    Ok(())
}
