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
    let _ = initialize_display()?;
    gtk::main();
    Ok(()) 
}

fn initialize_display() -> Result<(), Error> {
    if let Some(screen) = gdk::Screen::get_default() {
        initialize_window(&screen)
    } 
    else {
        Err(LabyrinthError::CouldNotGetDefaultScreen.into())
    }
}

fn initialize_window(screen : & gdk::Screen) -> Result<(), Error> {
    use gtk::prelude::*;
    use gdk::ScreenExt; 
    let monitor = screen.get_primary_monitor();
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.fullscreen_on_monitor(screen, monitor);
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        gtk::Inhibit(true)
    });
    let drawing_area = gtk::DrawingArea::new();
    let geometry = screen.get_monitor_geometry(monitor);
    drawing_area.set_size_request(geometry.width, geometry.height);
    window.add(&drawing_area);
    window.connect_key_press_event(|_, key| {
        match key.get_keyval() {
            gdk::enums::key::Escape => gtk::main_quit(),
            _ => ()
        }
        gtk::Inhibit(true)
    });
    window.show_all(); 
    Ok(())
}
