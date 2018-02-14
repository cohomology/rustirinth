struct LabyrinthState {
    width: i32,
    height: i32,
}

impl LabyrinthState {
    fn on_size_allocate(&mut self, rect: &gtk::Rectangle) {
        self.width = rect.width as i32;
        self.height = rect.height as i32;
    }
    fn on_draw(&mut self, cairo_context: &cairo::Context) {
        cairo_context.save();
        cairo_context.restore();
    }
    fn on_button_press(&mut self, _event: &gdk::EventButton) {}
}   
