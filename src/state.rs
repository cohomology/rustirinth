use cairo;
use gtk;
use gdk;

pub struct LabyrinthState {
    pub width: i32,
    pub height: i32,
    pub drawing_area: gtk::DrawingArea,
}

impl LabyrinthState {
    pub fn on_size_allocate(&mut self, rect: &gtk::Rectangle) {
        self.width = rect.width as i32;
        self.height = rect.height as i32;
    }
    pub fn on_draw(&mut self, cairo_context: &cairo::Context) {
        cairo_context.save();
        cairo_context.restore();
    }
    pub fn on_button_press(&mut self, _event: &gdk::EventButton) {}
}
