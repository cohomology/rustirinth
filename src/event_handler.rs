use cairo;
use gtk;
use gdk;
use labyrinth;
use state;

pub fn on_size_allocate(state: &mut state::LabyrinthState, rect: &gtk::Rectangle) {
    state.width = rect.width as i32;
    state.height = rect.height as i32;
    state.labyrinth = Some(labyrinth::Labyrinth::new(state.width, state.height));
}
pub fn on_draw(_state: &mut state::LabyrinthState, cairo_context: &cairo::Context) {
    cairo_context.save();
    cairo_context.restore();
}
pub fn on_button_press(_state: &mut state::LabyrinthState, _event: &gdk::EventButton) {}
pub fn on_motion_notify(_state: &mut state::LabyrinthState, _event: &gdk::EventMotion) {}
