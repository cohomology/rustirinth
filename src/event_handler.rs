use cairo;
use gtk;
use gdk;
use std;
use labyrinth;

pub struct EventHandler;

impl EventHandler {
    fn on_size_allocate(&mut self, state : &mut LabyrinthState, rect: &gtk::Rectangle) {
        state.width = rect.width as i32;
        state.height = rect.height as i32;
        state.labyrinth = Some(labyrinth::Labyrinth::new(self.width, self.height));
    }
    fn on_draw(&mut self, _state : &mut LabyrinthState, cairo_context: &cairo::Context) {
        cairo_context.save();
        cairo_context.restore();
    }
    fn on_button_press(&mut self, _state : &mut LabyrinthState, _event: &gdk::EventButton) {

    }
    pub fn on_motion_notify(&mut self, _state : &mut LabyrinthState, _event: &gdk::EventMotion) {

    }
}

