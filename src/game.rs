use gtk;
use gdk;
use std;

use failure;
use main_window;
use event_handler;
use state;

use gtk::WidgetExt;

#[derive(Debug, Fail)]
pub enum LabyrinthError {
    #[fail(display = "Could not get default screen")]
    CouldNotGetDefaultScreen,
}

pub struct LabyrinthGame {
    main_window: main_window::LabyrinthMainWindow,
    state: std::rc::Rc<std::cell::RefCell<state::LabyrinthState>>,
}

impl LabyrinthGame {
    pub fn run() -> Result<(), failure::Error> {
        gtk::init()?;
        let _ = LabyrinthGame::initialize_screen()?;
        gtk::main();
        Ok(())
    }
    fn initialize_screen() -> Result<LabyrinthGame, failure::Error> {
        match gdk::Screen::get_default() {
            Some(screen) => Ok(LabyrinthGame::initialize_window(&screen)),
            None => Err(LabyrinthError::CouldNotGetDefaultScreen.into()),
        }
    }
    fn initialize_window(screen: &gdk::Screen) -> LabyrinthGame {
        let main_window = main_window::LabyrinthMainWindow::new(screen);
        let requested_size = main_window.requested_size;
        LabyrinthGame {
            main_window: main_window,
            state: std::rc::Rc::new(std::cell::RefCell::new(state::LabyrinthState::new(
                requested_size,
            ))),
        }.connect_delete_event()
            .connect_key_press_event()
            .connect_button_press_event()
            .connect_motion_notify_event()
            .connect_on_size_allocate_event()
            .connect_on_draw_event()
            .show_all()
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
        self.main_window
            .window
            .connect_button_press_event(move |_, event| {
                let mut borrow = state.borrow_mut();
                event_handler::on_button_press(&mut *borrow, event);
                gtk::Inhibit(true)
            });
        self
    }
    fn connect_on_size_allocate_event(self) -> Self {
        let state = self.state.clone();
        self.main_window
            .drawing_area
            .connect_size_allocate(move |_, rect| {
                let mut borrow = state.borrow_mut();
                event_handler::on_size_allocate(&mut *borrow, rect);
            });
        self
    }
    fn connect_on_draw_event(self) -> Self {
        let state = self.state.clone();
        self.main_window
            .drawing_area
            .connect_draw(move |_, cairo_context| {
                let mut borrow = state.borrow_mut();
                event_handler::on_draw(&mut *borrow, cairo_context);
                gtk::Inhibit(true)
            });
        self
    }
    fn connect_motion_notify_event(self) -> Self {
        let state = self.state.clone();
        self.main_window
            .drawing_area
            .connect_motion_notify_event(move |_, event| {
                let mut borrow = state.borrow_mut();
                event_handler::on_motion_notify(&mut *borrow, event);
                gtk::Inhibit(true)
            });
        self
    }
    fn show_all(self) -> Self {
        self.main_window.window.show_all();
        self
    }
}
