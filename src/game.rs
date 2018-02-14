use gtk;
use gdk;
use std;

use failure;
use main_window;

#[derive(Debug, Fail)]
pub enum LabyrinthError {
    #[fail(display = "Could not get default screen")]
    CouldNotGetDefaultScreen,
}

pub struct LabyrinthGame {
    main_window: std::rc::Rc<main_window::LabyrinthMainWindow>,
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
        LabyrinthGame {
            main_window: std::rc::Rc::new(main_window::LabyrinthMainWindow::new(screen)),
        }.connect_delete_event()
            .connect_key_press_event()
            .connect_button_press_event()
            .connect_on_size_allocate_event()
            .connect_on_draw_event()
            .show_all()
    }
    fn connect_delete_event(self) -> Self {
        gtk::WidgetExt::connect_delete_event(&self.main_window.window, |_, _| {
            gtk::main_quit();
            gtk::Inhibit(true)
        });
        self
    }
    fn connect_key_press_event(self) -> Self {
        gtk::WidgetExt::connect_key_press_event(&self.main_window.window, |_, key| {
            if key.get_keyval() == gdk::enums::key::Escape {
                gtk::main_quit();
            }
            gtk::Inhibit(true)
        });
        self
    }
    fn connect_button_press_event(self) -> Self {
        let window_instance = self.main_window.clone();
        gtk::WidgetExt::connect_button_press_event(&self.main_window.window, move |_, event| {
            window_instance.state.borrow_mut().on_button_press(event);
            gtk::Inhibit(true)
        });
        self
    }
    fn connect_on_size_allocate_event(self) -> Self {
        let window_instance = self.main_window.clone();
        gtk::WidgetExt::connect_size_allocate(
            &self.main_window.state.borrow_mut().drawing_area,
            move |_, rect| {
                window_instance.state.borrow_mut().on_size_allocate(rect);
            },
        );
        self
    }
    fn connect_on_draw_event(self) -> Self {
        let window_instance = self.main_window.clone();
        gtk::WidgetExt::connect_draw(
            &self.main_window.state.borrow_mut().drawing_area,
            move |_, cairo_context| {
                window_instance.state.borrow_mut().on_draw(cairo_context);
                gtk::Inhibit(true)
            },
        );
        self
    }
    fn show_all(self) -> Self {
        gtk::WidgetExt::show_all(&self.main_window.window);
        self
    }
}
