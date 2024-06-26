#[cfg(feature = "terminal")]
use {
    cega::color::palette::palette_from_abbr,
    cega::image,
    cega::image::Image,
    cega::parser::ParserType,
    cega::terminal,
    cega::terminal::*,
    cega::ImageType,
    std::fs,
    std::path::{Path, PathBuf},
};

#[cfg(feature = "clap")]
use clap::Parser;

#[cfg(feature = "png")]
use cega::png;

#[cfg(feature = "sdl2")]
use cega::sdl::render_sdl;

#[cfg(feature = "clap")]
#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Kenzi Connor")]
struct Args {
    #[clap(name = "IMAGE")]
    image: PathBuf,

    #[clap(value_enum, short, long, value_parser = TerminalMode::from_short, help="images will horizontally crop to terminal\n[possible values: a, c, p, h]\na = plain ascii\nc = colored ascii\np = full pixels via ansi bg color\nh = horizontal half pixels (UGLY)")]
    ascii_mode: Option<TerminalMode>,

    #[clap(value_parser(["cga0", "cga0i", "cga1", "cga1i", "ega"]),num_args(0..=1), short, long, help="ega palette can be used for cga, but not the inverse\n")]
    palette: Option<String>,

    #[clap(short, long, value_parser(["ega_row_planar", "erp", "cga", "png"]), default_value="cga")]
    image_parser: String,

    #[clap(short, long, value_parser = parse_asci_param, help="4 or 16 chars palette like -a \" +%0\"")]
    custom_ascii: Option<String>,

    #[clap(short, long, default_value_t = 320)]
    width: usize,

    #[clap(
        short,
        long,
        help = "used for wrapping rows if retiling with tile_height"
    )]
    max_width: Option<usize>,

    #[clap(short, long, help = "format based on extension - see image crate")]
    output_file: Option<PathBuf>,

    #[clap(short, long)]
    ga_file: Option<PathBuf>,

    #[clap(short, long)]
    tile_height: Option<usize>,

    #[clap(short, long, default_value_t = false)]
    sdl: bool,

    #[clap(short, long, default_value_t = false)]
    quiet: bool,
}

#[cfg(feature = "clap")]
fn parse_asci_param(arg: &str) -> Result<String, String> {
    if let 0 | 4 | 16 = arg.len() {
        Ok(arg.to_string())
    } else {
        Err("requires a 4 or 16 character string like: -a \" +%0\"".to_string())
    }
}

#[cfg(feature = "terminal")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let reader = std::fs::read(Path::new(&args.image))?;
    let parser = ParserType::type_str(&args.image_parser);
    let image = Image::new(&reader, args.width, parser);

    let image_data = if args.tile_height.is_some() {
        image::tile(image.data(), args.tile_height.unwrap(), args.max_width)
    } else {
        image.data()
    };

    //dbg!(image_data.clone());

    let palette_string = if let ImageType::EGA = parser.image_type() {
        "ega".to_owned()
    } else {
        args.palette.unwrap_or("cga1".to_owned())
    };

    let palette = palette_from_abbr(&palette_string);

    #[cfg(feature = "png")]
    if let Some(output) = args.output_file {
        png::output(output, image_data.clone(), palette.clone())?
    }

    if let Some(ga_file) = args.ga_file {
        let bytes = parser.to_bytes(image_data.clone());
        fs::write(ga_file, bytes).unwrap();
    }

    if args.ascii_mode.is_some() {
        let ascii = if args.custom_ascii.is_some() {
            args.custom_ascii.unwrap().chars().collect()
        } else {
            default_char_palette(parser.image_type())
        };

        let tp = TerminalPalette::new(args.ascii_mode.unwrap(), ascii, palette.clone());
        print!(
            "{}",
            terminal::disable_wrapping(terminal::to_string(tp.apply(image_data.clone())))
        );
    }

    if !args.quiet && !image.is_fullscreen() {
        if args.width == 320 {
            println!("\nImage appears to not be fullscreen 320*200. It may be tiled, try setting a narrower -w width to detect tiles.");
            println!("Possible widths: {:?}", image.width_factors());
        } else if args.tile_height.is_none() && image.is_tall() {
            println!("\nImage height appears to >= 4x its width. If there are tiles, setting -t tile_height will make a more compact view");
            println!("Possible heights: {:?}", image.height_factors());
        }
    }

    if args.sdl {
        #[cfg(feature = "sdl2")]
        render_sdl(image_data, palette)?
    }
    Ok(())
}

#[cfg(not(feature = "terminal"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
