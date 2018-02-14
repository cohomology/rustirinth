extern crate cairo;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate gdk;
extern crate gtk;

mod main_window;
mod game; 
mod state;

fn main() {
    let result = game::LabyrinthGame::run();
    if let Err(ref e) = result {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let _ = writeln!(stderr, "Error: {}", e);
        let mut fail: &failure::Fail = e.cause();
        while let Some(cause) = fail.cause() {
            let _ = writeln!(stderr, "Caused by: {}", cause);
            fail = cause;
        }
    }
}
