#[macro_use]
extern crate clap;

extern crate svgbob;
extern crate svg;

use svgbob::Grid;
use svgbob::Settings;

fn main() {
    use clap::{Arg, App};
    use std::io::Read;

    let args = App::new("svgbob")
        .version(crate_version!())
        .about("SvgBobRus is an ascii to svg converter")
        .arg(Arg::with_name("input").index(1).help("svgbob text file to parse [default: STDIN]"))
        .arg(Arg::with_name("output")
             .short("o").long("output")
             .takes_value(true)
             .help("where to write svg output [default: STDOUT]"))
        .get_matches();

    let mut bob = String::new();
    if let Some(file) = args.value_of("input") {
        use std::fs::File;
        match File::open(file) {
            Ok(mut f) => {
                f.read_to_string(&mut bob).unwrap();
            },
            Err(e) => {
                use std::io::Write;
                use std::process::exit;

                writeln!(&mut std::io::stderr(), "Failed to open input file {}: {}", file, e).unwrap();
                exit(1);
            }
        }
    } else {
        use std::io;
        io::stdin().read_to_string(&mut bob).unwrap();
    }

    let g = Grid::from_str(&*bob);
    let svg = g.get_svg(&Settings::compact());

    if let Some(file) = args.value_of("output") {
        if let Err(e) = svg::save(file, &svg) {
            use std::io::Write;
            use std::process::exit;

            writeln!(&mut std::io::stderr(), "Failed to write to output file {}: {}", file, e).unwrap();
            exit(2);
        }
    } else {
        println!("{}", svg);
    }
}
