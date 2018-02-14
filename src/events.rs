use cairo;
use gtk;
use gdk;
use std;
use labyrinth;

pub struct LabyrinthState {
    width: i32,
    height: i32,
    labyrinth : std::option::Option<labyrinth::Labyrinth>
}

impl LabyrinthState {
    pub fn new(width : i32, height : i32) -> LabyrinthState {
        LabyrinthState {
            width: width,
            height: height,
            labyrinth : None, 
        }
    }   
};

pub struct EventHandler;

impl LabyrinthState {
    fn on_size_allocate(&mut self, rect: &gtk::Rectangle) {
        self.width = rect.width as i32;
        self.height = rect.height as i32;
        self.labyrinth = Some(labyrinth::Labyrinth::new(self.width, self.height));
    }
    fn on_draw(&mut self, cairo_context: &cairo::Context) {
        cairo_context.save();
        cairo_context.restore();
    }
    fn on_button_press(&mut self, _event: &gdk::EventButton) {
    
    }

    pub fn on_motion_notify(&mut self, _event: &gdk::EventMotion) {
        //use gtk::WidgetExt;
        ////let position = event.get_position();
        //self.drawing_area.queue_draw();
    }

    fn print_labyrinth(&mut self, _cairo_context: &cairo::Context) {

    }
}
