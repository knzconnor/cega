use std::path::{Path, PathBuf};

use clap::Parser;

use cega::color::palette::palette_from_abbr;
use cega::image::Image;
use cega::sdl::render_sdl;
use cega::terminal::{TerminalMode, TerminalPalette};

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Kenzi Connor")]
struct Args {
    #[clap(name = "IMAGE")]
    image: PathBuf,

    #[clap(value_enum, short, long, value_parser = TerminalMode::from_short, help="[possible values: a, c, p, h]\na = plain ascii\nc = colored ascii\np = full pixels via ansi bg color\nh = horizontal half pixels (UGLY)\nImages may be wider than terminal and will then crop")]
    terminal_output: Option<TerminalMode>,

    #[clap(value_parser(["cga0", "cga0i", "cga1", "cga1i", "ega"]),num_args(0..=1), short, long, default_value="ega", help="ega palette can be used for cga, but not the inverse\n")]
    palette: String,

    #[clap(short, long, value_parser(["ega_row_planar", "erp", "cga"]), default_value="cga")]
    image_parser: String,

    #[clap(short, long, value_parser = parse_asci_param, help="4 chars palette like -a \" +%0\"")]
    custom_ascii: Option<String>,

    #[clap(short, long)]
    width: Option<usize>,

    #[clap(short = 'm', long, default_value_t = 320)]
    max_width: usize,

    #[clap(short, long)]
    retile_height: Option<usize>,

    #[clap(short, long, default_value_t = false)]
    sdl: bool,

    #[clap(short, long, default_value_t = false)]
    quiet: bool,
}

fn parse_asci_param(arg: &str) -> Result<String, String> {
    if let 0 | 4 = arg.len() {
        Ok(arg.to_string())
    } else {
        Err(format!("requires a 4 character string like: -a \" +%0\""))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let reader = std::fs::read(&Path::new(&args.image))?;
    let palette = palette_from_abbr(&args.palette);

    let mut image = Image::new(&reader, args.width, palette, &args.image_parser);

    if args.width.is_some() {
        image.retile(args.width.unwrap(), args.retile_height, args.max_width);
    }

    if args.terminal_output.is_some() {
        print!(
            "{}",
            TerminalPalette::new(
                args.terminal_output.unwrap(),
                args.custom_ascii.as_deref(),
                image.palette.clone()
            )
            .output_image_string(&image)
        );
    }

    if !args.quiet {
        println!("\n---------");
        if !image.is_fullscreen() {
            if args.width.is_none() {
                println!("Image appears to not be fullscreen 320*200. It may be tiled, try setting an explicit -w width, which will also quiet this message.");
                println!("Possible widths: {:?}", image.width_factors());
            } else if image.is_tall() {
                println!("Image height appears to >= 4x its width. If there are tiles, setting -r (re)tile_height will make a more compact view");
                println!("Possible heights: {:?}", image.height_factors());
            }
        }
    }

    if args.sdl {
        render_sdl(image)?
    }
    Ok(())
}
