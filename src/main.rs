#![recursion_limit = "1024"]

extern crate failure; 
#[macro_use] extern crate failure_derive;
extern crate gtk;
extern crate gdk;

use failure::{Error, Fail};

#[allow(dead_code)]
#[derive(Debug, Fail)]
enum LabyrinthError {
    #[fail(display = "Could not get default screen")]
    CouldNotGetDefaultScreen
}

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let _ = writeln!(stderr, "Error: {}", e);
        let mut fail: &Fail = e.cause(); 
        while let Some(cause) = fail.cause() {
            let _ = writeln!(stderr, "Caused by: {}", cause);
            fail = cause;
        }
    }
}

fn run() -> Result<(), Error> {
    initialize_gtk()
}

fn initialize_gtk() -> Result<(), Error> {
    gtk::init()?;
    let _ = create_window()?;
    gtk::main();
    Ok(()) 
}

fn create_window() -> Result<gtk::Window, Error> {
    use gtk::prelude::*;
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.fullscreen();
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        gtk::Inhibit(true)
    });
    window.connect_key_press_event(move |_, key| {
        match key.get_keyval() {
            gdk::enums::key::Escape => gtk::main_quit(),
            _ => ()
        }
        gtk::Inhibit(false)
    });
    window.show_all(); 
    return Ok(window);
}
