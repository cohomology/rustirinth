use cairo;
use gtk;
use gdk;

pub struct LabyrinthState {
    pub drawing_area: gtk::DrawingArea, 

    width: i32,
    height: i32,
    mouse_pos : (u32, u32),
    mouse_pos_text_position : (f64, f64)
}

impl LabyrinthState {
    pub fn new(width : i32, height : i32, drawing_area : gtk::DrawingArea) -> LabyrinthState {
        LabyrinthState {
            width: width,
            height: height,
            drawing_area: drawing_area, 
            mouse_pos : (0,0),
            mouse_pos_text_position : (0.0, 0.0)
        }
    }
    pub fn on_size_allocate(&mut self, rect: &gtk::Rectangle) {
        self.width = rect.width as i32;
        self.height = rect.height as i32;
        self.mouse_pos_text_position = (0.0, 0.0);
    }
    pub fn on_draw(&mut self, cairo_context: &cairo::Context) {
        cairo_context.save();
        self.print_mouse_position(cairo_context);
        cairo_context.restore();
    }
    pub fn on_button_press(&mut self, _event: &gdk::EventButton) {
    
    }

    pub fn on_motion_notify(&mut self, event: &gdk::EventMotion) {
        use gtk::WidgetExt;
        let position = event.get_position();
        self.mouse_pos = ( position.0 as u32, position.1 as u32 );
        self.drawing_area.queue_draw();
    }

    fn print_mouse_position(&mut self, cairo_context: &cairo::Context) {
        cairo_context.set_font_size(18 as f64); 
        let dimensions = self.get_mouse_position_text_dimensions(cairo_context);
        let position_text = format!("{} : {}", self.mouse_pos.0, self.mouse_pos.1);
        cairo_context.move_to(dimensions.0, dimensions.1);
        cairo_context.show_text(&position_text);
    }

    fn get_mouse_position_text_dimensions(&mut self, cairo_context : &cairo::Context) -> (f64, f64) {
        if self.mouse_pos_text_position == (0.0, 0.0) {
            static RIGHT_MARGIN : f64 = 10.0;
            static BOTTOM_MARGIN : f64 = 2.0;
            let max_position = format!("{} : {}", self.width, self.height); 
            let extends = cairo_context.text_extents(&max_position);
            self.mouse_pos_text_position = (self.width as f64 - extends.width - RIGHT_MARGIN, 
                                            self.height as f64 - extends.height - BOTTOM_MARGIN);
        }
        self.mouse_pos_text_position
    }

}
