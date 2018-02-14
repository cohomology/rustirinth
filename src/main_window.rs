use std;
use gtk;
use gdk;

use state;

pub struct LabyrinthMainWindow {
    pub window: gtk::Window,
    pub state: std::cell::RefCell<state::LabyrinthState>,
}

impl LabyrinthMainWindow {
    pub fn new(screen: &gdk::Screen) -> LabyrinthMainWindow {
        use gtk::prelude::*;
        use gdk::ScreenExt;
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        let monitor = screen.get_primary_monitor();
        let monitor_workarea = screen.get_monitor_workarea(monitor);
        window.fullscreen_on_monitor(screen, monitor);
        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_size_request(monitor_workarea.width, monitor_workarea.height);
        window.add(&drawing_area);
        LabyrinthMainWindow {
            window: window,
            state: std::cell::RefCell::new(state::LabyrinthState {
                width: monitor_workarea.width,
                height: monitor_workarea.height,
                drawing_area: drawing_area,
            }),
        }
    }
}
