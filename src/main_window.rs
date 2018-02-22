use std;
use gtk;
use gdk;
use failure;

use basic_types;

#[derive(Debug)]
pub struct LabyrinthMainWindow {
    pub window: gtk::Window,
    pub drawing_area: std::rc::Rc<gtk::DrawingArea>,
    pub requested_size: (u32, u32),
}

impl LabyrinthMainWindow {
    pub fn new(screen: &gdk::Screen) -> Result<LabyrinthMainWindow, failure::Error> {
        use gtk::prelude::*;
        use gdk::ScreenExt;
        let event_mask : i32 = (gdk::EventMask::POINTER_MOTION_MASK.bits() |
                                gdk::EventMask::POINTER_MOTION_HINT_MASK.bits() |
                                gdk::EventMask::BUTTON_PRESS_MASK.bits()) as i32;
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        let monitor = screen.get_primary_monitor();
        let monitor_workarea = screen.get_monitor_workarea(monitor);
        window.fullscreen_on_monitor(screen, monitor);
        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_size_request(monitor_workarea.width, monitor_workarea.height);
        window.add(&drawing_area);
        drawing_area.set_can_default(true);
        drawing_area.grab_default();
        drawing_area.add_events(event_mask);
        let requested_width = basic_types::convert(monitor_workarea.width)?; 
        let requested_height = basic_types::convert(monitor_workarea.height)?;  
        Ok(LabyrinthMainWindow {
            window: window,
            drawing_area: std::rc::Rc::new(drawing_area),
            requested_size: ( requested_width, requested_height ),
        })
    }
}
