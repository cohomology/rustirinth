extern crate cairo;
#[macro_use]
extern crate clap;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate gdk;
extern crate gtk;
#[macro_use]
extern crate lazy_static;
extern crate ndarray;
extern crate conv;

mod main_window;
mod game;
mod event_handler;
mod labyrinth;
mod basic_types;

fn run() -> Result<(), failure::Error> {
    let default_box_size = "64";
    let args = clap::App::new("Rustirinth")
        .about("A simple labyrinth game")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            clap::Arg::with_name("box-size")
                .long("box-size")
                .short("s")
                .default_value(default_box_size)
                .help("The size of the boxes on the screen")
                .possible_values(&["16", "32", "64", "128"]),
        )
        .get_matches();
    let box_size = args.value_of("box-size")
        .unwrap_or(default_box_size)
        .parse::<u32>()?;
    game::LabyrinthGame::run(box_size)
}

fn main() {
    if let Err(ref e) = run() {
        game::LabyrinthGame::fatal_error(e);
    }
}
