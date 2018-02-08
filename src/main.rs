extern crate failure; 
#[macro_use] extern crate failure_derive;
extern crate gtk;
extern crate gdk;
extern crate cairo;

use failure::{Error, Fail};

#[allow(dead_code)]
#[derive(Debug, Fail)]
enum LabyrinthError {
    #[fail(display = "Could not get default screen")]
    CouldNotGetDefaultScreen
}

struct LabyrinthGame {
    window : gtk::Window,
    drawing_area : gtk::DrawingArea,
    width : u32,
    height : u32,
}

struct RefLabyrinthGame(std::rc::Rc<LabyrinthGame>);

impl RefLabyrinthGame {
    fn new() -> Result<RefLabyrinthGame, Error> {
        gtk::init()?;
        let game = RefLabyrinthGame::initialize_display()?;
        gtk::main();
        Ok(game)
    }
    fn initialize_display() -> Result<RefLabyrinthGame, Error> {
        if let Some(screen) = gdk::Screen::get_default() {
            RefLabyrinthGame::initialize_window(&screen)
        } 
        else {
            Err(LabyrinthError::CouldNotGetDefaultScreen.into())
        }
    } 
    fn initialize_window(screen : & gdk::Screen) -> Result<RefLabyrinthGame, Error> {
        use gtk::prelude::*;
        use gdk::ScreenExt; 
        use std::borrow::BorrowMut;
        let monitor = screen.get_primary_monitor();
        let mut game = RefLabyrinthGame(std::rc::Rc::new(LabyrinthGame { 
            window : gtk::Window::new(gtk::WindowType::Toplevel), 
            drawing_area : gtk::DrawingArea::new(), 
            width: 0, 
            height : 0 
        })); 
        let inner = game.0.clone();
        inner.window.fullscreen_on_monitor(screen, monitor);
        inner.window.connect_delete_event(|_,_| {
            gtk::main_quit();
            gtk::Inhibit(true)
        }); 
        inner.clone().drawing_area.connect_size_allocate(move |_, rect| {
          RefLabyrinthGame::on_size_allocate(inner, rect);
        });
        let geometry = screen.get_monitor_geometry(monitor);  
        game.0.drawing_area.set_size_request(geometry.width, geometry.height);
        game.0.window.add(&game.0.drawing_area);
        game.0.window.connect_key_press_event(|_, key| {
            match key.get_keyval() {
                gdk::enums::key::Escape => gtk::main_quit(),
                _ => ()
            }
            gtk::Inhibit(true)
        });
        game.0.window.show_all(); 
        Ok(game)
    } 
    fn on_size_allocate(mut game : std::rc::Rc<LabyrinthGame>, rect: &gdk::Rectangle) {
        game.width = rect.width as u32;
        game.height = rect.height as u32;
    }
}

fn main() {
    let game = RefLabyrinthGame::new();
    if let Err(ref e) = game {
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
