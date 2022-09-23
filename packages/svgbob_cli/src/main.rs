#![deny(warnings)]
#[macro_use]
extern crate clap;

extern crate svgbob;

use svgbob::Settings;

use clap::ArgMatches;
use std::error::Error;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;

fn main() {
    use clap::{App, Arg, SubCommand};

    let args = App::new("svgbob")
        .version(crate_version!())
        .about("SvgBobRus is an ascii to svg converter")
        .arg(Arg::with_name("background")
             .long("background")
             .takes_value(true)
             .help("backdrop background will be filled with this color (default: 'white')"))
        .arg(Arg::with_name("inline").short("s").help("parse an inline string"))
        .arg(Arg::with_name("input").index(1).help("svgbob text file or inline string to parse [default: STDIN]"))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("where to write svg output [default: STDOUT]"))
        .arg(Arg::with_name("fill-color")
             .long("fill-color")
             .takes_value(true)
             .help("solid shapes will be filled with this color (default: 'black')"))
        .arg(Arg::with_name("font-family")
             .long("font-family")
             .takes_value(true)
             .help("text will be rendered with this font (default: 'arial')"))
        .arg(Arg::with_name("font-size")
             .long("font-size")
             .takes_value(true)
             .help("text will be rendered with this font size (default: 14)"))
        .arg(Arg::with_name("stroke-width")
             .long("stroke-width")
             .takes_value(true)
             .help("stroke width for all lines (default: 2)"))
        .arg(Arg::with_name("scale")
             .long("scale")
             .takes_value(true)
             .help("scale the entire svg (dimensions, font size, stroke width) by this factor (default: 1)"))
        .subcommand(SubCommand::with_name("build")
            .about("Batch convert files to svg.")
            .version("0.0.1")
            .arg(Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("set input file pattern like: *.bob or dir/*.bob"))
            .arg(Arg::with_name("outdir")
                .short("o")
                .long("outdir")
                .takes_value(true)
                .help("set dir of svg files")))
        .get_matches();

    if let Some(sub_build) = args.subcommand_matches("build") {
        match build(sub_build) {
            Ok(_) => {}
            Err(e) => println!("{}", e),
        };
        exit(1);
    }

    let mut bob = String::new();

    if args.is_present("inline") {
        bob = args.value_of("input").unwrap().replace("\\n", "\n");
    } else if let Some(file) = args.value_of("input") {
        match File::open(file) {
            Ok(mut f) => {
                f.read_to_string(&mut bob).unwrap();
            }
            Err(e) => {
                use std::io::Write;

                writeln!(
                    &mut std::io::stderr(),
                    "Failed to open input file {}: {}",
                    file,
                    e
                )
                .unwrap();
                exit(1);
            }
        }
    } else {
        use std::io;
        io::stdin().read_to_string(&mut bob).unwrap();
    }

    let mut settings = Settings::default();

    if let Some(background) = args.value_of("background") {
        settings.background = background.to_string();
    }

    if let Some(fill_color) = args.value_of("fill-color") {
        settings.fill_color = fill_color.to_string();
    }

    if let Some(font_family) = args.value_of("font-family") {
        settings.font_family = font_family.to_string();
    }

    if let Some(font_size) = parse_value_of(&args, "font-size") {
        settings.font_size = font_size;
    }

    if let Some(stroke_width) = parse_value_of(&args, "stroke-width") {
        settings.stroke_width = stroke_width;
    }

    let scale: Option<f32> = parse_value_of(&args, "scale");
    if let Some(s) = scale {
        settings.scale *= s;
    }

    let svg = svgbob::to_svg_with_settings(&*bob, &settings);

    if let Some(file) = args.value_of("output") {
        if let Err(e) = fs::write(file, &svg) {
            use std::io::Write;

            writeln!(
                &mut std::io::stderr(),
                "Failed to write to output file {}: {}",
                file,
                e
            )
            .unwrap();
            exit(2);
        }
    } else {
        println!("{}", svg);
    }
}

fn parse_value_of<T: FromStr>(args: &ArgMatches, arg_name: &str) -> Option<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    return args.value_of(arg_name).map(|arg| match arg.parse::<T>() {
        Ok(a) => a,
        Err(e) => {
            use std::io::Write;

            writeln!(
                &mut std::io::stderr(),
                "Illegal value for argument {}: {}",
                arg_name,
                e
            )
            .unwrap();
            exit(1);
        }
    });
}

// Batch convert files to svg
// use svgbob build -i inputdir/*.bob -o outdir/
fn build(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let files_pattern = args.value_of("input").unwrap_or("*.bob");
    let outdir = args.value_of("outdir").unwrap_or("");
    let input_path = Path::new(files_pattern);
    let ext = input_path
        .extension()
        .unwrap_or_else(|| "bob".as_ref())
        .to_str()
        .unwrap();

    let input_dir = if input_path.is_dir() {
        input_path
    } else {
        input_path.parent().unwrap()
    };

    if !input_dir.is_dir() {
        return Err(Box::from(format!(
            "[Error]: No such dir name is {} !",
            input_dir.to_string_lossy()
        )));
    }

    let mut out_path = PathBuf::new();
    if outdir.is_empty() {
        out_path = input_dir.to_path_buf();
    } else {
        out_path.push(outdir)
    }

    if !out_path.is_dir() {
        fs::create_dir_all(out_path.clone())?;
    }

    let paths = fs::read_dir(input_dir).unwrap();
    for path in paths {
        let tmp_path = path.unwrap().path();
        if tmp_path.is_file() {
            let tmp_ext = tmp_path
                .extension()
                .unwrap_or_else(|| "".as_ref())
                .to_str()
                .unwrap();
            if tmp_ext == ext {
                let name = tmp_path.file_stem().unwrap().to_str().unwrap();
                let mut tmp = out_path.clone();
                tmp.push(format!("{}.svg", name));
                println!("{} => {}", tmp_path.display(), tmp.display());
                match convert_file(tmp_path.clone(), tmp) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
        }
    }

    Ok(())
}

fn convert_file(input: PathBuf, output: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut bob = String::new();
    let mut f = File::open(&input)?;
    f.read_to_string(&mut bob).unwrap();
    let svg = svgbob::to_svg_with_settings(&*bob, &Settings::default());
    fs::write(&output, &svg)?;
    Ok(())
}
