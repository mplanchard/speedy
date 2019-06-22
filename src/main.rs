extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};
use warp;


fn cli<'a>() -> ArgMatches<'a> {
    App::new("speedy")
        .subcommand(SubCommand::with_name("add-post"))
        .subcommand(SubCommand::with_name("run"))
        .get_matches()
}

fn run() {
    let index = warp::fs::dir("static");
    warp::serve(index).run(([127, 0, 0, 1], 5000));
}

fn main() {
    let opts = cli();
    match opts.subcommand_name() {
        Some("run") => run(),
        Some("add-post") => println!("add-post"),
        Some(_) => println!("??"),
        None => run(),
    }
}
