#![feature(proc_macro_hygiene, decl_macro)]

extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use clap::{Arg, ArgMatches, App, SubCommand};
use diesel_migrations::{RunMigrationsError};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use rocket_contrib::serve::StaticFiles;


embed_migrations!("./migrations");


fn cli<'a>() -> ArgMatches<'a> {
    App::new("speedy")
        .subcommand(SubCommand::with_name("add-post"))
        .subcommand(SubCommand::with_name("run"))
        .get_matches()
}


fn run() {
    let conn = create_connection();
    match conn {
        Ok(_) => {}
        Err(e) => {println!("{:?}", e)}
    }
    rocket::ignite()
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static"))
        ).launch();
}


fn create_connection() -> Result<SqliteConnection, RunMigrationsError> {
    let connection = SqliteConnection::establish(":memory:").unwrap();
    embedded_migrations::run(&connection)?;
    Ok(connection)
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


fn main() {
    let opts = cli();
    match opts.subcommand_name() {
        Some("run") => { run() }
        Some("add-post") => { println!("add-post") }
        Some(_) => { println!("??") }
        None => { run() }
    }
}
