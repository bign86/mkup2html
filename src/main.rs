use std::fs;
//use std::path::Path;
//use std::fs::File;
extern crate clap;
use clap::{Arg, App, ArgMatches};
extern crate markdown;


fn cli_args<'a>() -> ArgMatches<'a> {
    App::new("MyApp")
        .version("0.1")
        .author("AN")
        .about("Markup parser")
        .arg(
            Arg::with_name("input")
                .help("the input file to use")
                .index(1)
                .required(true),
        )
        .get_matches()
}

fn main() {
    let matches = cli_args();
    let input = matches.value_of("input").unwrap();
    
    // let in_name = Path::new(&input);
    // let mkup = File::open(&in_name).expect("Cannot open file");
    let mkup = fs::read_to_string(input).expect("Cannot open file");

    let html: String = markdown::to_html(&mkup);
    println!("{}", html)

}

