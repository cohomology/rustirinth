use cairo;
use gtk;
use gdk;
use std;
use labyrinth;

pub struct LabyrinthState {
    pub drawing_area: gtk::DrawingArea, 
    width: i32,
    height: i32,
    labyrinth : std::option::Option<Labyrinth>
}

impl LabyrinthState {
    pub fn new(width : i32, height : i32, drawing_area : gtk::DrawingArea) -> LabyrinthState {
        LabyrinthState {
            width: width,
            height: height,
            drawing_area: drawing_area,
            labyrinth : None, 
        }
    }
    pub fn on_size_allocate(&mut self, rect: &gtk::Rectangle) {
        self.width = rect.width as i32;
        self.height = rect.height as i32;
        self.labyrinth = Labyrinth::new(self.width, self.height);
    }
    pub fn on_draw(&mut self, cairo_context: &cairo::Context) {
        cairo_context.save();
        cairo_context.restore();
    }
    pub fn on_button_press(&mut self, _event: &gdk::EventButton) {
    
    }

    pub fn on_motion_notify(&mut self, event: &gdk::EventMotion) {
        use gtk::WidgetExt;
        let position = event.get_position();
        self.drawing_area.queue_draw();
    }

    fn print_labyrinth(&mut self, cairo_context: &cairo::Context) {

    }

    // fn get_mouse_position_text_dimensions(&mut self, cairo_context : &cairo::Context) -> (f64, f64) {
    //     if self.mouse_pos_text_position == (0.0, 0.0) {
    //         static RIGHT_MARGIN : f64 = 10.0;
    //         static BOTTOM_MARGIN : f64 = 2.0;
    //         let max_position = format!("{} : {}", self.width, self.height); 
    //         let extends = cairo_context.text_extents(&max_position);
    //         self.mouse_pos_text_position = (self.width as f64 - extends.width - RIGHT_MARGIN, 
    //                                         self.height as f64 - extends.height - BOTTOM_MARGIN);
    //     }
    //     self.mouse_pos_text_position
    // }
}
