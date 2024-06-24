use std::path::{Path, PathBuf};

use clap::Parser;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;

use cega::{cga, palette, sdl, terminal};

#[derive(Parser, Debug)]
#[clap(version = "0.1", author = "Kenzi Connor")]
struct Args {
    #[clap(name = "IMAGE")]
    image: PathBuf,

    #[clap(value_enum, short, long, value_parser = parse_mode_param, default_value="a", help="[possible values: a, c, p]\na = plain ascii\nc = colored ascii\np = full pixels via ansi bg color")]
    terminal_output: terminal::TerminalMode,

    #[clap(value_parser(["0", "0i", "1", "1i"]),num_args(0..=1), short, long)]
    palette: Option<String>,

    #[clap(short, long, value_parser = parse_asci_param, help="4 chars palette like -a \" +%0\"")]
    custom_ascii: Option<String>,

    #[clap(short, long)]
    width: Option<usize>,

    #[clap(short = 'm', long)]
    max_width: Option<usize>,

    #[clap(short = 'r', long)]
    tile_height: Option<usize>,

    #[clap(short, long, default_value_t = false)]
    sdl: bool,

    #[clap(short, long, default_value_t = false)]
    quiet: bool,
}

fn parse_mode_param(arg: &str) -> Result<terminal::TerminalMode, String> {
    match arg {
        "a" => Ok(terminal::TerminalMode::Ascii),
        "c" => Ok(terminal::TerminalMode::ColoredAscii),
        "p" => Ok(terminal::TerminalMode::Pixels),
        _ => Err(format!("possible values: a, c, p")),
    }
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

    let palette = if args.palette.is_some() {
        Some(palette::palette_from_abbr(&args.palette.unwrap()[..]))
    } else {
        None
    };

    let custom_ascii = if args.custom_ascii.is_some() {
        Some(
            args.custom_ascii
                .unwrap()
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    } else {
        None
    };

    //let mut width: usize = args.width.unwrap_or(80);

    let tp = terminal::TerminalPalette::new(args.terminal_output, custom_ascii, palette);
    //let width = args.width.unwrap_or(320);
    let mut image = cga::Image::new(&reader, args.width);
    image.tile(args.width, args.tile_height, args.max_width);
    for (i, index) in image.output.iter().enumerate() {
        if args.width.is_some() && (i % args.width.unwrap() == 0) {
            println!();
        }
        print!("{}", tp.terminal[*index as usize]);
    }

    if !args.quiet {
        println!("\n---------");
        if !image.is_fullscreen() {
            if args.width.is_none() {
                println!("Image appears to not be fullscreen 320*200. It may be tiled, try setting an explicit -w width, which will also quiet this message.");
                println!("Possible widths: {:?}", image.factors());
            } else if image.is_tall() {
                println!("Image appears to 4x as tall as wide or more. If it appears there are tiles, setting the height they appear to -r repeat at (and a max width -m for display wrapping) will make a more compact view");
            }
        }
    }

    if args.sdl {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("viewer", 128, 128)
            //.allow_highdpi()
            .build()
            .expect("could not initialize video subsystem");

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("could not make a canvas");
        canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        canvas.clear();

        // for (i, index) in indices.iter().enumerate() {
        //     let x = i % width;
        //     let y = i / width;
        //     canvas.pixel(
        //         x.try_into().unwrap(),
        //         y.try_into().unwrap(),
        //         crate::sdl::PALETTE1[*index as usize],
        //     )?;
        // }
        // canvas.present();

        let mut event_pump = sdl_context.event_pump()?;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
