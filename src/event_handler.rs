use cairo;
use gtk;
use gdk;
use labyrinth;
use state;
use ndarray;

#[derive(Debug)]
pub struct EventHandler;

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {}
    }
    pub fn on_size_allocate(&mut self, state: &mut state::LabyrinthState, rect: &gtk::Rectangle) {
        if rect.width > 0 && rect.height > 0 {
            state.width = rect.width as u32;
            state.height = rect.height as u32;
            state.labyrinth = Some(labyrinth::Labyrinth::new(state.width, state.height));
        }
    }
    pub fn on_draw(&mut self, state: &mut state::LabyrinthState, cairo_context: &cairo::Context) {
        cairo_context.save();
        if let Some(labyrinth) = state.labyrinth.as_mut() {
            self.print_labyrinth(labyrinth, cairo_context);
        }
        cairo_context.restore();
    }
    pub fn on_button_press(
        &mut self,
        state: &mut state::LabyrinthState,
        event: &gdk::EventButton,
    ) -> bool {
        if let Some(ref mut labyrinth) = state.labyrinth {
            return self.handle_mark_box(labyrinth, event.get_position());
        }
        false
    }
    pub fn on_motion_notify(
        &mut self,
        state: &mut state::LabyrinthState,
        event: &gdk::EventMotion,
    ) -> bool {
        if let Some(ref mut labyrinth) = state.labyrinth {
            if event.get_state() & gdk::ModifierType::BUTTON1_MASK != gdk::ModifierType::empty() {
                return self.handle_mark_box(labyrinth, event.get_position());
            }
        }
        false
    }
    fn print_labyrinth(&self, labyrinth: &labyrinth::Labyrinth, cairo_context: &cairo::Context) {
        cairo_context.set_source_rgb(0.0, 0.0, 0.0);
        for x_cnt in 0..(labyrinth.x_box_cnt + 1) {
            let x_pos = labyrinth.x + labyrinth::BOX_SIZE * x_cnt;
            cairo_context.move_to(f64::from(x_pos), f64::from(labyrinth.y));
            cairo_context.line_to(f64::from(x_pos), f64::from(labyrinth.y + labyrinth.height));
        }
        for y_cnt in 0..(labyrinth.y_box_cnt + 1) {
            let y_pos = labyrinth.y + labyrinth::BOX_SIZE * y_cnt;
            cairo_context.move_to(f64::from(labyrinth.x), f64::from(y_pos));
            cairo_context.line_to(f64::from(labyrinth.x + labyrinth.width), f64::from(y_pos));
        }
        cairo_context.stroke();
        for (index, _) in labyrinth.marked.indexed_iter().filter(|&(_, elem)| *elem) {
            let (x_box, y_box) = (index[0] as u32, index[1] as u32);
            if let Some(rectangle) = labyrinth.box_to_pixel((x_box, y_box)) {
                cairo_context.rectangle(
                    f64::from(rectangle.x),
                    f64::from(rectangle.y),
                    f64::from(rectangle.width),
                    f64::from(rectangle.height),
                );
                cairo_context.fill();
            }
        }
    }
    fn handle_mark_box(&self, labyrinth: &mut labyrinth::Labyrinth, position: (f64, f64)) -> bool {
        let clicked_box = labyrinth.pixel_to_box((position.0 as u32, position.1 as u32));
        if let Some(clicked_box) = clicked_box {
            if let Some(marked) = labyrinth.marked.get_mut(ndarray::IxDyn(&[
                clicked_box.0 as usize,
                clicked_box.1 as usize,
            ])) {
                if !*marked {
                    *marked = true;
                    return true;
                }
            }
        }
        false
    }
}
