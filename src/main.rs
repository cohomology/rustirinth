extern crate failure; 
#[macro_use] extern crate failure_derive;
extern crate gtk;
extern crate gdk;
extern crate cairo;

#[derive(Debug, Fail)]
enum LabyrinthError {
    #[fail(display = "Could not get default screen")]
    CouldNotGetDefaultScreen
}

struct LabyrinthMainWindow {
    window : gtk::Window,
    drawing_area : gtk::DrawingArea,
    state : std::cell::RefCell<LabyrinthState>
}

impl LabyrinthMainWindow {
    fn new(screen : &gdk::Screen) -> LabyrinthMainWindow { 
        use gtk::prelude::*;
        use gdk::ScreenExt; 
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        let monitor = screen.get_primary_monitor(); 
        let monitor_workarea = screen.get_monitor_workarea(monitor);
        window.fullscreen_on_monitor(screen, monitor);    
        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_size_request(monitor_workarea.width, monitor_workarea.height); 
        window.add(&drawing_area);  
        window.show_all();   
        LabyrinthMainWindow {
            window : window, 
            drawing_area : drawing_area, 
            state : std::cell::RefCell::new( LabyrinthState {
                width : monitor_workarea.width,
                height : monitor_workarea.height 
            })
        }  
    } 
}

struct LabyrinthGame {
    main_window : std::rc::Rc<LabyrinthMainWindow>
}

impl LabyrinthGame {
    fn new() -> Result<LabyrinthGame, failure::Error> {
        gtk::init()?;
        let game = LabyrinthGame::initialize_screen()?;
        gtk::main();
        Ok(game)              
    }
    fn initialize_screen() -> Result<LabyrinthGame, failure::Error> {
        match gdk::Screen::get_default() {
            Some(screen) => Ok(LabyrinthGame::initialize_window(&screen)),
            None => Err(LabyrinthError::CouldNotGetDefaultScreen.into()) 
        }
    } 
    fn initialize_window(screen : &gdk::Screen) -> LabyrinthGame {
        let game = LabyrinthGame {
            main_window : std::rc::Rc::new(LabyrinthMainWindow::new(&screen))
        };
        game.connect_delete_event();
        game.connect_key_press_event();
        game.connect_on_size_allocate_event();
        game
    }
    fn connect_delete_event(&self) {
        use gtk::WidgetExt;
        self.main_window.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(true)
        });
    }
    fn connect_key_press_event(&self) {
        use gtk::WidgetExt;
        self.main_window.window.connect_key_press_event(|_, key| {
            match key.get_keyval() {
                gdk::enums::key::Escape => gtk::main_quit(),
                _ => ()
            }
            gtk::Inhibit(true)
        });  
    }
    fn connect_on_size_allocate_event(&self) { 
        use gtk::WidgetExt;
        let window_instance = self.main_window.clone();
        self.main_window.drawing_area.connect_size_allocate(move |_, rect| {
            window_instance.state.borrow_mut().on_size_allocate(rect);
        }); 
    }
}

struct LabyrinthState {
    width : i32,
    height : i32, 
} 

impl LabyrinthState { 
    fn on_size_allocate(&mut self, rect: &gtk::Rectangle) {
        self.width = rect.width as i32;
        self.height = rect.height as i32;
    }
}

fn main() {
    let game = LabyrinthGame::new();
    if let Err(ref e) = game {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let _ = writeln!(stderr, "Error: {}", e);
        let mut fail: &failure::Fail = e.cause(); 
        while let Some(cause) = fail.cause() {
            let _ = writeln!(stderr, "Caused by: {}", cause);
            fail = cause;
        }
    }
}
