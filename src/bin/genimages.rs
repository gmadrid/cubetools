use anyhow::{anyhow, Context, Error};
use argh::FromArgs;
use cubetools::ollrender::render as oll_render;
use cubetools::ollspec::{parse_desc, Direction};
use cubetools::pllrender::render as pll_render;
use cubetools::pllspec::{parse_program, Program};
use cubetools::RenderOpts;
use once_cell::unsync::Lazy;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Pulls file names and cube image specs and generates images.
#[derive(Debug, FromArgs)]
struct Args {
    #[argh(positional)]
    input: PathBuf,

    #[argh(
        option,
        short = 'd',
        default = "FromStr::from_str(\"images/\").unwrap()"
    )]
    /// destination path for the images.
    dest_path: PathBuf,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
struct ImageDesc {
    file_stem: String,
    spec: CubeSpec,
}

impl ImageDesc {
    fn new(file_stem: &str, spec_str: &str) -> Result<ImageDesc> {
        Ok(ImageDesc {
            file_stem: file_stem.to_string(),
            spec: CubeSpec::new(spec_str)?,
        })
    }

    fn render(&self) -> Result<String> {
        self.spec.render()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CubeSpec {
    OLL(Vec<Direction>),
    PLL(Program),
}

impl CubeSpec {
    fn new(spec_str: &str) -> Result<CubeSpec> {
        if spec_str.contains('=') {
            Ok(CubeSpec::OLL(parse_desc(spec_str)?))
        } else if spec_str.contains('<') || spec_str.contains('>') {
            Ok(CubeSpec::PLL(parse_program(spec_str)?))
        } else {
            Err(anyhow!("'{}' is not a valid image spec"))
        }
    }

    fn render(&self) -> Result<String> {
        let specs = RenderOpts::with_cubie_size(25);
        let svg = match self {
            CubeSpec::OLL(oll_spec) => oll_render(oll_spec, &specs),
            CubeSpec::PLL(pll_spec) => pll_render(pll_spec, &specs),
        };
        Ok(svg)
    }
}

// Example: '[//]: # (bar  xUx===xDx)'
const IMAGE_DESC_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new("# *\\(([[:alnum:]]+)  (.*)\\)").unwrap());

fn process_line(line: &str) -> Result<Option<ImageDesc>> {
    if let Some(cap) = IMAGE_DESC_RE.captures(line) {
        Ok(Some(ImageDesc::new(&cap[1], &cap[2])?))
    } else {
        Ok(None)
    }
}

fn process_input(reader: impl BufRead) -> Result<Vec<ImageDesc>> {
    let mut descs = vec![];
    for line in reader.lines() {
        if let Some(image_desc) = process_line(&line?)? {
            descs.push(image_desc);
        }
    }

    Ok(descs)
}

fn render_descs(descs: &[ImageDesc], dest_path: &Path) -> Result<()> {
    for desc in descs {
        let full_path = dest_path.join(&desc.file_stem).with_extension("svg");
        let svg = desc.render()?;

        let mut output =
            File::create(&full_path).context(format!("Cannot create '{:?}'", &full_path))?;
        output.write_all(svg.as_bytes())?;
        output.write_all("\n".as_bytes())?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let input_reader = BufReader::new(File::open(args.input)?);
    let descs = process_input(input_reader)?;
    render_descs(&descs, &args.dest_path)?;

    Ok(())
}
