use gtk;
use gdk;

#[derive(Debug)]
pub struct LabyrinthMainWindow {
    pub window: gtk::Window,
    pub drawing_area: gtk::DrawingArea,
    pub requested_size: (i32, i32),
}

impl LabyrinthMainWindow {
    pub fn new(screen: &gdk::Screen) -> LabyrinthMainWindow {
        use gtk::prelude::*;
        use gdk::ScreenExt;
        lazy_static! {
            static ref EVENT_MASK : i32 = (gdk::EventMask::POINTER_MOTION_MASK.bits() |
                                           gdk::EventMask::POINTER_MOTION_HINT_MASK.bits() |
                                           gdk::EventMask::BUTTON_PRESS_MASK.bits()) as i32;
        }
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        let monitor = screen.get_primary_monitor();
        let monitor_workarea = screen.get_monitor_workarea(monitor);
        window.fullscreen_on_monitor(screen, monitor);
        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_size_request(monitor_workarea.width, monitor_workarea.height);
        window.add(&drawing_area);
        drawing_area.set_can_default(true);
        drawing_area.grab_default();
        drawing_area.add_events(*EVENT_MASK);
        LabyrinthMainWindow {
            window: window,
            drawing_area: drawing_area,
            requested_size: (monitor_workarea.width, monitor_workarea.height),
        }
    }
}
