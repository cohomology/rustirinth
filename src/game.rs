use std;
use gtk;
use gdk;

use std::rc::Rc;
use std::cell::RefCell;
use gtk::WidgetExt;

use event_handler::EventHandler;
use labyrinth::LabyrinthState;
use main_window::LabyrinthMainWindow;
use failure::{Error, Fail};
use basic_types::{LabyrinthError, Rectangle};

#[derive(Debug)]
pub struct LabyrinthGame {
    main_window: LabyrinthMainWindow,
    event_handler: Rc<RefCell<EventHandler>>,
    state: Rc<RefCell<LabyrinthState>>,
}

impl LabyrinthGame {
    pub fn run(box_size: u32) -> Result<(), Error> {
        gtk::init()?;
        let _ = LabyrinthGame::initialize_screen(box_size)?;
        gtk::main();
        Ok(())
    }
    pub fn fatal_error(error: &Error) {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let _ = writeln!(stderr, "Error: {}", error);
        let mut fail: &Fail = error.cause();
        while let Some(cause) = fail.cause() {
            let _ = writeln!(stderr, "Caused by: {}", cause);
            fail = cause;
        }
        std::process::exit(-1);
    }
    fn initialize_screen(box_size: u32) -> Result<LabyrinthGame, Error> {
        match gdk::Screen::get_default() {
            Some(screen) => LabyrinthGame::initialize_window(box_size, &screen),
            None => Err(LabyrinthError::CouldNotGetDefaultScreen.into()),
        }
    }
    fn initialize_window(box_size: u32, screen: &gdk::Screen) -> Result<LabyrinthGame, Error> {
        let main_window = LabyrinthMainWindow::new(screen)?;
        Ok(LabyrinthGame {
            main_window,
            event_handler: Rc::new(RefCell::new(EventHandler::new())),
            state: Rc::new(RefCell::new(LabyrinthState::new(box_size))),
        }.connect_delete_event()
            .connect_key_press_event()
            .connect_button_press_event()
            .connect_motion_notify_event()
            .connect_on_size_allocate_event()
            .connect_on_draw_event()
            .show_all())
    }
    fn connect_delete_event(self) -> Self {
        self.main_window.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(true)
        });
        self
    }
    fn connect_key_press_event(self) -> Self {
        self.main_window.window.connect_key_press_event(|_, key| {
            if key.get_keyval() == gdk::enums::key::Escape {
                gtk::main_quit();
            }
            gtk::Inhibit(true)
        });
        self
    }
    fn connect_button_press_event(self) -> Self {
        let state = self.state.clone();
        let event_handler = self.event_handler.clone();
        self.main_window
            .drawing_area
            .connect_button_press_event(move |drawing_area, event| {
                let mut borrowed_state = state.borrow_mut();
                event_handler
                    .borrow_mut()
                    .on_button_press(drawing_area, &mut *borrowed_state, event)
                    .unwrap_or_else(|e| LabyrinthGame::fatal_error(&e));
                gtk::Inhibit(true)
            });
        self
    }
    fn connect_on_size_allocate_event(self) -> Self {
        let state = self.state.clone();
        let event_handler = self.event_handler.clone();
        self.main_window
            .drawing_area
            .connect_size_allocate(move |_, rect| {
                let rectangle = Rectangle::from(rect).unwrap_or_else(|e| {
                    LabyrinthGame::fatal_error(&e);
                    Rectangle::default()
                });
                let mut borrowed_state = state.borrow_mut();
                event_handler
                    .borrow_mut()
                    .on_size_allocate(&mut *borrowed_state, &rectangle)
                    .unwrap_or_else(|e| LabyrinthGame::fatal_error(&e));
            });
        self
    }
    fn connect_on_draw_event(self) -> Self {
        let event_handler = self.event_handler.clone();
        let state = self.state.clone();
        self.main_window
            .drawing_area
            .connect_draw(move |_, cairo_context| {
                let mut borrowed_state = state.borrow_mut();
                event_handler
                    .borrow_mut()
                    .on_draw(&mut *borrowed_state, cairo_context)
                    .unwrap_or_else(|e| LabyrinthGame::fatal_error(&e));
                gtk::Inhibit(true)
            });
        self
    }
    fn connect_motion_notify_event(self) -> Self {
        let event_handler = self.event_handler.clone();
        let state = self.state.clone();
        self.main_window
            .drawing_area
            .connect_motion_notify_event(move |drawing_area, event| {
                let mut borrowed_state = state.borrow_mut();
                event_handler
                    .borrow_mut()
                    .on_motion_notify(drawing_area, &mut *borrowed_state, event)
                    .unwrap_or_else(|e| LabyrinthGame::fatal_error(&e));
                gtk::Inhibit(true)
            });
        self
    }
    fn show_all(self) -> Self {
        self.main_window.window.show_all();
        self
    }
}
