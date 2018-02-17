use cairo;
use gtk;
use gdk;
use failure;
use ndarray;

use basic_types;
use labyrinth;

use basic_types::IsARectangle;

#[derive(Debug)]
struct RepaintInfo {
    rectangle: basic_types::Rectangle,
    color: (f64, f64, f64),
}

#[derive(Debug)]
pub struct EventHandler {
    to_be_painted: RepaintInfo,
}

const COLOR_BLACK: (f64, f64, f64) = (0.0, 0.0, 0.0);
const INITIAL_RECTANGLE: basic_types::Rectangle = basic_types::Rectangle {
    x: 0,
    y: 0,
    width: 0,
    height: 0,
};
const INITIAL_REPAINT_INFO: RepaintInfo = RepaintInfo {
    rectangle: INITIAL_RECTANGLE,
    color: COLOR_BLACK,
};

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            to_be_painted: INITIAL_REPAINT_INFO,
        }
    }
    pub fn on_size_allocate(
        &mut self,
        state: &mut labyrinth::LabyrinthState,
        rect: &basic_types::Rectangle,
    ) {
        if rect.width > 0 && rect.height > 0 {
            state.width = rect.width as u32;
            state.height = rect.height as u32;
            state.labyrinth = Some(labyrinth::Labyrinth::new(
                state.box_size,
                state.width,
                state.height,
            ));
        } else {
            state.labyrinth = None;
            state.width = 0;
            state.height = 0;
        }
    }
    pub fn on_draw(
        &mut self,
        state: &mut labyrinth::LabyrinthState,
        cairo_context: &cairo::Context,
    ) -> Result<(), failure::Error> {
        if let Some(labyrinth) = state.labyrinth.as_mut() {
            self.draw(labyrinth, cairo_context)
        } else {
            Ok(())
        }
    }
    pub fn on_button_press(
        &mut self,
        drawing_area: &gtk::DrawingArea,
        state: &mut labyrinth::LabyrinthState,
        event: &gdk::EventButton,
    ) {
        if let Some(ref mut labyrinth) = state.labyrinth {
            self.handle_mark_box(drawing_area, labyrinth, event.get_position());
        }
    }
    pub fn on_motion_notify(
        &mut self,
        drawing_area: &gtk::DrawingArea,
        state: &mut labyrinth::LabyrinthState,
        event: &gdk::EventMotion,
    ) {
        if let Some(ref mut labyrinth) = state.labyrinth {
            if event.get_state() & gdk::ModifierType::BUTTON1_MASK != gdk::ModifierType::empty() {
                self.handle_mark_box(drawing_area, labyrinth, event.get_position());
            }
        }
    }
    fn draw(&mut self, labyrinth: &mut labyrinth::Labyrinth, cairo_context: &cairo::Context) -> Result<(), failure::Error> {
        let extents = cairo_context.clip_extents();
        let rectangle = basic_types::Rectangle::approx_from(&extents)?;
        if rectangle == self.to_be_painted.rectangle {
            self.repaint_box(cairo_context)?;
            self.to_be_painted = INITIAL_REPAINT_INFO;
        } else {
            self.trigger_complete_redraw(labyrinth, cairo_context);
        }
        Ok(())
    }
    fn trigger_complete_redraw(
        &mut self,
        labyrinth: &mut labyrinth::Labyrinth,
        cairo_context: &cairo::Context,
    ) {
        cairo_context.reset_clip();
        self.clear_surface(cairo_context);
        self.draw_axes(labyrinth, cairo_context);
        self.draw_marked_boxes(labyrinth, cairo_context);
    }
    fn clear_surface(&self, cairo_context: &cairo::Context) {
        cairo_context.save();
        cairo_context.set_source_rgb(255.0, 255.0, 255.0);
        cairo_context.restore();
    }
    fn draw_axes(&self, labyrinth: &labyrinth::Labyrinth, cairo_context: &cairo::Context) {
        cairo_context.save();
        cairo_context.set_source_rgb(0.0, 0.0, 0.0);
        for x_cnt in 0..(labyrinth.x_box_cnt + 1) {
            let x_pos = labyrinth.x + labyrinth.box_size * x_cnt;
            cairo_context.move_to(f64::from(x_pos), f64::from(labyrinth.y));
            cairo_context.line_to(f64::from(x_pos), f64::from(labyrinth.y + labyrinth.height));
        }
        for y_cnt in 0..(labyrinth.y_box_cnt + 1) {
            let y_pos = labyrinth.y + labyrinth.box_size * y_cnt;
            cairo_context.move_to(f64::from(labyrinth.x), f64::from(y_pos));
            cairo_context.line_to(f64::from(labyrinth.x + labyrinth.width), f64::from(y_pos));
        }
        cairo_context.stroke();
    }
    fn draw_marked_boxes(
        &mut self,
        labyrinth: &mut labyrinth::Labyrinth,
        cairo_context: &cairo::Context,
    ) {
        cairo_context.save();
        for (index, _) in labyrinth.marked.indexed_iter().filter(|&(_, &elem)| elem) {
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
        cairo_context.restore();
    }
    fn repaint_box(&self, cairo_context: &cairo::Context) -> Result<(), failure::Error> {
        let color = self.to_be_painted.color;
        let rectangle = self.to_be_painted.rectangle;
        let float_rectangle : basic_types::GeneralRectangle<f64> = rectangle.to()?;
        cairo_context.save();
        cairo_context.set_source_rgb(color.0, color.1, color.2);
        cairo_context.rectangle(float_rectangle.x(), float_rectangle.y(), float_rectangle.width(), 
                                float_rectangle.height());
        cairo_context.fill();
        cairo_context.restore();
        Ok(())
    }
    fn handle_mark_box(
        &mut self,
        drawing_area: &gtk::DrawingArea,
        labyrinth: &mut labyrinth::Labyrinth,
        position: (f64, f64),
    ) {
        use gtk::WidgetExt;
        let clicked_box = labyrinth.pixel_to_box((position.0 as u32, position.1 as u32));
        if let Some(clicked_box) = clicked_box {
            if self.update_marked(labyrinth, clicked_box) {
                let box_rectangle = labyrinth.box_to_pixel(clicked_box).unwrap();
                self.to_be_painted = RepaintInfo {
                    rectangle: box_rectangle,
                    color: COLOR_BLACK,
                };
                drawing_area.queue_draw_area(
                    box_rectangle.x as i32,
                    box_rectangle.y as i32,
                    box_rectangle.width as i32,
                    box_rectangle.height as i32,
                );
            }
        }
    }
    fn update_marked(
        &mut self,
        labyrinth: &mut labyrinth::Labyrinth,
        clicked_box: (u32, u32),
    ) -> bool {
        if let Some(marked) = labyrinth.marked.get_mut(ndarray::IxDyn(&[
            clicked_box.0 as usize,
            clicked_box.1 as usize,
        ])) {
            if !*marked {
                *marked = true;
                return true;
            }
        }
        false
    }
}
